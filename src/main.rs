fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "evelenth", "twelfth",
    ];

    let phrases = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let mut body = Vec::new();

    let tail = "A partridge in a pear tree.\n";

    for (i, day) in days.iter().enumerate() {
        println!("{}", set_day_in_head(day));
        print_body(&mut body);
        println!("{}", tail);

        if i < phrases.len() {
            push_phrase_to_body(&mut body, phrases[i])
        }
    }
}

fn set_day_in_head(day: &str) -> String {
    format!("On the {} day of Christmas,\nmy true love gave to me", day)
}

fn push_phrase_to_body<'a>(current_body: &mut Vec<&'a str>, phrase_to_push: &'a str) {
    current_body.push(phrase_to_push)
}

fn print_body(body: &Vec<&str>) {
    for phrase in body.iter().rev() {
        println!("{}", phrase)
    }
}
