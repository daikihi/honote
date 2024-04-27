use diesel::sql_types::Array;

#[derive(Debug)]
pub struct HoneyName {
    pub value: String,
}

#[derive(Debug)]
pub struct HoneyId {
    pub id: i64,
}

#[derive(Debug)]
pub struct Honey {
    pub id: HoneyId,
    pub name: HoneyName,
}

impl Honey {
    /// Creates a new [`Honey`].
    pub fn new(id: HoneyId, name: HoneyName) -> Self {
        Self { id, name }
    }

    pub fn to_string(&self) -> String {
        let id = self.id.id.to_string();
        let name = &self.name.value;
        format!("id = {}, name = {}", id, name)
    }
}

#[derive()]
pub struct Honeies {
    pub honey: Vec<Honey>,
}

impl Honeies {
    pub fn new(v: Vec<Honey>) -> Self {
        Self { honey: v }
    }

    pub fn save(&mut self, honey: Honey) {
        self.honey.push(honey);
    }

    /// Returns a reference to the get all honeies of this [`Honeies`].
    pub fn get_all_honeies(&self) -> &Vec<Honey> {
        let vec: &Vec<Honey> = &self.honey;
        vec
    }
}
