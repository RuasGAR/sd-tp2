use std::thread;
use std::sync::{Arc, WaitTimeoutResult};
use super::lock::Lock as Lock;

pub static mut ACC_LOCK:Lock<i64> = Lock::new(0); 
pub static mut EXPECTED_RES:Lock<i64> = Lock::new(0);

pub fn threaded_sum(n:usize,k:usize,n_vec:Vec<i8>) {

    let n_vec = Arc::new(n_vec);

    /* 
        Lógica para dividir o trabalho entre todas as threads. 
        A última deve ficar com os dados restantes da divisão de N pelas K-threads
    */
    let workload_each:usize = n/k; // quantos números cada thread vai usar
    let need_for_adjustments:bool = n%k != 0;

    print!("workload each:{}\n",workload_each);


    thread::scope(|s|{
        
        for i in 0..k {

            s.spawn({

                // Clonagem da referência para n_vec
                let n_vec = n_vec.clone();
                
                /* 
                    Índices de responsabilidade de cada thread:
                    
                    Se N%K for diferente de 0: ##########################################

                        Exemplo:
                            N = 100
                            K = 2

                        1a iteração:                        2a iteração:                        
                            i = 0                               i = 1   
                            start_index = 0*50 = 0;             start_index = 1*50 = 50;
                            end_index = 0+50 = 50;              end_index = 50+50 = 100;

                    Se houver resto:
                        
                        Exemplo:
                            N = 100
                            K = 3

                        1a iteração:                        2a iteração:                        
                            i = 0                               i = 1   
                            start_index = 0*33 = 0;             start_index = 1*33 = 33;
                            end_index = 0+33 = 33;              end_index = 33+33 = 66;

                        3a iteração:
                            i = 2
                            start_index = 2*33 = 66;
                            end_index = 100
                */

                let start_index:usize;
                let end_index:usize;
                
                if need_for_adjustments && (i == k-1) {
                    start_index = (k-1) * workload_each;
                    end_index = n_vec.len();
                } else {
                    start_index = i * workload_each;
                    end_index = start_index + if i==k {workload_each-1} else {workload_each};
                }

                print!("Start_index:{}\n", start_index);
                print!("End_index:{}\n", end_index);

                // ROTINA EXECUTADA POR CADA THREAD
                move || {    
                    let mut partial_result:i64 = 0;

                    for j in start_index..end_index {
                        // Vamos processar os valores k*workload + i para cada thread
                        partial_result += n_vec[j] as i64; 
                    }
    
                    unsafe { 
                        let mut guard = ACC_LOCK.acquire();
                        *guard += partial_result;
                        ACC_LOCK.release();
                    }
                }
            });
        }
    });

    // Thread para computar a soma de uma vez e checar se tá correto

    thread::spawn(move||{ 

        let mut sum:i64 = 0;

        for i in 0..n_vec.len() {
            sum += n_vec[i] as i64;
        }

        unsafe {
            let mut res_guard = EXPECTED_RES.acquire();
            *res_guard = sum;
            EXPECTED_RES.release();
        }

    }).join().unwrap();

    
    let printable_distributed_sum:i64;
    let printable_single_sum:i64;
    unsafe {
        
        let ds_guard = ACC_LOCK.acquire();
        printable_distributed_sum = *ds_guard as i64;
        ACC_LOCK.release();

        let single_guard = EXPECTED_RES.acquire();
        printable_single_sum = *single_guard as i64;
        EXPECTED_RES.release();
    }


    println!("Valor obtido pela soma direta: {}",&printable_single_sum);
    println!("Valor obtido pela soma distribuída: {}",&printable_distributed_sum);

}


