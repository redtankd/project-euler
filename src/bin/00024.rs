#[cfg(not(test))]
fn main() {
    use project_euler::*;

    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("answer={}\n", s1());

    stop_timer(t);
}

fn s1() -> String {
    let level = 1;
    let objects = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut p = vec![0; 11];
    let mut p_count = 0;
    f(level, objects, &mut p, &mut p_count).unwrap()
}

fn f(level: u8, objects: Vec<u8>, p: &mut Vec<u8>, p_count: &mut u64) -> Option<String> {
    if level == 11 {
        *p_count += 1;
        if *p_count == 1_000_000 {
            return Some(
                (*p)[1..]
                    .iter()
                    .map(|&x| format!("{}", x))
                    .collect::<String>(),
            );
        } else {
            return None;
        }
    } else {
        let mut choice_count = objects.len();
        while choice_count > 0 {
            let mut objects_n = objects.clone();
            (*p)[level as usize] = objects_n.remove(choice_count - 1);
            if let Some(m) = f(level + 1, objects_n, p, p_count) {
                return Some(m);
            } else {
                choice_count -= 1;
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1_test() {
        assert_eq!("2783915460", s1());
    }
}
