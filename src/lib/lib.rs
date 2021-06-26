use time::precise_time_s;

pub fn start_timer() -> f64 {
    precise_time_s()
}

pub fn stop_timer(t: f64) {
    println!("time elapse {} second", precise_time_s() - t);
}

pub fn primes() -> impl Iterator<Item = u32> {
    (2..)
        .scan(Vec::<u32>::new(), |state, x| {
            if state
                .iter()
                .take_while(|&y| *y <= x / 2)
                .all(|&y| x % y != 0)
            {
                state.push(x);
                Some(x)
            } else {
                Some(0)
            }
        })
        .filter(|&x| x != 0)
}

/**
    With FlatMap and Map, we expand the orignal list to every possbile combination. For example

    ([], [1,2,3])                                       map in draw() and flat_map in permutation() => 
    [ ([1], [2, 3]), ([2], [1, 3]), ([3], [1, 2]) ]     again =>
    [ ([1, 2], [3]), ([1, 3], [2]), ... ]               again =>
    ...
*/
pub fn permutation<'a, T: Clone>(list: &'a [T]) -> impl Iterator<Item = Vec<T>> + 'a {
    let p = vec![(Vec::new(), list.to_vec())];

    // Vec wrapped in FlatMap, whose type is very complicated,
    // so cast it to a trait object.
    let mut p_iter = Box::new(p.into_iter()) as Box<dyn Iterator<Item = (Vec<T>, Vec<T>)>>;

    for _ in 0..list.len() {
        p_iter = Box::new(p_iter.flat_map(|(drawn, to_draw)| draw(drawn, to_draw)));
    }

    fn draw<T: Clone>(drawn: Vec<T>, to_draw: Vec<T>) -> impl Iterator<Item = (Vec<T>, Vec<T>)> {
        (0..to_draw.len()).map(move |i| {
            let mut to_draw_new = to_draw.to_vec();
            let drawn_item = to_draw_new.remove(i);
            let mut drawn_new = drawn.to_vec();
            drawn_new.push(drawn_item);
            (drawn_new, to_draw_new)
        })
    }

    p_iter.map(|x| x.0)
}

/**
    For example [1, 2, 3, 4] => 1234
 */
pub fn list2number(list: &[u32]) -> u32 {
    let mut r: u32 = 0;
    for (i, v) in list.iter().rev().enumerate() {
        r += v * 10u32.pow(i as u32);
    }
    r
}