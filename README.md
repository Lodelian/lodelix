# Lodelix

## About

Lodelix is a Rust-written web application server that allows you to use language modules to run your projects and also
provides a flexible configuration system that makes it modern and convenient.

> This project is still in development. Don't use it in production.

## KEY FEATURE

First web server that provides MCP Server to control it.

## Features

- Configuration via RESTful API, gRPC, AI agents

## Build

### Development

1. `cargo build`
    1. 'cargo build --features grpc' to enable gRPC
2. `./target/debug/lodelix --help`

## Roadmap

- [ ] [Implement REST API](https://github.com/Lodelian/lodelix/wiki/Rest-Implementation)
- [ ] Implement gRPC
- [ ] Implement MCP Server

## Lodelix Operation Methods

### HTTP

Can be run

- as a Unix domain socket for Unix-like systems (Linux and MacOS). Default path is `/tmp/lodelix.sock`
- as a Named Pipe (Windows). Default path is `\\.\pipe\lodelix`
- on a selected port (Suitable for both). The default port is 9898

To specify the path or non-default port, use `--control` flag.

### gRPC

build with `--features grpc`

Use `--grpc` flag to enable gRPC.
Can be run on a selected port. The default port is 50051.

