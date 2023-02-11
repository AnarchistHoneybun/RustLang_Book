fn main() {
    
    let ref_to_none = dangler();
    
}

fn dangler() -> String {
    let s = String::from("hello");
    // ! This will cause a dangling pointer
    &s
}
