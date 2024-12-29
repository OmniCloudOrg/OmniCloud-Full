fn main() {
    std::process::Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init");
}