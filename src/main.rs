// fn my_function(name: &str, age: i8){

//     println!("{name}, {age}")
// }

// fn return_data(job: &str) -> &str{
//     let user_job = job;
//     user_job
// }

// fn simple_maths(num1:i32, num2: i32) ->(i32, i32, i32){
//     (num1 +num2, num1-num2, num1*num2)
// }

// fn user_info(name: &str, age: i8, ) -> (&str, i8){
//     let info=(name, age);
//     info
// }


fn prnt_num (mut x: i32){

    x = x + 43;
    println!("{x}")
}


fn shad_x (x: i32){
    let mut x = x;

    x= x+2;
    println!("{x}")
}
fn main(){

    let  x = 25;

    prnt_num(x);

    shad_x(x);

}


// fn main() {
//     my_function("Jason", 28); 

//     let name="John";
//     let age = 28;
//     my_function(name, age); 

//     let job = return_data("mechanic");

//     println!("{job}");

//     let results = simple_maths(4, 2);

//     let (result1, result2, result3) = simple_maths(4, 2);

//     let student = user_info("John", 28);

//     println!("this is the first registered student{:?}", student);

//     println!("{result1}, {result2}, {result3}");

//     println!("{:?}",results);

//     let full_name: String = {
//         let first_name = "Jane";
//         let last_name = "Doe";

//         format!("{first_name} {last_name}")       
//     };

//     let add: i32 ={
//         let a = 5;
//         let b = 5;

//         a+b
//     };

//     println!("this is the value of add {}", add);

//     println!("{full_name}");



//     // practicing tuples arrays and vectors

//     // let user_info :(&str, i8, &str) = ("Doe", 28, "Lawyer");

//     // let user_name = user_info.0;
//     // let user_age = user_info.1;
//     // let user_job = user_info.2;

//     // println!("this is user name{}, this is user age {}, this is user jos {}", user_name, user_age,user_job);

//     // let (user_name1, user_age1, user_job1) = user_info;

//     // let user_information = user_info;
//     // println!("this is user info: {:?}", user_information);


//     // let scores:[i8; 5] = [1,2,3,4,5];

//     // println!("this are user info {user_name1}, {user_age1}, {user_job1}");

//     // println!("this is scores{:?}", scores);


//     // let score1: i8 = scores[0];
//     // let score2: i8 = scores[1];

//     // println!("this is score 1{score1}, this is score 2{score2}");

//     // let mut vector_data: Vec<i8> = vec![1,2,3,4,5,6,7];

//     // let vec1: i8 = vector_data[0];

//     // vector_data.push(3);

//     // let vec7 = vector_data[6];

//     // println!("this is vector data {:?}", vector_data,);

//     // println!("this is vec7 {vec7}");
// }
