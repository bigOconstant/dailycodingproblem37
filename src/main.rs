

fn main() {
    println!("Daily Coding Problem #37 Easy. Find the powerset");
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    
    let output = power_set(numbers.clone());
    println!("input:{:?}",numbers);
    println!("powerset:{:?}",output);
}

/*
Convert to binary,iterate to 2^n if 1 add if 0 don't add 
*/

fn power_set(input:Vec<i32>)-> Vec<Vec<i32>>{
    let two:i32 = 2;
    let length = two.pow(input.len() as u32);
    let mut output: Vec<Vec<i32>> = Vec::new();

    for x in 0..length  {
        let  bin:String = format!("{:b}",x);
        let  bin = pad_zeros(bin,(input.len() -1) as i32);
        let mut inp:Vec<i32> = Vec::new();
        for (i,c) in bin.chars().enumerate(){
            
            if c == '1' {
                let vh = input.get(i).clone().unwrap();
                inp.push(*vh);
            }
        }
        output.push(inp);

    }
    return output;
}
/*
    insures binary leading zeros don't get cut off screwing up the calculations. 
*/
fn pad_zeros( input:String,pad_size:i32)->String{
    if input.len() < (pad_size +1) as usize {
        let mut nstring = String::from("0");
        nstring.push_str(&input.clone());
        return pad_zeros(nstring,pad_size);
    }
    return input;
}
