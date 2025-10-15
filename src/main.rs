use dash_lang::run;
use pest_derive::Parser;
use std::env;
use std::fs;

/// Pest parser definition using the grammar in `dash.pest`.
#[derive(Parser)]
#[grammar = "dash.pest"]
pub struct DashParser;

/// Entry point for the CLI interpreter.
/// If a filename is provided, it runs the script from that file.
/// Otherwise, it runs a default hardcoded script.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Run from file
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(mut source) => {
                // Convert CRLF (\r\n) to LF (\n)
                source = source.replace("\r\n", "\n");
                run(&source)
            },
            Err(e) => eprintln!("Error reading file '{}': {}", filename, e),
        }
    } else {
        // Run hardcoded script (fallback)
        run(r#"
let x = 0
while x < 5 {
  print(x)
  let x = x + 1
}
"#);
    }
}

