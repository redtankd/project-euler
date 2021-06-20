#[cfg(not(test))]
fn main() {
    use project_euler::*;

    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("count={}\n", s1());

    stop_timer(t);
}

fn s1() -> u16 {
    let days_of_month = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = 366;
    let mut count = 0;

    for year in 1901..2001 {
        for month in 1..13 {
            if days % 7 == 6 {
                count += 1
            }

            days += if month == 2 {
                if year % 4 == 0 {
                    29
                } else {
                    28
                }
            } else {
                days_of_month[month]
            };
        }
    }

    count
}

#[test]
fn s1_test() {
    assert_eq!(171, s1());
}
