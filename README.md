# quicdrop

[![CI](https://github.com/RazTan1234/quicdrop/actions/workflows/ci.yml/badge.svg)](https://github.com/RazTan1234/quicdrop/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

**Fast, secure, peer-to-peer file transfer over QUIC — built for bad networks.**

> ⚠️ **Early development.** Nothing works yet; see the [roadmap](#roadmap).

```console
$ quicdrop send dataset.tar
Your code is: 7-tiger-ocean

$ quicdrop receive 7-tiger-ocean
✓ Received dataset.tar (12 GB) in 3m10s, verified
```

## Why

TCP-based tools (scp, rsync, HTTP) slow down sharply on links with packet loss
and high latency — Wi-Fi, mobile networks, intercontinental or satellite links.
Commercial tools like IBM Aspera solve this for a lot of money. quicdrop aims to
be an open-source alternative that is:

- **Fast on bad networks** — QUIC transport with tunable congestion control and
  an experimental forward-error-correction mode, backed by reproducible benchmarks.
- **Secure by default** — end-to-end encrypted; peers authenticate each other
  with a short one-time code (PAKE), so not even the relay can read or tamper
  with the data.
- **Resumable and verified** — files are split into BLAKE3-hashed chunks;
  interrupted transfers continue where they stopped.
- **Zero setup** — no accounts; a relay handles peers behind NAT.

## Roadmap

- [x] **Phase 0** — Workspace, CI, licensing
- [ ] **Phase 1** — File transfer over QUIC on a local network
- [ ] **Phase 2** — Chunking, BLAKE3 integrity, resume, parallel streams
- [ ] **Phase 3** — PAKE authentication, threat model, fuzzing
- [ ] **Phase 4** — Relay server (Docker, Prometheus metrics, rate limiting)
- [ ] **Phase 5** — Benchmark harness vs scp / rsync / croc / sendme
- [ ] **Phase 6** — Turbo mode: congestion control tuning, RaptorQ FEC
- [ ] **Phase 7** — Cross-platform releases

## Project layout

| Crate | Purpose |
|---|---|
| [`quicdrop-core`](crates/quicdrop-core) | Transfer protocol, shared by everything else |
| [`quicdrop`](crates/quicdrop-cli) | Command-line tool |
| [`quicdrop-relay`](crates/quicdrop-relay) | Relay server for peers behind NAT |

## Building

```console
$ cargo build --release
$ ./target/release/quicdrop --help
```

Requires Rust 1.85 or newer.

## Security

See [SECURITY.md](SECURITY.md) for how to report vulnerabilities.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project shall be dual licensed as above, without any
additional terms or conditions.
