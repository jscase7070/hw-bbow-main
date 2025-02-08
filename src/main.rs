
mod bbow;

use std::env;

fn main()
{

    let args: Vec<String> = env::args().collect();

    let bag: bbow::Bbow<'_> = bbow::Bbow::new();

    for x  in 1..args.len() {
        bag.clone().extend_from_text(&args[x]);
        
    }

    
}