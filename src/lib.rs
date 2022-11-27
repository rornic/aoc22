pub mod input {
    use std::io;
    pub fn read_input(name: &str) -> Result<String, io::Error> {
        std::fs::read_to_string(format!("input/{}", name))
    }
}
