const std = @import("std");

const treet = @import("treet");
const stdout = std.io.getStdOut().writer();

const shards = @cImport({
    @cInclude("libshards.h");
});

const hello_world_in_c =
    \\#include <stdio.h>
    \\
    \\int main(int argc, char **argv) {
    \\    printf("hello world\n");
    \\    return 0;
    \\}
;

pub fn main() !void {
    const ast: shards.ShardsAst = treet.parse(&hello_world_in_c[0], hello_world_in_c.len, treet.LangType.c);
    if (!ast.is_valid) {
        _ = try stdout.write("bad ast opps");
        return;
    }
    var i: usize = 0;
    while (i < ast.tokens_count) : (i += 1) {
        const t: shards.ShardsToken = ast.tokens_pointer[i];

        switch (t.tag) {
            shards.Identifier => {
                const id: shards.struct_Identifier_Body = t.unnamed_0.identifier;
                _ = id;
            },
            shards.Operation => {
                const op: shards.struct_Operation_Body = t.unnamed_0.operation;
                _ = op;
            },
            else => unreachable,
        }
    }
}
