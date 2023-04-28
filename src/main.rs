use clap::Parser; 
use yew::prelude::*; 

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {  
    /// Trait requested
    tag_trait: String, 
    /// group requested
    group: String, 

    infinite_list: Vec<String>
    
}   
#[function_component(App)]
fn app() -> Html { 
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

