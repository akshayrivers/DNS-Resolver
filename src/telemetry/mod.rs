use std::time::Instant;

const ORANGE: &str = "\x1b[38;5;208m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

#[derive(Debug, Clone)]
pub enum Event {
    QuerySent {
        ns: String,
        name: String,
        qtype: u16,
    },
    ResponseReceived {
        ns: String,
        rcode: u8,
        answers: usize,
    },
    CacheHit {
        name: String,
        qtype: u16,
    },
    CacheMiss {
        name: String,
        qtype: u16,
    },
    ReferralFound {
        ns_ip: String,
    },
    CnameFollowed {
        from: String,
        to: String,
    },
}

pub struct Trace {
    pub events: Vec<(Instant, Event)>,
    pub start_time: Instant,
}

impl Trace {
    pub fn new() -> Self {
        Trace {
            events: Vec::new(),
            start_time: Instant::now(),
        }
    }

    pub fn record(&mut self, event: Event) {
        self.events.push((Instant::now(), event));
    }

    pub fn display(&self) {
        println!(
            "\n{}{}┌────────────────── DNS RESOLUTION TRACE ──────────────────┐{}",
            ORANGE, BOLD, RESET
        );
        for (time, event) in &self.events {
            let elapsed = time.duration_since(self.start_time).as_millis();
            let timestamp = format!("{}{:>4}ms{}", DIM, elapsed, RESET);

            match event {
                Event::QuerySent { ns, name, qtype } => {
                    println!(
                        "{}  {} {}❓ QUERY{}   {} {}for{} {}{}{} (Type: {})",
                        timestamp, ORANGE, BOLD, RESET, ns, DIM, RESET, CYAN, name, RESET, qtype
                    );
                }
                Event::ResponseReceived { ns, rcode, answers } => {
                    let color = if *rcode == 0 { GREEN } else { RED };
                    println!(
                        "{}  {} {}📩 RECV{}    {}from{} {} (RCODE {}{}{}, {} ans)",
                        timestamp, GREEN, BOLD, RESET, DIM, RESET, ns, color, rcode, RESET, answers
                    );
                }
                Event::CacheHit { name, qtype } => {
                    println!(
                        "{}  {} {}⚡ HIT{}     {} {}for{} {}{}{} (Type: {})",
                        timestamp, GREEN, BOLD, RESET, name, DIM, RESET, CYAN, name, RESET, qtype
                    );
                }
                Event::CacheMiss { name, qtype } => {
                    println!(
                        "{}  {} {}😔 MISS{}    {} {}for{} {}{}{} (Type: {})",
                        timestamp, RED, BOLD, RESET, name, DIM, RESET, CYAN, name, RESET, qtype
                    );
                }
                Event::ReferralFound { ns_ip } => {
                    println!(
                        "{}  {} {}😡 REFER{}   {} {}next NS:{} {}{}{}",
                        timestamp, CYAN, BOLD, RESET, ns_ip, DIM, RESET, GREEN, ns_ip, RESET
                    );
                }
                Event::CnameFollowed { from, to } => {
                    println!(
                        "{}  {} {}🔗 CNAME{}   {} {}->{} {}{}{}",
                        timestamp, ORANGE, BOLD, RESET, from, DIM, RESET, CYAN, to, RESET
                    );
                }
            }
        }
        println!("{}{: <59}┘{}", ORANGE, "└", RESET);
    }
}
