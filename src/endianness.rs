#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum Endianness {
    Little,
    #[default]
    Native,
    Big,
}
