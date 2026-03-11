pub struct BuildMetadata {
    pub name: &'static str,
    pub version: &'static str,
}

impl BuildMetadata {
    pub fn current() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
