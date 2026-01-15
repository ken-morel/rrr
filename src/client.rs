use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use super::server;

pub fn run_client(args: Vec<String>) {
    const WE: &str = "Error sending message to server";
    let mut stream = UnixStream::connect(server::RRR_SOCKET).expect("Failed to connect to server");
    let mut text = String::new();
    if args.len() > 1 {
        let replcmd = &args[1];
        if replcmd.starts_with("+") {
            if args.len() != 3 {
                panic!("Invalid number of arguments, use: +<name> <launcher>")
            }
            let mut replid = replcmd.clone();
            replid.remove(0);
            let launcher = &args[2];
            stream.write_all(b"create\n").expect(WE);
            stream.write_all(replid.as_bytes()).expect(WE);
            stream.write_all(b"\n").expect(WE);
            stream.write_all(launcher.as_bytes()).expect(WE);
            stream
                .shutdown(std::net::Shutdown::Write)
                .expect("Error shutting down read on connection");
            stream.flush().expect(WE);
            if let Ok(_) = stream.read_to_string(&mut text) {
                println!("{}", text);
            }
        } else if replcmd.starts_with("-") {
            todo!("Not implemented yet");
            // let mut replid = replcmd.clone();
            // replid.remove(0);
            // stream.write_all(b"repl\n").expect(WE);
            // stream.write_all(replid.as_bytes()).expect(WE);
            // stream.write_all(b"\n").expect(WE);
            // loop {
            //     let mut line = String::new();
            //     std::io::stdin()
            //         .read_line(&mut line)
            //         .expect("Error reading from stdin");
            //     stream.write_all(line.as_bytes()).expect(WE);
            //     stream.write_all(b"\n").expect(WE);
            //     stream.write_all(repl::END_TOKEN.as_bytes()).expect(WE);
            // }
        } else {
            // <name>
            let replid = replcmd;
            stream.write_all(b"run\n").expect(WE);
            stream.write_all(replid.as_bytes()).expect(WE);
            stream.write_all(b"\n").expect(WE);
            let mut content = String::new();
            std::io::stdin()
                .read_to_string(&mut content)
                .expect("Error");
            stream.write_all(content.as_bytes()).expect(WE);
            if let Err(err) = stream.shutdown(std::net::Shutdown::Write) {
                println!("  Error closing write stream: {}", err);
            }
            if let Ok(_) = stream.read_to_string(&mut text) {
                println!("{}", text);
            }
        }
    }
    stream
        .shutdown(std::net::Shutdown::Both)
        .expect("Error shutting down connection after end of operaitons");
}
