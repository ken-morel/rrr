use std::{
    io::{Read, Write}, // we are moving from UnixStream to tcp sockets
    net::TcpStream,
};

use crate::{config::ClientConfig, utils::read_line};

pub struct Client {
    config: ClientConfig,
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            stream: None,
        }
    }
    pub fn connect(&mut self) -> Result<(), String> {
        match TcpStream::connect(self.config.socket_addr) {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(
                "Could not connect to server, are you sure it's running?: ".to_string()
                    + e.to_string().as_str(),
            ),
        }
    }
    fn _request(&mut self, req: &String) -> Result<(), String> {
        self.connect()?;
        let query = super::cypher::cypher(req, &self.config.passcode).map_err(|e| {
            format!("Could not cypher data with passcode to send query to client: {e}",)
        })?;
        if let Some(stream) = &mut self.stream {
            if let Err(e) = stream.write_all(query.as_bytes()) {
                return Err("Error sending query to server: ".to_string() + e.to_string().as_str());
            }
            if let Err(e) = stream.shutdown(std::net::Shutdown::Write) {
                return Err(
                    "Error shutting down write stream: ".to_string() + e.to_string().as_str()
                );
            }
            Ok(())
        } else {
            Err(String::from("Client failed to connect"))
        }
    }

    pub fn create_repl(
        &mut self,
        replid: &str,
        template: &str,
        workdir: &str,
    ) -> Result<(), String> {
        let mut req = String::new();

        req += "create\n";
        req += replid;
        req += "\n";
        req += workdir;
        req += "\n";
        req += template;
        self._request(&req)
    }
    pub fn kill_repl(&mut self, name: &str) -> Result<(), String> {
        let mut req = String::new();

        req += "kill\n";
        req += name;
        req += "\n";
        self._request(&req)
    }
    fn ls(&mut self) -> Result<(), String> {
        let mut req = String::new();

        req += "ls\n";
        self._request(&req)
    }

    pub fn query(&mut self, replid: &str, query: &str, msg: &str) -> Result<(), String> {
        let mut req = String::new();

        req += "run\n";
        req += query;
        req += "\n";
        req += replid;
        req += "\n";
        req += msg;
        self._request(&req)
    }
}

impl Iterator for Client {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(stream) = &mut self.stream {
            read_line(stream)
        } else {
            None
        }
    }
}

pub fn run_client(
    conf: ClientConfig,
    args: Vec<String>,
    input: Option<String>,
) -> Result<(), String> {
    // println!("{args:?} {conf:?}");
    let mut client = Client::new(conf);
    if args[0].eq("ls") {
        client.ls()
    } else if args[0].starts_with("+") {
        // +<name>
        if args.len() < 2 {
            return Err("Invalid number of arguments, use: +<name> <launcher>".to_string());
        }
        let cwd = if args.len() == 3 {
            &args[2]
        } else {
            &".".to_string()
        };
        let mut replid = args[0].clone();
        replid.remove(0);
        client.create_repl(replid.as_str(), &args[1], cwd)
    } else if args[0].starts_with("-") {
        // -<name>
        let mut replid = args[0].clone();
        replid.remove(0);
        client.kill_repl(replid.as_str())
    } else if args[0].starts_with("%") {
        println!("RRR repl");
        let mut replid = args[0].clone();
        replid.remove(0);
        let runtype = "r";
        let mut running = true;
        while running {
            let mut prompt = Vec::new();
            if client.query(replid.as_str(), ".p", "").is_err() {
                prompt.append(&mut ".".as_bytes().to_vec());
                prompt.append(&mut replid.as_bytes().to_vec());
                prompt.append(&mut "% ".as_bytes().to_vec());
            } else {
                for mut ln in &mut client {
                    prompt.append(&mut ln);
                }
                if prompt.starts_with("ERRROR".as_bytes()) {
                    prompt.append(&mut "\n> ".as_bytes().to_vec());
                } else {
                    prompt.append(&mut " ".as_bytes().to_vec());
                }
            }
            let mut stdout = std::io::stdout().lock();
            _ = stdout.write_all(&prompt);
            _ = stdout.flush();
            drop(stdout);
            let mut code = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut code) {
                println!("ERRROR: reading from stdin {e}");
                continue;
            }
            if code.starts_with("/") {
                if code.starts_with("/k") {
                    if let Err(e) = client.kill_repl(replid.as_str()) {
                        println!("ERRROR: {e}");
                        running = false;
                    }
                } else if code.starts_with("/s") {
                    replid = code.split_at(3).1.to_string();
                } else if code.starts_with("/q") {
                    break;
                } else {
                    println!("Invalid slash code")
                }
            } else if code.starts_with("!") {
                code.remove(0);
                match &mut std::process::Command::new("sh")
                    .env("LANG", "en_US.UTF-8")
                    .env("LC_ALL", "en_US.UTF-8")
                    .arg("-c")
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .arg(code.as_str())
                    .spawn()
                {
                    Ok(c) => {
                        _ = c.wait();
                    }
                    Err(e) => println!("ERRROR: {e}"),
                };
            } else if let Err(e) = client.query(replid.as_str(), runtype, code.as_str()) {
                println!("ERRROR: querying server: {e}");
            }
            let mut stdout = std::io::stdout().lock();
            for ln in &mut client {
                _ = stdout.write_all(&ln);
                _ = stdout.write_all(b"\n");
                _ = stdout.flush();
            }
        }
        Ok(())
    } else {
        // <name>
        let runtype = if let Some(tp) = args.get(1) {
            tp.as_str()
        } else {
            "r"
        };

        let content = if let Some(txt) = input {
            txt
        } else {
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .expect("Error reading code from stdin");
            content
        };
        client.query(args[0].as_str(), runtype, content.as_str())
    }?;
    let mut stdout = std::io::stdout().lock();
    for line in client {
        _ = stdout.write_all(&line);
        _ = stdout.write_all(b"\n");
    }
    _ = stdout.flush();
    Ok(())
}
