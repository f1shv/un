use ar::convert;
use ar::string_to_target;

fn main() {
    let mut args = std::env::args();
    // println!("{:?}", args);
    // println!("{}", args.len());
    if args.len() < 3 {
        usage();      

        std::process::exit(0);
    }
    let Some(arg1) = args.nth(1) else {unreachable!()};
    let (quantity, unit_str) = if let Ok(quantity) = arg1.parse::<f64>() {
        if args.len() < 2 {
            usage();
            std::process::exit(0);
        }
        (quantity, args.nth(0).unwrap())
    } else {
        let mut found_unit = false;
        let mut val_buffer = "".to_string();
        let mut unit_buffer = "".to_string();

        for c in arg1.chars() {
            if !(c.is_alphanumeric() || c=='.'){
                println!("invalid char {:?}", c);
                invalid_input(&arg1);
                std::process::exit(0);
            }

            if found_unit {
                if c.is_alphabetic() {
                    unit_buffer.push(c);
                }else{
                    invalid_input(&arg1);
                    std::process::exit(0);
                }
            }else {
                if c.is_alphabetic(){
                    found_unit = true;
                    unit_buffer.push(c);
                } else {
                    val_buffer.push(c);
                }
            }

        }

        let val_buffer_f64 = val_buffer.parse::<f64>();
        if val_buffer_f64.is_err() {
            invalid_input(&val_buffer);
            std::process::exit(0);
        }
        (val_buffer_f64.unwrap(), unit_buffer)
    };
    let unit = string_to_target(unit_str.clone());
    // let mut targets = vec![];
    while let Some(target) = args.nth(0) {
        print!("{:?}{} ", convert(quantity, &unit.clone().unwrap(), &string_to_target(target.clone()).unwrap()).unwrap(), target);
        // targets.push(string_to_target(target).unwrap());
    }
    println!()
}

fn usage() {
    println!("Usage: ark <quantity><unit> <target-unit> ...\n");
    // println!("OR\n");
    // println!("Usage: ark <quantity> <unit> <target-unit> ...\n");
    
    println!("Input: ark 20km cm");
    println!("Output: 2000000cm");

    println!("Input: ark 20km cm m");
    println!("Output: 2000000cm 20000m");

    println!("Input: ark 20 km cm m");
    println!("Output: 2000000cm 20000m");    
}

fn invalid_input(arg: &str) {
    println!("Invalid input {}", arg);
}