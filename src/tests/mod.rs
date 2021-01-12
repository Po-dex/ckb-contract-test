use crate::TestInfo;

#[test]
fn test_read_files() {
    let tx_path = "./src/tests/json/transaction";
    let inputs = "./src/tests/json/input_cell";

    let info = TestInfo::new(tx_path, inputs).unwrap();
    println!(
        "{:?}\n {:?}\n {:?}",
        info.tx, info.inputs_info, info.scripts
    );
}
