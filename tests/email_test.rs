use std::env;

#[test]
fn it_adds_two() {
    let b = 2 + 2;
    assert_eq!(4, b);
}

#[test]
fn get_env_var() {
    let v = env::var("KBUSER").expect("$USER is not set");
    println!("{}", v);

}