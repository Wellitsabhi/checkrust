fn main() {
    let my_str = String::from("hello world");
    take_ownership(my_str);
    // println!("{}", my_fstr);     // This will give error, as it don't have ownership now. 
}

fn take_ownership(some_str: String){
    println!("{}", some_str);         // now, "some_str" own the data.
}