fn main() {
    //region Constants
    const DAYS_OF_CHRISTMAS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    const DAYS_OF_CHRISTMAS_DESCRIPTIONS: [&str; 12] = [
        "A partridge in a pear tree",

        "Two turtle doves, and",

        "Three french hens",

        "Four calling birds",

        "Five golden rings",

        "Six geese a-laying",

        "Seven swans a-swimming",

        "Eight maids a-milking",

        "Nine ladies dancing",

        "Ten lords a-leaping",

        "Eleven pipers piping",

        "Twelve drummers drumming"
    ];
    //endregion
    let mut current_day: usize = 1;
    println!("The Twelve Days of Christmas: \n");

    for day in DAYS_OF_CHRISTMAS.iter() {
        println!("On the {day} day of christmas my true love gave to me", day=day);

        for day_description in (1..=current_day).rev() {
            println!("{}", DAYS_OF_CHRISTMAS_DESCRIPTIONS[day_description - 1]);
        }

        println!();

        current_day += 1;
    }
}
