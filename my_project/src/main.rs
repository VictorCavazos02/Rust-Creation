//Instructions:
//1. Replace STUDENTNAME with your actual name
//2. Compile and run the program
//3. Compare debug vs release builds
//4. Use stat commmand to analyze the binaries



fn say_hi(x:i32){
    println!("Hi John! My favoirite number is {}", x)
}

fn double(x:i32) -> i32 {
    //return x*2;
    //x*2 //also acceptable
    let y = 10;
    x*2*y
}
    
    


fn main() {
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
