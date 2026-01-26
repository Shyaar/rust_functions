
// fn take(vec: Vec<i32>){
//     println!("this is vec {vec:?}");
// }

// fn give()-> String {
//     String::from ("John doe")
// }

// fn take_give(mut _name: String ) -> String{
//     _name = String::from ("Jane Doe:");
//     _name
// }


// fn stack(mut age: i32){
//     age = 24;
//     println!("this is age {age}")
// }
// fn main(){
//     // ownership in functions

//     //functions that take ownership
//     // when we pass a heap value to a function, it has the same effect as passing a heap value to another variable and the value is moved to the new function

//     let vector: Vec<i32> = vec![1,2,3,4,5];

//     // take(vector);
//     // can fix by clone

//     take(vector.clone());
//     let c = vector.clone();

//     println!("{vector:?}");

//     // // functions that give ownership back
//     let name_of = give();
//     println!("this is name {name_of}");

//     // // functions that take and return ownership
//     let name:String = take_give(name_of.clone());

//     println!("this is name of{name_of}");
//     println!("this is name {name}");

//     // functions that take and give ownership when the take ownership, the ownership is no longer available for use, same as covered... thus you must clone or refenence


//     // stack only functions/ownership
//     let age = 43;
//     stack(age);
//     println!("this is initial age {age}");

// }


fn main(){
    //borrowing
    // when borrowing, you can only have one mutable reference or multiple immutable reference
    // a reference it identified by &

    let mut x: Vec<i32> = vec![1,2,3,4,5];

    let y = &mut x;
    let z = &mut x;

    println!("this is y {y}");
    println!("this is z")  
}