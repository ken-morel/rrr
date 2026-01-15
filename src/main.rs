mod client;
mod eres;
mod repl;
mod repls;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1].eq("help") {
        println!(
            "
rrr, the remote repl runner:
   rrr                    -- to start the server at /tmp/rrr.sock
   rrr +<id> <replcmd>    -- to create a new repl named <id>
   rrr <id>               -- to send stdin to repl <id>
   rrr -<id>              -- to open repl on <id>
            "
        )
    } else if args.len() == 1 {
        println!("run rrr help for help");
        server::run_server();
    } else {
        client::run_client(args);
    }
}
