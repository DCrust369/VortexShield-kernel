// ============================================================
//  detector.rs — Detecção do Ambiente de Desktop
// ============================================================

use std::env;

/// Ambientes de desktop suportados
#[derive(Debug, Clone, PartialEq)]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    Lxde,
    Lxqt,
    Gnu,       // fallback genérico GTK/GNU
    Unknown,
}

impl DesktopEnvironment {
    pub fn name(&self) -> &str {
        match self {
            Self::Gnome   => "GNOME",
            Self::Kde     => "KDE Plasma",
            Self::Xfce    => "XFCE",
            Self::Lxde    => "LXDE",
            Self::Lxqt    => "LXQt",
            Self::Gnu     => "GNU / Generic",
            Self::Unknown => "Desconhecido",
        }
    }

    /// Ícone ASCII para o ambiente
    pub fn icon(&self) -> &str {
        match self {
            Self::Gnome   => "🦶",
            Self::Kde     => "🔷",
            Self::Xfce    => "🐭",
            Self::Lxde    => "🪶",
            Self::Lxqt    => "🪶",
            Self::Gnu     => "🐃",
            Self::Unknown => "❓",
        }
    }
}

/// Detecta o ambiente de desktop lendo variáveis de ambiente padrão do Linux.
pub fn detect_desktop_environment() -> DesktopEnvironment {
    // XDG_CURRENT_DESKTOP é o padrão FreeDesktop
    let xdg = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();

    // DESKTOP_SESSION é usado por vários display managers
    let session = env::var("DESKTOP_SESSION")
        .unwrap_or_default()
        .to_lowercase();

    // KDE_FULL_SESSION só existe no Plasma
    let kde_full = env::var("KDE_FULL_SESSION").is_ok();

    // GNOME_DESKTOP_SESSION_ID existe no GNOME clássico
    let gnome_id = env::var("GNOME_DESKTOP_SESSION_ID").is_ok();

    if kde_full || xdg.contains("kde") || session.contains("plasma") || session.contains("kde") {
        DesktopEnvironment::Kde
    } else if gnome_id || xdg.contains("gnome") || session.contains("gnome") {
        DesktopEnvironment::Gnome
    } else if xdg.contains("xfce") || session.contains("xfce") {
        DesktopEnvironment::Xfce
    } else if xdg.contains("lxqt") || session.contains("lxqt") {
        DesktopEnvironment::Lxqt
    } else if xdg.contains("lxde") || session.contains("lxde") || session.contains("lxsession") {
        DesktopEnvironment::Lxde
    } else if xdg.contains("gnustep") || session.contains("gnustep") {
        DesktopEnvironment::Gnu
    } else {
        // Tenta inferir pela presença de processos ou variáveis residuais
        infer_from_env()
    }
}

fn infer_from_env() -> DesktopEnvironment {
    // Verifica se o compositor/wm KDE está rodando
    if std::path::Path::new("/usr/bin/plasmashell").exists() {
        return DesktopEnvironment::Kde;
    }
    if std::path::Path::new("/usr/bin/gnome-shell").exists() {
        return DesktopEnvironment::Gnome;
    }
    if std::path::Path::new("/usr/bin/xfce4-session").exists() {
        return DesktopEnvironment::Xfce;
    }
    if std::path::Path::new("/usr/bin/lxsession").exists() {
        return DesktopEnvironment::Lxde;
    }
    if std::path::Path::new("/usr/bin/lxqt-session").exists() {
        return DesktopEnvironment::Lxqt;
    }
    DesktopEnvironment::Unknown
}
