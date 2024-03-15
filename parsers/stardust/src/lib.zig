const std = @import("std");

const c = @cImport({
    // See https://github.com/ziglang/zig/issues/515
    @cDefine("_NO_CRT_STDIO_INLINE", "1");
    @cInclude("stdio.h");
    // @cInclude("/home/focus/dev/shards/libs/libshards.h");
});

export fn ahfbea() void {
    _ = c.printf("Hello World\n");
}

// export fn parse() c.ShardsAst {
//     return c.shards_invalid_ast();
// }
