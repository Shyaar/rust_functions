fn main(){
    let name = String::from("Boris");
    drop(name);

    println!("{name}");
}