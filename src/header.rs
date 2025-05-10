pub struct Header {
    pub name: String,
    pub value: String,
}
impl Header {
    pub fn new(str: String) -> Self {
        let mut parts = str.splitn(2, ": ");
        let name = parts.next().unwrap_or("").to_string();
        let value = parts.next().unwrap_or("").to_string();

        Self {
            name,
            value,
        }
    }
}