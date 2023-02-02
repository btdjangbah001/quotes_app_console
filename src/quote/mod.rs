#[derive(Debug)]
pub struct Quote<'a> {
    pub author: & 'a str,
    pub quote: & 'a str,
}

impl Quote<'_> {
    pub fn new<'a>(author: &'a str, quote: &'a str) -> Quote<'a>{
        Quote{
            author,
            quote
        }
    }
}