extern crate nom;
extern crate rustyline;

use nom::IResult;
use parse::r7rs;

pub struct Repl {
    editor: rustyline::Editor<()>,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            editor: rustyline::Editor::new(),
        }
    }

    pub fn run(&mut self) {
      loop {
          match self.editor.readline("r7rs> ") {
              Ok(line) => {
                  if Repl::is_exit_request(&line) {
                      break;
                  }
                  self.editor.add_history_entry(&line.as_str());
                  match r7rs(line.as_bytes()) {
                      IResult::Done(_, output) => println!("{}", output),
                      IResult::Error(e) => println!("Error: {}", e),
                      IResult::Incomplete(i) => println!("Incomplete: {:?}", i)
                  }
              }

              Err(err) => { format!("Encountered error!~%{}", err); }
          }
      }
    }

    fn is_exit_request(input: &String) -> bool {
        input == "exit" || input == "e" || input == "quit" || input == "q"
    }
}
