// ============================================================
//  loader.rs — Carregamento Dinâmico de Bibliotecas
// ============================================================

use libloading::Library;
use std::time::{Duration, Instant};

/// Resultado do carregamento de uma biblioteca
#[derive(Debug, Clone)]
pub struct LibResult {
    pub group:    String,       // Grupo (GNOME, KDE, etc.)
    pub name:     String,       // Nome amigável
    pub soname:   String,       // Nome do arquivo .so
    pub loaded:   bool,         // Carregada com sucesso?
    pub version:  Option<String>,
    pub load_time: Duration,
    pub error:    Option<String>,
}

// ─── Definição das bibliotecas a tentar carregar ───────────────────────────

struct LibSpec {
    group:   &'static str,
    name:    &'static str,
    sonames: &'static [&'static str], // tentativas em ordem
}

/// Catálogo completo de bibliotecas por ambiente
const LIBRARY_CATALOG: &[LibSpec] = &[
    // ── GNOME / GTK ──────────────────────────────────────────
    LibSpec { group: "GNOME", name: "GTK 4",           sonames: &["libgtk-4.so.1", "libgtk-4.so"] },
    LibSpec { group: "GNOME", name: "GTK 3",           sonames: &["libgtk-3.so.0", "libgtk-3.so"] },
    LibSpec { group: "GNOME", name: "GLib 2",          sonames: &["libglib-2.0.so.0", "libglib-2.0.so"] },
    LibSpec { group: "GNOME", name: "GObject 2",       sonames: &["libgobject-2.0.so.0", "libgobject-2.0.so"] },
    LibSpec { group: "GNOME", name: "GIO 2",           sonames: &["libgio-2.0.so.0", "libgio-2.0.so"] },
    LibSpec { group: "GNOME", name: "Pango",           sonames: &["libpango-1.0.so.0", "libpango-1.0.so"] },
    LibSpec { group: "GNOME", name: "Cairo",           sonames: &["libcairo.so.2", "libcairo.so"] },
    LibSpec { group: "GNOME", name: "GDK-Pixbuf",     sonames: &["libgdk_pixbuf-2.0.so.0", "libgdk_pixbuf-2.0.so"] },
    LibSpec { group: "GNOME", name: "ATK",             sonames: &["libatk-1.0.so.0", "libatk-1.0.so"] },
    LibSpec { group: "GNOME", name: "libgnome-ui",     sonames: &["libgnome-2.so.0", "libgnome-2.so"] },
    LibSpec { group: "GNOME", name: "GSettings",       sonames: &["libglib-2.0.so.0"] },
    LibSpec { group: "GNOME", name: "libadwaita",      sonames: &["libadwaita-1.so.0", "libadwaita-1.so"] },

    // ── GNU / Sistema ─────────────────────────────────────────
    LibSpec { group: "GNU",   name: "GNU C Library",   sonames: &["libc.so.6", "libc.so"] },
    LibSpec { group: "GNU",   name: "GNU Math",        sonames: &["libm.so.6", "libm.so"] },
    LibSpec { group: "GNU",   name: "GNU Pthreads",    sonames: &["libpthread.so.0", "libpthread.so"] },
    LibSpec { group: "GNU",   name: "GNU DL",          sonames: &["libdl.so.2", "libdl.so"] },
    LibSpec { group: "GNU",   name: "GNU Readline",    sonames: &["libreadline.so.8", "libreadline.so.7", "libreadline.so"] },
    LibSpec { group: "GNU",   name: "GNU NCurses",     sonames: &["libncurses.so.6", "libncursesw.so.6", "libncurses.so"] },
    LibSpec { group: "GNU",   name: "GNU TLS",         sonames: &["libgnutls.so.30", "libgnutls.so"] },
    LibSpec { group: "GNU",   name: "OpenSSL (libssl)", sonames: &["libssl.so.3", "libssl.so.1.1", "libssl.so"] },
    LibSpec { group: "GNU",   name: "zlib",            sonames: &["libz.so.1", "libz.so"] },
    LibSpec { group: "GNU",   name: "libxml2",         sonames: &["libxml2.so.2", "libxml2.so"] },
    LibSpec { group: "GNU",   name: "libffi",          sonames: &["libffi.so.8", "libffi.so.7", "libffi.so"] },
    LibSpec { group: "GNU",   name: "DBus",            sonames: &["libdbus-1.so.3", "libdbus-1.so"] },

    // ── XFCE ──────────────────────────────────────────────────
    LibSpec { group: "XFCE",  name: "libxfce4ui",      sonames: &["libxfce4ui-2.so.0", "libxfce4ui-1.so.0", "libxfce4ui-2.so"] },
    LibSpec { group: "XFCE",  name: "libxfce4util",    sonames: &["libxfce4util.so.7", "libxfce4util.so.6", "libxfce4util.so"] },
    LibSpec { group: "XFCE",  name: "libxfconf",       sonames: &["libxfconf-0.so.3", "libxfconf-0.so.2", "libxfconf-0.so"] },
    LibSpec { group: "XFCE",  name: "Garcon (menu)",   sonames: &["libgarcon-1.so.0", "libgarcon-gtk3-1.so.0", "libgarcon-1.so"] },
    LibSpec { group: "XFCE",  name: "Thunar (lib)",    sonames: &["libthunarx-3.so.0", "libthunarx-2.so.0", "libthunarx-3.so"] },
    LibSpec { group: "XFCE",  name: "libexo",          sonames: &["libexo-2.so.0", "libexo-1.so.0", "libexo-2.so"] },

    // ── KDE Plasma / Qt ───────────────────────────────────────
    LibSpec { group: "KDE",   name: "Qt6 Core",        sonames: &["libQt6Core.so.6", "libQt6Core.so"] },
    LibSpec { group: "KDE",   name: "Qt6 Widgets",     sonames: &["libQt6Widgets.so.6", "libQt6Widgets.so"] },
    LibSpec { group: "KDE",   name: "Qt6 GUI",         sonames: &["libQt6Gui.so.6", "libQt6Gui.so"] },
    LibSpec { group: "KDE",   name: "Qt6 Network",     sonames: &["libQt6Network.so.6", "libQt6Network.so"] },
    LibSpec { group: "KDE",   name: "Qt6 DBus",        sonames: &["libQt6DBus.so.6", "libQt6DBus.so"] },
    LibSpec { group: "KDE",   name: "Qt5 Core",        sonames: &["libQt5Core.so.5", "libQt5Core.so"] },
    LibSpec { group: "KDE",   name: "Qt5 Widgets",     sonames: &["libQt5Widgets.so.5", "libQt5Widgets.so"] },
    LibSpec { group: "KDE",   name: "KF6 CoreAddons",  sonames: &["libKF6CoreAddons.so.6", "libKF6CoreAddons.so"] },
    LibSpec { group: "KDE",   name: "KF5 CoreAddons",  sonames: &["libKF5CoreAddons.so.5", "libKF5CoreAddons.so"] },
    LibSpec { group: "KDE",   name: "KF5 Config",      sonames: &["libKF5ConfigCore.so.5", "libKF5ConfigCore.so"] },
    LibSpec { group: "KDE",   name: "Plasma Framework", sonames: &["libKF5Plasma.so.5", "libKF5Plasma.so"] },
    LibSpec { group: "KDE",   name: "KWin (lib)",      sonames: &["libkwin.so.6", "libkwin.so.5"] },

    // ── LXDE / LXQT ───────────────────────────────────────────
    LibSpec { group: "LXDE",  name: "libfm (core)",    sonames: &["libfm.so.4", "libfm.so"] },
    LibSpec { group: "LXDE",  name: "libfm-gtk",       sonames: &["libfm-gtk.so.4", "libfm-gtk3.so.4", "libfm-gtk.so"] },
    LibSpec { group: "LXDE",  name: "liblxpanel",      sonames: &["liblxpanel.so.0", "liblxpanel.so"] },
    LibSpec { group: "LXDE",  name: "menu-cache",      sonames: &["libmenu-cache.so.3", "libmenu-cache.so"] },
    LibSpec { group: "LXDE",  name: "LXQt Core",       sonames: &["liblxqt.so.1", "liblxqt.so.0", "liblxqt.so"] },
    LibSpec { group: "LXDE",  name: "LXQt Utils",      sonames: &["liblxqtutils.so.0", "liblxqtutils.so"] },
    LibSpec { group: "LXDE",  name: "PCManFM-Qt (lib)", sonames: &["libfm-qt.so.9", "libfm-qt.so.8", "libfm-qt.so"] },
];

// ─── Função principal de carregamento ─────────────────────────────────────

pub fn load_all_libraries() -> Vec<LibResult> {
    println!("{}", "Carregando bibliotecas...".bold());
    println!("{}", "─".repeat(55).dimmed());

    let mut results = Vec::new();

    for spec in LIBRARY_CATALOG {
        let result = try_load(spec);
        print_single_result(&result);
        results.push(result);
    }

    println!("{}", "─".repeat(55).dimmed());
    results
}

/// Tenta cada soname na ordem e retorna o primeiro que funcionar
fn try_load(spec: &LibSpec) -> LibResult {
    let start = Instant::now();

    for &soname in spec.sonames {
        match unsafe { Library::new(soname) } {
            Ok(lib) => {
                let elapsed = start.elapsed();
                // Tenta ler versão simbólica (opcional)
                let version = try_read_version(&lib);
                // Lib deve ficar aberta enquanto usada; aqui fazemos drop imediato
                // porque o objetivo é apenas verificar disponibilidade.
                drop(lib);

                return LibResult {
                    group:     spec.group.to_string(),
                    name:      spec.name.to_string(),
                    soname:    soname.to_string(),
                    loaded:    true,
                    version,
                    load_time: elapsed,
                    error:     None,
                };
            }
            Err(_) => continue,
        }
    }

    LibResult {
        group:    spec.group.to_string(),
        name:     spec.name.to_string(),
        soname:   spec.sonames.last().unwrap_or(&"").to_string(),
        loaded:   false,
        version:  None,
        load_time: start.elapsed(),
        error:    Some("Biblioteca não encontrada no sistema".to_string()),
    }
}

/// Tenta obter versão de símbolos padrão (melhor esforço)
fn try_read_version(lib: &Library) -> Option<String> {
    // Muitas libs exportam `gtk_get_major_version`, `qt_version`, etc.
    // Aqui verificamos os mais comuns de forma segura.
    unsafe {
        if let Ok(f) = lib.get::<unsafe extern "C" fn() -> u32>(b"gtk_get_major_version\0") {
            let major = f();
            if let Ok(f2) = lib.get::<unsafe extern "C" fn() -> u32>(b"gtk_get_minor_version\0") {
                let minor = f2();
                return Some(format!("{}.{}", major, minor));
            }
        }
        if let Ok(f) = lib.get::<unsafe extern "C" fn() -> *const i8>(b"glib_check_version\0") {
            let _ = f; // apenas confirma existência
        }
    }
    None
}

fn print_single_result(r: &LibResult) {
    let status = if r.loaded {
        "✔".green().bold()
    } else {
        "✘".red().bold()
    };

    let group_col = format!("[{:<6}]", r.group).cyan();
    let name_col  = format!("{:<22}", r.name);
    let time_col  = format!("{:.2?}", r.load_time).dimmed();

    if r.loaded {
        let soname = r.soname.yellow();
        let version = r.version.as_deref()
            .map(|v| format!("  v{}", v).dimmed().to_string())
            .unwrap_or_default();
        println!("  {} {} {}  →  {}{}", status, group_col, name_col, soname, version);
        let _ = time_col; // mostramos no relatório, não aqui
    } else {
        println!("  {} {} {}  →  {}", status, group_col, name_col.dimmed(), "não disponível".red().dimmed());
    }
}
