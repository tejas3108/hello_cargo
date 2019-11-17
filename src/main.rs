fn main() {
    let x= 43 / 5;
    println!("The value of x is {}", x);

    //now trying to initialise an array using the value of x
    let arr = vec![0;x];
    println!("The size of arr is {}", arr.len());

    //call helper method
    println!("{}", sample_printer(3,5));
}

fn sample_printer(num1:u32, num2:u32) -> u32{
    num1 + num2
}
