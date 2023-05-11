use clap::Parser; 
use yew::prelude::*; 
//use bevy::prelude::*;  
//use wasm_bindgen::prelude::*;
//use wasm_bindgen::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {  
    /// Trait requested
    tag_trait: String, 
    /// group requested
    group: String, 

    infinite_list: Vec<String>
    
}    

/* 
struct test { 
    test:String
} */


#[function_component(App)]
fn app() -> Html {  
   // let test = test {test : String::from("howdy")}; 
    let test = String::from("hi");
    html! { 
        <div>  
        <p>{"stuff"}</p> 
        </div>
    }
}

//using clap for to learn more about rust cli.
fn main() { 

    let args = Args::parse();   
    
   println!("good choice {}", args.tag_trait);  
  // println!("good choice 2 {}", args.infinite_list[0]);
  

yew::start_app::<App>();

}  

