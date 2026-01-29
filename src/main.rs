use std::collections::HashMap;

mod client;
mod config;
mod eres;
mod repl;
mod server;

const RRR_HELP: &str = "
rrr, the remote repl runner:
   rrr [conf] server                 -- to start the server at /tmp/rrr.sock
   rrr [conf] +<id> <launcher> [dir] -- to create a new repl named <id> in <dir>
   rrr [conf] <id> [runtype]         -- to send stdin to repl <id>
   rrr [conf] -<id>                  -- to kill repl <id>
[conf]
    ip=127.0.0.1                # server listening address(default all available(UNSPECIFIED))
                                # client query addres
    p=80142                     # server listening port | client query port
    l=/usr/share/rrr/launchers  # server launchers location
<launcher>
    An executable file located in the directory specified by launchers config
    option.
";

fn main() -> Result<(), String> {
    let os_args: Vec<String> = std::env::args().collect();
    let mut conf_args = HashMap::<&str, &str>::new();
    let mut args = Vec::<String>::new();

    let mut is_conf = true;
    for arg in (&os_args)
        .split_first()
        .expect("ERRROR: RRR did not receive program name")
        .1
    {
        if is_conf {
            if let Some(parts) = arg.split_once("=") {
                conf_args.insert(parts.0, parts.1);
                continue;
            } else {
                is_conf = false
            }
        }
        args.push(arg.clone())
    }
    // println!("{args:#?} and {conf_args:#?}");
    if args.len() == 0 || args[0].eq("help") {
        println!("{RRR_HELP}");
        Ok(())
    } else if args[0].eq("server") {
        let conf = config::ServerConfig::parse(conf_args)?;
        server::run_server(conf)
    } else {
        let conf = config::ClientConfig::parse(conf_args)?;
        client::run_client(conf, os_args)
    }
}
