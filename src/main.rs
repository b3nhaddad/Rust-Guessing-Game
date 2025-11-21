use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Guessing game");
        
        const MINIMUM: u32= 1;
        const MAXIMUM: u32= 10;

        let secret_num = rand::thread_rng().gen_range(MINIMUM..=MAXIMUM);

        
        println!("Secret number is between range {MINIMUM} to {MAXIMUM}"); 

        guess(secret_num);

	println!("You got it. Answer {secret_num}");
}

fn guess(secret_number : u32){
    
        
	println!("Please Input Your Guess.");
	let mut guess1 = String::new();
        
	std::io::stdin()
		.read_line(&mut guess1)
		.expect("Failed to read line");

        let guess1: u32 = match guess1.trim().parse(){
            Ok(num) => num,
            Err(_) => 0, 
        }; 
        match guess1.cmp(&secret_number){
        
            Ordering::Less => {println!("WRONG! \n Guess is too Low"); guess(secret_number);}, 
            
            Ordering::Greater => {println!("WRONG! \n Guess is too High"); guess(secret_number);}, 
            Ordering::Equal => println!("Hurray Huzz!"),
        }
}
