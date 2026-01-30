use std::{
    io::{Read, Write}, // we are moving from UnixStream to tcp sockets
    net::TcpStream,
};

use crate::config::ClientConfig;

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }
    pub fn connect(&self) -> Result<TcpStream, String> {
        match TcpStream::connect(self.config.socket_addr) {
            Ok(stream) => Ok(stream),
            Err(_) => Err("Could not connect to server, are you sure it's running?".to_string()),
        }
    }
    fn _request(&self, req: &String) -> Result<String, String> {
        let mut stream = self.connect()?;
        let mut res = String::new();
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
            return Err("Error shutting down write stream: ".to_string() + e.to_string().as_str());
        }
        if let Err(e) = stream.read_to_string(&mut res) {
            return Err("Error reading data from connection: ".to_string() + e.to_string().as_str());
        }
        Ok(res)
    }

    pub fn create_repl(
        &self,
        replid: &str,
        template: &str,
        workdir: &str,
    ) -> Result<String, String> {
        let mut req = String::new();

        req += "create\n";
        req += replid;
        req += "\n";
        req += workdir;
        req += "\n";
        req += template;
        self._request(&req)
    }
    pub fn kill_repl(&self, name: &str) -> Result<String, String> {
        let mut req = String::new();

        req += "kill\n";
        req += name;
        req += "\n";
        self._request(&req)
    }
    pub fn query(&self, replid: &str, query: &str, msg: &str) -> Result<String, String> {
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

pub fn run_client(conf: ClientConfig, args: Vec<String>) -> Result<(), String> {
    // println!("{args:?} {conf:?}");
    let client = Client::new(conf);
    let response = if (&args[0]).starts_with("+") {
        // +<name>
        if args.len() < 2 {
            return Err("Invalid number of arguments, use: +<name> <launcher>".to_string());
        }
        let cwd = if args.len() == 3 {
            &args[2]
        } else {
            &".".to_string()
        };
        let mut replid = (&args[0]).clone();
        replid.remove(0);
        client.create_repl(replid.as_str(), &args[1], cwd)
    } else if (&args[0]).starts_with("-") {
        // -<name>
        let mut replid = (&args[0]).clone();
        replid.remove(0);
        client.kill_repl(replid.as_str())
    } else {
        // <name>
        let runtype = if let Some(tp) = args.get(1) {
            tp.as_str()
        } else {
            "r"
        };

        let mut content = String::new();
        std::io::stdin()
            .read_to_string(&mut content)
            .expect("Error");
        client.query((&args[0]).as_str(), runtype, content.as_str())
    }?;
    println!("{response}");
    Ok(())
}
