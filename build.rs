const GEOMETRIC_PROTOBUF_PATH: &str = "./proto/geometric.proto";
const PROTOBUF_DIR_PATH: &str = "./proto";

fn main() {
    tonic_build::configure()
        .out_dir("./src/proto")
        .compile_protos(&[GEOMETRIC_PROTOBUF_PATH], &[PROTOBUF_DIR_PATH])
        .expect("Protobuf files generation failed");
}
