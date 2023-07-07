
pub fn main(){

    for day in 0..12 {
        println!("On the {} day of christmas my true love gave to me", DAYS[day]);
        for gift in 0..=day {
            println!("{}", GIFTS[gift]);
        }
        println!();
    }

}

static GIFTS: [&str; 12] = 
    [ "A partridge in a pear tree" 
    , "Two turtle doves" 
    , "Three French hens"
    , "Four calling birds"
    , "Five gold rings"
    , "Six geese a-laying"
    , "Seven swans a-swimming"
    , "Eight maids a-milking"
    , "Nine ladies dancing"
    , "Ten lords a-leaping"
    , "Eleven pipers piping"
    , "Twelve drummers drumming"
];

static DAYS: [&str; 12] = 
    [ "First"
    , "Second"
    , "Third"
    , "Fourth"
    , "Fith"
    , "Sixth"
    , "Seventh"
    , "Eigth"
    , "Ninth"
    , "Tenth"
    , "Eleventh"
    , "Twelfth"
];