use crate::cache::DnsCache;
use crate::packet::Message;
use crate::resolver::iterrative;
use crate::telemetry::{Event, Trace};

pub fn resolve(
    name: &str,
    qtype: u16,
    cache: &mut DnsCache,
    trace: &mut Trace,
) -> Result<Message, String> {
    // 0 we clear the expired cache
    cache.clear_expired();
    // 1. we first check Cache
    if let Some(records) = cache.get(name, qtype) {
        trace.record(Event::CacheHit {
            name: name.to_string(),
            qtype,
        });
        let mut msg = Message::new();
        msg.answers = records;
        return Ok(msg);
    }

    trace.record(Event::CacheMiss {
        name: name.to_string(),
        qtype,
    });

    // 2. perform iterative resolution
    let response = iterrative::resolve(name, qtype, trace)?;

    // 3. store in Cache if successful
    if !response.answers.is_empty() {
        cache.insert(name, qtype, response.answers.clone());
    }

    Ok(response)
}
