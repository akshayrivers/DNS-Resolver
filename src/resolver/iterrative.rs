use std::net::UdpSocket;
use std::time::Duration;
use crate::packet::Message;
use crate::packet::question::Question;
use crate::resolver::slist::ServerList;
use crate::telemetry::{Trace, Event};

pub fn resolve(name: &str, qtype: u16, trace: &mut Trace) -> Result<Message, String> {
    let root_servers = ServerList::root_servers();
    let mut nameserver = root_servers.servers[0].clone();

    loop {
        trace.record(Event::QuerySent { 
            ns: nameserver.clone(), 
            name: name.to_string(), 
            qtype 
        });

        let response = query_nameserver(&nameserver, name, qtype)?;
        
        trace.record(Event::ResponseReceived { 
            ns: nameserver.clone(), 
            rcode: response.header.flags.rcode, 
            answers: response.answers.len() 
        });

        if !response.answers.is_empty() && response.header.flags.rcode == 0 {
            return Ok(response);
        }

        if response.header.flags.rcode == 3 {
            return Err("Name Error (NXDOMAIN)".to_string());
        }

        // Check for CNAME in answers
        if let Some(cname) = find_cname(&response) {
            trace.record(Event::CnameFollowed { from: name.to_string(), to: cname.clone() });
            return resolve(&cname, qtype, trace);
        }

        if let Some(ns_ip) = find_ns_ip(&response) {
            trace.record(Event::ReferralFound { ns_ip: ns_ip.clone() });
            nameserver = ns_ip;
        } else {
            return Err(format!("No more nameservers to query for {}. Last queried: {}", name, nameserver));
        }
    }
}

fn query_nameserver(ns: &str, name: &str, qtype: u16) -> Result<Message, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.set_read_timeout(Some(Duration::from_secs(5))).map_err(|e| e.to_string())?;

    let mut msg = Message::new();
    msg.header.id = rand_id();
    msg.header.qd_count = 1;
    msg.header.flags.rd = false;
    msg.questions.push(Question::new(name.to_string(), qtype, 1));

    let bytes = msg.to_bytes();
    let addr = format!("{}:53", ns);
    socket.send_to(&bytes, &addr).map_err(|e| e.to_string())?;

    let mut buf = [0u8; 1024]; // Increased buffer size
    let (size, _) = socket.recv_from(&mut buf).map_err(|e| e.to_string())?;

    Ok(Message::from_bytes(&buf[..size]))
}

fn rand_id() -> u16 {
    // Basic random ID
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    (now & 0xFFFF) as u16
}

fn find_cname(msg: &Message) -> Option<String> {
    for rr in &msg.answers {
        if rr.rr_type == 5 { // CNAME
            // This is a bit tricky because CNAME rdata is a compressed domain name
            // For now, we'll need a better way to parse it. 
            // In V0, we had parse_qname. We can use it if we have the full buffer.
            // But Message doesn't store the full buffer.
            // Let's assume for this MVP we only handle simple RRs or we need to improve Message.
        }
    }
    None
}

fn find_ns_ip(msg: &Message) -> Option<String> {
    for ns_rr in &msg.authorities {
        if ns_rr.rr_type == 2 { // NS
            // Look for matching A record in additional
            for add_rr in &msg.additionals {
                if add_rr.rr_type == 1 { // A
                    return Some(format!("{}.{}.{}.{}", add_rr.rdata[0], add_rr.rdata[1], add_rr.rdata[2], add_rr.rdata[3]));
                }
            }
        }
    }
    None
}
