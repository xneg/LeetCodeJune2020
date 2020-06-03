pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut w: Vec<(i32, Vec<i32>)> = costs.into_iter().map(|x| (x[0] - x[1], x)).collect();
    w.sort_by_key(|k| k.0);
    let len = w.len();

    let (a, b) = w.split_at_mut(len/2);

    let s1: i32 = a.into_iter().map(|(_, x)| x[0]).sum();
    let s2: i32 = b.into_iter().map(|(_, x)| x[1]).sum();
    s1 + s2
}