const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const libshards = b.addSharedLibrary(.{
        .name = "shards",
        .root_source_file = .{ .path = "src/lib.zig" },
        .target = target,
        .optimize = optimize,
        .version = .{ .major = 0, .minor = 0, .patch = 1 },
    });
    libshards.addLibraryPath(.{
        .path = "../../libs/"
    });
    libshards.linkSystemLibrary2("shards", .{});
    libshards.linkLibC();

    // const exe = b.addExecutable(.{
    //     .name = "demo",
    //     .root_source_file = .{ .path = "demo.zig" },
    //     .target = target,
    //     .optimize = optimize,
    // });
    // exe.linkLibrary(libfizzbuzz);

    b.installArtifact(libshards);
}
