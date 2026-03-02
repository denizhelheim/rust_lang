fn main(){
    println!("Hello, World!"); //BASIC SYNTAX ----

    //this is a comment ----

    let number1 = 5; //THIS VARIABLE CANNOT BE CHANGED. ----
    let mut number2 = 6; //THIS VARIABLE CAN BE CHANGED. ----

    

    println!("Hello {} World!", number1);
    
    println!("Hello {} World!", number2);

    number2 = 17;
    println!("Hello {} World!", number2);

    // VARIABLE TYPES ----
    let my_num: i32 = 5; // integer
    let my_double: f64 = 3.14; // float
    let my_letter: char = 'A'; // character
    let my_bool: bool = true; // boolean
    let my_text: &str = "Hello"; // string


    // CONSTANTS
    const BIRTHYEAR: i32  = 1980;
    const MINUTESPERHOUR: i32 = 60; // CONSTANTS MUST HAVE A TYPE
    // const SUTLACISSOCUTE = 999, ERROR: MISSING TYPE

    // IF-ELSE STATEMENTS
    let is_logged_in: bool = true;
    
    if is_logged_in{
        println!("You have not logged in yet");
    } else{
        println!("You have logged in!");
    }


    if 7 > 5{
        println!("7 is greated than 5");
    }
}