use embed_resource;

fn main() {
    // Tell Cargo to rerun this build script if the version.rc file changes
    println!("cargo:rerun-if-changed=version.rc");

    // Compile the version.rc file into the EXE
    embed_resource::compile::<&str, &str, Vec<_>>("version.rc", vec![]);
}
