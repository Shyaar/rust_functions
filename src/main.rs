// ownership in rust 

fn main(){
    // in rust, each value has a variable as it owner
    // there can only be one owner of a (heap) value, thus two variable cannot own the same value;
    // if the owner goes out of scope, the value is dropped

    let a =String::from ("given");
    let b =a;
    let c = b;

    // here, strings are stored on the heap, when we move owner from a to b, a is no longer accessoble on the heap, be now owns the value give. same goes when the value is moved from b to c

    // println!("this is a {a}, this is b {b}");

    // to solve this we can clone or borrow;;

    let z =String::from("read");
    let y = z.clone();

    // this would clone z into y thus having different allocations of the same value in the heap;; or

    let x = &z;


    // values also only live within their scope//

    {
        let w = z;
    }

    // let e = w; we get this error cannot find value `w` in this scope:: because after the block scope, w is droped and inaccessible



  


}