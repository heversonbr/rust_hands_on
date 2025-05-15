use std::env;
use std::path::PathBuf;

fn main() {
    // prost_build::compile_protos(&["proto/person.proto"], &["proto/"]).unwrap();
    // prost_build::compile_protos(&["proto/person.proto"], &["proto/"]).expect("Failed to compile .proto files");
    
    // version debug for build script: 
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {:?}", out_dir);

    if let Err(e) = prost_build::compile_protos(&["proto/person.proto"], &["proto/"]) {
        panic!("Failed to compile .proto files: {:?}", e);
    }
}