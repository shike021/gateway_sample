//! Axum Grid Server main entry point
//!
//! This file is only responsible for starting and configuring the server, and does not contain specific business logic.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

mod config;
mod handlers;
mod routes;
mod server;

mod protos {
    pub mod helloworld {
        include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
    }
}

use server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start server
    start_server().await
}
