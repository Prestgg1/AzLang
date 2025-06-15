const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var ededler = try std.ArrayList(usize).initCapacity(allocator, 4);
try ededler.appendSlice(&[_]usize{ 1, 2, 3, 5 });
    for (ededler.items ) |eded| {
std.debug.print("{}\n", .{eded});
}
    const a: usize = 10;
    if ((a == 10)) {
std.debug.print("{s}\n", .{"a 10-e beraber"});
} else {
if ((a > 10)) {
std.debug.print("{s}\n", .{"a 10-d\u{259}n boyuk"});
} else {
std.debug.print("{s}\n", .{"a 10-d\u{259}n kicik"});
}
}
    ededler.deinit();
}
