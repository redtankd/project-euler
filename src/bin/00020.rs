#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1());

    project_euler::stop_timer(t);

    let t = project_euler::start_timer();

    println!("\nsolution 2:\n");
    println!("sum={}\n", s2());

    project_euler::stop_timer(t);
}

fn s1() -> u64 {
    let mut n = vec![1];

    for i in 2..101 {
        n = n.iter().map(|x| x * i).collect();
        carry(&mut n);
    }

    n.iter().fold(0, |s, &x| s + x)
}

fn carry(n: &mut Vec<u64>) {
    let mut adv = 0;

    for ni in n.iter_mut() {
        let _ni = adv + *ni;
        adv = _ni / 10;
        *ni = _ni % 10;
    }
    while adv > 9 {
        n.push(adv % 10);
        adv /= 10;
    }
    if adv > 0 {
        n.push(adv);
    }
}

fn s2() -> u64 {
    (2..101)
        .fold(vec![1], |fac, n| {
            // 1*2*...*100
            let mut carry = 0;
            fac.iter()
                .map(|x| {
                    let _x = x * n + carry;
                    carry = _x / 10; // carry
                    _x % 10
                })
                .collect::<Vec<u64>>()
                .apply(|f| {
                    // apply() appends new insignificance bits
                    // and return Vec<u64> self.
                    f.append(
                        &mut ((1..)
                            .scan(carry, |state, _| match *state {
                                x if x > 9 => {
                                    *state /= 10;
                                    Some(x % 10)
                                }
                                x @ 1..=9 => {
                                    *state = 0;
                                    Some(x)
                                }
                                _ => None,
                            })
                            .fuse()
                            .collect::<Vec<u64>>()),
                    );
                })
        })
        .iter()
        .fold(0, |s, &x| s + x)
}

// just for chaining calls
trait Call {
    fn apply<F>(mut self, mut f: F) -> Self
    where
        Self: Sized,
        F: FnMut(&mut Self),
    {
        f(&mut self);
        self
    }
}

impl Call for Vec<u64> {}

#[test]
fn s1_test() {
    assert_eq!(648, s1());
}

#[test]
fn s2_test() {
    assert_eq!(648, s2());
}
