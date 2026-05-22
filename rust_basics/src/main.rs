fn main(){
    let mut arr1:[u8; 5] = [1, 2, 3, 4, 5];
    change_arr(arr1); //pass by value
    println!("Changed array : {}", arr1);
}

fn change_arr(mut arr1:[u8;5]){
    arr1[0] = 8;
    println!("{}", arr1);
}

