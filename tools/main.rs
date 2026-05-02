// ============================================================
//  DE Loader — Carregador Universal de Bibliotecas de Desktop
//  Suporte: GNOME · GNU · XFCE · KDE Plasma · LXDE
// ============================================================

mod detector;
mod loader;
mod report;
mod utils;

use colored::*;
use std::process;

fn main() {
    print_banner();

    // 1. Detectar ambiente de desktop atual
    let env = detector::detect_desktop_environment();
    println!(
        "{} Ambiente detectado: {}\n",
        "►".cyan().bold(),
        env.name().yellow().bold()
    );

    // 2. Carregar todas as bibliotecas disponíveis
    let results = loader::load_all_libraries();

    // 3. Exibir relatório completo
    report::print_report(&results);

    // 4. Dicas de uso para o ambiente atual
    utils::print_tips(&env);

    // 5. Salvar relatório em JSON
    match report::save_json_report(&results) {
        Ok(path) => println!(
            "\n{} Relatório salvo em: {}",
            "✔".green().bold(),
            path.cyan()
        ),
        Err(e) => eprintln!("{} Não foi possível salvar relatório: {}", "✘".red(), e),
    }

    // Código de saída: 0 se ao menos uma lib foi carregada
    let any_ok = results.iter().any(|r| r.loaded);
    process::exit(if any_ok { 0 } else { 1 });
}

fn print_banner() {
    println!("{}", "╔══════════════════════════════════════════════════════╗".purple().bold());
    println!("{}", "║        DE Loader  v1.0  —  Linux Library Manager     ║".purple().bold());
    println!("{}", "║   GNOME · GNU · XFCE · KDE Plasma · LXDE             ║".purple().bold());
    println!("{}", "╚══════════════════════════════════════════════════════╝".purple().bold());
    println!();
}
