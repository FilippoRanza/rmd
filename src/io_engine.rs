
use std::io::*;


pub fn remove_question(name: &str) -> Result<bool> {
    let mut buffer = String::new();
    
    println!("remove: {}?", name);
    std::io::stdin().read_line(&mut buffer)?;
 
    let lower = buffer.to_lowercase();
    let ans = lower.trim();

    let out = match ans {
        "yes" => true,
        "y" => true,
        _ => false
    } ;

    Ok(out)
}