use std::{
    collections::HashMap,
    env::current_dir,
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
};

use crate::server::{RRR_PASSCODE, RRR_PORT};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub launchers: PathBuf,
    pub socket_addr: SocketAddrV4,
    pub passcode: String,
}

pub fn read_env(map: &mut HashMap<String, String>) {
    std::env::vars().for_each(|(key, value)| {
        if key.starts_with("RRR_") {
            let k = key.to_lowercase();
            let n = k.split_at(4);
            let v = value.clone();
            map.insert(n.1.to_string(), v);
        }
    });
}

fn parse_passcode(map: &HashMap<String, String>) -> Result<String, String> {
    if let Some(pass_str) = map.get("k") {
        Ok(pass_str.clone())
    } else {
        Ok(String::from(RRR_PASSCODE))
    }
}
fn parse_socket_addr(
    map: &HashMap<String, String>,
    default_ip: Ipv4Addr,
    default_port: u16,
) -> Result<SocketAddrV4, String> {
    let ip = if let Some(ip_str) = map.get("ip") {
        match ip_str.parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(msg) => {
                return Err(String::from("Could not parse ip address: ") + msg.to_string().as_str())
            }
        }
    } else {
        default_ip
    };
    let port = if let Some(port) = map.get("p") {
        match port.parse::<u16>() {
            Ok(p) => p,
            Err(e) => return Err(String::from("Error parsing port: ") + e.to_string().as_str()),
        }
    } else {
        default_port
    };
    Ok(SocketAddrV4::new(ip, port))
}

impl ServerConfig {
    pub fn parse(map: HashMap<String, String>) -> Result<Self, String> {
        let launchers = if let Some(l_str) = map.get("l") {
            PathBuf::from(l_str)
        } else {
            match current_dir() {
                Ok(d) => d.join("launchers"),
                Err(e) => {
                    return Err(String::from(
                        "Error locating launchers from current working directory: ",
                    ) + e.to_string().as_str())
                }
            }
        };
        let socket_addr = parse_socket_addr(&map, Ipv4Addr::UNSPECIFIED, RRR_PORT)?;
        let passcode = parse_passcode(&map)?;

        Ok(Self {
            launchers,
            socket_addr,
            passcode,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub socket_addr: SocketAddrV4,
    pub passcode: String,
}
impl ClientConfig {
    pub fn parse(map: HashMap<String, String>) -> Result<Self, String> {
        let socket_addr = parse_socket_addr(&map, Ipv4Addr::LOCALHOST, RRR_PORT)?;
        let passcode = parse_passcode(&map)?;
        Ok(Self {
            socket_addr,
            passcode,
        })
    }
}
