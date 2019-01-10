#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Error {
    NotEnoughPoints,
    NotEnoughGP,
    WrongState,
    CantGo,
    Blind,
    OutOfBounds,
    BribeNotPossible,
    BribeMustProposition, // Need to proposition successfully before calling bribe()
    VendorMustOfferTreasure, // Need to offer a treasure before calling accept or reject
}
