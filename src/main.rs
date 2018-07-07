static UNIT: [&str; 11] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"
];

static TEEN: [&str; 10] = [
    "", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
    "eighteen", "nineteen",
];

static TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];


static HUNDREDS: &str = "hundred";

static THOUSAND: &str = "thousand";

static LACK: &str = "lack";

static CRORE: &str = "crore";

static SPACE: &str = " ";

fn one_to_ninety_nine(mut number: usize) -> String {
    let mut word: String = "".clone().to_string();

    if 21 <= number && number < 100 {
        word.push_str(TENS[number / 10].clone());
        word.push_str(SPACE.clone());
        number = number % 10;
    }

    if 11 <= number && number < 20 {
        word.push_str(TEEN[number % 10].clone())
    }

    if 0 < number && number <= 10 {
        word.push_str(UNIT[number].clone());
    }
    return word;
}

fn digit_to_word(number: usize) -> String {
    let mut number: usize = number;
    let mut word: String = "".clone().to_string();
    if 10000000 <= number && number < 1000000000 {
        let res = format!(
            "{} {} {}",
            one_to_ninety_nine(number / 10000000), CRORE.clone(), SPACE.clone()
        );
        word.push_str(&res);
        number %= 10000000;
    }
    if 100000 <= number && number < 10000000 {
        let res = format!(
            "{} {} {}",
            one_to_ninety_nine(number / 100000), LACK.clone(), SPACE.clone()
        );
        word.push_str(&res);
        number %= 100000;
    }
    if 1000 <= number && number < 100000 {
        let res = format!(
            "{} {} {}",
            one_to_ninety_nine(number / 1000), THOUSAND.clone(), SPACE.clone()
        );
        word.push_str(&res);
        number %= 1000;
    }
    if 100 <= number && number < 1000 {
        let res = format!("{} {} ", UNIT[number / 100].clone(), HUNDREDS.clone());
        word.push_str(&res);
        number %= 100;
    }
    if 0 < number && number < 100 {
        word.push_str(&one_to_ninety_nine(number));
    }

    return word;
}

fn main() {
    let digit_to_word = digit_to_word(7564234234324232342);
    println!("{}", digit_to_word);
}
