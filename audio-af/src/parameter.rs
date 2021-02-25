pub struct Parameter {
    name: &'static str,
    value: f32,
}

impl Parameter {
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    } 
}
