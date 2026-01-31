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
            Err(_) => Err("Could not connect to server, are you sure it's running?".to_string()),
        }
    }
    fn _request(&mut self, req: &String) -> Result<(), String> {
        self.connect()?;
        if let Some(stream) = &mut self.stream {
            if let Err(e) = stream.write_all((self.config.passcode.clone() + "\n").as_bytes()) {
                return Err(
                    "Error querying server, sending initial passcode".to_string()
                        + e.to_string().as_str(),
                );
            }
            if let Err(e) = stream.write_all(req.as_bytes()) {
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
    type Item = String;
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
    if args[0].starts_with("+") {
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
                .expect("Error");
            content
        };
        client.query(args[0].as_str(), runtype, content.as_str())
    }?;
    for line in client {
        println!("{line}");
    }
    Ok(())
}
