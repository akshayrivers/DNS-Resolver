# DNS Resolver (RFC 1035) 
for more details refer to /notesv0.md and notesv1.md
<img width="949" height="900" alt="Screenshot 2026-05-25 at 1 22 40 AM" src="https://github.com/user-attachments/assets/09b0dde6-92a9-49bf-93f0-1738f65e3da9" />



A fully functional, recursive DNS resolver implemented in **Rust** from scratch. This project was built to understand the inner workings of the Domain Name System, including packet parsing, pointer compression, and the iterative resolution process.

![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)

## 🚀 Features

- **Recursive Resolution:** Automatically traverses the DNS hierarchy (Root -> TLD -> Authoritative) to find answers.
- **IPv4 & IPv6 Support:** Handles both `A` (IPv4) and `AAAA` (IPv6) records.
- **DNS Caching:** Implements a time-to-live (TTL) aware cache with automatic expiration.
- **Telemetry & Tracing:** Provides a detailed visual trace of every step in the resolution process, including cache hits/misses and nameserver referrals.
- **Robust Parsing:** Safe handling of malformed packets and protection against malicious pointer loops (infinite recursion).
- **Security Simulations:** Includes built-in tools to simulate and test Cache Poisoning and Packet Spoofing attacks.

## 🛠️ Usage

1. **Clone and Run:**
   ```bash
   git clone https://github.com/akshayrivers/DNS-Resolver.git
   cd DNS-Resolver
   cargo run
   ```

2. **Commands:**
   - `resolve <domain> [type]` - Resolves a domain (e.g., `resolve google.com AAAA`).
   - `poison <domain> <ip>` - Injects a malicious entry into the local cache.
   - `spoof <domain> <ip>` - Simulates a packet interception attack.
   - `help` - Shows all available commands.

## 🛡️ Security Features

The resolver is designed with basic security and robustness in mind:
- **Loop Protection:** Detects and breaks out of malicious DNS pointer loops.
- **Bounds Checking:** Rigorous checks during packet parsing to prevent panics on malformed data.
- **Attack Testing:** Integrated environment for studying DNS vulnerabilities.

## 📖 Technical Implementation

- **No Libraries:** Built using only the Rust Standard Library (std).
- **Custom Parser:** Implements RFC 1035 byte-level parsing for Headers, Questions, and Resource Records.
- **UDP Socketry:** Manual management of UDP communication with global nameservers.

## 📝 Author

**Vinod Akshat**  
*Note: Gimme a Job 😔*

