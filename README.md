Network-Recon
application capabilities
    - mass lan scanning
    - port scanning
    - vuln assessment
    - bandwidth limiting
    - arp spoofing
    - dns spoofing

A high-performance, asynchronous LAN discovery and port scanning utility built in Rust. This tool leverages non-blocking I/O to achieve maximum throughput while maintaining memory safety.
 Features

    ARP-based Host Discovery: Rapidly identify active devices on the local subnet.

    Asynchronous TCP/UDP Scanning: Powered by tokio for concurrent port probing.

    Service Version Detection: Basic fingerprinting of common services.

    JSON/CSV Export: Structured output for integration with other DevOps pipelines.

 Architecture

The tool is built using a Producer-Consumer pattern to handle high-concurrency scanning without overwhelming the host OS file descriptors.

    Scanner Core: Dispatches asynchronous probes.

    Collector: Aggregates responses and handles timeouts.

    Reporter: Formats and outputs the results.

 Getting Started
Prerequisites

    Rust 1.75+

    libpcap or npcap (for raw packet manipulation)

Installation
Bash

git clone https://github.com/guysa-AMG/Network-Recon.git
cd Network-Recon
cargo build --release

 Usage

To scan a specific range with a custom concurrency limit:
Bash

./target/release/Network-Recon --target 192.168.1.0/24 --ports 1-1024 --threads 500

 Foundational Principles

    Memory Safety: Utilizes Rust’s ownership model to prevent data races during concurrent scans.

    Rate Limiting: Implements adaptive timing to avoid triggering network intrusion detection systems (IDS) unnecessarily.

    Error Handling: Uses the anyhow and thiserror crates for robust, traceable error reporting.

