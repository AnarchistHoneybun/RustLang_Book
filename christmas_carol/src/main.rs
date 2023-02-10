fn main() {
    let n: u8 = 12;
    let mut i: u8 = 1;

    while i <= n {

        // ? how to print the number with the correct suffix
        // ? (1st, 2nd, 3rd, 4th, 5th, 6th, 7th, 8th, 9th, 10th, 11th, 12th)
        // ? without using a match statement
        println!("On the {} day of Christmas my true love sent to me", i);
        match i {
            1 => println!("A partridge in a pear tree"),
            2 => println!("Two turtle doves"),
            3 => println!("Three french hens"),
            4 => println!("Four calling birds"),
            5 => println!("Five golden rings"),
            6 => println!("Six geese a-laying"),
            7 => println!("Seven swans a-swimming"),
            8 => println!("Eight maids a-milking"),
            9 => println!("Nine ladies dancing"),
            10 => println!("Ten lords a-leaping"),
            11 => println!("Eleven pipers piping"),
            12 => println!("Twelve drummers drumming"),
            _ => println!(""),
        }
        i += 1;
    }
}
