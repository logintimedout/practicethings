struct Owner{
    test2: String,
    output: i32,
}

fn main(){
    let test2 = String::from("ee");
    let test3 = "test";
    let output = 42;
    let structe = Owner {test2, output};
    println!("{}",structe.output);
    let myarray = ["test","test1","test2"];
    let array2: [u8; 4]= [1,2,3,4];
    let array2slice = &array2[0..2];
    println!("{}", array2slice[1]);
    let vector1 = vec![1,2,3,4];
    let test: i32 = 10;
    let vector2 = vector1[1];
    checker(test);
    println!("{} {} {}", myarray[1].len(),array2[1],vector1[2]);
    println!("{}", isnegative(vector1[2]));
    println!("{}", structe.test2);
    stringlength(test3);
    let check2:u8 = 10;
    check(check2);
}
fn check(s: u8){
    for i in 0..s{
        println!("testing");
    }
}
fn checker(n: i32){
    if n > 10 {
        println!("test");
    } else {
        println!("Test");
    }
}

fn isnegative(n: u8) -> bool{
    n > 5
}

fn stringlength(s: &str) {
    if s.len() > 10{
        println!("Larger than 10");
    } else {
        println!("Smaller than 10");
    }
}
