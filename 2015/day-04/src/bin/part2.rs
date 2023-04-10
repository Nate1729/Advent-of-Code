use md5;


fn main() {

    let secret_key = "ckczppom";

    for i in 1..100_000_000 {
        let string = format!("{}{}", secret_key, i);
        let digest_string = format!("{:x}", md5::compute(string.as_bytes()));
        if &digest_string[0..6] == "000000" {
            println!("Found solution: {}", i);
            break;
        }
    }
    
}
