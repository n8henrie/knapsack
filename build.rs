fn main() {
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,/Applications/Xcode.app/Contents/Developer/Library/Frameworks"
    );
}
