use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

struct Scanner {
    found_files: Vec<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            found_files: Vec::new(),
        }
    }

    fn scan_directory(&mut self, start_path: &str) {
        for entry in WalkDir::new(start_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "exe" || extension == "bat" {
                        if let Some(path_str) = path.to_str() {
                            self.found_files.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    fn generate_report(&self) -> std::io::Result<()> {
        let datetime = Local::now().format("%Y%m%d_%H%M%S");
        let report_name = format!("scan_report_{}.txt", datetime);
        let mut file = fs::File::create(&report_name)?;

        writeln!(file, "Scan Report - {}", datetime)?;
        writeln!(
            file,
            "Achamos {} arquivos suspeitos\n",
            self.found_files.len()
        )?;

        for file_path in &self.found_files {
            writeln!(file, "{}", file_path)?;
        }

        println!("Relatório gerado: {}", report_name);
        Ok(())
    }

    fn remove_file(&self, path: &str) -> std::io::Result<()> {
        // Basic safety check - don't delete system files
        let path = Path::new(path);
        if path.starts_with("/windows") || path.starts_with("C:\\Windows") {
            println!("Não foi possível deletar os aquivos.");
            return Ok(());
        }

        match fs::remove_file(path) {
            Ok(_) => println!("Removido com sucesso: {}", path.display()),
            Err(e) => println!("Falha ao remover{}: {}", path.display(), e),
        }
        Ok(())
    }
}

fn main() {
    let mut scanner = Scanner::new();

    // Scan the current directory
    println!("Começando escaneamento...");
    scanner.scan_directory(".");

    // Generate report
    if let Err(e) = scanner.generate_report() {
        println!("Erro ao gerar relatório: {}", e);
    }

    // Optional: Remove files (with confirmation)
    println!("\nQuer remover os potenciais arquivos maliciosos? (s/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "s" {
        for file in &scanner.found_files {
            println!("Remover {}? (s/n)", file);
            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).unwrap();

            if confirm.trim().to_lowercase() == "s" {
                if let Err(e) = scanner.remove_file(file) {
                    println!("Erro removendo o arquivo: {}", e);
                }
            }
        }
    }
}
