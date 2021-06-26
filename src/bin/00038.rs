use project_euler::{list2number, permutation};

#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    let list = (1..=9).collect::<Vec<u32>>();
    let mut max = 0;

    for p in permutation(&list) {
        (0..6).for_each(|i| {
            let first = list2number(&p[0..=i]);
            if check(&p, first, 1) {
                let pandigital = list2number(&p);
                if pandigital > max {
                    max = pandigital;
                }
            }
        });
    }

    max
}

fn check(list: &[u32], first: u32, multiplier: u32) -> bool {
    // 'list' is split completely, so it is a concatenated product of 'first'
    if list.len() == 0 {
        return true;
    }

    let product = first * multiplier;
    let product_len = product.to_string().len();

    // 'list' is shorter than product, so 'list' can't be split completely.
    if product_len > list.len() {
        return false;
    }

    let sub_list_number = list2number(&list[0..product_len]);
    if product == sub_list_number {
        return check(&list[product_len..], first, multiplier + 1);
    } else {
        // not concatenated product
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(932718654, s1());
    }
}
