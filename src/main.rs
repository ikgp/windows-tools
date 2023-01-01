use std::path::Path;

use kant_tools::songs::last_christmas;
use kant_tools::js::execute_js_from_path;

#[tokio::main]
async fn main() {
    println!("Last christmas, I gave you my heart...");
    last_christmas().expect("Failed to play last christmas!");
    println!("But the very next day, you gave it away...");
    execute_js_from_path(&Path::new("examples/tetris.js")).await.expect("Failed to execute JS");
}
