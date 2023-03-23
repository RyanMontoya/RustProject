
struct Parameters { 
    query_trait : String,  
}
//use std::env;
//use std::io::stdin; 
/* 
trait ReadParam { 
   fn read(&self);
}

impl ReadParam for Parameters  { 
    fn read(&self)  { 
    
    }
}
*/
fn main() {
    println!("Hello, world!");  
   
    let temp : String = read(); 
    let query = Parameters { query_trait: temp};
} 


fn read() -> String {   
    let mut line = String::new();
    println!("Enter query trait:");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line); 
    return line; 

   // println!("no of bytes read , {}", b1);
} 

