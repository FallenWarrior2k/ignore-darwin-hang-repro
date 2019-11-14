use ignore::WalkBuilder;

fn main() {
    for x in WalkBuilder::new("/")
        .max_depth(Some(2)).same_file_system(true).build() {
        match x {
            Ok(entry) => println!("{:?}", entry),
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }
}
