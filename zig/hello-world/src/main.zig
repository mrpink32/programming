const std = @import("std");

pub fn main() anyerror!void {
    std.log.info("All your codebase are belong to us.", .{});
    const a: u8 = 56;
    std.log.debug("test: {}", .{a});
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}
