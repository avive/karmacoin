// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::path::{Path, PathBuf};
use std::{fs, io};

// we allow this as we often comment out this method to cut time of local builds
#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ////////////////////
    // note: uncomment when proto change to recompile
    // this is commented out to shorten build times when proto generated code didn't change
    ////////////////////

    // We add serde support for Protobuf structs that needs to be persisted locally as member of rust structs.

    std::env::set_var("OUT_DIR", "src");
    let original_out_dir = PathBuf::from("src/karma_coin");

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/karma_coin")
        .type_attribute(
            "TransactionData",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "SignedTransaction",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "SmsInviteMetadata",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "Transaction",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "SignedTransactionsHashes",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "LeaderBoard",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "TransactionEvent",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "CommunityMembership",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "TransactionEvents",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "TransactionResult",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "BlockEvents",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "TokenomicsData",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("MemPool", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "BlockchainStats",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "TraitName",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Traits", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "VerifierInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Amount", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "Signature",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "PublicKey",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute(
            "MobileNumber",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("Balance", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "TraitScore",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("PreKey", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(
            "AccountId",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .type_attribute("User", "#[derive(serde::Serialize, serde::Deserialize)]")
        .file_descriptor_set_path(original_out_dir.join("descriptor.bin"))
        .compile(
            &[
                "proto/karma_coin/core_types/types.proto",
                "proto/karma_coin/core_types/verifier.proto",
                "proto/karma_coin/core_types/auth.proto",
                "proto/karma_coin/core_types/api.proto",
                "proto/karma_coin/client.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let src = Path::new("src/karma_coin");
    rename_prost_generated_filenames(src).unwrap();

    Ok(())
}

// Ugly hack because prost output rust file names with `.` when packages are used, e.g. snp.foo, and the rust module system doesn't support . in file names.
fn rename_prost_generated_filenames(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "bin" {
                        continue;
                    }
                }
                let file_stem_renamed = &path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".", "_");

                fs::rename(&path, dir.join(format!("{}.rs", file_stem_renamed)))?;
            }
        }
    }

    Ok(())
}
