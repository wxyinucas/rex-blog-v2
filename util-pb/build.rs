use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sqlx_from_row(&["models.Category", "models.Tag"], None)
        .with_serde(&["models.Category", "models.Tag"], true, false, None)
        .with_derive_builder(&["models.QueryArticle"], None)
        .compile(&["proto/models.proto"], &["."])
        .unwrap();

    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo:rerun-if-changed=proto");
}
