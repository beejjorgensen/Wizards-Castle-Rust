pub const CURSE_COUNT: u32 = 3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CurseType {
    None,
    Forgetfulness,
    TheLeech,
    Lethargy,
}

pub struct Curse {}

impl Curse {
    pub fn get_curse_by_id(id: u32) -> CurseType {
        match id {
            0 => CurseType::Forgetfulness,
            1 => CurseType::TheLeech,
            2 => CurseType::Lethargy,
            _ => panic!("get_curse_by_id: unknown id"),
        }
    }
}
