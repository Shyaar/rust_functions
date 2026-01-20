fn main(){

    //conditionals flow

    let num = 600;

    //if else
    if num < 50 {
        println!("this number is less than 50 {}", num);
    } else {
        println!("this number is greater than 50 {num}")
    };

    // if else if

    fn get_grade(s_score: i8, s_name: &str) -> (&str, char){

            let grade = if s_score > 90 {
                'A'
            } else if s_score >= 70 && s_score <90 {
                'B'
            } else if s_score >= 55 && s_score <80 {
                'C'
            }else if s_score >=40 && s_score < 55{
                'D'}else {
                'F'};

                (s_name, grade)
    }

    let student_1 = get_grade(50, "john");

    println!("this is student 1s grade {student_1:?}");


    let salary = 40;
    let mut category = 'A';

    match salary {
        90..=100 => category = 'A',
        80..=89 => category = 'B',
        70..=79 => category = 'C',
        _=> category = 'D'
    };

    println!("this is category {category}");

    let match_with_tuple = ('a','b');

    let value = match match_with_tuple {
        ('a', y) =>{ 
            println!("this has a, second value is {}",y);
    y
}
        (z, 'b') => {println!("this has b, first value is {}",z); z},
        (a,b) => {println!("this is both values {},{}", a,b); a}   
    };

    println!{"this is value{value:?}"};
    
}