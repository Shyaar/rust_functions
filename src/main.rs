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

    // let pairs: Vec<(&str, i32)> = vec![("john", 32), ("mary",43)];

    // let pairing:[(i32, i8); 2] = [(3,6),(4,5)];

    // println!("this is array of arrays {:?}", pairing);

    // for (name,age) in pairs{
    //     println!("{},{}", name, age)
    // }

    let n: i32 = 5;

    let mut square_of_sum = 0;
    let mut sum_of_square = 0;

    let mut sq = 0;
    let mut s=0;


   fn squar_sum(sum: i32)-> i32{
        sum ^2
    }

    square_of_sum = squar_sum(s);

    for i in 1..=n {
        s = s+i; 
        println!("this is s{s}");
    };

    for j in 1..n {
        sq = sq*2 + j*2;
        println!("this is sq{sq}");
    };



    // this is single line comment 
    // you have to start with a double slash at the begining of every line to comment 

    /*this is multi line comment
    it spans from the begining of the "/*" to "*/"
     */

    //print 

    // to print, there are 2 ways, 
    //print! which doesnt specify line to print and if there are multiple prints, it owuld print all on the same line

    print!("this is the line to print");
    print!("this is another different line to print");

    // the println! on the other hand would print each on a different line
    println!("this is the line to print");
    println!("this is another different line to print");

    
}