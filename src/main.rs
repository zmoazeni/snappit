extern crate libc;
use libc::size_t;

// if the build.rs also sets "-l static=snappy", this becomes redundant
#[link(name = "snappy", kind = "static")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}
