#[cfg(not(feature = "bindgen"))]
fn main() {}

#[cfg(feature = "bindgen")]
fn main() {
    prost_build::compile_protos(
        &[
            "proto/BrokerService.proto",
            "proto/MasterService.proto",
            "proto/RPC.proto",
        ],
        &["proto/"],
    )
    .unwrap();
}
