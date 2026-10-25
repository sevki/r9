fn main() {
    println!("cargo:rerun-if-changed=c");
    cc::Build::new()
        .file("c/mul64fract.c")
        .file("c/ipchecksum.c")
        .file("c/latin1.c")
        .file("c/watermarks.c")
        .file("c/seprint.c")
        // The C is deliberately kept as Harvey wrote it, including the
        // signed-int sum in ipchksum.
        .warnings(false)
        .compile("harvey_oracle");
}
