
use hello::hi;

fn main() {
    variables();

    constants();

    scopes();

    shadowed();

    type_change();

    memory_safety();

    println!("{}",sum(10, 12));

    println!("{}",multiply(10, 12));

    hello::hello();

    hi();

}
fn multiply(x: i32,y: i32) -> i32{
    x*y
}
fn sum(x: i32,y: i32) -> i32{
    return x+y;
}

fn memory_safety() {
    // memory safety
    let new_num: i32;
    // we can not use the variable before initilizing
    // so print function will give error in compile time
    //println!("{}",new_num);

    // though we know that it will go to if statement 
    // but still it will give error if we try to use this way
    if true{
        // new_num=10;
    }
    // println!("{}",new_num);


    // but if we use if else then 
    // it will not give error as the compiler 
    // will know that in compile time either of 
    // one case will occur
    if 3>4{
        new_num=3;
    }else{
        new_num=4;
    }
    println!("{}",new_num);
}

fn type_change() {
    let number_value="12345";
    println!("{}",number_value);
    // we can also change the type of the variable in same scope
    let number_value=12345;
    println!("{}",number_value);
}

fn shadowed() {
    let mut x2=5;
    println!("{}",x2);
    x2=25;
    println!("{}",x2);
    let x2=x2;
    // here mutable x2 is shadowed to immutable x2
    // redefines the same variable with different mutibility
    // but this is done in compile time
    // so nothing actually happens in binary file
    println!("{}",x2);
}

fn scopes() {
    let x1 =10;
    {
        let y1=20;
        println!("{} {}",x1,y1);
    }
    //println!("{} {}",x1,y1); -> Error
}

fn constants() {
    const WORKER_COUNT:i32=5;
    println!("{}",WORKER_COUNT);
}

fn variables()  {
    let num=10;
    println!("{}",num);

    let (x,y)=(10,11);
    println!("{} {}",x,y);

    // num=32;
}
