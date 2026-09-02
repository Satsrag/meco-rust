use meco_core::{translate, version, CodeType};
use std::io::{self, Read, Write};
use std::process::ExitCode;

fn usage(program: &str) -> String {
    format!("usage: {program} translate --from <encoding> --to <encoding> [text]")
}

fn help(program: &str) -> String {
    format!(
        "Mongolian encoding converter\n\n\
Usage:\n  {program} translate --from <encoding> --to <encoding> [text]\n\n\
When [text] is omitted, meco reads UTF-8 text from stdin. Converted UTF-8 text is written to stdout without adding a newline.\n\n\
Encodings:\n  zvvnmod\n  delehi\n  menk_shape\n  menk_letter\n  oyun\n  utn57\n  z52\n\n\
UTN #57 is an output-only target; oyun is not supported.\n"
    )
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<String, String> {
    let mut arguments = arguments.into_iter();
    let program = arguments.next().unwrap_or_else(|| "meco".to_owned());

    let command = arguments.next();
    if matches!(command.as_deref(), Some("--help" | "-h")) && arguments.next().is_none() {
        return Ok(help(&program));
    }
    if matches!(command.as_deref(), Some("--version" | "-V")) && arguments.next().is_none() {
        return Ok(format!("meco {}\n", version()));
    }
    if command.as_deref() != Some("translate") {
        return Err(usage(&program));
    }
    if arguments.next().as_deref() != Some("--from") {
        return Err(usage(&program));
    }
    let from = arguments.next().ok_or_else(|| usage(&program))?;
    if arguments.next().as_deref() != Some("--to") {
        return Err(usage(&program));
    }
    let to = arguments.next().ok_or_else(|| usage(&program))?;
    let input = match arguments.next() {
        Some(input) => input,
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("could not read stdin: {error}"))?;
            input
        }
    };
    if arguments.next().is_some() {
        return Err(usage(&program));
    }

    let from = from
        .parse::<CodeType>()
        .map_err(|error| error.to_string())?;
    let to = to.parse::<CodeType>().map_err(|error| error.to_string())?;
    translate(from, to, &input).map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    match run(std::env::args()) {
        Ok(output) => match io::stdout().write_all(output.as_bytes()) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("meco: could not write stdout: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("meco: {error}");
            ExitCode::FAILURE
        }
    }
}
