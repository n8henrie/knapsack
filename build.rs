fn main() {
    println!(
        "cargo:rustc-link-search=native={}{}",
        env!("PYENV_ROOT"),
        "/versions/3.9.6/Python.framework/Versions/3.9/lib"
    );
}
