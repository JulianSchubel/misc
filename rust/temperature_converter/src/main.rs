use std::io::{self, Write};

const KELVIN_CELSIUS_OFFSET: f64 = 273.15;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * (5.0 / 9.0) + KELVIN_CELSIUS_OFFSET
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return (celsius * 9.0 / 5.0) + 32.0
}

/* 0 celsius is 273.15 kelvin */
fn celsius_to_kelvin(celsius: f64) -> f64 {
    return celsius + KELVIN_CELSIUS_OFFSET;
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    return ((kelvin - KELVIN_CELSIUS_OFFSET) * 9.0 / 5.0) + 32.0
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    return kelvin - KELVIN_CELSIUS_OFFSET
}

fn main() {
    const TITLE: &str = r#"
 _____                                   _                  
|_   _|__ _ __ ___  _ __   ___ _ __ __ _| |_ _   _ _ __ ___ 
  | |/ _ \ '_ ` _ \| '_ \ / _ \ '__/ _` | __| | | | '__/ _ \
  | |  __/ | | | | | |_) |  __/ | | (_| | |_| |_| | | |  __/
  |_|\___|_| |_| |_| .__/ \___|_|  \__,_|\__|\__,_|_|  \___|
                   |_|                                      
  ____                          _            
 / ___|___  _ ____   _____ _ __| |_ ___ _ __ 
| |   / _ \| '_ \ \ / / _ \ '__| __/ _ \ '__|
| |__| (_) | | | \ V /  __/ |  | ||  __/ |   
 \____\___/|_| |_|\_/ \___|_|   \__\___|_|   

    "#;
    println!("{}", TITLE);
    let temperatures = [
        "Fahrenheit",
        "Celsius",
        "Kelvin"
    ];
    for (i, v) in temperatures.iter().enumerate() {
       println!("{i}: {v}");
    }

    'main_loop: loop {
        print!("Select input temperature unit: ");
        io::stdout()
            .flush()
            .expect("Failed to flush sdout");
        let mut input_unit = String::new();
        io::stdin()
            .read_line(&mut input_unit)
            .expect("Failed to read input");
        let input_unit: u8 = match input_unit.trim().parse() {
            Ok(n) => n,
            Err(_) => continue 'main_loop
        };

        print!("Select output temperature unit: ");
        io::stdout()
            .flush()
            .expect("Failed to flush sdout");
        let mut output_unit = String::new();
        io::stdin()
            .read_line(&mut output_unit)
            .expect("Failed to read input");
        let output_unit: u8 = match output_unit.trim().parse() {
            Ok(n) => if n != input_unit {n} else { continue 'main_loop },
            Err(_) => continue 'main_loop
        };

        print!("Input the temperature: ");
        io::stdout()
            .flush()
            .expect("Failed to flush sdout");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read input");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(n) => n,
            Err(_) => continue 'main_loop
        };


        let mut output_temperature: f64 = 0.0;
        /* input is fahrenheit */
        if let 0 = input_unit {
            output_temperature = match output_unit {
                1 => fahrenheit_to_celsius(temperature),
                2 => fahrenheit_to_kelvin(temperature),
                _ => continue 'main_loop
            }
        }

        /* input is celsius */
        if let 1 = input_unit {
            output_temperature = match output_unit {
                0 => celsius_to_fahrenheit(temperature),
                2 => celsius_to_kelvin(temperature),
                _ => continue 'main_loop
            }
        }

        /* input is kelvin */
        if let 2 = input_unit {
            output_temperature = match output_unit {
                0 => kelvin_to_fahrenheit(temperature),
                1 => kelvin_to_celsius(temperature),
                _ => continue 'main_loop
            }
        }

        println!("{} {} = {} {}\n", 
            temperature, 
            temperatures[input_unit as usize], 
            output_temperature, 
            temperatures[output_unit as usize]
        );
    }
}
