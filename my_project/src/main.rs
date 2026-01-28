//Instructions:
//1. Replace STUDENTNAME with your actual name
//2. Compile and run the program
//3. Compare debug vs release builds
//4. Use stat commmand to analyze the binaries



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

    
    // Shadowing
    let x = 5;
    let x = x + 1;  // Creates a new variable
    
    // Mutation
    let mut y = 5;
    y = y + 1;  // Modifies the existing variable
    
    println!("x: {}, y: {}", x, y);


}
