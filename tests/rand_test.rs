
use rand::Rng;

#[test]
fn rand_test () {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(10000000..99999999));
}