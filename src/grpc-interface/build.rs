/// `cargo build`時に自動的にコンパイルされる。
/// cargo build --package grpc-interface
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // tonic_build::compile_protos("proto/account.proto")?;
    // // // tonic_build::compile_protos("proto/get.proto")?;
    // // // tonic_build::compile_protos("proto")?;
    // tonic_build::configure()
    //     .build_server(false)
    //     .compile(&["proto/account.proto", "proto/get.proto"], &["proto"])
    //     .unwrap();
    // Ok(())
    let dir_path = "protobuf/";
    let filenames = ["account.proto", "get.proto", "add.proto"];
    let build_pathes = filenames.map(|filename| format!("{dir_path}{filename}"));
    tonic_build::configure()
        .build_server(false)
        .compile(&build_pathes, &["protobuf"])
        .unwrap();
    Ok(())
}
