fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
    .out_dir("src")
    .input("./src/hello.proto")
    .rust_protobuf(true)
    .run()
    .expect("error compiling protocol buffer");
}