use ignore::WalkBuilder;

fn main() {
    for x in WalkBuilder::new("/").same_file_system(true).build() {
        if let Err(err) = x {
            eprintln!("Error: {:?}", err);
        }
    }

    println!("Outside walk.")
}
