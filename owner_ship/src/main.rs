fn main() {
    let mut s = String::from("Hola");

    s.push_str(", yo soy Doraemon 😼");

    {
        let s = String::from("Hola, yo soy Nobita 😼");
        println!("{}", s);
    }

    println!("{}", s);
}
