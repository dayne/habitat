// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::env;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use hab_core::config::ConfigFile;
use hab_core::os::system::{Architecture, Platform};
use hab_core::package::PackageTarget;
use hab_net::config::{GitHubCfg, GitHubOAuth, RouteAddrs, RouterAddr, RoutersCfg};
use redis;
use toml;

use error::{Error, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub path: String,
    pub listen_addr: SocketAddr,
    pub datastore_addr: SocketAddr,
    /// List of net addresses for routing servers to connect to
    pub routers: RoutersCfg,
    pub github: GitHubCfg,
    /// allows you to upload packages and public keys without auth
    pub insecure: bool,
    /// Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Whether to schedule builds on package upload
    pub builds_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
    /// Supported targets - comma separated
    pub supported_targets: Vec<PackageTarget>,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "/hab/svc/hab-depot/data".to_string(),
            listen_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9632)),
            datastore_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6379)),
            routers: vec![RouterAddr::default()],
            github: GitHubCfg::default(),
            insecure: false,
            events_enabled: false, // TODO: change to default to true later
            builds_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
            supported_targets: vec![PackageTarget::new(Platform::Linux, Architecture::X86_64),
                                    PackageTarget::new(Platform::Windows, Architecture::X86_64)],
        }
    }
}

impl<'a> redis::IntoConnectionInfo for &'a Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        format!("redis://{}:{}",
                self.datastore_addr.ip(),
                self.datastore_addr.port())
                .into_connection_info()
    }
}

impl RouteAddrs for Config {
    fn route_addrs(&self) -> &Vec<SocketAddr> {
        &self.routers
    }
}

impl GitHubOAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}
