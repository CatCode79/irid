
//= STRUCTS ========================================================================================

pub struct Config {
    pub clear_color: wgpu::Color,
}


impl Config {
    ///
    // TODO: da fare
    pub fn new(_filename: &String) -> Self {
        Config::default()
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
        }
    }
}
