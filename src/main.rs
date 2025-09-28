// Using BigInts here so we can do **very** big numbers
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
// So we can get command arguments
use std::env;

// List of English words up to twenty (20). Includes the teens bc i didn't want to make another list
const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
// I include zero in all of them to make it more readable, though it's not necessary
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const POWERS: [&str; 102] = [
    "zero",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
    "octillion",
    "nonillion",
    "decillion",
    "undecillion",
    "duodecillion",
    "tredecillion",
    "quattuordecillion",
    "quindecillion",
    "sexdecillion",
    "septendecillion",
    "octodecillion",
    "novemdecillion",
    "vigintillion",
    "unvigintillion",
    "duovigintillion",
    "trevigintillion",
    "quattuorvigintillion",
    "quinvigintillion",
    "sexvigintillion",
    "septenvigintillion",
    "octovigintillion",
    "novemvigintillion",
    "trigintillion",
    "untrigintillion",
    "duotrigintillion",
    "tretrigintillion",
    "quattuortrigintillion",
    "quintrigintillion",
    "sextrigintillion",
    "septentrigintillion",
    "octotrigintillion",
    "novemtrigintillion",
    "quadragintillion",
    "unquadragintillion",
    "duoquadragintillion",
    "trequadragintillion",
    "quattuorquadragintillion",
    "quinquadragintillion",
    "sexquadragintillion",
    "septenquadragintillion",
    "octoquadragintillion",
    "novemquadragintillion",
    "quinquagintillion",
    "unquinquagintillion",
    "duoquinquagintillion",
    "trequinquagintillion",
    "quattuorquinquagintillion",
    "quinquinquagintillion",
    "sexquinquagintillion",
    "septenquinquagintillion",
    "octoquinquagintillion",
    "novemquinquagintillion",
    "sexagintillion",
    "unsexagintillion",
    "duosexagintillion",
    "tresexagintillion",
    "quattuorsexagintillion",
    "quinsexagintillion",
    "sexsexagintillion",
    "septsexagintillion",
    "octosexagintillion",
    "novemsexagintillion",
    "septuagintillion",
    "unseptuagintillion",
    "duoseptuagintillion",
    "treseptuagintillion",
    "quattuorseptuagintillion",
    "quinseptuagintillion",
    "sexseptuagintillion",
    "septseptuagintillion",
    "octoseptuagintillion",
    "novemseptuagintillion",
    "octogintillion",
    "unoctogintillion",
    "duooctogintillion",
    "treoctogintillion",
    "quattuoroctogintillion",
    "quinoctogintillion",
    "sexoctogintillion",
    "septoctogintillion",
    "octooctogintillion",
    "novemoctogintillion",
    "nonagintillion",
    "unnonagintillion",
    "duononagintillion",
    "trenonagintillion",
    "quattuornonagintillion",
    "quinnonagintillion",
    "sexnonagintillion",
    "septnonagintillion",
    "octononagintillion",
    "novemnonagintillion",
    "centillion",
];

fn wordify(x: BigInt) -> String {
    let b = BigInt::from; // i didn't want to type `BigInt::from()` every single time

    // rq if you're not familiar with rust, usize is the number type used (among other things) to index lists
    if x < b(0) {
        format!("negative {}", wordify(-x))
    } else if x == b(0) {
        "zero".to_string()
    } else if x < b(20) {
        // Okay this one sucks to look at, but for whatever reason I needed to convert to isize first before usize.
        // Unwrap here is safe because value was checked
        ONES[x.to_isize().unwrap().to_usize().unwrap()].to_string()
    } else if x < b(100) {
        let tens = TENS[(x.clone() / b(10)).to_usize().unwrap()];
        let ones = ONES[(x % b(10)).to_usize().unwrap()];
        if ones == "zero" {
            tens.to_string()
        } else {
            format!("{}-{}", tens, ones)
        }
    } else if x < b(1000) {
        // Simplifies recursive calls to have this be a separate case
        let hundreds = ONES[(x.clone() / b(100)).to_usize().unwrap()];
        let tens = wordify(x % b(100));
        format!("{} hundred {}", hundreds, tens)
    } else {
        // rather confusingly, I called the "comma group" a power...
        // In human speak: make a list of the **numbers** at each group of three digits (ie thousand, million, billion, trillion, etc.)
        let mut powers = (0..POWERS.len()).map(|i| b(1000).pow(i as u32));
        // makes a list of tuples where the format is (numerical power group, how many of that power group). ie 14,000,000 -> (1,000,000, 14)
        // ^ but it does this for **all** values in the POWERS list, so we need to find the one we want
        // then filters list to only the 'how many' number exists. ie where x >= power group
        // takes the highest value with last
        // this leaves you with the value mentioned in the first line of this comment
        let (power, amount) = powers
            .clone()
            .map(|p| (p.clone(), &x / &p))
            .take_while(|(_, n)| *n > b(0))
            .last()
            .unwrap();
        // Gets the junk after the most significant power group. ie 14,123,456 -> 123,456
        let rest = &x % &power;

        let power_word = POWERS[powers.position(|p| p == power).unwrap()];
        let rest_word = wordify(rest);
        let word = format!("{} {}", wordify(amount), power_word);
        let word = format!("{} {}", word, rest_word);
        word.trim().to_string()
    }
}

fn main() {
    let arg = env::args().nth(1);
    if arg.is_none() {
        eprintln!("Provide a number as an argument");
        return;
    }
    let arg = arg.unwrap();

    let num = arg.parse::<BigInt>();
    if num.is_err() {
        eprintln!("Argument must be a number, instead recieved `{}`", arg);
        return;
    }

    let mut word = wordify(num.unwrap());

    let mut count = 0u32;
    while word != "four" {
        let len = word.len();

        let capitalized_word = format!(
            "{}{}",
            word.to_uppercase().chars().next().unwrap(),
            &word[1..]
        );
        println!("{}: {} is {}", count + 1, capitalized_word, len);

        word = wordify(BigInt::from(len));
        count += 1;
    }
    println!("Four is the perfect number.");
    println!("\nIt took {} iterations to reach four.", count);
}
