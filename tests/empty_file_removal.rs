mod utils {
    pub mod file_operations;
}
// use env_logger::{Builder, Env};
#[test]
fn empty_file_removal() {
    // Set the logger to the max_level trace
    // Builder::from_env(Env::default().default_filter_or("trace")).init();
    let dir1 = "./tests/data/empty_file_removal/";
    let dir2 = "./tests/data/empty_file_removal_ideal/";
    let result = utils::file_operations::compare_directories(dir1, dir2, 4);
    assert_eq!(result, true);
}