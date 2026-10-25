pub struct Constants {
    pub addr: &'static str,
    pub port: u16,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1",
            port: 8080,
        }
    }
}
