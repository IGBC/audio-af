mod parameter;

pub use audio_af_macros::AafBlock;
pub use parameter::Parameter;

type Frame = ();

pub trait AafAudioInterface {
    fn on_receive_block(self, frame: Frame) -> Frame;
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::AafBlock;
    use super::Parameter;

    #[derive(AafBlock)]
    struct _MyBlock {
        volume: Parameter,
        other: u32,
    }

    #[test]
    fn test_derive() {
        
    }
}
