use std::io;


fn main() {
    // Endless loop
    loop {
        println!("Wanna do a loop? y/n");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        println!("Input: {:?}", input);
        input = input.to_lowercase();
        println!("Lowercase: {:?}", input);
        let trimmed = input.trim();
        println!("Trimmed: {:?}", trimmed);
        
        match trimmed {
            "y" => {},
            "n" => {println!("byebye :)"); break;}
            _   => println!("Invalid input")
        }
    }
    
}
