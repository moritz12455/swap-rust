fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love sent to me:", ordinals[day]);

        for gift in (0..=day).rev() {
            if gift == 0 && day > 0 {
                println!("And a partridge in a pear tree");
            } else {
                println!("{}", gifts[gift]);
            }
        }

        println!();
    }
}
