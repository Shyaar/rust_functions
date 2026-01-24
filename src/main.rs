fn main(){
    // associativity
    // when same operators are successive and follow each other, rust handles it from left to right meaning, it evaluate the first 2, then evaluates its value with the next

    let a = 8/2/2;
    //this equals (8/2)/2 

    // when we use assignment, it evaluates from right to left... meaning it evaluates the value first before the variable
    let b = 40;
    // first it evelaute 40 before it evaluates b as = 40

    // thus when we do b=y=0 because y=0 is evaluated first and y = 0 being a statement returns a unit type, b throws a mismatch error,

    
}