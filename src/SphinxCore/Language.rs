#[derive(Eq, PartialEq, Clone)]
pub enum language {
    GCC,
    GNU,
    CLANG,
    CLANGPP,
    JAVA,
    PY2,
    PY3,
    RUST,
}

impl language {
    pub fn extension(&self) -> String {
        match self {
            language::GCC => "c".to_string(),
            language::GNU => "cpp".to_string(),
            language::CLANG => "c".to_string(),
            language::CLANGPP => "cpp".to_string(),
            language::JAVA => "java".to_string(),
            language::PY2 => "py".to_string(),
            language::PY3 => "py".to_string(),
            language::RUST => "rs".to_string(),
        }
    }

    pub fn compile(&self) -> bool {
        match self {
            language::GCC => true,
            language::GNU => true,
            language::CLANG => true,
            language::CLANGPP => true,
            language::JAVA => true,
            language::PY2 => false,
            language::PY3 => false,
            language::RUST => true,
        }
    }

    pub fn compile_command(&self, p: String) -> String {
        match self {
            language::GCC => format!(
                "gcc {}/Main.c -o {}/o -O2 -Wall -std=c11 -fmax-errors=15",
                p, p
            ),
            language::GNU => format!(
                "g++ {}/Main.cpp -o {}/o -O2 -Wall -std=c++17 -fmax-errors=15",
                p, p
            ),
            language::CLANG => format!(
                "clang {}/Main.c -o {}/o -O2 -Wall -std=c11 -fmax-errors=15",
                p, p
            ),
            language::CLANGPP => format!(
                "clang++ {}/Main.cpp -o {}/o -O2 -Wall -std=c++17 -fmax-errors=15",
                p, p
            ),
            language::JAVA => format!("javac {}/Main.java", p),
            language::RUST => format!("rustc -O {}/Main.rs -o {}/o", p, p),
            _ => String::new(),
        }
    }

    pub fn running_command(&self, p: String) -> String {
        match self {
            language::GCC => format!("\"{}/o\"", p),
            language::GNU => format!("\"{}/o\"", p),
            language::CLANG => format!("\"{}/o\"", p),
            language::CLANGPP => format!("\"{}/o\"", p),
            language::JAVA => format!("\"java -cp {} Main\"", p),
            language::PY2 => format!("\"python {}/Main.py\"", p),
            language::PY3 => format!("\"python3 {}/Main.py\"", p),
            language::RUST => format!("\"{}/o\"", p),
        }
    }
}
