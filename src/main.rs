fn my_function(name: &str, age: i8){

    println!("{name}, {age}")
}

fn return_data(job: &str) -> &str{
    let user_job = job;
    user_job
}

fn simple_maths(num1:i32, num2: i32) ->(i32, i32, i32){
    (num1 +num2, num1-num2, num1*num2)
}

fn user_info(name: &str, age: i8, ) -> (&str, i8){
    let info=(name, age);
    info
}

fn main() {
    my_function("Jason", 28); 

    let name="John";
    let age = 28;
    my_function(name, age); 

    let job = return_data("mechanic");

    println!("{job}");

    let results = simple_maths(4, 2);

    let (result1, result2, result3) = simple_maths(4, 2);

    let student = user_info("John", 28);

    println!("this is the first registered student{:?}", student);

    println!("{result1}, {result2}, {result3}");

    println!("{:?}",results);

    let full_name: String = {
        let first_name = "Jane";
        let last_name = "Doe";

        format!("{first_name} {last_name}")       
    };

    println!("{full_name}")
}
