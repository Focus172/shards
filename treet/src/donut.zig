//! Made using `zig translate-c` and a bit of editing to write the data more
//! efficently (and clean up the mess of the translation)

const std = @import("std");
const sin = std.math.sin;
const cos = std.math.cos;

const X = 80;
const Y = 28; // 22;
const size = X * Y;

const donut_inner_radius = 2.0;

pub fn run() !void {
    const stdout = std.io.getStdOut().writer();

    _ = try stdout.write("\x1b[2J");

    var A: f32 = 0.0;
    var B: f32 = 0.0;

    // render the most forward point for a given thing
    var view_stack = [_]f32{0.0} ** size;
    // the character to render
    var buffer = [_]u8{' '} ** size;

    while (true) {
        @memset(&view_stack, 0.0);
        @memset(&buffer, ' ');

        // iterate over the whole surface of the DONUT not screen
        var pitch: f32 = 0.0;
        while (pitch < std.math.tau) : (pitch += 0.07) {
            var yaw: f32 = 0.0;
            while (yaw < std.math.tau) : (yaw += 0.02) {
                const length_x = sin(yaw);
                const length_y = cos(yaw);

                const ring_outwrd_len = cos(pitch);
                const ring_upward_len = sin(pitch);

                const sin_a = sin(A);
                const cos_a = cos(A);

                const cos_b = cos(B);
                const sin_b = sin(B);

                const donut_outer_radius = ring_outwrd_len + donut_inner_radius;
                const z_axis = 1.0 / ((length_x * donut_outer_radius * sin_a) + (ring_upward_len * cos_a) + 5.0);
                const t = (length_x * donut_outer_radius * cos_a) - (ring_upward_len * sin_a);

                const N: i32 = @intFromFloat(8.0 * ((ring_upward_len * sin_a * cos_b) - (length_x * ring_outwrd_len * cos_a * cos_b) - (length_x * ring_outwrd_len * sin_a) - (ring_upward_len * cos_a) - (length_y * ring_outwrd_len * sin_b)));

                // convert point on donut to point on screen
                const x: u32 = @intFromFloat(40.0 + (30.0 * z_axis * ((length_y * donut_outer_radius * cos_b) - (t * sin_b))));
                const y: u32 = @intFromFloat(12.0 + (15.0 * z_axis * ((length_y * donut_outer_radius * sin_b) + (t * cos_b))));

                const index = (X * y) + x;

                if (y < Y and x < X and view_stack[index] < z_axis) {
                    view_stack[index] = z_axis;
                    buffer[index] = ".,-~:;=!*#$@"[(if (N > 0) @intCast(N) else 0)];
                }
            }
        }

        _ = try stdout.write("\x1b[H");

        var ind: usize = 0;
        while (ind < size) : (ind += X) {
            buffer[ind] = '\n';
        }

        try stdout.writeAll(&buffer);

        std.time.sleep(10_000_000);

        A += 0.04;
        B += 0.02;
    }
}
