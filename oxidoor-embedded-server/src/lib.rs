// Copyright 2025 AprilNEA LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

mod config;
use once_cell::sync::Lazy;

static CONFIG: Lazy<config::Settings> =
    Lazy::new(|| config::Settings::new().expect("Failed to load configuration"));

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_level(true)
                .with_filter(tracing_subscriber::filter::LevelFilter::from(CONFIG.log_level)),
        )
        .init();

    tracing::info!("Starting OXIDOOR Server");
    tracing::info!("Configuration: {:?}", CONFIG);
}
