fn main(){
  let s1:String = String::from("Hello");
  let len:usize = get_length(s1.clone()); // ownership is transfered when there is no clone() used
  println!("String is {} and length is {}", s1, len);
}

fn get_length(string:String) -> usize{
  return string.len();
}