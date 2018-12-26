#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Armor {
    None,
    Leather,
    Chainmail,
    Plate,
}

impl Armor {
    pub fn cost(a:Armor, is_vendor:bool) -> usize {
        let value;

        if is_vendor {
            if a == Armor::None {
                value = 0;

            } else {
                let id = Armor::to_id(a);

                value = (id + 1) * 10;
            }

        } else {
            value = match a {
                Armor::None => 0,
                Armor::Leather => 1250,
                Armor::Chainmail => 1500,
                Armor::Plate => 2000,
            }
        }

        value
    }

    fn to_id(a:Armor) -> usize {
        match a {
            Armor::None => 9999,
            Armor::Leather => 0,
            Armor::Chainmail => 1,
            Armor::Plate => 2,
        }
    }
}