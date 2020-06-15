use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use std::env;

const CONFIG_FILE: &str = "/home/vlad2001mfs/.config/cpuboostmgrrc";

#[derive(Debug)]
struct Config {
    pub update_time: Duration,
    pub max_temp: f32,
    pub min_temp: f32,
    pub cpu_boost_file: String,
    pub cpu_temp_file: String,
    pub is_log_temp: bool,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let mut file = File::open(path).expect(&format!("Failed to open config {}", path));
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read config");
        let lines: Vec<&str> = data.lines().collect();

        let mut cfg = Config::default();
        for line in lines {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let value_str = parts[1].trim();
                let value_i32_r = value_str.parse::<i32>();
                let value_bool_r = value_str.parse::<bool>();
                if let Ok(value_i32) = value_i32_r {
                    match name {
                        "update_time" => cfg.update_time = Duration::from_millis(value_i32 as u64),
                        "max_temp" => cfg.max_temp = value_i32 as f32,
                        "min_temp" => cfg.min_temp = value_i32 as f32,
                        _ => println!("Invalid config line: {}", line),
                    }
                }
                else if let Ok(value_bool) = value_bool_r {
                    match name {
                        "is_log_temp" => cfg.is_log_temp = value_bool,
                        _ => println!("Invalid config line: {}", line),
                    }
                }
                else {
                    match name {
                        "cpu_boost_file" => cfg.cpu_boost_file = value_str.to_owned(),
                        "cpu_temp_file" => cfg.cpu_temp_file = value_str.to_owned(),
                        _ => println!("Invalid config line: {}", line),
                    }
                }
            }
        }
        cfg
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            update_time: Duration::from_millis(1000),
            max_temp: 75.0,
            min_temp: 65.0,
            cpu_boost_file: "/sys/devices/system/cpu/cpufreq/boost".to_owned(),
            cpu_temp_file: "/sys/class/hwmon/hwmon3/temp2_input".to_owned(),
            is_log_temp: false
        }
    }
}

fn main() {
    println!("CPU Boost Manager daemon started");

    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);

    let cfg;
    if Path::new(CONFIG_FILE).is_file() {
        cfg = Config::from_file(CONFIG_FILE);
        println!("Loaded user config: {:?}", cfg);
    }
    else {
        cfg = Config::default();
        println!("Loaded default config: {:?}", cfg);
    }

    let mut state = true;
    loop {
        let mut temp_file = File::open(&cfg.cpu_temp_file).expect("Failed to open cpu temp file");
        let mut temp_str = String::new();
        temp_file.read_to_string(&mut temp_str).expect("Failed to read temperature from file");
        let temp = temp_str.trim().parse::<f32>().unwrap() / 1000.0;

        if temp > cfg.max_temp {
            state = set_cpu_boost(&cfg, false);
        }
        else if temp < cfg.min_temp {
            state = set_cpu_boost(&cfg, true);
        }

        if cfg.is_log_temp {
            println!("TEMP: {:.3} | CPU-BOOST: {}", temp, state);
        }

        std::thread::sleep(cfg.update_time);
    }
}

fn set_cpu_boost(cfg: &Config, state: bool) -> bool {
    let mut file = File::create(&cfg.cpu_boost_file).expect("Failed to open file");
    if state {
        file.write("1".as_bytes()).expect("Failed to write new state 1 to file");
    }
    else {
        file.write("0".as_bytes()).expect("Failed to write new state 0 to file");
    }

    state
}
