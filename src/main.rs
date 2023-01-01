use kant_tools::songs::last_christmas;

#[tokio::main]
async fn main() {
    println!("Last christmas, I gave you my heart...");
    last_christmas().expect("Failed to play last christmas!");
}
