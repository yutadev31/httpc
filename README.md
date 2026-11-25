# httpc

A lightweight HTTP client for the command line, written in Rust.

`httpc` streams response bodies to standard output by default, so you can pipe
them into other commands or save them to a file.

## Installation

### Cargo

```sh
cargo install --git https://github.com/yutadev31/httpc
```

### Nix

```sh
nix profile install github:yutadev31/httpc
```

## Usage

### Send a GET request

```sh
httpc https://example.com/
```

GET is the default method.

### Send a JSON body

```sh
httpc -m POST -H 'Content-Type: application/json' -d '{"message":"Hello"}' https://httpbin.org/post
```

Use `-m` (`--method`) to select the HTTP method, `-d` (`--data`) to pass a
request body, and `-H` (`--header`) once for each request header.

### Show response headers

```sh
httpc -i https://example.com/
```

`-i` (`--include`) writes the HTTP version, status, and response headers to
standard error. The body still goes to standard output unless you specify an
output file.

### Save the response body

```sh
httpc -o index.html https://example.com/
```

`-o` (`--output`) creates or overwrites the specified file.

### Set a timeout and fail on HTTP errors

```sh
httpc --timeout 10 --fail https://example.com/
```

The default timeout is 30 seconds. With `--fail`, HTTP 4xx and 5xx responses
cause exit status 1 before the body is written. By default, those response
bodies are written normally.

Run `httpc --help` for the full list of options.

## Development

Enter the Nix development environment.

```sh
nix develop
```

Run the flake checks before committing.

```sh
nix flake check
```

Run checks individually with Cargo.

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Build the release binary.

```sh
cargo build --release
```

## License

Licensed under the [MIT License](LICENSE.txt).
