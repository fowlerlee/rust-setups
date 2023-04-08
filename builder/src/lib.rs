

#[derive(Clone, Debug)]
struct House {
    floors: i32,
    bedrooms: i32,
    bathrooms: i32,
}


trait HouseBuilder {
    fn new() -> Self;
    fn set_floors(&mut self, floors: i32);
    fn set_bedrooms(&mut self, bedrooms: i32);
    fn set_bathrooms(&mut self, bathrooms: i32);
    fn build(&self) -> House;
}

#[derive(Debug)]
struct SmallHouseBuilder {
    house: House,
}

impl HouseBuilder for SmallHouseBuilder {
    fn new() -> Self {
        Self {
            house: House {
                floors: 1,
                bedrooms: 2,
                bathrooms: 1,
            },
        }
    }

    fn set_bathrooms(&mut self, bathrooms: i32) {
        self.house.bedrooms = bathrooms;
    }

    fn set_bedrooms(&mut self, bedrooms: i32) {
        self.house.bedrooms = bedrooms;
    }

    fn set_floors(&mut self, floors: i32) {
        self.house.floors = floors;
    }

    fn build(&self) -> House {
        self.house.clone()
    }
}

