// ============================================================
//  utils.rs — Dicas e Utilitários por Ambiente de Desktop
// ============================================================

use colored::*;
use crate::detector::DesktopEnvironment;

/// Imprime dicas de uso específicas para o ambiente detectado
pub fn print_tips(env: &DesktopEnvironment) {
    println!("\n{} {} — Dicas de uso:",
        "💡".yellow(),
        env.name().cyan().bold()
    );
    println!("{}", "─".repeat(55).dimmed());

    let tips = get_tips(env);
    for (i, tip) in tips.iter().enumerate() {
        println!("  {}. {}", (i + 1).to_string().yellow(), tip);
    }

    println!("\n{}", get_install_cmd(env).dimmed());
}

fn get_tips(env: &DesktopEnvironment) -> Vec<&'static str> {
    match env {
        DesktopEnvironment::Gnome => vec![
            "Use `gsettings` para configurar o GNOME via terminal",
            "Instale extensões em https://extensions.gnome.org",
            "Atalho Alt+F2 abre o lançador de comandos GNOME",
            "Use GNOME Tweaks para ajustes avançados de aparência",
            "Wayland é o padrão — exporte GDK_BACKEND=x11 se precisar de X11",
        ],
        DesktopEnvironment::Kde => vec![
            "Use `qdbus` para inspecionar serviços D-Bus do Plasma",
            "Atividades do KDE (Super+Tab) separam áreas de trabalho por contexto",
            "kwriteconfig5/6 edita configurações KConfig via terminal",
            "Dolphin suporta scripts de serviço em ~/.local/share/kio/servicemenus/",
            "Krunner (Alt+Space) é o launcher mais poderoso do Plasma",
        ],
        DesktopEnvironment::Xfce => vec![
            "Edite atalhos em Configurações › Gerenciador de Janelas",
            "xfconf-query permite configurar o XFCE pelo terminal",
            "Thunar tem ações personalizadas (Editar › Configurar Ações)",
            "xfce4-terminal tem abas, drops e transparência nativa",
            "O painel XFCE4 suporta plugins .so instaláveis",
        ],
        DesktopEnvironment::Lxde | DesktopEnvironment::Lxqt => vec![
            "O LXDE/LXQt usa PCManFM como gerenciador de arquivos padrão",
            "Edite autostart em ~/.config/autostart/",
            "lxpanel --profile LXDE reinicia o painel sem fazer logout",
            "O LXQt Wallet gerencia segredos via libsecret",
            "Use compton/picom para efeitos de composição leves",
        ],
        DesktopEnvironment::Gnu => vec![
            "GNUstep usa defaults write/read para configuração",
            "gopen abre arquivos com o aplicativo padrão GNUstep",
            "NSGlobalDomain contém preferências globais do sistema",
        ],
        DesktopEnvironment::Unknown => vec![
            "Ambiente não reconhecido — verifique XDG_CURRENT_DESKTOP",
            "Execute com: XDG_CURRENT_DESKTOP=GNOME ./de_loader",
            "Tente instalar as bibliotecas GTK3/4 ou Qt5/6 base",
        ],
    }
}

fn get_install_cmd(env: &DesktopEnvironment) -> String {
    let (apt, dnf, pacman) = match env {
        DesktopEnvironment::Gnome => (
            "apt install libgtk-4-dev libgtk-3-dev libglib2.0-dev",
            "dnf install gtk4-devel gtk3-devel glib2-devel",
            "pacman -S gtk4 gtk3 glib2",
        ),
        DesktopEnvironment::Kde => (
            "apt install qt6-base-dev libkf5coreaddons-dev libkf6coreaddons-dev",
            "dnf install qt6-qtbase-devel kf6-kcoreaddons-devel",
            "pacman -S qt6-base kf6-kcoreaddons",
        ),
        DesktopEnvironment::Xfce => (
            "apt install libxfce4ui-2-dev libxfce4util-dev libxfconf-0-dev",
            "dnf install xfce4-dev-tools libxfce4ui-devel",
            "pacman -S libxfce4ui libxfce4util xfconf",
        ),
        DesktopEnvironment::Lxde | DesktopEnvironment::Lxqt => (
            "apt install libfm-dev liblxqt-dev libfm-qt-dev",
            "dnf install libfm-devel lxqt-build-tools",
            "pacman -S libfm lxqt-build-tools",
        ),
        _ => (
            "apt install libgtk-3-dev libglib2.0-dev",
            "dnf install gtk3-devel glib2-devel",
            "pacman -S gtk3 glib2",
        ),
    };

    format!(
        "Instalar dependências:\n  Debian/Ubuntu : {}\n  Fedora/RHEL   : {}\n  Arch Linux    : {}",
        apt, dnf, pacman
    )
}
