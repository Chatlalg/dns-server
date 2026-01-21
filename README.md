# dns_server

A small recursive DNS server written in Rust. It listens on UDP `0.0.0.0:2053`, walks the DNS hierarchy starting from the root server at `198.41.0.4`, and returns answers for common record types (`A`, `AAAA`, `NS`, `CNAME`, `MX`).

## Build
- Install Rust (edition 2024 is used).
- From the project root, run:
  - `cargo build`

## Run
- Start the server:
  - `cargo run`
- The process logs incoming questions and the records returned.

## Try it
- From another terminal, ask for a record with `dig`:
  - `dig @127.0.0.1 -p 2053 example.com A`
  - Swap `A` for `AAAA`, `NS`, `CNAME`, or `MX` to try other types.

## Notes
- UDP only; no TCP fallback.
- No caching; each query is resolved from the root.
- Response size is limited to 512 bytes.
- No DNSSEC handling.

