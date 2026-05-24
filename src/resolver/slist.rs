pub struct ServerList {
    pub servers: Vec<String>,
}

impl ServerList {
    pub fn root_servers() -> Self {
        ServerList {
            servers: vec![
                "198.41.0.4".to_string(),   // a.root-servers.net
                "199.9.14.201".to_string(), // b.root-servers.net
                "192.33.4.12".to_string(),  // c.root-servers.net
                "199.7.91.13".to_string(),  // d.root-servers.net
                "192.203.230.10".to_string(), // e.root-servers.net
                "192.5.5.241".to_string(),  // f.root-servers.net
                "192.112.36.4".to_string(), // g.root-servers.net
                "198.97.190.53".to_string(), // h.root-servers.net
                "192.36.148.17".to_string(), // i.root-servers.net
                "192.58.128.30".to_string(), // j.root-servers.net
                "193.0.14.129".to_string(), // k.root-servers.net
                "199.7.83.42".to_string(),  // l.root-servers.net
                "202.12.27.33".to_string(), // m.root-servers.net
            ],
        }
    }
}
