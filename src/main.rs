fn main(){
    let name = String::from("Boris");

    let person = &name;

    println!("{} , {}", name, person);

    println!("{}", *person);


}