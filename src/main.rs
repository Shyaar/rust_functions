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

    let mut _name = String::from("Henry");
    _name.push_str(" John");
    let country = "denva";

    println!("{_name}");

    let ref1 = &vec1;

    take_in_borrow(&ref1);
    let _ref2 = &mut vec1;

    println!("{vec1:?}");

    let email = String:: from("john");
    format_mail(email);

    //functions that mutably borrow
    
}
