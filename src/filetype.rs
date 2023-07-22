#[derive(Debug)]
pub struct FileType {
    name: String,
    hl_opts: HighLightingOptions,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct HighLightingOptions {
    numbers: bool,
    strings: bool,
}

impl HighLightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }

    pub fn strings(self) -> bool {
        self.strings
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighLightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighLightingOptions { numbers: true, strings: true },
            };
        }
        Self::default()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighLightingOptions {
        self.hl_opts
    }
}
