fn main(){
  println!("Hello, here is referencing of borrowing and referening concepts");
  let mut a = 12;

  println!("Value of a is {}", a);

  let mut b = a;

  println!("Value of b is : {}", b);
}