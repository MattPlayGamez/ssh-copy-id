/// Compile the Windows icon resource file into the crate.
///
/// This function is invoked by Cargo when building the crate. It is not
/// intended to be called directly.
fn main() {
    let _ = embed_resource::compile("assets/icon.rc", embed_resource::NONE);
}
