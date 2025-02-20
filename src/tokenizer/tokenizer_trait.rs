/**
   This should be the only part exposed to lib.rs
*/
pub trait Tokenizer {
    fn segment(&self, text: &str, safe: Option<bool>, parallel: Option<bool>) -> Vec<String>;

    fn segment_to_string(
        &self,
        text: &str,
        safe: Option<bool>,
        parallel: Option<bool>,
    ) -> Vec<String>;
}
