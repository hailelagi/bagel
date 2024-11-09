const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "main",
        .target = target,
        .optimize = optimize,
    });
    exe.addCSourceFile(.{
        .file = .{ .cwd_relative = "main.cc" },
        .flags = &.{"-std=c++17"},
    });
    exe.addCSourceFile(.{
        .file = .{ .cwd_relative = "free_list.cc" },
        .flags = &.{"-std=c++17"},
    });
    exe.linkLibCpp();
    b.installArtifact(exe);

    const free_list_lib = b.addStaticLibrary(.{
        .name = "free_list",
        .target = target,
        .optimize = optimize,
    });
    free_list_lib.addCSourceFile(.{
        .file = .{ .cwd_relative = "free_list.cc" },
        .flags = &.{"-std=c++17"},
    });
    free_list_lib.linkLibCpp();
    b.installArtifact(free_list_lib);

    // GoogleTest Setup
    const gtest = b.addStaticLibrary(.{
        .name = "gtest",
        .target = target,
        .optimize = optimize,
    });

    const gtest_include_flags = [_][]const u8{
        "-std=c++17",
        "-Igoogletest/googletest/include",
        "-Igoogletest/googletest",
        "-pthread",
    };

    // automate the git clone if the googletest folder is not present
    const fetch_gtest = b.addSystemCommand(&.{
        "sh",
        "-c",
        "if [ ! -d googletest ]; then git clone https://github.com/google/googletest.git; fi",
    });

    gtest.step.dependOn(&fetch_gtest.step);

    gtest.addCSourceFile(.{
        .file = .{ .cwd_relative = "googletest/googletest/src/gtest-all.cc" },
        .flags = &gtest_include_flags,
    });
    gtest.addCSourceFile(.{
        .file = .{ .cwd_relative = "googletest/googletest/src/gtest_main.cc" },
        .flags = &gtest_include_flags,
    });
    gtest.linkLibCpp();

    // Test Executable
    const free_list_test = b.addExecutable(.{
        .name = "free_list_test",
        .target = target,
        .optimize = optimize,
    });
    free_list_test.addCSourceFile(.{
        .file = .{ .cwd_relative = "free_list_test.cc" },
        .flags = &.{
            "-std=c++17",
            "-Igoogletest/googletest/include",
            "-pthread",
        },
    });

    free_list_test.linkLibrary(gtest);
    free_list_test.linkLibrary(free_list_lib);
    free_list_test.linkLibCpp();
    b.installArtifact(free_list_test);

    // fmt
    const format_step = b.step("fmt", "Format C++ code with clang-format");
    const cpp_files = &[_][]const u8{
        "main.cc",
        "free_list.cc",
        "free_list_test.cc",
    };
    for (cpp_files) |file| {
        const format_cmd = b.addSystemCommand(&.{ "clang-format", "-i", file });
        format_step.dependOn(&format_cmd.step);
    }

    // test
    const test_step = b.step("test", "Run GTest tests");
    const run_test_cmd = b.addRunArtifact(free_list_test);
    test_step.dependOn(&run_test_cmd.step);

    // run
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "run app");
    run_step.dependOn(&run_cmd.step);
}
