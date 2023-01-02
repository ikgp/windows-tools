use kant_tools::songs::last_christmas;
use kant_tools::js::execute_js_from_url;

#[tokio::main]
async fn main() {
    println!("Last christmas, I gave you my heart...");
    if let Err(e) = last_christmas() {
        println!("Failed to play last christmas: {e}");
    };
    println!("But the very next day, you gave it away...");
    if let Err(e) = execute_js_from_url("https://raw.githubusercontent.com/ikgp/windows-tools/1e7fc3e8ffc9fa549190906543d6ade3932c7e30/examples/tetris.js").await {
        println!("Failed to play the tetris theme: {e}");
    };
}
