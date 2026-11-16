use std::{
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

use anyhow::Context;
use clap::Parser;
use reqwest::{
    Method,
    blocking::Client,
    header::{HeaderName, HeaderValue},
};

#[derive(Parser)]
struct Cli {
    /// Request URL
    url: String,

    /// HTTP method
    #[arg(short = 'm', long, default_value = "GET")]
    method: Method,

    /// Request header (NAME: VALUE)
    #[arg(short = 'H', long)]
    header: Vec<String>,

    /// Request body
    #[arg(short = 'd', long)]
    data: Option<String>,

    /// Include response headers
    #[arg(short = 'i', long)]
    include: bool,

    /// Output file
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// Request timeout in seconds
    #[arg(long, default_value_t = 30)]
    timeout: u64,

    /// Fail on HTTP 4xx/5xx responses
    #[arg(long)]
    fail: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("httpc: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = Client::builder()
        .timeout(Duration::from_secs(cli.timeout))
        .build()?;

    let mut request = client.request(cli.method, &cli.url);

    for header in &cli.header {
        let (name, value) = header
            .split_once(':')
            .context("invalid header format, expected NAME: VALUE")?;

        let name = HeaderName::from_bytes(name.trim().as_bytes())?;
        let value = HeaderValue::from_str(value.trim())?;

        request = request.header(name, value);
    }

    if let Some(data) = cli.data {
        request = request.body(data);
    }

    let mut response = request.send()?;

    let status = response.status();

    if cli.include {
        eprintln!("HTTP/1.1 {status}");

        for (name, value) in response.headers() {
            eprintln!("{name}: {}", value.to_str()?);
        }

        eprintln!();
    }

    if cli.fail && !status.is_success() {
        anyhow::bail!("HTTP request failed: {status}");
    }

    let mut output: Box<dyn Write> = match cli.output {
        Some(path) => Box::new(std::fs::File::create(path)?),
        None => Box::new(io::stdout().lock()),
    };

    io::copy(&mut response, &mut output)?;

    Ok(())
}
