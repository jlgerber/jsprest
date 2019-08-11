use failure::Fail;

#[derive(Debug, Fail, PartialEq, Clone)]
pub enum JspRestError {
        #[fail(display = "Incorrect Data: {:?}", _0)]
        IncorrectData(String)
}
