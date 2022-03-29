use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

#[test]
fn hash_pw () {
    // 密码加盐算hash

    let pw = b"mypwd123";
    let salt = "SpcYWWjgBzk6amkHs4G3JLE27ow5yTq8";
    let hashed_pw = Argon2::default()
        .hash_password(pw, &salt)
        .unwrap()
        .to_string();
    println!("hashed pw: {}", hashed_pw);
    let hash = PasswordHash::new(&hashed_pw).unwrap();
    let result = Argon2::default().verify_password(pw, &hash);
    match result {
        Ok(()) => {
            println!("ok ");
        },
        Err(e) => {
            println!(" error: {}", e);
        }
    }
}