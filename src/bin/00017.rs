#[cfg(not(test))]
fn main() {
    use project_euler::*;

    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("length={}\n", s1());

    stop_timer(t);
}

fn s1() -> usize {
    let mut size: usize = 0;

    for i in 1..1001 {
        let mut ns = i.to_string();

        // 0 to 9
        let n1 = match ns.pop() {
            Some('0') => "0",
            Some('1') => "one",
            Some('2') => "two",
            Some('3') => "three",
            Some('4') => "four",
            Some('5') => "five",
            Some('6') => "six",
            Some('7') => "seven",
            Some('8') => "eight",
            Some('9') => "nine",
            _ => "",
        };

        // tens
        let n2 = match ns.pop() {
            Some('0') => "0",
            Some('1') => "1",
            Some('2') => "twenty",
            Some('3') => "thirty",
            Some('4') => "forty",
            Some('5') => "fifty",
            Some('6') => "sixty",
            Some('7') => "seventy",
            Some('8') => "eighty",
            Some('9') => "ninety",
            _ => "",
        };

        let n12 = if n2 == "0" && n1 == "0" {
            // "00"
            "".to_string()
        } else if n2 == "0" {
            // 1 to 9
            n1.to_string()
        } else if n2 == "1" {
            // teens
            match n1 {
                "0" => "ten",
                "one" => "eleven",
                "two" => "twelve",
                "three" => "thirteen",
                "four" => "fourteen",
                "five" => "fifteen",
                "six" => "sixteen",
                "seven" => "seventeen",
                "eight" => "eighteen",
                "nine" => "nineteen",
                _ => "",
            }
            .to_string()
        } else if n1 == "0" {
            // 20, 30, 40, 50, 60, 70, 80, 90
            n2.to_string()
        } else {
            // others greater than 19
            n2.to_string() + n1
        };

        // hundred
        let n3 = if n12 != "" {
            // hundred and ...
            match ns.pop() {
                Some('1') => "onehundredand",
                Some('2') => "twohundredand",
                Some('3') => "threehundredand",
                Some('4') => "fourhundredand",
                Some('5') => "fivehundredand",
                Some('6') => "sixhundredand",
                Some('7') => "sevenhundredand",
                Some('8') => "eighthundredand",
                Some('9') => "ninehundredand",
                _ => "",
            }
        } else {
            // 100, 200, 300, 400, 500, 600, 700, 800, 900
            match ns.pop() {
                Some('1') => "onehundred",
                Some('2') => "twohundred",
                Some('3') => "threehundred",
                Some('4') => "fourhundred",
                Some('5') => "fivehundred",
                Some('6') => "sixhundred",
                Some('7') => "sevenhundred",
                Some('8') => "eighthundred",
                Some('9') => "ninehundred",
                _ => "",
            }
        };

        // thousand
        let n4 = match ns.pop() {
            Some('1') => "onethousand",
            Some('2') => "twothousand",
            Some('3') => "threethousand",
            Some('4') => "fourthousand",
            Some('5') => "fivethousand",
            Some('6') => "sixthousand",
            Some('7') => "seventhousand",
            Some('8') => "eightthousand",
            Some('9') => "ninethousand",
            _ => "",
        };

        size += n12.len() + n3.len() + n4.len();
    }

    size
}

#[test]
fn s1_test() {
    assert_eq!(21124, s1());
}
