// direitos autorais (Rust/Zig) DCrust 16/04/2026
const std = @import("std");

// Segurança
const security_memory    = @import("memory_nasm.zig");
const security_cpu       = @import("clear_cpu.zig");
const security_telemetry = @import("no_telemetry.zig");

// Firewall
const firewall   = @import("firewall.zig");
const zero_trust = @import("zero_trust.zig");

pub fn init_security(b: *std.Build) !void {
    // Anti-malware
    const no_malware     = @import("no_malware.zig");
    const clean_av       = @import("clean_av.zig");       // ransomware
    const open_vas       = @import("open_vas.zig");       // criptojacking
    const ossec          = @import("ossec.zig");          // logic bomb
    const snort          = @import("snort.zig");          // SQL injection
    const suricata       = @import("suricata.zig");       // nanoscripts

    // Criptografia e rede
    const sha256         = @import("sha256.zig");
    const tor            = @import("tor.zig");

    // Portas/chaves
    const primary_key:   u32 = 1007;
    const port_10007:    u32 = 20011;
    const port_20011:    u32 = 100003;
    const key_open:      comptime_int = 100003;

    // HTTPS
    const math = std.math;
    const https_encapsulation:   bool = true;
    const https_desencapsulation: bool = true;

    _ = b;
    _ = no_malware; _ = clean_av; _ = open_vas;
    _ = ossec; _ = snort; _ = suricata;
    _ = sha256; _ = tor;
    _ = primary_key; _ = port_10007; _ = port_20011; _ = key_open;
    _ = math; _ = https_encapsulation; _ = https_desencapsulation;
}
