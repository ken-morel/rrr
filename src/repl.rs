use std::{cell::RefMut, io::{BufRead, BufReader, Read, Write}, process::{Child, ChildStdin, ChildStdout}};

use super::eres;

pub type SpawnResult<T> = Result<T, String>;

pub const END_TOKEN: &str = "~THE END~";

pub trait Repl<R> {
    fn new(stdin: ChildStdin, stdout:  ChildStdout) -> R;
    fn streams<'a>(&'a self) -> (RefMut<'a, ChildStdin>, RefMut<'a, ChildStdout>) ;
    fn parse_eval_result(&self, txt: String) -> eres::EvalResult;
    fn evaluate(&self, txt: String) -> Result<eres::EvalResult, String> {
        self.sendmsg(txt)?;
        let out = self.readmsg()?;
        Ok(self.parse_eval_result(out))
    }
    fn writeln(&self, txt: String) -> Result<(), std::io::Error> {
        let (mut input, _) = self.streams();
        writeln!(input, "{}", txt)
    }
    
    fn from_child(child: Child) -> Result<R, String> {
        if let Some(stdin) =  child.stdin && let Some(stdout) = child.stdout {
            // FIX: wanted to put child here, but lsp talks about some partial
            // move crap. :-(
            return Ok(Self::new(stdin,  stdout));
        } else {
            return Err(String::from("Error getting spawned child input and output streams"));
        }

    }
    fn sendmsg(&self, msg: String) -> Result<(), String> {
        self.writeln(String::from(END_TOKEN)).unwrap();
        self.writeln(msg).unwrap();
        self.writeln(String::from(END_TOKEN)).unwrap();
        Ok(())
    }
    fn readmsg(&self) -> Result<String, String> {
        let (_, mut output) = self.streams();
        // how did I even know by_ref will work :D
        let mut reader = BufReader::new(output.by_ref());
        let mut ln = String::new();
        
        let mut txt = String::new();
        loop {
            ln.clear();
            match reader.read_line(&mut ln) {
                Err(err) => return Err(err.to_string()),
                Ok(_) =>  {
                    if std::cmp::Ordering::Equal == ln.trim().cmp(END_TOKEN) {
                        break Ok(txt);
                    } else {
                        txt += ln.as_str();
                    }
                },
            };
            
        }
    }

}
