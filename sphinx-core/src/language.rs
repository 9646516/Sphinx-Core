#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Language {
    GCC,
    GNU,
    CLANG,
    CLANGPP,
    JAVA,
    PY2,
    PY3,
    RUST,
}

impl Language {
    pub fn from(str: u64) -> Language {
        match str {
            0 => Language::GCC,
            1 => Language::GNU,
            2 => Language::CLANG,
            3 => Language::CLANGPP,
            4 => Language::JAVA,
            5 => Language::RUST,
            6 => Language::PY2,
            7 => Language::PY3,
            _ => panic!("lang err"),
        }
    }
    pub fn extension(&self) -> String {
        match self {
            Language::GCC => "c".to_string(),
            Language::GNU => "cpp".to_string(),
            Language::CLANG => "c".to_string(),
            Language::CLANGPP => "cpp".to_string(),
            Language::JAVA => "java".to_string(),
            Language::PY2 => "py".to_string(),
            Language::PY3 => "py".to_string(),
            Language::RUST => "rs".to_string(),
        }
    }

    pub fn compile(&self) -> bool {
        match self {
            Language::GCC => true,
            Language::GNU => true,
            Language::CLANG => true,
            Language::CLANGPP => true,
            Language::JAVA => true,
            Language::PY2 => false,
            Language::PY3 => false,
            Language::RUST => true,
        }
    }

    pub fn compile_command(&self, p: String) -> String {
        match self {
            Language::GCC => format!(
                "gcc {}/Main.c -o {}/o -O2 -Wall -std=c11 -fmax-errors=15",
                p, p
            ),
            Language::GNU => format!(
                "g++ {}/Main.cpp -o {}/o -O2 -Wall -std=c++17 -fmax-errors=15",
                p, p
            ),
            Language::CLANG => format!(
                "clang {}/Main.c -o {}/o -O2 -Wall -std=c11 -fmax-errors=15",
                p, p
            ),
            Language::CLANGPP => format!(
                "clang++ {}/Main.cpp -o {}/o -O2 -Wall -std=c++17 -fmax-errors=15",
                p, p
            ),
            Language::JAVA => format!("javac {}/Main.java", p),
            Language::RUST => format!("rustc -O {}/Main.rs -o {}/o", p, p),
            _ => String::new(),
        }
    }

    pub fn running_command(&self, p: String) -> String {
        match self {
            Language::GCC => format!("\"{}/o\"", p),
            Language::GNU => format!("\"{}/o\"", p),
            Language::CLANG => format!("\"{}/o\"", p),
            Language::CLANGPP => format!("\"{}/o\"", p),
            Language::JAVA => format!("\"java -cp {} -Xms64m -Xmx128m Main\"", p),
            Language::PY2 => format!("\"python {}/Main.py\"", p),
            Language::PY3 => format!("\"python3 {}/Main.py\"", p),
            Language::RUST => format!("\"{}/o\"", p),
        }
    }
}
