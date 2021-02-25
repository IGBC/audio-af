mod parameter;
mod buffer;

pub use audio_af_macros::AafBlock;
pub use parameter::Parameter;

type Frame = ();

pub trait AafAudioInterface {
    fn on_receive_block(self, frame: Frame) -> Frame;
}






#[cfg(test)]
mod tests {
    use super::AafBlock;
    use super::Parameter;

    #[derive(AafBlock)]
    struct MyBlock {
        _volume: Parameter,
        _other: u32,
    }

    #[test]
    fn test_derive() {
        let t = MyBlock {
            _volume: ("Volume", 0.0).into(),
            _other: 0,
        };
        println!("{:?}", t.list_parameter());
        assert_eq!(t.list_parameter(), vec!["Volume"]);
        assert_eq!(MyBlock::num_parameter(), 1);
    }
}
