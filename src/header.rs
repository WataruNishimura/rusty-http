pub struct Header {
    pub name: String,
    pub value: String,
}
impl Header {
    pub fn new(str: String) -> Self {
        let mut parts = str.splitn(2, ": ");
    }
}