use std::env;
use std::process;
use std::collections::HashSet;
use std::process::ExitCode;
#[cfg(not(feature = "u64"))]
type StepsType = u32;

#[cfg(feature = "u64")]
type StepsType = u64;

fn string_to_int(input: String) -> StepsType {
    match input.parse() {
        Ok(output) => {
            return output;
        }
        Err(_) => {
            eprintln!("Bad arguments.");
            process::exit(64);
        }
    }
}

fn collatz(seed: StepsType, print_steps: bool, known_good: &HashSet<StepsType>, return_steps: bool) -> Vec<StepsType> {
    let mut steps: Vec<StepsType> = vec!();
    let mut x: StepsType = seed;

    while !known_good.contains(&x) {
        if x%2==0 {
            x = x/2;
        }
        else {
            x = 3*x+1;
        }
        if return_steps {steps.push(x)};

        if print_steps {println!("{}", x);}
    }
    return steps;
}

fn collatz_ranged(start: StepsType, end: StepsType, use_slow: bool) -> u32 {
    let mut known_good:HashSet<StepsType> = HashSet::from([1]);
    let mut total_steps:u32 = 0;
    for i in start..=end {
        let steps:Vec<StepsType> = collatz(i, false, &known_good, true);
        total_steps += steps.len() as u32;
        if !use_slow {known_good.extend(steps);}
    }
    return total_steps;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {if string_to_int(args[1].clone())>0 {collatz(string_to_int(args[1].clone()), true, &HashSet::from([1]), false);} else {eprintln!("Bad arguments."); return ExitCode::from(64);}}
        3 => {if string_to_int(args[1].clone())>0 {println!("Finished in {} steps.", collatz_ranged(string_to_int(args[1].clone()), string_to_int(args[2].clone()), env::var("USE_SLOW").unwrap_or_default() == "true"))} else {eprintln!("Bad arguments."); return ExitCode::from(64);}},
        _ => {eprintln!("Bad arguments."); return ExitCode::from(64);}
    }

    return ExitCode::SUCCESS;
}
