#version 450 core

// Inputs: position and color
layout(location = 0) in vec4 pos;
layout(location = 1) in vec4 vcolor;

// Outputs: color passed to fragment shader
layout(location = 0) out vec4 fcolor;

void main(void) {
    fcolor = vcolor;
    gl_Position = pos;
}
