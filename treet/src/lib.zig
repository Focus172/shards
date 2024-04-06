const std = @import("std");

const donut = @import("donut.zig");

const ts = @cImport({
    @cInclude("tree_sitter/api.h");
});

// const c = @cImport({
//     // See https://github.com/ziglang/zig/issues/515
//     @cDefine("_NO_CRT_STDIO_INLINE", "1");
//     @cInclude("stdio.h");
//     // @cInclude("/home/focus/dev/shards/libs/libshards.h");
// });

pub export fn thing() i32 {
    donut.run() catch return 1;
    // _ = c.printf("Hello World\n");
    return 0;
}

// export fn parse() c.ShardsAst {
//     return c.shards_invalid_ast();
// }
