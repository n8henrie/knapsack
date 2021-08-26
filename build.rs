fn main() {
    println!(
        "cargo:rustc-link-search=native={}/versions/3.9.6/Python.framework/Versions/3.9/lib",
        env!("PYENV_ROOT"),
    );
}
