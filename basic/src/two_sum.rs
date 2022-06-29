pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m:std::collections::HashMap<i32,i32> = std::collections::HashMap::new();
    for (index,num ) in nums.iter().enumerate() {
        println!("{},{}", index, num);
        m.insert(*num, index as i32);
    }

    let mut result:Vec<i32> = Vec::new();
    for (k,v) in m.iter().enumerate(){
        let kk = k as i32;
        let k2 = target-kk;
        if m.contains_key(&k2){
            let v2 = m.get(&k2).unwrap();
            result.push(*v);
            result.push(*v2);
            return result;
        }
    }
    return result;
}