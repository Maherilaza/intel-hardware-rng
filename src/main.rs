use colored::Colorize;
use core::arch::x86_64::_rdrand64_step;
use log::{error, info, warn};
use serde::Serialize;
use std::{fs::File, io::Write};

#[derive(Serialize)]
struct RandomNumbers {
    numbers: Vec<u64>,
}

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    check_arch();

    let mut random_numbers = RandomNumbers { numbers: vec![] };

    for _ in 0..100 {
        let random_number: Result<u64, String> = gen_rand();

        if let Ok(nb_rand) = random_number {
            // println!("{} {}", "[+]".green(), nb_rand);
            random_numbers.numbers.push(nb_rand);
        }
    }

    match save_as_json(&random_numbers, "rand.json") {
        Ok(_) => info!("Random numbers have been successfully recorded in the file 'rand.json'"),
        Err(e) => error!("Failed to save JSON file: {}", e),
    }
}

fn gen_rand() -> Result<u64, String> {
    let mut random: u64 = 0;
    unsafe {
        let b_check: i32 = _rdrand64_step(&mut random);
        if b_check != 1 as i32 {
            return Err(format!("{}", "[-] Cannot generate a random number".red()));
        }
        Ok(random)
    }
}

fn check_arch() {
    info!("Checking system architecture: x86_64");
    if cfg!(target_arch = "x86") || cfg!(target_arch = "x86_64") {
        info!("System architecture detected: x86_64.");
    } else {
        error!("Error: This program requires Intel x86 or x86_64 architecture");
    }
}

fn save_as_json(data: &RandomNumbers, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}
