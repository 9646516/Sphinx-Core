#[derive(Eq, PartialEq, Clone)]
pub enum language {
    GCC,
    GNU,
}

impl language {
    pub fn extension(&self) -> String {
        match self {
            language::GCC => "c".to_string(),
            language::GNU => "cpp".to_string(),
        }
    }

    pub fn compile(&self) -> bool {
        match self {
            language::GCC => true,
            language::GNU => true,
        }
    }

    pub fn compile_command(&self, p: String) -> String {
        match self {
            language::GCC => format!("gcc {}/main.c -o {}/o -O2 -Wall -std=c11", p, p),
            language::GNU => format!("g++ {}/main.cpp -o {}/o -O2 -Wall -std=c++17", p, p),
        }
    }

    pub fn running_command(&self, p: String) -> String {
        match self {
            language::GCC => format!("\"{}/o\"", p),
            language::GNU => format!("\"{}/o\"", p),
        }
    }
}
