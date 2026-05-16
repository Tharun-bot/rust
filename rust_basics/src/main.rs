fn main() {
    let s1:String  = get_string();
    println!("S1 : {}", s1);

    let s2:String = String::from("World");
    let s3:String = send_get_string(s2);

    println!("S3 : {}", s3);
}

fn get_string() -> String{
    let n_string:String = String::from("Hello");
    return n_string;
}

fn send_get_string(get_string:String)->String{
    let new_string: String = get_string;
    return new_string;
}