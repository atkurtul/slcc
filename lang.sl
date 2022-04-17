
kernel fn main(input_image: image2d, size: int) -> (
      output_image: image2d, 
      output_buffer1: [float2],
      output_buffer2: [float2],
      output_buffer3: [float2],
      output_buffer4: [float2]
    )
{ 
    int2 image_size  = input_image.size;

    uint3 coord = get_global_id();

    float4 sum  = 0;
    uint   num  = 0;

    int2 shift = 0;

    for (shift.x = -size; shift.x <= size; ++shift.x) {
        for (shift.y = -size; shift.y <= size; ++shift.y) {
            int2 cur = coord + shift;
            if ((0 <= cur.x) && (cur.x < width) && (0 <= cur.y) && (cur.y < height)) {
                ++num;
                sum += read_image(input_image, cur);
            }
        }
    }
    write_image(output_image, coord, sum  / num);
}