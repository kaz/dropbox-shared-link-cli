struct Error(String);
impl std::error::Error for Error {}

macro_rules! implement {
    ($trait:ident) => {
        impl std::fmt::$trait for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

implement!(Debug);
implement!(Display);

pub fn emit<S>(msg: S) -> Box<dyn std::error::Error>
where
    S: Into<String>,
{
    Box::new(Error(msg.into()))
}
