use std::process::ExitCode;

use rs_flebf2console::stdin2flebuf2stdout_default;

fn main() -> ExitCode {
    stdin2flebuf2stdout_default()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
