use simple_ft::MyProgram;
use std::{env, fs::File, path::PathBuf};

fn main() {
    gwasm_builder::build();

    let manifest_dir_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let idl_file_path = manifest_dir_path.join("ft.idl");

    let idl_file = File::create(idl_file_path).unwrap();

    sails_idl_gen::program::generate_idl::<MyProgram>(idl_file).unwrap();
}
