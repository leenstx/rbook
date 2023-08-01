use std::io;

pub fn wait_user_input_a_number(min: Option<i32>, max: Option<i32>) -> i32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input an number");
                continue;
            }
        };
        if min.is_some() && min.unwrap() > number {
            println!("Please input an number must great than {}", min.unwrap());
            continue;
        }
        if max.is_some() && max.unwrap() < number {
            println!("Please input an number must less than {}", max.unwrap());
            continue;
        }
        return number;
    }
}
