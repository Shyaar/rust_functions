
fn take(vec: Vec<i32>){
    println!("this is vec {vec:?}");
}

fn give()-> String {
    String::from ("John doe")
}

// fn take_give(mut name: &str) -> &str{
//     name = "firstname: ";
// }

fn main(){
    // ownership in functions

    //functions that take ownership
    // when we pass a heap value to a function, it has the same effect as passing a heap value to another variable and the value is moved to the new function

    let vector: Vec<i32> = vec![1,2,3,4,5];

    // take(vector);
    // can fix by clone

    take(vector.clone());
    let c = vector.clone();

    println!("{vector:?}");

    // // functions that give ownership back
    let name_of = give();
    println!("this is name {name_of}");

    // // functions that take and return ownership
    // let mut name = "name";

    // name = take_give(name);

    


}