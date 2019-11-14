use ignore::WalkBuilder;

fn main() {
    for x in WalkBuilder::new("/").same_file_system(true).build() {
        match x {
            Ok(entry) => println!("{:?}", entry),
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }
}
