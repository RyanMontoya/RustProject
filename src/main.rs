use clap::Parser; 
//use yew ::prelude::*; 
/* 
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
    */ 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {  
    /// Trait requested
    tag_trait: String, 
    /// group requested
    group: String, 

    infinite_list: Vec<String>
    
} 

//using clap for to learn more about rust cli.
fn main() { 

    let args = Args::parse();   
    
   println!("good choice {}", args.tag_trait);  
   println!("good choice 2 {}", args.infinite_list[0]);
   
    /* 
    println!("Hello, world!");  
   
    let temp : String = read(); 
    let query = Parameters { query_trait: temp}; 
    println!("howdy, {}", query.query_trait); 
    boxlearn(); 
    let a = TraitVariants::TraitA(String::from("concentration"));   
    let b = TraitVariants::TraitB(String::from("concentration"),String::from("move"),String::from("beans"));
    //let b = TraitVariants::TraitB("concentration","move",);
    
   // println!("multi trait enum pass {}",b);
    */  

//yew ::start_app::<App>();

}  
/* 
#[function_component(App)]
fn app() -> Html { 
    html! {  
        <h1>{"howdy there!"}</h1> 
    }
} */

 /* 
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
*/

