fn main() -> Result<(), Box<dyn std::error::Error>> {

    let proto = tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .type_attribute("logcanal.MessageRequest","use serde::{Serialize};")
        .type_attribute("logcanal.MessageRequest", "#[derive(Serialize)]")
        .type_attribute("logcanal.Message", "#[derive(Serialize)]")
        .type_attribute("logcanal.Service", "#[derive(Serialize)]");
    proto.compile(&["logcanal.proto"], &["proto/"]).unwrap();
    Ok(())
}
