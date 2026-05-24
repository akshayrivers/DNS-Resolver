mod packet;
use std::io::{self, Write};
mod cache;
use cache::DnsCache;
mod telemetry;
use telemetry::{Event, Trace};
mod resolver;
use resolver::recursive;
mod attack;
use attack::{cacheposion, spoof};

const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

fn print_banner() {
    println!(
        r#"{bold}
                    ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓██████████████▓▒░░▒▓████████▓▒░░▒▓███████▓▒░▒▓███████▓▒░  
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░             ░▒▓█▓▒░ 
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░             ░▒▓█▓▒░ 
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░  ░▒▓██████▓▒░   ░▒▓████▓▒░  
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░             ░▒▓█▓▒░              
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░             ░▒▓█▓▒░              
                    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓███████▓▒░   ░▒▓█▓▒░     
                                                                                       
                                                                                                                                            
                                                            DNS RESOLVER
                                                       RFC 1035 Implementation

                                                     {dim}Author:{reset} Vinod Akshat{reset}
                                                     {dim}Note:  {reset} Gimme a Job 😔{reset}

"#,
        bold = BOLD,
        dim = DIM,
        reset = RESET
    );
}

fn main() {
    print_banner();

    let mut cache = DnsCache::new();

    loop {
        print!("\n{} > ", BOLD);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "resolve" => {
                if parts.len() < 2 {
                    println!("{}Usage: resolve <domain>{}", RED, RESET);
                    continue;
                }
                let name = parts[1];
                let mut trace = Trace::new();
                match recursive::resolve(name, 1, &mut cache, &mut trace) {
                    Ok(msg) => {
                        trace.display();
                        println!("\n{}Final results for {}{}{}:", BOLD, CYAN, name, RESET);
                        for (i, ans) in msg.answers.iter().enumerate() {
                            if ans.rr_type == 1 {
                                println!(
                                    "  {}[{}] {}{} {}.{}.{}.{}",
                                    GREEN,
                                    i + 1,
                                    RESET,
                                    CYAN,
                                    ans.rdata[0],
                                    ans.rdata[1],
                                    ans.rdata[2],
                                    ans.rdata[3]
                                );
                            } else {
                                println!(
                                    "  {}[{}] {}Type {}: {:?}",
                                    GREEN,
                                    i + 1,
                                    RESET,
                                    ans.rr_type,
                                    ans.rdata
                                );
                            }
                        }
                    }
                    Err(e) => {
                        trace.display();
                        println!("\n{}Error: {}{}", RED, e, RESET);
                    }
                }
            }
            "poison" => {
                if parts.len() < 3 {
                    println!("{}Usage: poison <domain> <ip>{}", RED, RESET);
                    continue;
                }
                let domain = parts[1];
                let ip_str = parts[2];
                let ip_parts: Vec<u8> = ip_str.split('.').filter_map(|s| s.parse().ok()).collect();
                if ip_parts.len() == 4 {
                    let mut ip = [0u8; 4];
                    ip.copy_from_slice(&ip_parts);
                    cacheposion::poison_cache(&mut cache, domain, ip);
                    println!(
                        "{}Successfully injected malicious record: {} -> {}{}",
                        GREEN, domain, ip_str, RESET
                    );
                } else {
                    println!("{}Invalid IP format.{}", RED, RESET);
                }
            }
            "spoof" => {
                if parts.len() < 3 {
                    println!("{}Usage: spoof <domain> <fake_ip>{}", RED, RESET);
                    continue;
                }
                let domain = parts[1];
                let ip_str = parts[2];
                let ip_parts: Vec<u8> = ip_str.split('.').filter_map(|s| s.parse().ok()).collect();

                if ip_parts.len() == 4 {
                    let mut ip = [0u8; 4];
                    ip.copy_from_slice(&ip_parts);

                    println!("\n--- 🎭 SIMULATING SPOOFING ATTACK ---{}", RESET);
                    let mut trace = Trace::new();
                    trace.record(Event::QuerySent {
                        ns: "198.41.0.4".to_string(),
                        name: domain.to_string(),
                        qtype: 1,
                    });

                    let _spoofed_msg = spoof::simulate_spoof(domain, ip);
                    trace.record(Event::ResponseReceived {
                        ns: "ATTACKER_SPOOFED_IP".to_string(),
                        rcode: 0,
                        answers: 1,
                    });

                    trace.display();
                    println!(
                        "\n{}🚨 RESOLVER COMPROMISED! Spoofed response accepted.{}",
                        RED, RESET
                    );
                    println!(
                        "  IP: {}{}.{}.{}.{}{}",
                        CYAN, ip[0], ip[1], ip[2], ip[3], RESET
                    );
                    println!("--------------------------------------{}", RESET);
                } else {
                    println!("{}Invalid IP format.{}", RED, RESET);
                }
            }
            "exit" => break,
            "help" => {
                println!("\n{}Available Commands:{}", BOLD, RESET);
                println!(
                    "  {}resolve{} <domain>   Standard recursive resolution",
                    GREEN, RESET
                );
                println!(
                    "  {}poison{}  <domain> <ip> Injects fake data into cache",
                    GREEN, RESET
                );
                println!(
                    "  {}spoof{}   <domain> <ip> Simulates packet interception",
                    GREEN, RESET
                );
                println!("  {}exit{}                Exit the tool", GREEN, RESET);
            }
            _ => println!("{}Unknown command. Type 'help' for options.{}", RED, RESET),
        }
    }
}
