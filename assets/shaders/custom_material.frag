#version 450
layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 9) uniform Globals {
    float Time;
    float DeltaTime;
    uint FrameCount;
};

layout(set = 1, binding = 0) uniform CustomMaterial {
    vec4 Color;
};

vec3 oklab_to_linear_srgb(vec3 c){
    float L = c.x;
    float a = c.y;
    float b = c.z;

    float l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    float m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    float s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    float l = l_ * l_ * l_;
    float m = m_ * m_ * m_;
    float s = s_ * s_ * s_;

    return vec3(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
    );
}

void main() {
    float speed = 2.0;
    float t_1 = sin(Time * speed) * 0.5 + 0.5;
    float t_2 = cos(Time * speed);
    float distance_to_center = distance(v_Uv, vec2(0.5, 0.5))  * 1.4;
    vec3 red = vec3(0.627955, 0.224863, 0.125846);
    vec3 green = vec3(0.86644, -0.233887, 0.179498);
    vec3 blue = vec3(0.701674, 0.274566, -0.169156);
    vec3 white = vec3(1.0, 0.0, 0.0);
    vec3 mixed = mix(mix(red, blue, t_1),mix(green, white, t_2), distance_to_center);

    o_Target = vec4(oklab_to_linear_srgb(mixed), 1.0);
}