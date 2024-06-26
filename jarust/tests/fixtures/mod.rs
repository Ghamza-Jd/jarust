#![allow(dead_code)]

pub const FIXTURE_SESSION_ID: u64 = 2;
pub const FIXTURE_HANDLE_ID: u64 = 3;
pub const FIXTURE_KA_INTERVAL: u32 = 10;
pub const FIXTURE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(1);
pub const FIXTURE_CAPACITY: usize = 10;

pub const FIXTURE_PLUGIN_ID: &str = "jarust.plugin.fixture";
pub const FIXTURE_URL: &str = "fixture://www.jarust.rs";
pub const FIXTURE_NAMESPACE: &str = "fixture_nsp";
