#version 460 core
#define PI 3.141592653

uniform float time;
uniform vec2 resolution;

in vec3 clrs;
in vec4 gl_FragCoord;

out vec4 fragColor;

// Signed Distance to a circle
float sdCircle(in vec2 o, in vec2 p, in float r) {
    return distance(o,p) - r;
}

// My moving circle
vec2 circle(in float t, in float r) {
    float x = r * 10.0 * sin((2.0*PI)+t); 
    float y = r * 4.0 * cos((2.0*PI*2.0)+t);
    return vec2(x,y);
}

// Shifting color palette
vec3 palette(in float t) {
    vec3 a = vec3(0.5,0.5,0.5);
    vec3 b = vec3(0.5,0.5,0.5);
    vec3 c = vec3(1.0,1.0,1.0);
    vec3 d = vec3(0.268,0.416,0.557);

    return a + b * cos((2.0*PI)*((c*t)+d));
}

void main() {
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = (gl_FragCoord.xy * 2.0 - resolution.xy) / resolution.y;
    vec2 uv0 = uv; // Save orginal origin

    // How Fast the Circle Moves
    float speed = 0.25;
    float t = time * speed;    

    // Final Output Color
    vec3 finalColor = vec3(0.0);

    float l = sdCircle(circle(t,0.25),uv0,0.5);
    l = 2.0 * sin(l*8. + time) + 2.5;
    int layers = int(round(l));

    for(int i = 0; i < layers; i++) {
        // Circle Center Location
        vec2 circleLoc = circle(t,0.25);
        
        // Repeat in a grid
        uv *= (resolution.x/resolution.y) / 2.0;
        float lf = float(l);
        uv = fract((uv * -1.753) - (lf * 0.1)) - 0.5;
        
        float d = sdCircle(circleLoc,uv,0.5);
        d *= exp(-sdCircle(circleLoc,uv0,0.1));
        // Initial Color
        vec3 col = palette(distance(circle(t,.25),uv0) + t);

        // Rings around the center, move towards center with time
        d = (0.5*(sin(d*2.*PI/4. + time+float(i)))+1.);

        // Vingette around the edges
        float v = length(uv);
        float fade = 1.0 - smoothstep(0.25,0.4,v);
        col = col * pow(d,1.2) * fade;
        finalColor += col / float(layers);
    }

    // Output to screen
    finalColor += clrs * 0.5;
    fragColor = vec4(finalColor, 0.33);
}
