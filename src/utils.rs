use rand::Rng;

pub fn generate_num_vec(n:i64) -> Vec<i8> {
    
    let mut nums: Vec<i8> = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..n {
        let random_num = rng.gen_range(-100..=100); 
        nums.push(random_num);
    }
    return nums;
}
