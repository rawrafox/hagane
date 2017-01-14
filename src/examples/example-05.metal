#include <metal_stdlib>
#include <metal_matrix>

using namespace metal;

struct Light
{
    float3 direction;
    float3 ambient_color;
    float3 diffuse_color;
    float3 specular_color;
};

constant Light light = {
    .direction = { 0.13, 0.72, 0.68 },
    .ambient_color = { 0.05, 0.05, 0.05 },
    .diffuse_color = { 0.9, 0.9, 0.9 },
    .specular_color = { 1, 1, 1 }
};

struct Material
{
    float3 ambient_color;
    float3 diffuse_color;
    float3 specular_color;
    float specular_power;
};

constant Material material = {
    .ambient_color = { 0.9, 0.1, 0 },
    .diffuse_color = { 0.9, 0.1, 0 },
    .specular_color = { 1, 1, 1 },
    .specular_power = 100
};

struct VertexInput {
  float x;
  float y;
  float z;
  float u;
  float v;
  float w;
  float2 texture_coordinates;
};

struct VertexOutput {
    float4 position [[position]];
    float3 eye;
    float3 normal;
    float2 texture_coordinates;
};

struct Uniforms {
    float4x4 model_view_projection_matrix;
    float4x4 model_view_matrix;
    float3x3 normal_matrix;
};


vertex VertexOutput vertex_main(device VertexInput *vertices [[buffer(0)]], constant Uniforms &uniforms [[buffer(1)]], uint vid [[vertex_id]]) {
  VertexInput input = vertices[vid];

  VertexOutput output;

  output.position = uniforms.model_view_projection_matrix * float4(input.x, input.y, input.z, 1.0);
  output.eye =  -(uniforms.model_view_matrix * float4(input.x, input.y, input.z, 1.0)).xyz;
  output.normal = uniforms.normal_matrix * float3(input.u, input.v, input.w);
  output.texture_coordinates = input.texture_coordinates;

  return output;
}

fragment float4 fragment_main(VertexOutput input [[stage_in]], constant Uniforms &uniforms [[buffer(0)]], texture2d<float> texture [[texture(0)]], sampler texture_sampler [[sampler(0)]]) {
  float3 normal = normalize(input.normal);

  float3 ambient_term = light.ambient_color * material.ambient_color;

  float3 diffuse_color = texture.sample(texture_sampler, input.texture_coordinates).rgb;

  float diffuse_intensity = saturate(dot(normal, light.direction));
  float3 diffuse_term = light.diffuse_color * diffuse_color * diffuse_intensity;

  float3 specular_term(0);

  if (diffuse_intensity > 0) {
      float3 eye_direction = normalize(input.eye);
      float3 halfway = normalize(light.direction + eye_direction);

      float specular_factor = pow(saturate(dot(normal, halfway)), material.specular_power);
      specular_term = light.specular_color * material.specular_color * specular_factor;
  }

  return float4(ambient_term + diffuse_term + specular_term, 1);
}
