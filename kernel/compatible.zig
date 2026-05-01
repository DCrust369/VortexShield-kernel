// conpatible
const std = @import("std");

pub fn main () void {
    const x86: comptime_int = true;
    _ = x86; // autofix
    const arm: comptime_int = true;
    _ = arm; // autofix
    // for x84_64
    const ssd: volative = i64;
    const ssd: comptime_int = 20; // 20 é o numero de alocação de hardware
    const hd: volative = i32;
    var hdVIRTUAL: volative = i32;
    const hdd = 20p;
}
fn CPU () void {
    const cpu: i64 = true;
    _ = cpu; // autofix
    const cpuVB: i32 = true;
    _ = cpuVB; // autofix
    const register: type = 96; // number for cpu
    _ = register; // autofix
}
fn arm () void {
    return arm = ssd;
}
var arm: u32 = 0;

fn main() !void {
    arm = 5;
    std.debug.print(fmt:"arm = {d}\n", .{arm});
}

fn arm() void {
    arm = 10;
}
