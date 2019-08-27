#[derive(Eq, PartialEq, Clone)]
pub enum language {
    GCC,
    GNU,
    JAVA,
}

impl language {
    pub fn extension(&self) -> String {
        match self {
            language::GCC => "c".to_string(),
            language::GNU => "cpp".to_string(),
            language::JAVA => "java".to_string(),
        }
    }

    pub fn compile(&self) -> bool {
        match self {
            language::GCC => true,
            language::GNU => true,
            language::JAVA => true,
        }
    }

    pub fn compile_command(&self, p: String) -> String {
        match self {
            language::GCC => format!("gcc {}/Main.c -o {}/o -O2 -Wall -std=c11", p, p),
            language::GNU => format!("g++ {}/Main.cpp -o {}/o -O2 -Wall -std=c++17", p, p),
            language::JAVA => format!("javac {}/Main.java", p),
        }
    }

    pub fn running_command(&self, p: String) -> String {
        match self {
            language::GCC => format!("\"{}/o\"", p),
            language::GNU => format!("\"{}/o\"", p),
            language::JAVA => format!("\"java -cp {} Main\"", p),
        }
    }
}
