// IKinder

pub fn start() {
    Temp::get_info().convert();
}

enum TempType {
    Celsius,
    Fahrenheit,
}
impl Default for TempType {
    fn default() -> Self {
        Self::Celsius
    }
}

#[derive(Default)]
struct Temp {
    value: f32,
    from_type: TempType,
    converted: bool,
}
impl Temp {
    fn new(value: f32, from_type: TempType, converted: bool) -> Self {
        Self {
            value,
            from_type,
            converted,
        }
    }
    #[must_use]
    fn get_info() -> Self {
        loop {
            if let Some(first_temperature) = Self::get_value() {
                return first_temperature;
            }
        }
    }
    fn get_value() -> Option<Self> {
        println!("Temperature converter.\n(1) C -> F\n(2) F -> C");
        let mut temp_type_input = String::new();
        std::io::stdin().read_line(&mut temp_type_input).ok()?;
        let from_type = match temp_type_input.trim().parse::<u8>().ok()? {
            1 => Some(TempType::Celsius),
            2 => Some(TempType::Fahrenheit),
            _ => {
                println!("Invalid option, enter 1 or 2!");
                return None;
            }
        }?;

        println!("Enter the number:");
        let mut temp_value_input = String::new();
        std::io::stdin().read_line(&mut temp_value_input).ok()?;
        let value = temp_value_input.trim().parse::<f32>().ok()?;

        Some(Temp::new(value, from_type, false))
    }
    fn convert(self) {
        match self.from_type {
            TempType::Celsius => {
                let result = self.value * (9.0 / 5.0) + 32.0;
                println!("{} in C is {:.0} in F", self.value, result);
            }
            TempType::Fahrenheit => {
                let result = (self.value - 32.0) * (5.0 / 9.0);
                println!("{}°F is {:.0}°C", self.value, result);
            }
        };
    }
}
