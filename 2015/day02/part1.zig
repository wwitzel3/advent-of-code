const std = @import("std");
const data = @embedFile("./puzzle.input");

pub fn main() !void {
    var lines = std.mem.tokenize(u8, data, "\n");
    var total: i64 = 0;

    var totalRibbon: i64 = 0;

    while (lines.next()) |line| {
        var values = std.mem.tokenize(u8, line, "x");

        const l: i64 = try std.fmt.parseInt(i64, values.next().?, 10);
        const w: i64 = try std.fmt.parseInt(i64, values.next().?, 10);
        const h: i64 = try std.fmt.parseInt(i64, values.next().?, 10);

        var lengths = [3]i64{ l, w, h };
        std.sort.sort(i64, &lengths, {}, comptime std.sort.asc(i64));

        var ribbon: i64 = lengths[0] * 2 + lengths[1] * 2;

        const volume: i64 = l * w * h;
        totalRibbon = totalRibbon + ribbon + volume;

        const lA: i64 = l * w;
        const wA: i64 = w * h;
        const hA: i64 = h * l;

        var sides = [3]i64{ lA, wA, hA };
        std.sort.sort(i64, &sides, {}, comptime std.sort.asc(i64));

        const extra = sides[0];
        const area: i64 = 2 * lA + 2 * wA + 2 * hA;

        total = total + area + extra;
    }
    std.debug.print("total paper: {d}\n", .{total});
    std.debug.print("total ribbon: {d}\n", .{totalRibbon});
}
