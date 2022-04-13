
kernel [1,1,1]
fn histogram(input_image: image2d) -> (hist: image)
{ 
  uint2 uv = [ get_global_id(0), get_global_id(1) ];
  for(uint i = 0; i < get_image_size(hist); ++i)
  {
    float4 rgb = read_image(input_image, uv);
    uint idx = 256 * (rgb.x * 0.2 + rgb.y * 0.7 + rgb.z * 0.1);
    write_image(hist, idx, float4(1.0 / 256.0));
  }
}