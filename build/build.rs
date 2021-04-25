use bundle_rs::{Bundle, ModuleFileSystem};

fn main() {
    let mut bundle = Bundle::new("main", ModuleFileSystem::new(vec!["./src"]));
    bundle.load().unwrap();
    let mut file = std::fs::File::create("./dist/singlefile.rs").unwrap();
    bundle.write(&mut file).unwrap();
}
