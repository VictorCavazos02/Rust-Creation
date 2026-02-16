

fn say_hi(x:i32){
    println!("Hi John! My favoirite number is {}", x)
}

fn double(x:i32) -> i32 {
    //return x*2;
    //x*2 //also acceptable
    let y = 10;
    x*2*y
}

fn fahrenheit_to_celsius(f:f64) -> f64{
    return (f - 32.0) * (5.0/9.0) as f64;
}

fn celsius_to_fahrenheit(c:f64) -> f64{
    return (c + 32.0) * (9.0 /5.0 ) as f64;
}

fn is_even(n: i32) -> bool{
    return n % 2 == 0
}
    


fn main() {
    let freezing_water_fahr:f64 = 32.0;
    let mut _temp_changing:f64 = 32.0;
    println!("{} \n", fahrenheit_to_celsius(freezing_water_fahr as f64) as f64);

    while _temp_changing < 38.0{
        println!("Number {} degrees fahrenheit converted to celsius is: {} degrees\n", _temp_changing, fahrenheit_to_celsius(_temp_changing as f64) as f64);
        println!("Number {} degrees celsius converted to fahrenheit is: {} degrees\n", _temp_changing, celsius_to_fahrenheit(_temp_changing as f64) as f64);
        _temp_changing += 1.0;
    }

    if is_even(10){
        println!("10 is even");
    }else{
        println!("10 is not even");
    }
    println!("Hello VICTOR CAVAZOS!");

    //let mut x = 10; // int -> int8, int16, int32, int 64, int128
    //x += 10;

    /*println!("5*2 = {}",x);

    let mut result : f32 = 0.0;
    let y:i32 = 5; //float
    result = result + y as f32; //no implicit conversion results in error

    
    println!("{}", result);
    

    let x:i32 = 5;

    let x:f32 = 1.012 + x as f32;

    println!("{}",x);
    */

    say_hi(5);
    println!("Double {} is = {}",5,double(5));

}
