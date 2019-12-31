fn main() {
    let binary = String::from("110101");
    let mut r:i32 = -1;
    let mut z = 0;
    for y in (binary.chars()).rev(){
    r += 1;
    let x = '1';
    if y == x{
        let  dd = power(2,r as u64);
        z += dd
    };
    };
    println!("{}",z);
}
pub fn power(binary: u64, y: u64) -> u64{
    let a = binary as u32;
    let b = y as u32;
    let mut res: u64 = 0;

    if b == 0 {
        res = 1;  
    }else if b == 1{
        res = a as u64;
    }else{
    for _i in 1..b {
    if res != 0 {
        res = res * (a as u64);
    } else {
    res = (a as u64) * (a as u64);          
}}}
    res
}