use num::clamp;

pub struct Parameter {
    pub name: &'static str,
    pub value: f32,
}

impl Parameter {
    pub fn new(name: &'static str, value: f32) -> Self {
        Self {
            name,
            value,
        }
    }
    
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = clamp(value, 0.0, 1.0);
    } 
}

impl From<(&'static str, f32)> for Parameter {
    fn from(input: (&'static str, f32)) -> Self {
        Self::new(input.0, input.1)
    }
} 