fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut build = cc::Build::new();
    build.file("src/triple.c");
    build.compile("triple");
    Ok(())
}
