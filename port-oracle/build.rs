fn main() {
    println!("cargo:rerun-if-changed=harvey");
    println!("cargo:rerun-if-changed=c");
    let h = "harvey";
    let mut build = cc::Build::new();
    build
        // Harvey's own sources, unmodified, against Harvey's own headers.
        .file(format!("{h}/sys/src/9/port/mul64fract.c"))
        .file(format!("{h}/sys/src/9/port/ipchecksum.c"))
        .file(format!("{h}/sys/src/9/port/latin1.c"))
        .file(format!("{h}/sys/src/9/port/watermarks.c"))
        // seprint() comes from the hosted libc here.
        .file("c/seprint.c")
        .include(format!("{h}/amd64/include"))
        .include(format!("{h}/sys/include"))
        .include(format!("{h}/sys/src/9/amd64"))
        .include(format!("{h}/sys/src/9/port"))
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        // As in Harvey's own clang.json: the headers rely on common symbols.
        .flag("-fcommon")
        .define("__HARVEY__", None)
        // The C is deliberately kept as Harvey wrote it, including the
        // signed-int sum in ipchksum.
        .warnings(false);
    // Prefix the C entry points so they don't clash with the c2rust copies.
    for f in ["mul64fract", "ipchksum", "latin1", "unicode", "initmark", "notemark", "seprintmark"] {
        build.define(f, format!("c_{f}").as_str());
    }
    build.compile("harvey_oracle");
}
