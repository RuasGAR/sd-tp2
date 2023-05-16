mod lock_sum;
mod prod_cons;
mod utils;
use lock_sum::study_case::basic_test;
use prod_cons::produter_consumer::share_memory;


fn main() {
    share_memory(1,1,1,100000);
    basic_test();
}
