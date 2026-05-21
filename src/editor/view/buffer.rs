use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(filename: String) -> Result<Self, std::io::Error> {
        let contents = read_to_string(filename)?;
        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(line.to_string());
        }
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty();
    }
}
