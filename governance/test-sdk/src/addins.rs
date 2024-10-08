use {
    lazy_static::lazy_static,
    lumos_program_test::find_file,
    std::{process::Command, sync::Mutex},
};

lazy_static! {
    pub static ref VOTER_WEIGHT_ADDIN_BUILD_GUARD: Mutex::<u8> = Mutex::new(0);
}

pub fn ensure_addin_mock_is_built() {
    if find_file("lpl_governance_voter_weight_addin_mock.so").is_none() {
        let _guard = VOTER_WEIGHT_ADDIN_BUILD_GUARD.lock().unwrap();
        if find_file("lpl_governance_addin_mock.so").is_none() {
            assert!(Command::new("cargo")
                .args([
                    "build-sbf",
                    "--manifest-path",
                    "../addin-mock/program/Cargo.toml",
                ])
                .status()
                .expect("Failed to build lpl-governance-addin-mock program")
                .success());
        }
    }
}
