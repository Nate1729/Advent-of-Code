use md5;


fn main() {

    let secret_key = "ckczppom";

    for i in 1..1_000_000 {
        let string = format!("{}{}", secret_key, i);
        let digest_string = format!("{:x}", md5::compute(string.as_bytes()));
        if &digest_string[0..5] == "00000" {
            println!("Found solution: {}", i);
            break;
        }
    }
    
}
