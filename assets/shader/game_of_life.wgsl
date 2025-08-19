// The shader reads the previous frame's state from the `input` texture, and writes the new state of
// each pixel to the `output` texture. The textures are flipped each step to progress the
// simulation.
// Two textures are needed for the game of life as each pixel of step N depends on the state of its
// neighbors at step N-1.


// ! render world 使用 group(0) 
@group(0) @binding(0) var input: texture_storage_2d<rgba32float, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba32float, write>;
@group(0) @binding(2) var<uniform> config: GameOfLifeUniforms;

struct GameOfLifeUniforms {
    alive_color: vec4<f32>,
}

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}


// vec4 has many properties
// 位置系：x, y, z, w
// 颜色系：r, g, b, a
// 纹理系：s, t, p, q
// -- 
// let v: vec4<f32> = vec4(1.0, 2.0, 3.0, 4.0);
// let a = v.x;        // 标量
// let c2 = v.xy;      // vec2(1,2)
// let c3 = v.yzx;     // vec3(2,3,1)
// let c4 = v.bgra;    // vec4(3,2,1,4)
// let rep = v.xxzz;   // vec4(1,1,3,3)
// let uv = v.st;      // vec2(1,2)
// -- 
// 也可用下标访问单个分量（u32 索引）：v[0u]、v[1u] 等。
@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let randomNumber = randomFloat((invocation_id.y << 16u) | invocation_id.x);
    let alive = randomNumber > 0.9;

    // Use alpha channel to keep track of cell's state
    let color = vec4(config.alive_color.rgb, f32(alive));

    textureStore(output, location, color);
}

fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32) -> i32 {
    let value: vec4<f32> = textureLoad(input, location + vec2<i32>(offset_x, offset_y));
    return i32(value.a);
}

fn count_alive(location: vec2<i32>) -> i32 {
    return is_alive(location, -1, -1) +
           is_alive(location, -1,  0) +
           is_alive(location, -1,  1) +
           is_alive(location,  0, -1) +
           is_alive(location,  0,  1) +
           is_alive(location,  1, -1) +
           is_alive(location,  1,  0) +
           is_alive(location,  1,  1);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let n_alive = count_alive(location);

    var alive: bool;
    // a cell is born if it has exactly 3 neighbors
    if (n_alive == 3) {
        alive = true;
    } 
    // keep alive if it has exactly 2 neighbors
    else if (n_alive == 2) {
        let currently_alive = is_alive(location, 0, 0);
        alive = bool(currently_alive);
    } 
    // otherwise, it dies
    else {
        alive = false;
    }
    let color = vec4(config.alive_color.rgb, f32(alive));

    textureStore(output, location, color);
}
