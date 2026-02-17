
/*

fn say_hi(x:i32){
    println!("Hi John! My favoirite number is {}", x)
}

fn double(x:i32) -> i32 {
    //return x*2;
    //x*2 //also acceptable
    let y = 10;
    x*2*y
}
*/

fn fahrenheit_to_celsius(f:f64) -> f64{
    return (f - 32.0) * (5.0/9.0) as f64;
}

fn celsius_to_fahrenheit(c:f64) -> f64{
    return (c + 32.0) * (9.0 /5.0 ) as f64;
}

fn is_even(n: i32) -> bool{
    return n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret {
        return 0 as i32;
    }else if guess > secret{
        return 1 as i32;
    }else{
        return -1 as i32;
    }
}
    


fn main() {



    // Part 1--------------------------------------------------------------------------------------
    
    //data
    let freezing_water_fahr:f64 = 32.0;
    let mut _temp_changing:f64 = 32.0;
    //test with freezing point of water
    println!("{} \n", fahrenheit_to_celsius(freezing_water_fahr as f64) as f64);

    //actual tests of function
    while _temp_changing < 38.0{
        println!("Number {} degrees fahrenheit converted to celsius is: {} degrees\n", _temp_changing, fahrenheit_to_celsius(_temp_changing as f64) as f64);
        println!("Number {} degrees celsius converted to fahrenheit is: {} degrees\n", _temp_changing, celsius_to_fahrenheit(_temp_changing as f64) as f64);
        _temp_changing += 1.0;
    }


    // Part 2--------------------------------------------------------------------------------------

    //Data
    let array_num = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    
    //Actual tests with function *step 3
    for idx in 0..(array_num.len()){
        if is_even(array_num[idx]){
            println!("{} is even", array_num[idx]);
        }else{
            print!("{} is odd ", array_num[idx]);
            if (array_num[idx] % 5 == 0) && (array_num[idx] % 3 == 0){
                println!("FizzBuzz");
            }else if array_num[idx] % 3 == 0{
                println!("Fizz");
            }else if array_num[idx] % 5 == 0{
                println!("Buzz");
            }
        }
    }

    //Sum of numbers *step 4
    let mut index = 0;
    let mut sum = 0;
    while index < array_num.len(){
        sum += array_num[index];
        index += 1;
    }

    println!("The sum of all numbers in the array of numbers is {}...", sum);

    //Print the largest number in the array *step 5
    let mut largest_num = 0;
    for idx in array_num{
        if idx > largest_num{
            largest_num = idx;
        }
    }
    println!("The largest number in the array is {}...", largest_num);

        // Part 3-------------------------------------------------------------------------------------
        let mut secret_number = 39;
        let hard_coded_guesses = [2, 100, 22, 80, 39, 90];
        let mut idx = 0;
        while idx < hard_coded_guesses.len(){
            if check_guess(hard_coded_guesses[idx] as i32, secret_number as i32) == 0{break;
            }else if check_guess(hard_coded_guesses[idx] as i32, secret_number as i32) == 1{println!("Your guess of {} was too high!", idx);
            }else if check_guess(hard_coded_guesses[idx] as i32, secret_number as i32) == -1{println!("Your guess of {} was too low!", idx);
            }else{println!("Error found during guess checking loop...");}
            idx += 1;
        }
        println!("Congratulations! It took you {} attempts to guess the secret number!", idx);

    
}
