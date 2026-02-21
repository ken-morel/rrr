use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use super::{config::ServerConfig, repl::Repl};

pub const R: u16 = 'r' as u16; // 114
pub const RRR_PORT: u16 = R * (26 ^ 3); // 2967
pub const RRR_PASSCODE: &str = "rrr";

pub fn run_server(conf: ServerConfig) -> Result<(), String> {
    let mut launcher_prefix =
        String::from(conf.launchers.as_path().to_str().expect("Internal error"));
    launcher_prefix += "/";

    let mut repls: HashMap<String, RefCell<Repl>> = HashMap::new();

    println!("Starting server at {:?}", conf.socket_addr);
    match TcpListener::bind(conf.socket_addr) {
        Err(err) => {
            panic!("{}", err.to_string());
        }
        Ok(lis) => {
            for stream in lis.incoming() {
                println!("New connection: ");
                match stream {
                    Ok(mut conn) => {
                        let mut buf = String::new();
                        let content = conn.read_to_string(&mut buf);
                        let mut sendl = |str: Vec<&[u8]>| match super::cypher::cypher(
                            &match String::from_utf8(str.concat()) {
                                Ok(txt) => txt.clone(),
                                Err(err) => {
                                    println!("ERRROR constructing utf8 string: {err}");
                                    return;
                                }
                            },
                            &conf.passcode,
                        ) {
                            Ok(txt) => {
                                _ = conn.write_all(txt.as_bytes());
                                _ = conn.write_all(b"\n");
                            }
                            Err(e) => println!("ERRROR cyphering text: {e}"),
                        };
                        match content {
                            Ok(_) => {
                                let content = match super::cypher::uncypher(buf, &conf.passcode) {
                                    Ok(content) => content,
                                    Err(msg) => {
                                        println!(
                                            "ERRROR: Invalid request could not be decyphered: {}",
                                            msg
                                        );
                                        sendl(vec![b"ERRROR: Invalid passcode: ", msg.as_bytes()]);
                                        _ = conn.shutdown(std::net::Shutdown::Both);
                                        continue;
                                    }
                                };
                                let lines = content.lines().collect::<Vec<_>>();

                                let len = lines.len();
                                if lines[0].eq("create") {
                                    if len != 4 {
                                        sendl(vec![b"ERRROR: Server error: Invalid number of argument lines in request"]);
                                        _ = conn.shutdown(std::net::Shutdown::Both);
                                        continue;
                                    }
                                    let name = lines[1];
                                    if repls.contains_key(name) {
                                        println!("ERRROR: repl {} already exists", name);
                                        sendl(vec![b"ERRROR: Repl already exists"]);
                                    } else {
                                        let mut cmd = String::from(lines[2]);
                                        let dir = String::from(lines[3]);
                                        if cmd.starts_with("+") {
                                            cmd.replace_range(
                                                ..1, // sorry, it was too tempting :)
                                                &launcher_prefix,
                                            );
                                        }
                                        println!("  Spawning: {}", cmd);
                                        match Repl::spawn(dir.as_str(), cmd.as_str()) {
                                            Ok(repl) => {
                                                println!("Shell spawned");
                                                repls.insert(name.to_string(), RefCell::new(repl));
                                                sendl(vec![b"REPL created succesfully"]);
                                                println!("  REPL: {} created", name);
                                            }
                                            Err(err) => {
                                                println!("Error spawning shell: {}", err);
                                                let mut msg = String::from("Errror creating repl:");
                                                msg += err.to_string().as_str();
                                                sendl(vec![msg.as_bytes()]);
                                            }
                                        };
                                    }
                                } else if lines[0].eq("kill") {
                                    if lines.len() != 2 {
                                        sendl(vec![b"ERRROR: Server error: invalid number of argument lines in request"]);
                                        _ = conn.shutdown(std::net::Shutdown::Both);
                                        continue;
                                    }
                                    let replid = lines[1];

                                    if repls.contains_key(replid) {
                                        println!("  Killing repl: {replid}");
                                        repls[replid].borrow_mut().kill();
                                        repls.remove(replid);
                                        sendl(vec![b"Kill signal sent to shell"]);
                                    } else {
                                        sendl(vec![b"ERRROR: Shell does not exist\n"]);
                                    }
                                } else if lines[0].eq("run") {
                                    let runtype = lines[1];
                                    let replid = lines[2];
                                    let codelines = match lines.split_first_chunk::<3>() {
                                        Some(val) => val.1,
                                        None => {
                                            sendl(vec![
                                                b"ERRROR: Could not extract code from message",
                                            ]);
                                            _ = conn.shutdown(std::net::Shutdown::Both);
                                            println!("  Error reading code");
                                            continue;
                                        }
                                    };
                                    let code = codelines.join("\n");
                                    if let Some(replcell) = repls.get(replid) {
                                        let mut repl = replcell.borrow_mut();
                                        match repl.evaluate(&runtype, code.as_str()) {
                                            Ok(_) => {
                                                for line in repl.by_ref() {
                                                    sendl(vec![&line]);
                                                }
                                            }
                                            Err(err) => {
                                                sendl(vec![err.as_bytes()]);
                                            }
                                        };
                                    } else {
                                        sendl(vec![b"ERRROR: Repl does not exist"]);
                                    }
                                } else if lines[0].eq("ls") {
                                    for (name, repl) in &repls {
                                        sendl(vec![
                                            name.as_bytes(),
                                            b" ",
                                            repl.borrow().exe.as_bytes(),
                                        ]);
                                    }
                                } else {
                                    sendl(vec![b"ERRROR: Client sent invalid message query\n"]);
                                }
                            }
                            Err(err) => {
                                println!("Closing connection, could not read from it: {}", err);
                            }
                        }
                        if let Err(err) = conn.shutdown(std::net::Shutdown::Both) {
                            println!("Failed closing connection: {}", err);
                        }
                    }
                    Err(err) => {
                        println!("CONERRROR: {}", err);
                    }
                }
            }
        }
    }
    Ok(())
}
