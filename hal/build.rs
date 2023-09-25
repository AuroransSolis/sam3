use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

fn main() {
    let mut memory_files = Vec::new();
    FEATURE_TO_FILE
        .into_iter()
        .filter_map(|[chip, memory_file]| env::var_os(chip).map(|_| memory_file))
        .for_each(|memory_file| memory_files.push(memory_file));
    if memory_files.is_empty() {
        println!("cargo:warning=No memory file being specified.");
    } else if memory_files.len() > 1 {
        let mut msg = format!(
            "cargo:warning=Multiple devices enabled: [{}",
            memory_files[0]
        );
        for memory_file in memory_files {
            msg.push_str(", ");
            msg.push_str(memory_file);
        }
        msg.push(']');
        println!("{msg}");
        panic!();
    } else {
        let memory_file = memory_files.pop().unwrap();
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(read_to_string(memory_file).unwrap().as_bytes())
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed={memory_file}");
    }
}

const FEATURE_TO_FILE: [[&str; 2]; 40] = [
    ["CARGO_FEATURE_SAM3A4C_RT", "memory/atsam3a4c.x"],
    ["CARGO_FEATURE_SAM3A8C_RT", "memory/atsam3a8c.x"],
    ["CARGO_FEATURE_SAM3N0A_RT", "memory/atsam3n0a.x"],
    ["CARGO_FEATURE_SAM3N0B_RT", "memory/atsam3n0b.x"],
    ["CARGO_FEATURE_SAM3N0C_RT", "memory/atsam3n0c.x"],
    ["CARGO_FEATURE_SAM3N00A_RT", "memory/atsam3n00a.x"],
    ["CARGO_FEATURE_SAM3N00B_RT", "memory/atsam3n00b.x"],
    ["CARGO_FEATURE_SAM3N1A_RT", "memory/atsam3n1a.x"],
    ["CARGO_FEATURE_SAM3N1B_RT", "memory/atsam3n1b.x"],
    ["CARGO_FEATURE_SAM3N1C_RT", "memory/atsam3n1c.x"],
    ["CARGO_FEATURE_SAM3N2A_RT", "memory/atsam3n2a.x"],
    ["CARGO_FEATURE_SAM3N2B_RT", "memory/atsam3n2b.x"],
    ["CARGO_FEATURE_SAM3N2C_RT", "memory/atsam3n2c.x"],
    ["CARGO_FEATURE_SAM3N4A_RT", "memory/atsam3n4a.x"],
    ["CARGO_FEATURE_SAM3N4B_RT", "memory/atsam3n4b.x"],
    ["CARGO_FEATURE_SAM3N4C_RT", "memory/atsam3n4c.x"],
    ["CARGO_FEATURE_SAM3S1A_RT", "memory/atsam3s1a.x"],
    ["CARGO_FEATURE_SAM3S1B_RT", "memory/atsam3s1b.x"],
    ["CARGO_FEATURE_SAM3S1C_RT", "memory/atsam3s1c.x"],
    ["CARGO_FEATURE_SAM3S2A_RT", "memory/atsam3s2a.x"],
    ["CARGO_FEATURE_SAM3S2B_RT", "memory/atsam3s2b.x"],
    ["CARGO_FEATURE_SAM3S2C_RT", "memory/atsam3s2c.x"],
    ["CARGO_FEATURE_SAM3S4A_RT", "memory/atsam3s4a.x"],
    ["CARGO_FEATURE_SAM3S4B_RT", "memory/atsam3s4b.x"],
    ["CARGO_FEATURE_SAM3S4C_RT", "memory/atsam3s4c.x"],
    ["CARGO_FEATURE_SAM3S8B_RT", "memory/atsam3s8b.x"],
    ["CARGO_FEATURE_SAM3S8C_RT", "memory/atsam3s8c.x"],
    ["CARGO_FEATURE_SAM3SD8B_RT", "memory/atsam3sd8b.x"],
    ["CARGO_FEATURE_SAM3SD8C_RT", "memory/atsam3sd8c.x"],
    ["CARGO_FEATURE_SAM3U1C_RT", "memory/atsam3u1c.x"],
    ["CARGO_FEATURE_SAM3U1E_RT", "memory/atsam3u1e.x"],
    ["CARGO_FEATURE_SAM3U2C_RT", "memory/atsam3u2c.x"],
    ["CARGO_FEATURE_SAM3U2E_RT", "memory/atsam3u2e.x"],
    ["CARGO_FEATURE_SAM3U4C_RT", "memory/atsam3u4c.x"],
    ["CARGO_FEATURE_SAM3U4E_RT", "memory/atsam3u4e.x"],
    ["CARGO_FEATURE_SAM3X4C_RT", "memory/atsam3x4c.x"],
    ["CARGO_FEATURE_SAM3X4E_RT", "memory/atsam3x4e.x"],
    ["CARGO_FEATURE_SAM3X8C_RT", "memory/atsam3x8c.x"],
    ["CARGO_FEATURE_SAM3X8E_RT", "memory/atsam3x8e.x"],
    ["CARGO_FEATURE_SAM3X8H_RT", "memory/atsam3x8h.x"],
];
