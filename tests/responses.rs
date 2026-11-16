use std::{
    io::{Read, Write},
    net::TcpListener,
    process::{Command, Output},
    thread,
};

fn request(response: &'static [u8], arguments: &[&str]) -> Output {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = Vec::new();

        while !request.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            request.push(byte[0]);
        }

        stream.write_all(response).unwrap();
    });

    let output = Command::new(env!("CARGO_BIN_EXE_httpc"))
        .arg(format!("http://{address}"))
        .args(arguments)
        .env("NO_PROXY", "*")
        .output()
        .unwrap();

    server.join().unwrap();

    output
}

#[test]
fn fail_only_rejects_client_and_server_errors() {
    for (response, success) in [
        (
            b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".as_slice(),
            true,
        ),
        (b"HTTP/1.1 304 Not Modified\r\n\r\n", true),
        (
            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n",
            false,
        ),
        (
            b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n",
            false,
        ),
    ] {
        assert_eq!(request(response, &["--fail"]).status.success(), success);
    }
}

#[test]
fn include_preserves_body_with_non_ascii_header() {
    let output = request(
        b"HTTP/1.1 200 OK\r\nX-Name: caf\xe9\r\nContent-Length: 2\r\n\r\nOK",
        &["-i"],
    );

    assert!(output.status.success());
    assert_eq!(output.stdout, b"OK");
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("x-name: caf\\xe9")
    );
}

#[test]
fn include_displays_actual_protocol_version() {
    for response in [
        b"HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nOK".as_slice(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK",
    ] {
        let output = request(response, &["-i"]);

        assert!(output.status.success());
        assert_eq!(output.stdout, b"OK");
        assert!(output.stderr.starts_with(&response[..15]));
    }
}
