const std = @import("std");

pub fn main() !void {
    // const allocator = std.heap.page_allocator;

    const input = @embedFile("input.txt");

    // std.debug.print("{s}", .{input});

    // const lines = std.ArrayList([]const u8).init();
    // defer lines.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");

    while (lines.next()) |line| {
        std.debug.print("{s}\n", .{line});
    }
}

// test "simple test" {
//     var list = std.ArrayList(i32).init(std.testing.allocator);
//     defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
//     try list.append(42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
