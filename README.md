# Phoenix 🔥

> **HTTP/2 Stress Testing & Attack Simulation Framework**  
> Built in Rust. Raw frame-level. CVE-accurate.

```
 ██████╗ ██╗  ██╗ ██████╗ ███████╗███╗   ██╗██╗██╗  ██╗
 ██╔══██╗██║  ██║██╔═══██╗██╔════╝████╗  ██║██║╚██╗██╔╝
 ██████╔╝███████║██║   ██║█████╗  ██╔██╗ ██║██║ ╚███╔╝ 
 ██╔═══╝ ██╔══██║██║   ██║██╔══╝  ██║╚██╗██║██║ ██╔██╗ 
 ██║     ██║  ██║╚██████╔╝███████╗██║ ╚████║██║██╔╝ ██╗
 ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝
```

⚠️ **USE ONLY ON SYSTEMS YOU OWN OR HAVE EXPLICIT AUTHORIZATION TO TEST.**

---

## What is Phoenix?

Phoenix is a Rust-based HTTP/2 stress testing and security research framework. Unlike generic load testers, Phoenix operates at the **raw HTTP/2 frame level**, enabling accurate simulation of known CVE attack patterns.

## Attack Modules

| Module | CVE | Description |
|--------|-----|-------------|
| `rapid-reset` | CVE-2023-44487 | HTTP/2 Rapid Reset — 398M rps DDoS record |
| `continuation-flood` | CVE-2024-27983 | CONTINUATION frame flood |
| `hpack-bomb` | — | HPACK compression bomb |
| `settings-flood` | — | SETTINGS frame flood |
| `ping-flood` | — | PING frame flood |
| `load-test` | — | Legitimate HTTP/2 load test |

## Workspace Structure

```
phoenix/
├── phoenix-core/       ← TLS, raw HTTP/2 frames, connection pool
├── phoenix-attacks/    ← All attack modules
├── phoenix-metrics/    ← HDR histogram, ratatui live dashboard
├── phoenix-report/     ← JSON + terminal report generation
└── phoenix-cli/        ← Main binary (phoenix)
```

## Quick Start

```bash
# Build
cargo build --release

# Rapid Reset attack (CVE-2023-44487)
phoenix attack rapid-reset --target https://example.com --connections 10 --duration 30s

# Load test
phoenix attack load-test --target https://example.com --rps 1000 --duration 60s --report report.json

# Scan for vulnerabilities
phoenix scan --target https://example.com
```

## Research

All underlying research is available at: [http2-deep-research](https://github.com/dryfryce/http2-deep-research)

- 16,000+ lines of HTTP/2 protocol research
- Full CVE analysis and attack documentation  
- Architecture and implementation guides

## License

MIT — Research and authorized testing only.
