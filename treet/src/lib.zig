const std = @import("std");

const ts = @cImport({
    @cInclude("tree_sitter/api.h");
});

const shards = @cImport({
    @cInclude("libshards.h");
});

const tsr = {};
pub const LangType = enum(u8) { rust, c };

pub export fn parse(str: *const u8, len: usize, lang: LangType) shards.ShardsAst {
    _ = lang;
    _ = len;
    _ = str;

    const parser: *ts.TSParser = ts.ts_parser_new() orelse return shards.shards_invalid_ast();
    defer ts.ts_parser_delete(parser);

    // switch (lang) {
    //     .rust => {
    //         const rust = tsr.tree_sitter_rust();
    //         ts.ts_parser_set_language(parser, rust);
    //         @panic("fuck");
    //     },
    // }
    //
    // const tree: *ts.TSTree = ts.ts_parser_parse_string(parser, null, str, len);
    // defer ts.ts_tree_delete(tree);

    return shards.shards_invalid_ast();
}
