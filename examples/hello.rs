extern crate xigh_chain_rs;

fn main() {
    let xighcoin = &mut xigh_chain_rs::Blockchain::new();
    
    println!("add vecs:");
    xighcoin.add(vec![0, 1, 2]);
    xighcoin.add(vec![3, 4, 5, 6]);
    xighcoin.dump_chain();
    
    println!("trying to tamper data:");
    xighcoin.tamper_data(1, vec!(7, 8));
    println!("is_valid {}", xighcoin.is_valid());
}
