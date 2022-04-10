kernel fn blur_box(input_image: image2d, size: int) -> (output_image: image2d)
{
    int2 image_size  = get_image_size(input_image);
    int width = image_size.x;
    int height = image_size.y;

    uint2 coord = [ get_global_id(0), get_global_id(1) ];
    
    uint4 sum  = 0;
    uint  num  = 0;

    int2 shift = [-size, -size];

    for (; shift.x <= size; ++shift.x) {
        for (; shift.y <= size; ++shift.y) {
            int2 cur = coord + shift;
            if ((0 <= cur.x) && (cur.x < width) && (0 <= cur.y) && (cur.y < height)) {
                ++num;
                sum += read_image(input_image, cur);
            }
        }
    }
    write_image(output_image, coord, (sum + num / 2) / num);
}
