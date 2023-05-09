use rand::Rng;

mod utilities {
    
    fn generate_num_vec(n:i32) -> Vec<i8> {
        
        let nums: Vec<i8> = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..n {
            let mut random_num = rng.gen_range(-100..=100)  
            nums.push(random_num);
        }
        return nums;
    }



}