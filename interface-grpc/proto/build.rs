fn main() {
    // TODO: Add feature gates for client and server
    tonic_build::compile_protos("proto/photonic.proto").expect("Failed to compile GRPC proto");
}
