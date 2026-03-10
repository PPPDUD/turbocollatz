use std::env;
use std::process;
use std::process::ExitCode;
use turbocollatz_internal::StepsType;
use turbocollatz_internal::easy_collatz;
use turbocollatz_internal::collatz_ranged;
fn string_to_int(input: String) -> StepsType {
    match input.parse() {
        Ok(output) => {
            output
        }
        Err(_) => {
            eprintln!("Bad arguments.");
            process::exit(64);
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {if string_to_int(args[1].clone())>0 {easy_collatz(string_to_int(args[1].clone()), true);} else {eprintln!("Bad arguments."); return ExitCode::from(64);}}
        3 => {if string_to_int(args[1].clone())>0 {println!("Finished in {} steps.", collatz_ranged(string_to_int(args[1].clone()), string_to_int(args[2].clone()), env::var("USE_SLOW").unwrap_or_default() == "true"))} else {eprintln!("Bad arguments."); return ExitCode::from(64);}},
        _ => {eprintln!("Bad arguments."); return ExitCode::from(64);}
    }

    ExitCode::SUCCESS
}
