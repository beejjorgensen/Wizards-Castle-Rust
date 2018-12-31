#[derive(PartialEq,Copy,Clone,Debug)]
pub enum Error {
    NotEnoughPoints,
    NotEnoughGP,
    WrongState,
}