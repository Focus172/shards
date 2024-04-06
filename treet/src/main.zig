const std = @import("std");
const mem = std.mem; // will be used to compare bytes

const treet = @import("treet");

// extern fn thing() i32;

pub fn main() !void {
    std.debug.print("Hello, World!\n", .{});

    const ret = treet.thing();
    std.debug.print("out:, {}\n", .{ret});
}

fn helper() f80 {
    return 0.3;
}

const hello_world_in_c =
    \\#include <stdio.h>
    \\
    \\int main(int argc, char **argv) {
    \\    printf("hello world\n");
    \\    return 0;
    \\}
;
