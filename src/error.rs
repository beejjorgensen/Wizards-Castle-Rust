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
    VendorNoTreasure,     // If the player is trying to get the vendor to offer on no treasure
    VendorMustOfferTreasure, // Need to offer a treasure before calling accept or reject
}
