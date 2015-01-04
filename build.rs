fn main() {
    // could also send "-l static=snappy"
    println!("cargo:rustc-flags= -L /usr/local/lib")
}
