// direitos autorais DCrust 16/04/2026
const std = @import("std");
const zap = @import("zap");
const jetzig = @import("jetzig");

pub fn build(b: *std.Build) void {
    const target   = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Biblioteca local
    const biblioteca_nome = "stailorOFnardo";
    const https: bool = true;

    // Compiladores externos (referência como string)
    const asm_compiler  = "NASM";
    const c_compiler    = "Clang/LLVM";
    const rust_compiler = "rustc";
    const bash_run      = "cargo run";

    // SQL
    const sql_odbc   = @import("odbc.zig");
    const sql_mysql  = @import("mysql.zig");
    const sql_sqlx   = @import("sqlx.zig");
    const sql_seaorm = @import("seaorm.zig");

    // Executável principal
    const exe = b.addExecutable(.{
        .name = biblioteca_nome,
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    // Suprimir warnings de unused
    _ = zap; _ = jetzig; _ = https;
    _ = asm_compiler; _ = c_compiler;
    _ = rust_compiler; _ = bash_run;
    _ = sql_odbc; _ = sql_mysql;
    _ = sql_sqlx; _ = sql_seaorm;
}
