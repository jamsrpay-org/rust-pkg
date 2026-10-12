#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenStandard {
    Trc20,
    Bep20,
    Erc20,
    Spl,
}
