use std::io;

fn main() { 

    let mut x = 10;
    x += 5;

    println!("x : {}",x);
    last_ques();
    
}



fn last_ques() {
    println!("Enter a number");
    let mut num = String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read the input");

    let x: i32 = num
        .trim() 
        .parse() 
        .expect("Please enter a valid number");

    let y = 3 * x * x + 2 * x + 1;
    println!("y: {}",y);

}