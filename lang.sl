kernel fn blur_box(input_image: image2d, size: int) -> (output_image: image2d)
{

    uint4 sum  = 0;
    uint  num  = 0;

    int2 shift = [-size, -size];

    for (; shift.x <= size; ++shift.x) {
        ++num;
    }
    
}
