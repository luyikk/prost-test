fn main() {
    // prost_build::compile_protos(&["proto/test.proto"],&["proto/"])
    //     .unwrap();

    prost_build::Config::new().out_dir("src/protobuf/").compile_protos(&["proto/test.proto"],&["proto/"]).unwrap()
}