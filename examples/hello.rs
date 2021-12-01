extern crate xigh_chain_rs;

fn main() {
    let xc = &mut xigh_chain_rs::Blockchain::new();
    
    println!("add vec 1:");
    if !xc.add(vec![0, 1, 2]) {
        panic!("could not add vec 1");
    }

    println!("add vec 2:");
    if !xc.add(vec![3, 4, 5, 6]) {
        panic!("could not add vec 1");
    }
    
    println!("dump chain:");
    xc.dump_chain();
    
    println!("trying to tamper data:");
    xc.tamper_data(1, vec!(7, 8));
    println!("is_valid {}", xc.is_valid());
}
