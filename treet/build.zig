const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const libtreet = b.addSharedLibrary(.{
        .name = "treet",
        .root_source_file = .{ .path = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
        .version = .{ .major = 0, .minor = 0, .patch = 1 },
    });

    libtreet.linkSystemLibrary2("tree-sitter", .{});
    // libtreet.linkLibC();

    b.installArtifact(libtreet);

    // const ttest = b.step("test", "Run unit tests");
    // const tests = b.addTest(.{
    //     .root_source_file = .{ .path = "test/main.zig" },
    //     .target = target,
    //     .optimize = optimize,
    // });
    // ttest.dependOn(&b.addRunArtifact(tests).step);

    const demo = b.option(bool, "demo", "build the demo") orelse false;
    if (demo) {
        const tdemo = b.addExecutable(.{
            .name = "tdemo",
            .root_source_file = .{ .path = "src/main.zig" },
            .target = target,
            .optimize = optimize,
        });

        tdemo.addModule("treet", b.addModule("treet", .{ .source_file = .{ .path = "src/lib.zig" } }));
        b.installArtifact(tdemo);
    }
}
