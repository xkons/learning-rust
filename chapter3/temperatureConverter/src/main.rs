use std::io;

fn get_conversion_unit_input() -> TemperatureUnit {
    println!("Which conversion would you like to do? (Choose 1 or 2)");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    let choice: u32 = loop {
        let mut conversion_input = String::new();
        io::stdin()
        .read_line(&mut conversion_input)
        .expect("Failed to read line");

        match conversion_input.trim().parse() {
            Ok(choice) => {
                if choice == 1 || choice == 2 {
                    break choice
                } else {
                    println!("You have to choose either 1 or 2. Please choose again:");
                    continue
                    
                }
            },
            Err(_) => {
                println!("You did not enter a number. Please choose between 1 and 2:");
                continue
            },
        };
    };
    if choice == 1 {
        return TemperatureUnit::Celsius;
    } else {
        return TemperatureUnit::Fahrenheit;
    }
}

fn get_degree_input(unit: TemperatureUnit) -> f32 {
    match unit {
        TemperatureUnit::Celsius => {
            println!("Please enter your Celsius value:");
        },
        TemperatureUnit::Fahrenheit => {
            println!("Please enter your Fahrenheit value:")
        }
    }
    let degrees: f32 = loop {
        let mut degrees_input = String::new();
        io::stdin()
        .read_line(&mut degrees_input)
        .expect("Failed to read line");

        match degrees_input.trim().parse::<f32>() {
            Ok(degrees) => break degrees,
            Err(_) => {
                println!("You did not enter a signed float number. Please try again:");
                continue
            },
        };
    };
    return degrees
}

#[derive(Copy, Clone)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

fn main() {
    println!("Welcome to the Rust temperature converter!");

    let original_unit: TemperatureUnit = get_conversion_unit_input();
    let degrees_input: f32 = get_degree_input(original_unit);

    match original_unit {
        TemperatureUnit::Celsius => {
            let fahrenheit_value: f32 = (degrees_input * 1.8) + 32.0;
            println!("{} degrees celsius equals {:.2} degrees Fahrenheit", degrees_input, fahrenheit_value);
        },
        TemperatureUnit::Fahrenheit => {
            let celsius_value: f32 = (degrees_input - 32.0) * 5./9.;
            println!("{} degrees celsius equals {:.2} degrees Fahrenheit", degrees_input, celsius_value);
        }
    }
}
