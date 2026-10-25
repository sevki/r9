fn main() {
    println!("cargo:rerun-if-changed=c");
    let mut build = cc::Build::new();
    build
        .file("c/mul64fract.c")
        .file("c/ipchecksum.c")
        .file("c/latin1.c")
        .file("c/watermarks.c")
        .file("c/seprint.c")
        // The C is deliberately kept as Harvey wrote it, including the
        // signed-int sum in ipchksum.
        .warnings(false);
    // Prefix the C entry points so they don't clash with the c2rust copies.
    for f in ["mul64fract", "ipchksum", "latin1", "unicode", "initmark", "notemark", "seprintmark"]
    {
        build.define(f, format!("c_{f}").as_str());
    }
    build.compile("harvey_oracle");
}
