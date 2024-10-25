use std::io::{self, Write};

fn main() {
    let title: &str = r#"
 _____ _   _ _____   _____ ___ ____   ___  _   _    _    ____ ____ ___ 
|_   _| | | | ____| |  ___|_ _| __ ) / _ \| \ | |  / \  / ___/ ___|_ _|
  | | | |_| |  _|   | |_   | ||  _ \| | | |  \| | / _ \| |  | |    | | 
  | | |  _  | |___  |  _|  | || |_) | |_| | |\  |/ ___ \ |__| |___ | | 
  |_| |_| |_|_____| |_|   |___|____/ \___/|_| \_/_/   \_\____\____|___|
                                                                       
 ____  _____ ___  _   _ _____ _   _  ____ _____ 
/ ___|| ____/ _ \| | | | ____| \ | |/ ___| ____|
\___ \|  _|| | | | | | |  _| |  \| | |   |  _|  
 ___) | |__| |_| | |_| | |___| |\  | |___| |___ 
|____/|_____\__\_\\___/|_____|_| \_|\____|_____|
"#; 
    println!("{}", title);
    'main_loop: loop { 
        print!("Enter the desired term starting from 1: ");
        io::stdout().flush().expect("Failed to flush sdout");
        let mut term = String::new();
        io::stdin()
            .read_line(&mut term)
            .expect("Failed to read line");

        // must supply a data type here for .parse()
        let term: u64 = match term.trim().parse() {
            Ok(n) => if n >= 1 { n } else { continue 'main_loop},
            Err(_) => continue 'main_loop,
        };

        let mut m: u64 = 0;
        let mut n: u64 = 1;
        let mut fib: u64 = 1;

        if term <= 2 {
            println!("{}", term - 1);
        } else {
            for i in 1..term {
                let result: (u64, bool) = m.overflowing_add(n);
                if !result.1 {
                    fib = result.0;
                } else {
                    println!("Term value exceeds maximum representable size of (2^64)-1 = {}", u64::MAX);
                    continue 'main_loop;
                }
                n = m;
                m = fib;
            } 
            println!("{}", fib);
        }
    }
}
