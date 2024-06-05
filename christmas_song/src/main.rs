fn main() {
    let months_lines = [
        "",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings!",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Eight maids a milkin'",
        "Nine ladies dancin'",
        "Ten lords a leapin'",
        "Eleven pipers pipin'",
        "Twelve drummers drummin'"
    ];

    let cardinal_to_ordinal = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for month in 1..12+1 {
        if month > 1 {
            println!("\n{}", months_lines[month - 1]);
        }

        println!("On the {} day of Christmas{}", 
            cardinal_to_ordinal[month-1],
            if month == 4 {" (what's a calling bird)"} else {""}
        );

        println!("My true love sent to me");

        if month > 1 {
            for i in (2..month + 1).rev() {
                println!("{}{}",
                months_lines[i - 1],
                if month == 8 && i == 4 {" (calling birds)"} else {""}
            );
            }
        }
        println!("{} partridge in a pear tree!",
            if month == 1 {"A"} else {"And a"}
        );
    }
}
