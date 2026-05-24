# DNS-Resolver

An implementation of the client side of DNS in **Rust**

![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)

Note: Currently, this client sends queries to Google DNS (8.8.8.8:53) as a placeholder local resolver. Future versions will implement full recursive querying and caching.

## Table of Contents

- [WHYs](#whys)
  - [Why this project?](#why-this-project)
  - [Why In Rust?](#why-in-rust)
- [How does DNS actually work](#how-does-dns-actually-work)
  - [Overview (protocol + distributed hierarchical database)](#overview-protocol--distributed-hierarchical-database)
  - [How it works (hostname-to-ip translation service)](#how-it-works-hostname-to-ip-translation-service)
  - [Why not centralized?](#why-not-centralized)
  - [DNS Hierarchy](#a-distributed-and-hierarchical-database)
  - [Dns Caching](#dns-caching)
  - [Resource Records](#resource-records)
  - [DNS Messages](#dns-messages)
  - [How do new domains get registered?](#how-do-new-domains-get-registered)
- [My Implementation](#my-implementation)
  - [Progress](#progress)
  - [Future enhancement plans](#future-enhancement-plans)
  - [TODO (Lazy)](#todo-lazy)
  - [Implementation Details](#implementation-details)
  - [Challenges](#challenges)
- [Setup & Testing](#setup--testing)
- [Key Learnings](#key-learnings)
- [References](#references)
- [Contributing](#contributing-to-dns-resolver)
  - [🛠️ How to Contribute](#️-how-to-contribute)
  - [💡 Want to help but don’t know where to start?](#️-want-to-help-but-dont-know-where-to-start)

## WHYs

### Why this project?

So I was studying computer networks — the **application layer** to be specific — and there was a dedicated section explaining what actually is DNS (a protocol and distributed database) and how things work under the hood. There were structures like **Record type** and **DNS Messages** explained in detail in the book.

As I was approaching the end, I had an itch.  
Yes, an itch to build it myself from scratch — not using any libraries or DNS resolvers like Google or Cloudflare (as of now I am using the Google resolver) to do the heavy lifting.

### Why In Rust?

Well, I have been learning Rust (from _The Rust Book_), and I had the impression that I would be able to approach this project using Rust. I wanted to apply and solidify what I’ve learned so far.

I’ve also heard somewhere that **Rust is very good for building low-level stuff**.

I also wanted to challenge myself:  
**Could I really build something like this with just the Rust standard library and my current knowledge?**

---

## How does DNS actually work

### Overview (protocol + distributed hierarchical database)

DNS is:

1. A **distributed database**, implemented in a hierarchy of DNS servers.
2. An **application-layer protocol** that allows hosts to query the distributed database.

The DNS servers are often UNIX machines running the Berkeley Internet Name Domain (BIND) software [BIND 2020].  
The DNS protocol runs over **UDP** and uses **port 53**.

DNS is usually employed by other application-layer protocols, including HTTP, SMTP, to translate **user-supplied hostnames to IP addresses**.

It also provides a few other services:

- Host aliasing
- Mail server aliasing (MX records)
- Load distribution (DNS rotation)

### How it works (hostname-to-IP translation service)

Suppose that some application (e.g. a web browser or mail client) running on a user’s host needs to translate a hostname to an IP address.

1. The application invokes the client side of DNS, specifying the hostname.
2. The DNS code on the host sends a query message over UDP to port 53.
3. After a delay, it receives a DNS reply message with the IP address mapping.
4. This mapping is passed back to the application.

From the application's point of view, **DNS is a black box**.  
But in reality, it's a complex system involving:

- A large number of distributed servers
- An application-layer protocol
- Hierarchical querying

### Why not centralized?

- A single point of failure
- High traffic volume
- Distant centralized database

---

### A distributed and hierarchical database

In order to deal with the issue of scale, the DNS uses a large number of servers,
organized in a hierarchical fashion and distributed around the world. No single DNS
server has all of the mappings for all of the hosts in the Internet. Instead, the map-
pings are distributed across the DNS servers. DNS hierarchy consists of:

- Root DNS servers
- Top-Level Domain (TLD) DNS servers
- Authoritative DNS servers

![DNS Heirarchy](<assets/Screenshot 2025-05-18 at 3.52.15 AM.png>)

- There is another important type of DNS
  server called the local DNS server. A local DNS server does not strictly belong to
  the hierarchy of servers but is nevertheless central to the DNS architecture. Each
  ISP—such as a residential ISP or an institutional ISP—has a local DNS server (also
  called a default name server)

![An example of how the communication happens](<assets/Screenshot 2025-05-18 at 3.52.32 AM.png>)
Here The requesting host is us but in place of local DNS server I am using the Google dns server ("8.8.8.8:53") but our request type is recursive

### Dns Caching

DNS caching, a critically important feature
of the DNS system. In truth, DNS extensively exploits DNS caching in order to
improve the delay performance and to reduce the number of DNS messages
ricocheting around the Internet. The idea behind DNS caching is very simple. In a
query chain, when a DNS server receives a DNS reply (containing, for example, a
mapping from a hostname to an IP address), it can cache the mapping in its local
memory. For example, in Figure 2.19, each time the local DNS server dns.nyu.edu
receives a reply from some DNS server, it can cache any of the information contained
in the reply. If a hostname/IP address pair is cached in a DNS server and another
query arrives to the DNS server for the same hostname, the DNS server can provide
the desired IP address, even if it is not authoritative for the hostname. Because hosts
and mappings between hostnames and IP addresses are by no means permanent,
DNS servers discard cached information after a period of time (often set to two days).

![Recursive queries in DNS](<assets/Screenshot 2025-05-18 at 3.53.23 AM.png>)

### Resource records

The DNS servers that together implement the DNS distributed database store
resource records (RRs), including RRs that provide hostname-to-IP address map-
pings. Each DNS reply message carries one or more resource records

A resource record is a four-tuple that contains the following fields:
(Name, Value, Type, TTL)
TTL is the time to live of the resource record; it determines when a resource should
be removed from a cache. In the example records given below, we ignore the TTL
field. The meaning of Name and Value depend on Type:

- If Type=A, then Name is a hostname and Value is the IP address for the host-
  name. Thus, a Type A record provides the standard hostname-to-IP address map-
  ping. As an example, (relay1.bar.foo.com, 145.37.93.126, A) is
  a Type A record.
- If Type=NS, then Name is a domain (such as foo.com) and Value is the host-
  name of an authoritative DNS server that knows how to obtain the IP addresses
  for hosts in the domain. This record is used to route DNS queries further along in
  the query chain. As an example, (foo.com, dns.foo.com, NS) is a Type
  NS record.
- If Type=CNAME, then Value is a canonical hostname for the alias hostname
  Name. This record can provide querying hosts the canonical name for a host-
  name. As an example, (foo.com, relay1.bar.foo.com, CNAME) is a
  CNAME record.
- If Type=MX, then Value is the canonical name of a mail server that has an alias
  hostname Name. As an example, (foo.com, mail.bar.foo.com, MX)
  is an MX record. MX records allow the hostnames of mail servers to have simple
  aliases. Note that by using the MX record, a company can have the same aliased
  name for its mail server and for one of its other servers (such as its Web server).
  To obtain the canonical name for the mail server, a DNS client would query for
  an MX record; to obtain the canonical name for the other server, the DNS client
  would query for the CNAME record.

### DNS messages

![DNS Message format](<assets/Screenshot 2025-05-18 at 3.53.41 AM.png>)

- The first 12 bytes is the header section, which has a number of fields. The first
  field is a 16-bit number that identifies the query. This identifier is copied into the
  reply message to a query, allowing the client to match received replies with sent
  queries. There are a number of flags in the flag field. A 1-bit query/reply flag indi-
  cates whether the message is a query (0) or a reply (1). A 1-bit authoritative flag is
  set in a reply message when a DNS server is an authoritative server for a queried
  name. A 1-bit recursion-desired flag is set when a client (host or DNS server)
  desires that the DNS server perform recursion when it doesn’t have the record. A
  1-bit recursion-available field is set in a reply if the DNS server supports recur-
  sion. In the header, there are also four number-of fields. These fields indicate the
  number of occurrences of the four types of data sections that follow the header.

- The question section contains information about the query that is being made.
  This section includes (1) a name field that contains the name that is being que-
  ried, and (2) a type field that indicates the type of question being asked about the
  name—for example, a host address associated with a name (Type A) or the mail
  server for a name (Type MX).

- In a reply from a DNS server, the answer section contains the resource records for
  the name that was originally queried. Recall that in each resource record there is the
  Type (for example, A, NS, CNAME, and MX), the Value, and the TTL. A reply can
  return multiple RRs in the answer, since a hostname can have multiple IP addresses
  (for example, for replicated Web servers, as discussed earlier in this section).

- The authority section contains records of other authoritative servers.

- The additional section contains other helpful records. For example, the answer
  field in a reply to an MX query contains a resource record providing the canoni-
  cal hostname of a mail server. The additional section contains a Type A record
  providing the IP address for the canonical hostname of the mail server.

### How do new domains get registered ?

Domain registration involves working with registrars accredited by ICANN. When a domain is registered, the registrar updates the TLD servers with the authoritative DNS server information for the domain.

# My Implementation:

## Progress

- [x] `DNSMessage` struct
- [x] `DNSMessage` to bytes conversion
- [x] Parse `DNSMessage` from raw bytes
- [x] UDP socket to send and receive DNS packets
- [x] Unit tests

### Future enhancement plans:

- [ ] My own DNS server (like Google or Cloudflare) that handles:
  - Caching
  - Querying root, TLD, and authoritative servers
- [ ] Dynamic handling of multiple question types (currently hardcoded to IPv4) and r_data
- [ ] Support for **DNSSEC** (stretch goal)

### TODO (Lazy)

- [ ] Proper error handling using `Result`

## Implementation Details

I skipped detailed code breakdown here because the actual Rust source is well-commented. Most logic centers around converting DNS messages to/from bytes, extracting fields from r_data, and socket-based UDP communication.

## Challenges:

<!-- - Handling DNS message(byte) parsing manually because UDP protocol operates on raw bytes

- Understanding and Implementing pointer compression- smart way to store the domain name

- UDP connection -->

### ♝ Byte-Level DNS Parsing

DNS messages are sent over UDP, which is a connectionless protocol that transmits raw bytes. This means there’s no built-in structure—you have to:

- Manually parse headers, questions, and resource records from byte slices.
- Handle numeric conversions (e.g., u16::from_be_bytes) and byte slicing with care to avoid panics or misinterpretation.

### ♞ Pointer Compression

DNS uses a clever space-saving mechanism: pointer compression.

Instead of repeating domain names multiple times, the DNS message can include a pointer (with the top 2 bits as 11) to a previously mentioned name in the byte stream.

- you can checkout how I handled this in my code

### ♛ UDP Communication with Google DNS

I used UDPSocket to manually:

- Bind to a local ephemeral(temporary) port
- Send a raw byte-encoded DNS query to 8.8.8.8:53(Google dns server)
- Wait for a response with recv_from, using a timeout for robustness

## Setup & Testing

**Prerequisites:**  
Ensure you have Rust installed. [Install Rust](https://www.rust-lang.org/tools/install)

**Steps:**

```bash
git clone https://github.com/akshayrivers/DNS-Resolver.git
cd DNS-Resolver
cargo run
```

Run Tests:

```bash
cargo test
```

## Key learnings :

IN Computer Networks:

- DNS as a layered, distributed protocol
- Message format and compression
- Query flow (recursive vs iterative)

IN RUST:

- How DNS message compression using pointers actually works
- Handling raw bytes, bit manipulation, and endian issues
- UDP socket programming in Rust
- Working with the borrow checker while parsing nested structures

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or ideas for improvement, feel free to open an issue or submit a pull request.

### 🛠️ How to Contribute

1. Fork this repository.
2. Create a new branch: `git checkout -b feature-name`
3. Make your changes and commit: `git commit -m 'Add feature'`
4. Push to your fork: `git push origin feature-name`
5. Open a Pull Request and describe your changes.

### 💡 Looking for a place to start?

You can contribute to:

- Improving error handling using Result properly.
- Extending support for different DNS record types. please fix my table of contents

---

Thanks for helping make this project better!

## References:

- "Computer Networks: A Top-Down Approach" by James F. Kurose and Keith W. Ross
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)

---
