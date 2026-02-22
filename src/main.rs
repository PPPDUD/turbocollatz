use std::env;
use std::process;
use std::collections::HashSet;
use std::process::ExitCode;


fn string_to_int(input: String) -> i64 {
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

fn collatz(seed: i64, print_steps: bool, known_good: &HashSet<i64>, return_steps: bool) -> Vec<i64> {
    let mut steps: Vec<i64> = vec!();
    let mut x = seed;
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

fn collatz_ranged(start: i64, end: i64, use_slow: bool) -> u32 {
    let mut known_good:HashSet<i64> = HashSet::from([1]);
    let mut total_steps:u32 = 0;
    //println!("{}", use_slow);
    for i in start..=end {
        //println!("{}", i);
        let steps:Vec<i64> = collatz(i, false, &known_good, true);
        total_steps += steps.len() as u32;
        if !use_slow {known_good.extend(steps);}
    }
    return total_steps;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {collatz(string_to_int(args[1].clone()), true, &HashSet::from([1]), false);}
        3 => {println!("Finished in {} steps.", collatz_ranged(string_to_int(args[1].clone()), string_to_int(args[2].clone()), env::var("USE_SLOW").unwrap_or_default() == "true"))},
        _ => {eprintln!("Bad arguments."); return ExitCode::from(64);}
    }

    return ExitCode::SUCCESS;
}
