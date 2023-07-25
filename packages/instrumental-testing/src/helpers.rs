use cosmwasm_std::Event;
use osmosis_test_tube::{OsmosisTestApp, SigningAccount, Wasm};

pub fn wasm_file(contract_name: String) -> String {
    let snaked_name = contract_name.replace('-', "_");

    let target = format!("../../../target/wasm32-unknown-unknown/release/{}.wasm", snaked_name);

    if std::path::Path::new(&target).exists() {
        target
    } else {
        let arch = std::env::consts::ARCH;
        let artifacts_dir =
            std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_string());

        // Check for file with arch suffix
        let path_with_arch = format!("../../../{}/{}-{}.wasm", artifacts_dir, snaked_name, arch);

        // Check for file without arch suffix
        let path_without_arch = format!("../../../{}/{}.wasm", artifacts_dir, snaked_name);

        if std::path::Path::new(&path_with_arch).exists() {
            path_with_arch
        } else if std::path::Path::new(&path_without_arch).exists() {
            path_without_arch
        } else {
            panic!("No wasm file found for contract {}", contract_name);
        }
    }
}

pub fn store_code(
    wasm: &Wasm<OsmosisTestApp>,
    owner: &SigningAccount,
    contract_name: String,
) -> u64 {
    println!("wasm_file: {}", wasm_file(contract_name.clone()));
    let wasm_byte_code = std::fs::read(wasm_file(contract_name)).unwrap();
    wasm.store_code(&wasm_byte_code, None, owner).unwrap().data.code_id
}

pub fn parse_event_attribute(events: Vec<Event>, event: &str, key: &str) -> String {
    events
        .iter()
        .find(|e| e.ty == event)
        .unwrap()
        .attributes
        .iter()
        .find(|e| e.key == key)
        .unwrap()
        .value
        .clone()
}
