
fn take(vec: Vec<i32>){
    println!("this is vec {vec:?}");
}

fn give()-> &str {
    "name"
}

fn main(){
    // ownership in functions

    //functions that take ownership
    // when we pass a heap value to a function, it has the same effect as passing a heap value to another variable and the value is moved to the new function

    let vector: Vec<i32> = vec![1,2,3,4,5];

    // take(vector);
    // can fix by clone

    take(vector.clone());
    let c = vector;

    // functions that give ownership back
    let name_of = give()
    println!("this is name {name_of}")



}