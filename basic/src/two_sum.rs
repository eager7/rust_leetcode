fn two_sum_with_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        m.insert(*num, index as i32);
    }
    for (index, num) in nums.iter().enumerate() {
        if !m.contains_key(&(target - num)) {
            continue;
        }
        if let Some(x) = m.get(&(target - num)) {
            if *x as usize == index {
                continue;
            }
            return vec![index as i32, *x];
        }
    }
    return vec![];
}

fn two_sum_by_loop(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, num) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if index == index2 {
                continue;
            }
            if num + num2 == target {
                return vec![index as i32, index2 as i32];
            }
        }
    }
    return vec![];
}

#[test]
fn two_sum_test() {
    assert_eq!(vec![1, 2], two_sum_by_loop(vec![3, 4, 5], 9));
    assert_eq!(vec![0, 1], two_sum_by_loop(vec![3, 3], 6));
    assert_eq!(vec![1, 2], two_sum_by_loop(vec![3, 2, 4], 6));
    assert_eq!(vec![1, 2], two_sum_with_map(vec![3, 4, 5], 9));
    assert_eq!(vec![0, 1], two_sum_with_map(vec![3, 3], 6));
    assert_eq!(vec![1, 2], two_sum_with_map(vec![3, 2, 4], 6));
}

#[test]
fn vec_to_map() {
    let v = vec![1, 2, 3];
    let mut k = Vec::new();
    for i in 0..v.len() {
        k.push(i);
    }
    let m: std::collections::HashMap<_, _> = v.into_iter().zip(k.into_iter()).collect();
    println!("{:?}", m);
    for (k, v) in m.iter() {
        println!("{:?},{:?}", k, v);
    }
}
#[test]
fn vec_to_map2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: std::collections::HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
    for (k, v) in scores.iter() {
        println!("{:?},{:?}", k, v);
    }
}