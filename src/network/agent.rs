use std::{sync, time::Duration};

use crate::models::config::{get_combine_config};

static AGENT: sync::OnceLock<reqwest::blocking::Client> = sync::OnceLock::new();

pub fn get_agent() -> &'static reqwest::blocking::Client {
    AGENT.get_or_init(|| {
        reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_millis(
                get_combine_config().timeout.unwrap().into(),
            ))
            .tcp_nodelay(true)
            .user_agent(format!("questpackagemanager-rust2/{}", env!("CARGO_PKG_VERSION")).as_str())
            .build()
            .expect("Client agent was not buildable")
    })
}
