use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const NUM_INJECTIONS: usize = 240;
const NUM_RATS: usize = 5;

fn get_schedule(injections: usize) -> ([bool; NUM_RATS], [bool; NUM_RATS]) {

    let binary_code = format!("{:010b}", injections);
    let mut day1 = [false; NUM_RATS];
    let mut day2 = [false; NUM_RATS];

    for (i, c) in binary_code.chars().take(5).enumerate() {
        if c == '1' {
            day1[i] = true;
        }
    }
    for (i, c) in binary_code.chars().skip(5).take(5).enumerate() {
        if c == '1' {
            day2[i] = true;
        }
    }
    println!("Debug: injection {} as binary: {}", injections, binary_code);
    (day1,day2)
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let injection: usize = if args.len() > 1 {
        match args[1].parse::<usize>() {
            Ok(n) if n < NUM_INJECTIONS => n,
            _ => {
                eprintln!(
                    "Please provide a valid injection umber btween 0 and {}.",
                    NUM_INJECTIONS - 1
                );
                return;
            }
        }
    } else {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let seed = since_epoch.as_secs();
        let random_injection = (seed % NUM_INJECTIONS as u64) as usize;
        println!(
            "No injection number provided. Randomly selected injection: {}",
            random_injection
        );
        random_injection
    };

    

    let (day1, day2) = get_schedule(injection);

    println!("\nSchedule for each rat:");
    println!("{:<8} {:<16} {:<16}", "Rat", "Day 1 (bit)", "Day 2 (bit)");
    for i in 0..NUM_RATS {
        println!("Rat {:<4} {:<16} {:<16}:", i, day1[i], day2[i]);
    }

    let faint_day1 = day1;
    let faint_day2 = day2;

    println!("\nResults:");
    println!("Rats that fainted on Day 1:");
    for (i, &fainted) in faint_day1.iter().enumerate() {
        if fainted {
            println!(" Rat {}", i);
        }
    }
    println!("Rats that fainted on Day 2:");
    for (i, &fainted) in faint_day2.iter().enumerate() {
        if fainted {
            println!(" Rat {}", i);
        }
    }


    let mut recovered_binary = String::new();
    for &fainted in &faint_day1 {
        recovered_binary.push(if fainted { '1' } else { '0' });
    }
    for &fainted in &faint_day2 {
        recovered_binary.push(if fainted { '1' } else { '0' });
    }

    let recovered_injection = usize::from_str_radix(&recovered_binary, 2).unwrap();

    println!("\nRecovered binary: {}", recovered_binary);
    println!("Identified injection with anaesthesia: {}", recovered_injection);

    if recovered_injection == injection {
        println!("\nSuccess: The correct injection was identified!");
    } else {
        println!("\nError: The recovered injection number does not match the expected value.");
    }
}
