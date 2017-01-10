#include <metal_stdlib>

using namespace metal;

struct VertexInput {
  float4 position [[position]];
  float4 color;
};

struct VertexOutput {
  float4 position [[position]];
  float4 color;
};

struct Uniforms {
    float4x4 modelViewProjectionMatrix;
};


vertex VertexOutput vertex_project(device VertexInput *vertices [[buffer(0)]], constant Uniforms &uniforms [[buffer(1)]], uint vid [[vertex_id]]) {
  VertexInput input = vertices[vid];

  VertexOutput output;

  output.position = uniforms.modelViewProjectionMatrix * input.position;
  output.color = input.color;

  return output;
}

fragment half4 fragment_flatcolor(VertexOutput input [[stage_in]]) {
    return half4(input.color);
}
