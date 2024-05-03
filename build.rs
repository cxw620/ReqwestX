fn main() {
    rust2go::Builder::new()
        .with_go_src("./reqwestx_go/ffi")
        // .with_copy_lib(rust2go::CopyLib::DefaultPath)
        // .with_link(rust2go::LinkType::Dynamic)
        // .with_regen("./src/structs.rs", "./go/gen.go")
        .build();
}
