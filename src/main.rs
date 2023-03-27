

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

enum TraitVariants { 
    TraitA(String), 
    TraitB(String, String, String),
}

fn main() { 
 


    println!("Hello, world!");  
   
    let temp : String = read(); 
    let query = Parameters { query_trait: temp}; 
    println!("howdy, {}", query.query_trait); 
    boxlearn(); 
    let a = TraitVariants::TraitA(String::from("concentration"));   
    let b = TraitVariants::TraitB(String::from("concentration"),String::from("move"),String::from("beans"));
    //let b = TraitVariants::TraitB("concentration","move",);
    
   // println!("multi trait enum pass {}",b);

} 

fn read() -> String {   
    let mut line = String::new();
    println!("Enter query trait:");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line); 
    return line; 

   // println!("no of bytes read , {}", b1);
}  


fn boxlearn() { 
    let boxtrait = Box::new(10); 
    println!("box containing {}", boxtrait); 

} 

fn enumlearn() { 

}


