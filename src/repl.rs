extern crate rustyline;

use rustyline::Editor;

fn main() {
    loop {
        let mut editor = Editor::<()>::new();
        match editor.readline("r7rs> ") {
            Ok(line) => {
                if line == "exit" {
                    break;
                }
                editor.add_history_entry(&line.as_str());
                print!("{}", line);
            }
            Err(err) => { format!("Encountered error!~%{}", err); }
        }
    }
}
