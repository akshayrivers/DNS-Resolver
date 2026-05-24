use crate::packet::rr::ResourceRecord;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct CacheEntry {
    records: Vec<ResourceRecord>,
    expires_at: Instant,
}

pub struct DnsCache {
    entries: HashMap<(String, u16), CacheEntry>,
}

impl DnsCache {
    pub fn new() -> Self {
        DnsCache {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str, qtype: u16) -> Option<Vec<ResourceRecord>> {
        if let Some(entry) = self.entries.get(&(name.to_string(), qtype)) {
            if Instant::now() < entry.expires_at {
                return Some(entry.records.clone());
            }
        }
        None
    }

    pub fn insert(&mut self, name: &str, qtype: u16, records: Vec<ResourceRecord>) {
        if records.is_empty() {
            return;
        }

        let min_ttl = records.iter().map(|r| r.ttl).min().unwrap_or(60);
        let expires_at = Instant::now() + Duration::from_secs(min_ttl as u64);

        self.entries.insert(
            (name.to_string(), qtype),
            CacheEntry {
                records,
                expires_at,
            },
        );
    }

    pub fn clear_expired(&mut self) {
        let now = Instant::now();
        self.entries.retain(|_, entry| now < entry.expires_at);
    }
}
