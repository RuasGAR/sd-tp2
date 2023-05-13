use std::thread;
use std::sync::Arc;
use super::lock::Lock as Lock;

//static mut AC:i64 = 0;
pub static mut ACC_LOCK:Lock<i64> = Lock::new(0); 

pub fn threaded_sum(n:usize,k:usize,n_vec:Vec<i8>) {

    let n_vec = Arc::new(n_vec);

    let workload_each:usize = n/k; // quantos números cada thread vai usar
    let mut extra_workload:usize = 0;

    let need_for_adjustments:bool = n%k != 0;
    if need_for_adjustments {
        extra_workload = workload_each + n%k;
    }

    let mut expected_result:i64 = 0;

    thread::scope(|s|{
        
        let n_vec = n_vec.clone();

        // k - 1 threads
        for i in 1..=k {

            s.spawn({
                let n_vec = n_vec.clone();

                move || {

                    if i == k && need_for_adjustments {
            
                        // pontos para situar a "última" thread
                        let beginning:usize = n-extra_workload-1;
                        let ending:usize = (n+extra_workload)-1;
                        
                        let last_sum:i64 = n_vec[beginning..ending].iter().fold(0,|a,n|a+(*n as i64));
            
                        unsafe { 
                            let mut guard = ACC_LOCK.acquire();
                            *guard += last_sum;
                            println!("Valor obtido pela soma distribuída:{}", *guard);
                        }
                        
                       
                    }
    
                    let mut partial_result:i64 = 0;
    
                    for j in 0..workload_each-1 {
                        // Vamos processar os valores k*workload + i para cada thread
                        println!("{}",j);
                        partial_result += n_vec[k*workload_each+j] as i64; 
                    }
    
                    unsafe { 
                        let mut guard = ACC_LOCK.acquire();
                        *guard += partial_result;
                    }
                }
            });
        }
    });

    // Thread para computar a soma de uma vez e checar se tá correto

    thread::spawn(move||{
        let sum:i64 = n_vec.iter().fold(0, |a,n| a + (*n as i64));
        expected_result = sum;
        println!("Valor obtido pela soma direta: {}",&expected_result);
    });


}

    //let correctness = assert_eq!(expected_result, acc);
    //println!("A soma da divisão deu {}",correctness);

