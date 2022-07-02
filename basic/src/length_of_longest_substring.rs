use std::collections::HashMap;
use std::ops::Index;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut list = Vec::new();
    let mut len = 0;
    for v in s.chars() {
        if list.contains(&v) {
            let mut index = 0;
            for &lv in list.iter() {
                index += 1;
                if lv == v {
                    break;
                }
            }
            list = list.split_off(index);
        }
        list.push(v);
        len = len.max(list.len());
    }
    return len as i32;
}

#[test]
pub fn ex_length_of_longest_substring() {
    assert_eq!(3, length_of_longest_substring("dvdf".to_string()));
    assert_eq!(3, length_of_longest_substring("hello".to_string()));
    assert_eq!(3, length_of_longest_substring("abcabcbb".to_string()));
    assert_eq!(3, length_of_longest_substring("pwwkew".to_string()));
    assert_eq!(1, length_of_longest_substring("bbbbb".to_string()));
    assert_eq!(1, length_of_longest_substring(" ".to_string()));
}

#[test]
fn for_each() {
    let v = vec![10, 20, 30, 40];
    let mut n = 0;
    v.iter().for_each(|x| {
        n += 1;
        if n == 2 {
            return;
        }
    });
    println!("{}", n);

    println!("{}", v.iter().find(|&&x| x == 30).unwrap());

    let s = "hello world".to_string();
    for (k,v) in s.chars().enumerate() {
        println!("{},{}", k,v);
    }

    let index:Vec<i32>=vec![-1;128];
    println!("{:?}", index);
}
