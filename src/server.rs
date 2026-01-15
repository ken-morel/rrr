use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    io::{Read, Write},
    os::unix::net::UnixListener,
};

use crate::repl::Repl;

use super::repls::SimpleRepl;

pub const RRR_SOCKET: &str = "/tmp/rrr.sock";

pub fn run_server() {
    let home = current_dir().expect("Error locating current directory");
    let launchers = home.join("launchers");
    let mut launcher_prefix = String::from(launchers.as_path().to_str().expect("Internal error"));
    launcher_prefix += "/";

    _ = fs::remove_file(RRR_SOCKET);

    let mut shells: HashMap<String, SimpleRepl> = HashMap::new();

    println!("Starting server at {}", RRR_SOCKET);
    // took tip from gemini too
    match UnixListener::bind(RRR_SOCKET) {
        Err(err) => {
            panic!("{}", err.to_string());
        }
        Ok(lis) => {
            for stream in lis.incoming() {
                println!("New connection: ");
                match stream {
                    Ok(mut conn) => {
                        let mut buf = String::new();
                        match conn.read_to_string(&mut buf) {
                            Ok(_) => {
                                let content = buf.trim();
                                let lines: Vec<&str> = content.split("\n").collect();
                                let len = lines.len();
                                if len == 3 && lines[0].eq("create") {
                                    let name = lines[1];
                                    if shells.contains_key(name) {
                                        println!("ERRROR: repl {} already exists", name);
                                    } else {
                                        let mut cmd = String::from(lines[2]);
                                        if cmd.starts_with("+") {
                                            cmd.replace_range(
                                                ..1, // sorry, it was too tempting :)
                                                &launcher_prefix,
                                            );
                                        }
                                        println!("  Spawning: {}", cmd);
                                        match SimpleRepl::spawn(cmd.as_str()) {
                                            Ok(repl) => {
                                                println!("Shell spawned");
                                                _ = conn.write_all(b"REPL created succesfully");
                                                shells.insert(name.to_string(), repl);
                                                println!("  REPL: {} created", name);
                                            }
                                            Err(err) => {
                                                println!("Error spawning shell: {}", err);
                                                let mut msg = String::from("Errror creating repl:");
                                                msg += err.to_string().as_str();
                                                _ = conn.write_all(msg.as_bytes());
                                            }
                                        };
                                    }
                                } else if len > 2 && lines[0].eq("run") {
                                    let replid = lines[1];
                                    let codelines = match lines.split_first_chunk::<2>() {
                                        Some(val) => val.1,
                                        None => {
                                            _ = conn
                                                .write_all(b"Could not extract code from message");
                                            _ = conn.shutdown(std::net::Shutdown::Both);
                                            println!("  Error reading code");
                                            continue;
                                        }
                                    };
                                    let code = codelines.join("\n");
                                    if let Some(shell) = shells.get(replid) {
                                        let output = match shell.evaluate(code) {
                                            Ok(res) => res.to_string(),
                                            Err(err) => err,
                                        };
                                        if let Err(err) = conn.write_all(output.as_bytes()) {
                                            println!("  Error sending output to client: {}", err);
                                        }
                                    } else {
                                        _ = conn.write_all(b"Chell does not exist");
                                    }
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
                        println!("CONERRROR: {}", err.to_string());
                    }
                }
            }
        }
    }
}
