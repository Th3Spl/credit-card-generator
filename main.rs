
//Dep
use std::io;    // <-- Standard I/O lib
use rand::Rng;  // <-- Standard random lib

fn card_number_generator() -> String { //<-- we defined the function as a function which will return a String

    let mut card_number: String = String::new(); // This is the variable which will contain the card number
    let mut index2: i32 = 0; //Second index so we can add the -

    for _x in 0..16{ // This cycle will be repeated 16 times (we use the _ before the x so we can say that we're not gonna use it)

        if let 4 = index2 { //if the index2 count is = 4 
            card_number += "-"; //We add a - to the return string
            index2 = 0;         //So we give the valeu 0 to the variable cuz we will repeat this for 4 times
        }
        let num = rand::thread_rng().gen_range(0..9); //We want to generate a number between 0 and 9
        card_number += &num.to_string();                   //We have to assign the value to the string
        index2 += 1;                                       //We add 1 to the index2 variable
    }

    return card_number; //Here we're gonna return the result of the function that we've just written
}

fn cvv_generator() -> String {
    
    let mut cvv: String = String::new();

    for _x in 0..3 {
        let num: i32 = rand::thread_rng().gen_range(0..9);
        cvv += &num.to_string();
    }

    return cvv;
}

fn card_number_generator_with_options(card_type: String) -> String {


    let mut card_number: String = String::new(); // This is the variable which will contain the card number
    let mut index2: i32 = 1; //Second index so we can add the -

    if card_type.trim() == "1" {
        card_number += "2";
    }else if card_type.trim() == "2" {
        card_number += "4";
    }else if card_type.trim() == "3" {
        card_number += "3";
    }

    for _x in 0..15{ // This cycle will be repeated 16 times (we use the _ before the x so we can say that we're not gonna use it)

        if let 4 = index2 { //if the index2 count is = 4 
            card_number += "-"; //We add a - to the return string
            index2 = 0;         //So we give the valeu 0 to the variable cuz we will repeat this for 4 times
        }
        let num = rand::thread_rng().gen_range(0..9); //We want to generate a number between 0 and 9
        card_number += &num.to_string();                   //We have to assign the value to the string
        index2 += 1;                                       //We add 1 to the index2 variable
    }

    return card_number; //Here we're gonna return the result of the function that we've just written
}

fn main() { //This is the main function

    loop { //This is an infinite loop 

        let mut choise: String = String::new(); //This will be the input string

        //We have to print the menu
        println!("Choose between these 2 options:"); 
        println!("1 | Generate a new card number and cvv.");
        println!("2 | Generate new card number and cvv with options.");
        println!("3 | Exit the program.");

        //Here we're gonna get the input from the user
        io::stdin()
            .read_line(&mut choise)
            .expect("The input generated an error!");


        if choise.trim() == "1" { //if the coise is 1 then we will generate a new card number

            let card_number: String = card_number_generator(); //We call the function and then we assign it to the current card number variable
            let cvv: String = cvv_generator();                 //We call the function and then we assign it to the current cvv variable
            println!("\nResult:\nCard Number: {}", card_number);           //Then we print the result1
            println!("Cvv: {}\n", cvv);                                    //Then we print the result2

        }else if choise.trim() == "2" {

            println!("\nYou have to choose one of the following credit cards.");
            println!("1 | Mastercard.");
            println!("2 | Visa.");
            println!("3 | American Express.");

            let mut card_type: String = String::new();
            io::stdin()
                .read_line(&mut card_type)
                .expect("The input generated an error!");

            let card_number: String = card_number_generator_with_options(card_type);
            let card_cvv: String = cvv_generator();

            println!("\nResult: ");
            println!("Card Number: {}", card_number);
            println!("Card Cvv: {}\n", card_cvv);

        }else if choise.trim() == "3" {   //instead if the choise is equal to 2 we're gonna exit the program

            return; //This will make us exit the program

        }else { //Else

            println!("\nYour input doesn't match any command!"); //if the inserted string is not equal to any command then we'll return this error message
        }
    }

}
