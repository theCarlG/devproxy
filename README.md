<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🌐 devproxy`

A local development proxy for testing different network conditions

[![Crates.io](https://img.shields.io/crates/v/devproxy.svg)](https://crates.io/crates/devproxy)
[![Docs](https://docs.rs/devproxy/badge.svg)](https://docs.rs/devproxy)
[![Build Status](https://github.com/theCarlG/devproxy/workflows/CI/badge.svg)](https://github.com/theCarlG/devproxy/actions?workflow=CI)
</div>

## Install

```shell
$ cargo install devproxy
```

## Building

```shell
$ git clone https://github.com/theCarlG/devproxy
$ cd devproxy
$ cargo build --release
$ ./target/release/devproxy --version
0.1.1
```

## How to use devproxy

#### Basic usage

This is the basic way to use devproxy. 
```shell
#          LISTEN ADDR    ENDPOINT ADDR
$ devproxy 127.0.0.1:8081 127.0.0.1:8080
```

#### Limit speed

Limit the connection speed to 1000KiB/s
```shell
$ devproxy --speed 1000 127.0.0.1:8081 127.0.0.1:8080
```

## Options

```
A local development proxy for testing different network conditions

Usage: devproxy [OPTIONS] <LISTEN> <ENDPOINT>

Arguments:
  <LISTEN>    [env: LISTEN=]
  <ENDPOINT>  [env: ENDPOINT=]

Options:
  -l, --latency <LATENCY>
          Latency in ms [env: LATENCY=]
  -s, --speed <SPEED>
          The network speed in Kib/s [env: SPEED=] [default: 10000]
  -c, --connect-failure-rate <CONNECT_FAILURE_RATE>
          Connect failure rate [env: CONNECT_FAILURE_RATE=]
  -t, --transfer-failure-rate <TRANSFER_FAILURE_RATE>
          Data transfer failure rate [env: TRANSFER_FAILURE_RATE=]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in devproxy, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to CarlG or any other licensee/user of the contribution.
