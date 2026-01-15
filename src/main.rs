mod client;
mod eres;
mod repl;
mod repls;
mod server;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1].eq("help") {
        println!(
            "
rrr, the remote repl runner:
   rrr                        -- to start the server at /tmp/rrr.sock
   rrr +<id> <launcher> [dir] -- to create a new repl named <id> in <dir>
   rrr <id> [runtype]         -- to send stdin to repl <id>
   rrr -<id>                  -- to kill repl <id>
The <launcher> may refer to a file located in the launchers/ directory located
at the repository root, you can create new executable files there implementing
the protocol to suite yourself.
            "
        );
        Ok(())
    } else if args.len() == 1 {
        println!("run rrr help for help");
        server::run_server()
    } else {
        client::run_client(args)
    }
}
