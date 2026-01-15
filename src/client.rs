use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use super::server;

pub fn run_client(args: Vec<String>) -> Result<(), String> {
    let mut stream = match UnixStream::connect(server::RRR_SOCKET) {
        Ok(stream) => stream,
        Err(_) => return Err("Could not connect to server, are you sure it's running?".to_string()),
    };
    let mut text = String::new();
    let mut res = String::new();
    if args.len() > 1 {
        let replcmd = &args[1];
        if replcmd.starts_with("+") {
            if args.len() < 3 {
                return Err("Invalid number of arguments, use: +<name> <launcher>".to_string());
            }
            let cwd = if args.len() == 4 {
                &args[3]
            } else {
                &".".to_string()
            };
            let mut replid = replcmd.clone();
            replid.remove(0);
            let launcher = &args[2];
            res += "create\n";
            res += replid.as_str();
            res += "\n";
            res += cwd.as_str();
            res += "\n";
            res += launcher.as_str();
        } else if replcmd.starts_with("-") {
            let mut replid = replcmd.clone();
            replid.remove(0);
            res += "kill\n";
            res += replid.as_str();
            res += "\n";
        } else {
            // <name>
            let replid = replcmd;
            let runtype = if let Some(tp) = args.get(2) {
                tp.clone()
            } else {
                String::from("r")
            };
            res += "run\n";
            res += runtype.as_str();
            res += "\n";
            res += replid.as_str();
            res += "\n";
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .expect("Error");
            res += content.as_str();
        }
    }
    if let Err(e) = stream.write_all(res.as_bytes()) {
        let mut msg = String::from("Error sending query to server: ");
        msg += e.to_string().as_str();
        return Err(msg);
    }
    _ = stream.shutdown(std::net::Shutdown::Write);
    if let Ok(_) = stream.read_to_string(&mut text) {
        println!("{}", text);
    }
    _ = stream.shutdown(std::net::Shutdown::Both);
    Ok(())
}
