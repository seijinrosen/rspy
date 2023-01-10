use rspy::input;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("current version: {VERSION}");
    let new_version = input("? new version: ");

    println!("Bump version from v{VERSION} to v{new_version}");
}
