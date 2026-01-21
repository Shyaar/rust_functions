//control flow

fn main(){

//simple loop
    // loop{
    //     println!("this is loop");
    //     break;
    // }

    //for loop

    // let vector: Vec<i32> = vec![1,2,3,4,5];

    // for i in vector{
    //     println!("this is i {i}");
    // }

    //while loop

    // let mut num = 0;

    // while num<=5{
    //     num +=1;
    //     let num_loop = num;
    //     println!("this is i {num_loop}")
    // }

    //printing in acending order

    // for i in 0..=10 {
    //     println!("i={i}")
    // }

    // printing in decending order

    // range is only computed when start is less that end
    // a range would end when start becomes greater than or equals to end pending on .. or ..=

    // to print in reverse, we use the rev method

    // for i in (0..=10).rev(){
    //     println!("i = {i}")
    // }


    //iterating with a step size
    // for this we use a stepby method

    // for i in (0..=10).step_by(2){
    //     println!("i = {i}")
    // }


    //rust ranges cant handle floats
    // 0.0..=0.9 is invalid
    // to use floats in loops, you reassign i as a float

    // for i in 0..=8 {
    //     let i = i as f32 * 0.2;

    //     if i == 0 as f32 {
    //         continue
    //     } else {
    //         println!("this is i {i}")
    //     }
    // }


    //.. type is a valid type in rust

    let _r = 0..4;
    
    // let mut t: String = String::from ("shta");

    // t.push('t');

    // println!("this is t {t}")

    let pairs: Vec<(&str, i32)> = vec![("john", 32), ("mary",43)];

    let pairing:[(i32, i8); 2] = [(3,6),(4,5)];

    println!("this is array of arrays {:?}", pairing);

    for (name,age) in pairs{
        println!("{},{}", name, age)
    }
    
}