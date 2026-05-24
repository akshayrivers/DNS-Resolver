use crate::cache::DnsCache;
use crate::packet::rr::ResourceRecord;
pub fn poison_cache(cache: &mut DnsCache, name: &str, fake_ip: [u8; 4]) {
    let rr = ResourceRecord::new(
        name.to_string(),
        1, // Type A
        1, // Class IN
        3600,
        fake_ip.to_vec(),
    );
    cache.insert(name, 1, vec![rr]);
}
