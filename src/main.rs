extern crate systemstat;

use std::{thread};
use std::time::Duration;
use systemstat::{ByteSize, Platform, System};

fn main() {
    let sys = System::new();

    println!("Debug:");
    let bat_score = score_bat(&sys);
    let mem_score = score_mem(&sys);
    let cpu_score = score_cpu(&sys);

    let avg = [bat_score, mem_score, cpu_score];
    let sum: u32 = Iterator::sum(avg.iter());
    let avg = u32::from(sum) / (avg.len() as u32);

    println!("\nScores:");
    println!("- Average Score: {}", name_score(&avg));
    println!("- Battery Score: {}", name_score(&bat_score));
    println!("- Memory Score:  {}", name_score(&mem_score));
    println!("- CPU Score:     {}", name_score(&cpu_score));

    // match sys.uptime() {
    //     Ok(uptime) => println!("\nUptime: {:?}", uptime),
    //     Err(x) => println!("\nUptime: error: {}", x),
    // }

    // match sys.cpu_temp() {
    //     Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
    //     Err(x) => println!("\nCPU temp: {}", x),
    // }
}

fn score_bat(sys: &System) -> u32 {
    match sys.battery_life() {
        Ok(bat) => {
            let bat = bat.remaining_capacity * 100.0;
            println!("- BAT: {}", bat); // DEBUG INFO
            return get_score(&(bat as u32));
        }
        Err(_) => panic!("Couldn't get Battery usage"),
    }
}

fn score_cpu(sys: &System) -> u32 {
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("- CPU: {}", cpu.idle * 100.0); // DEBUG INFO
            let cpu = (cpu.idle * 100.0) as u32;

            return get_score(&cpu);
        }
        Err(_) => panic!("Couldn't get CPU usage"),
    }
}

fn score_mem(sys: &System) -> u32 {
    match sys.memory() {
        Ok(mem) => {
            println!("- MEM: {} / {}", ByteSize::b(mem.total.as_u64() - mem.free.as_u64()), mem.total); // DEBUG INFO
            let mem = mem.free.as_u64() * 100 / mem.total.as_u64();
            return get_score(&(mem as u32));
        }
        Err(_) => panic!("Couldn't get Memory usage."),
    }
}

fn get_score(percent: &u32) -> u32 {
    return match 100 - percent {
        0..=10 => 1,   // Perfect
        11..=33 => 2,  // Good
        34..=50 => 3,  // Fair
        51..=75 => 4,  // Poor
        76..=90 => 5,  // Bad
        91..=100 => 6, // Awful
        _ => panic!("Out of scoring range."),
    };
}

fn name_score(score: &u32) -> String {
    return match score {
        1 => String::from("Perfect"),
        2 => String::from("Good"),
        3 => String::from("Fair"),
        4 => String::from("Poor"),
        5 => String::from("Bad"),
        6 => String::from("Awful"),
        _ => panic!("Out of scoring range."),
    };
}
