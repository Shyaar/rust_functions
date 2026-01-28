fn take_in_borrow(vac: &Vec<i32>) {
    let x = vac;
    println!("this is x {x:?}");
}

fn format_mail(email: String){
    let mut email = String::from (email);
    email.push('@');

    println!("{email}");

}
fn main() {
    //borrowing in functions

    //functions that immutably borrow

    let mut vec1 = vec![12, 34, 56];

    let ref1 = &vec1;

    take_in_borrow(&ref1);
    let ref2 = &mut vec1;

    println!("{vec1:?}");

    let email = String:: from("john");
    format_mail(email);

    //functions that mutably borrow
    
}
