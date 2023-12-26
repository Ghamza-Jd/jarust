#[cfg(feature = "echotest")]
pub mod echotest;

pub enum Plugin {
    #[cfg(feature = "echotest")]
    EchoTest,
}
