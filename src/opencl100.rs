pub use crate::*;
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenCL100 {
    acos(ID) = 0,
    acosh(ID) = 1,
    acospi(ID) = 2,
    asin(ID) = 3,
    asinh(ID) = 4,
    asinpi(ID) = 5,
    atan(ID) = 6,
    atan2(ID, ID) = 7,
    atanh(ID) = 8,
    atanpi(ID) = 9,
    atan2pi(ID, ID) = 10,
    cbrt(ID) = 11,
    ceil(ID) = 12,
    copysign(ID, ID) = 13,
    cos(ID) = 14,
    cosh(ID) = 15,
    cospi(ID) = 16,
    erfc(ID) = 17,
    erf(ID) = 18,
    exp(ID) = 19,
    exp2(ID) = 20,
    exp10(ID) = 21,
    expm1(ID) = 22,
    fabs(ID) = 23,
    fdim(ID, ID) = 24,
    floor(ID) = 25,
    fma(ID, ID, ID) = 26,
    fmax(ID, ID) = 27,
    fmin(ID, ID) = 28,
    fmod(ID, ID) = 29,
    fract(ID, ID) = 30,
    frexp(ID, ID) = 31,
    hypot(ID, ID) = 32,
    ilogb(ID) = 33,
    ldexp(ID, ID) = 34,
    lgamma(ID) = 35,
    lgamma_r(ID, ID) = 36,
    log(ID) = 37,
    log2(ID) = 38,
    log10(ID) = 39,
    log1p(ID) = 40,
    logb(ID) = 41,
    mad(ID, ID, ID) = 42,
    maxmag(ID, ID) = 43,
    minmag(ID, ID) = 44,
    modf(ID, ID) = 45,
    nan(ID) = 46,
    nextafter(ID, ID) = 47,
    pow(ID, ID) = 48,
    pown(ID, ID) = 49,
    powr(ID, ID) = 50,
    remainder(ID, ID) = 51,
    remquo(ID, ID, ID) = 52,
    rint(ID) = 53,
    rootn(ID, ID) = 54,
    round(ID) = 55,
    rsqrt(ID) = 56,
    sin(ID) = 57,
    sincos(ID, ID) = 58,
    sinh(ID) = 59,
    sinpi(ID) = 60,
    sqrt(ID) = 61,
    tan(ID) = 62,
    tanh(ID) = 63,
    tanpi(ID) = 64,
    tgamma(ID) = 65,
    trunc(ID) = 66,
    half_cos(ID) = 67,
    half_divide(ID, ID) = 68,
    half_exp(ID) = 69,
    half_exp2(ID) = 70,
    half_exp10(ID) = 71,
    half_log(ID) = 72,
    half_log2(ID) = 73,
    half_log10(ID) = 74,
    half_powr(ID, ID) = 75,
    half_recip(ID) = 76,
    half_rsqrt(ID) = 77,
    half_sin(ID) = 78,
    half_sqrt(ID) = 79,
    half_tan(ID) = 80,
    native_cos(ID) = 81,
    native_divide(ID, ID) = 82,
    native_exp(ID) = 83,
    native_exp2(ID) = 84,
    native_exp10(ID) = 85,
    native_log(ID) = 86,
    native_log2(ID) = 87,
    native_log10(ID) = 88,
    native_powr(ID, ID) = 89,
    native_recip(ID) = 90,
    native_rsqrt(ID) = 91,
    native_sin(ID) = 92,
    native_sqrt(ID) = 93,
    native_tan(ID) = 94,
    s_abs(ID) = 141,
    s_abs_diff(ID, ID) = 142,
    s_add_sat(ID, ID) = 143,
    u_add_sat(ID, ID) = 144,
    s_hadd(ID, ID) = 145,
    u_hadd(ID, ID) = 146,
    s_rhadd(ID, ID) = 147,
    u_rhadd(ID, ID) = 148,
    s_clamp(ID, ID, ID) = 149,
    u_clamp(ID, ID, ID) = 150,
    clz(ID) = 151,
    ctz(ID) = 152,
    s_mad_hi(ID, ID, ID) = 153,
    u_mad_sat(ID, ID, ID) = 154,
    s_mad_sat(ID, ID, ID) = 155,
    s_max(ID, ID) = 156,
    u_max(ID, ID) = 157,
    s_min(ID, ID) = 158,
    u_min(ID, ID) = 159,
    s_mul_hi(ID, ID) = 160,
    rotate(ID, ID) = 161,
    s_sub_sat(ID, ID) = 162,
    u_sub_sat(ID, ID) = 163,
    u_upsample(ID, ID) = 164,
    s_upsample(ID, ID) = 165,
    popcount(ID) = 166,
    s_mad24(ID, ID, ID) = 167,
    u_mad24(ID, ID, ID) = 168,
    s_mul24(ID, ID) = 169,
    u_mul24(ID, ID) = 170,
    u_abs(ID) = 201,
    u_abs_diff(ID, ID) = 202,
    u_mul_hi(ID, ID) = 203,
    u_mad_hi(ID, ID, ID) = 204,
    fclamp(ID, ID, ID) = 95,
    degrees(ID) = 96,
    fmax_common(ID, ID) = 97,
    fmin_common(ID, ID) = 98,
    mix(ID, ID, ID) = 99,
    radians(ID) = 100,
    step(ID, ID) = 101,
    smoothstep(ID, ID, ID) = 102,
    sign(ID) = 103,
    cross(ID, ID) = 104,
    distance(ID, ID) = 105,
    length(ID) = 106,
    normalize(ID) = 107,
    fast_distance(ID, ID) = 108,
    fast_length(ID) = 109,
    fast_normalize(ID) = 110,
    bitselect(ID, ID, ID) = 186,
    select(ID, ID, ID) = 187,
    vloadn(ID, ID, u32) = 171,
    vstoren(ID, ID, ID) = 172,
    vload_half(ID, ID) = 173,
    vload_halfn(ID, ID, u32) = 174,
    vstore_half(ID, ID, ID) = 175,
    vstore_half_r(ID, ID, ID, FPRoundingMode) = 176,
    vstore_halfn(ID, ID, ID) = 177,
    vstore_halfn_r(ID, ID, ID, FPRoundingMode) = 178,
    vloada_halfn(ID, ID, u32) = 179,
    vstorea_halfn(ID, ID, ID) = 180,
    vstorea_halfn_r(ID, ID, ID, FPRoundingMode) = 181,
    shuffle(ID, ID) = 182,
    shuffle2(ID, ID, ID) = 183,
    printf(ID, Vec<ID>) = 184,
    prefetch(ID, ID) = 185,
}
impl OpenCL100 {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    pub fn read_word<Env: Environ + std::fmt::Debug>(
        chunk: &[u32],
        env: &mut Env,
        idx: &mut usize,
    ) -> Self {
        use OpenCL100::*;
        let mask = u16::MAX as usize;
        let len = (chunk[*idx] >> 16) as usize & mask;
        let opc = chunk[*idx] as usize & mask;
        let chunk = &chunk[..*idx + len];
        let mark = *idx;
        *idx += 1;
        let re = match opc {
            0 => {
                let x0 = Writer::read_word(chunk, env, idx);
                acos(x0)
            }
            1 => {
                let x0 = Writer::read_word(chunk, env, idx);
                acosh(x0)
            }
            2 => {
                let x0 = Writer::read_word(chunk, env, idx);
                acospi(x0)
            }
            3 => {
                let x0 = Writer::read_word(chunk, env, idx);
                asin(x0)
            }
            4 => {
                let x0 = Writer::read_word(chunk, env, idx);
                asinh(x0)
            }
            5 => {
                let x0 = Writer::read_word(chunk, env, idx);
                asinpi(x0)
            }
            6 => {
                let x0 = Writer::read_word(chunk, env, idx);
                atan(x0)
            }
            7 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                atan2(x0, x1)
            }
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                atanh(x0)
            }
            9 => {
                let x0 = Writer::read_word(chunk, env, idx);
                atanpi(x0)
            }
            10 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                atan2pi(x0, x1)
            }
            11 => {
                let x0 = Writer::read_word(chunk, env, idx);
                cbrt(x0)
            }
            12 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ceil(x0)
            }
            13 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                copysign(x0, x1)
            }
            14 => {
                let x0 = Writer::read_word(chunk, env, idx);
                cos(x0)
            }
            15 => {
                let x0 = Writer::read_word(chunk, env, idx);
                cosh(x0)
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                cospi(x0)
            }
            17 => {
                let x0 = Writer::read_word(chunk, env, idx);
                erfc(x0)
            }
            18 => {
                let x0 = Writer::read_word(chunk, env, idx);
                erf(x0)
            }
            19 => {
                let x0 = Writer::read_word(chunk, env, idx);
                exp(x0)
            }
            20 => {
                let x0 = Writer::read_word(chunk, env, idx);
                exp2(x0)
            }
            21 => {
                let x0 = Writer::read_word(chunk, env, idx);
                exp10(x0)
            }
            22 => {
                let x0 = Writer::read_word(chunk, env, idx);
                expm1(x0)
            }
            23 => {
                let x0 = Writer::read_word(chunk, env, idx);
                fabs(x0)
            }
            24 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fdim(x0, x1)
            }
            25 => {
                let x0 = Writer::read_word(chunk, env, idx);
                floor(x0)
            }
            26 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                fma(x0, x1, x2)
            }
            27 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fmax(x0, x1)
            }
            28 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fmin(x0, x1)
            }
            29 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fmod(x0, x1)
            }
            30 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fract(x0, x1)
            }
            31 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                frexp(x0, x1)
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                hypot(x0, x1)
            }
            33 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ilogb(x0)
            }
            34 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ldexp(x0, x1)
            }
            35 => {
                let x0 = Writer::read_word(chunk, env, idx);
                lgamma(x0)
            }
            36 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                lgamma_r(x0, x1)
            }
            37 => {
                let x0 = Writer::read_word(chunk, env, idx);
                log(x0)
            }
            38 => {
                let x0 = Writer::read_word(chunk, env, idx);
                log2(x0)
            }
            39 => {
                let x0 = Writer::read_word(chunk, env, idx);
                log10(x0)
            }
            40 => {
                let x0 = Writer::read_word(chunk, env, idx);
                log1p(x0)
            }
            41 => {
                let x0 = Writer::read_word(chunk, env, idx);
                logb(x0)
            }
            42 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                mad(x0, x1, x2)
            }
            43 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                maxmag(x0, x1)
            }
            44 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                minmag(x0, x1)
            }
            45 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                modf(x0, x1)
            }
            46 => {
                let x0 = Writer::read_word(chunk, env, idx);
                nan(x0)
            }
            47 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                nextafter(x0, x1)
            }
            48 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                pow(x0, x1)
            }
            49 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                pown(x0, x1)
            }
            50 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                powr(x0, x1)
            }
            51 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                remainder(x0, x1)
            }
            52 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                remquo(x0, x1, x2)
            }
            53 => {
                let x0 = Writer::read_word(chunk, env, idx);
                rint(x0)
            }
            54 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                rootn(x0, x1)
            }
            55 => {
                let x0 = Writer::read_word(chunk, env, idx);
                round(x0)
            }
            56 => {
                let x0 = Writer::read_word(chunk, env, idx);
                rsqrt(x0)
            }
            57 => {
                let x0 = Writer::read_word(chunk, env, idx);
                sin(x0)
            }
            58 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                sincos(x0, x1)
            }
            59 => {
                let x0 = Writer::read_word(chunk, env, idx);
                sinh(x0)
            }
            60 => {
                let x0 = Writer::read_word(chunk, env, idx);
                sinpi(x0)
            }
            61 => {
                let x0 = Writer::read_word(chunk, env, idx);
                sqrt(x0)
            }
            62 => {
                let x0 = Writer::read_word(chunk, env, idx);
                tan(x0)
            }
            63 => {
                let x0 = Writer::read_word(chunk, env, idx);
                tanh(x0)
            }
            64 => {
                let x0 = Writer::read_word(chunk, env, idx);
                tanpi(x0)
            }
            65 => {
                let x0 = Writer::read_word(chunk, env, idx);
                tgamma(x0)
            }
            66 => {
                let x0 = Writer::read_word(chunk, env, idx);
                trunc(x0)
            }
            67 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_cos(x0)
            }
            68 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                half_divide(x0, x1)
            }
            69 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_exp(x0)
            }
            70 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_exp2(x0)
            }
            71 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_exp10(x0)
            }
            72 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_log(x0)
            }
            73 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_log2(x0)
            }
            74 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_log10(x0)
            }
            75 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                half_powr(x0, x1)
            }
            76 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_recip(x0)
            }
            77 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_rsqrt(x0)
            }
            78 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_sin(x0)
            }
            79 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_sqrt(x0)
            }
            80 => {
                let x0 = Writer::read_word(chunk, env, idx);
                half_tan(x0)
            }
            81 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_cos(x0)
            }
            82 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                native_divide(x0, x1)
            }
            83 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_exp(x0)
            }
            84 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_exp2(x0)
            }
            85 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_exp10(x0)
            }
            86 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_log(x0)
            }
            87 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_log2(x0)
            }
            88 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_log10(x0)
            }
            89 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                native_powr(x0, x1)
            }
            90 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_recip(x0)
            }
            91 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_rsqrt(x0)
            }
            92 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_sin(x0)
            }
            93 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_sqrt(x0)
            }
            94 => {
                let x0 = Writer::read_word(chunk, env, idx);
                native_tan(x0)
            }
            141 => {
                let x0 = Writer::read_word(chunk, env, idx);
                s_abs(x0)
            }
            142 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_abs_diff(x0, x1)
            }
            143 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_add_sat(x0, x1)
            }
            144 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_add_sat(x0, x1)
            }
            145 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_hadd(x0, x1)
            }
            146 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_hadd(x0, x1)
            }
            147 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_rhadd(x0, x1)
            }
            148 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_rhadd(x0, x1)
            }
            149 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                s_clamp(x0, x1, x2)
            }
            150 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                u_clamp(x0, x1, x2)
            }
            151 => {
                let x0 = Writer::read_word(chunk, env, idx);
                clz(x0)
            }
            152 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ctz(x0)
            }
            153 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                s_mad_hi(x0, x1, x2)
            }
            154 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                u_mad_sat(x0, x1, x2)
            }
            155 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                s_mad_sat(x0, x1, x2)
            }
            156 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_max(x0, x1)
            }
            157 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_max(x0, x1)
            }
            158 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_min(x0, x1)
            }
            159 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_min(x0, x1)
            }
            160 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_mul_hi(x0, x1)
            }
            161 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                rotate(x0, x1)
            }
            162 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_sub_sat(x0, x1)
            }
            163 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_sub_sat(x0, x1)
            }
            164 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_upsample(x0, x1)
            }
            165 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_upsample(x0, x1)
            }
            166 => {
                let x0 = Writer::read_word(chunk, env, idx);
                popcount(x0)
            }
            167 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                s_mad24(x0, x1, x2)
            }
            168 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                u_mad24(x0, x1, x2)
            }
            169 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                s_mul24(x0, x1)
            }
            170 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_mul24(x0, x1)
            }
            201 => {
                let x0 = Writer::read_word(chunk, env, idx);
                u_abs(x0)
            }
            202 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_abs_diff(x0, x1)
            }
            203 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                u_mul_hi(x0, x1)
            }
            204 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                u_mad_hi(x0, x1, x2)
            }
            95 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                fclamp(x0, x1, x2)
            }
            96 => {
                let x0 = Writer::read_word(chunk, env, idx);
                degrees(x0)
            }
            97 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fmax_common(x0, x1)
            }
            98 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fmin_common(x0, x1)
            }
            99 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                mix(x0, x1, x2)
            }
            100 => {
                let x0 = Writer::read_word(chunk, env, idx);
                radians(x0)
            }
            101 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                step(x0, x1)
            }
            102 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                smoothstep(x0, x1, x2)
            }
            103 => {
                let x0 = Writer::read_word(chunk, env, idx);
                sign(x0)
            }
            104 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                cross(x0, x1)
            }
            105 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                distance(x0, x1)
            }
            106 => {
                let x0 = Writer::read_word(chunk, env, idx);
                length(x0)
            }
            107 => {
                let x0 = Writer::read_word(chunk, env, idx);
                normalize(x0)
            }
            108 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                fast_distance(x0, x1)
            }
            109 => {
                let x0 = Writer::read_word(chunk, env, idx);
                fast_length(x0)
            }
            110 => {
                let x0 = Writer::read_word(chunk, env, idx);
                fast_normalize(x0)
            }
            186 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                bitselect(x0, x1, x2)
            }
            187 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                select(x0, x1, x2)
            }
            171 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vloadn(x0, x1, x2)
            }
            172 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vstoren(x0, x1, x2)
            }
            173 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                vload_half(x0, x1)
            }
            174 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vload_halfn(x0, x1, x2)
            }
            175 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vstore_half(x0, x1, x2)
            }
            176 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                vstore_half_r(x0, x1, x2, x3)
            }
            177 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vstore_halfn(x0, x1, x2)
            }
            178 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                vstore_halfn_r(x0, x1, x2, x3)
            }
            179 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vloada_halfn(x0, x1, x2)
            }
            180 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                vstorea_halfn(x0, x1, x2)
            }
            181 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                vstorea_halfn_r(x0, x1, x2, x3)
            }
            182 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                shuffle(x0, x1)
            }
            183 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                shuffle2(x0, x1, x2)
            }
            184 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                printf(x0, x1)
            }
            185 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                prefetch(x0, x1)
            }
            wtf => panic!("{}", wtf),
        };
        assert_eq!(*idx - mark, len);
        re
    }
    pub fn write_word<Env: Environ>(&self, env: &Env, sink: &mut Vec<u32>) {
        use OpenCL100::*;
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            acos(x0) => {
                x0.write_word(env, sink);
            }
            acosh(x0) => {
                x0.write_word(env, sink);
            }
            acospi(x0) => {
                x0.write_word(env, sink);
            }
            asin(x0) => {
                x0.write_word(env, sink);
            }
            asinh(x0) => {
                x0.write_word(env, sink);
            }
            asinpi(x0) => {
                x0.write_word(env, sink);
            }
            atan(x0) => {
                x0.write_word(env, sink);
            }
            atan2(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            atanh(x0) => {
                x0.write_word(env, sink);
            }
            atanpi(x0) => {
                x0.write_word(env, sink);
            }
            atan2pi(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            cbrt(x0) => {
                x0.write_word(env, sink);
            }
            ceil(x0) => {
                x0.write_word(env, sink);
            }
            copysign(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            cos(x0) => {
                x0.write_word(env, sink);
            }
            cosh(x0) => {
                x0.write_word(env, sink);
            }
            cospi(x0) => {
                x0.write_word(env, sink);
            }
            erfc(x0) => {
                x0.write_word(env, sink);
            }
            erf(x0) => {
                x0.write_word(env, sink);
            }
            exp(x0) => {
                x0.write_word(env, sink);
            }
            exp2(x0) => {
                x0.write_word(env, sink);
            }
            exp10(x0) => {
                x0.write_word(env, sink);
            }
            expm1(x0) => {
                x0.write_word(env, sink);
            }
            fabs(x0) => {
                x0.write_word(env, sink);
            }
            fdim(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            floor(x0) => {
                x0.write_word(env, sink);
            }
            fma(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            fmax(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            fmin(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            fmod(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            fract(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            frexp(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            hypot(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ilogb(x0) => {
                x0.write_word(env, sink);
            }
            ldexp(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            lgamma(x0) => {
                x0.write_word(env, sink);
            }
            lgamma_r(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            log(x0) => {
                x0.write_word(env, sink);
            }
            log2(x0) => {
                x0.write_word(env, sink);
            }
            log10(x0) => {
                x0.write_word(env, sink);
            }
            log1p(x0) => {
                x0.write_word(env, sink);
            }
            logb(x0) => {
                x0.write_word(env, sink);
            }
            mad(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            maxmag(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            minmag(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            modf(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            nan(x0) => {
                x0.write_word(env, sink);
            }
            nextafter(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            pow(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            pown(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            powr(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            remainder(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            remquo(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            rint(x0) => {
                x0.write_word(env, sink);
            }
            rootn(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            round(x0) => {
                x0.write_word(env, sink);
            }
            rsqrt(x0) => {
                x0.write_word(env, sink);
            }
            sin(x0) => {
                x0.write_word(env, sink);
            }
            sincos(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            sinh(x0) => {
                x0.write_word(env, sink);
            }
            sinpi(x0) => {
                x0.write_word(env, sink);
            }
            sqrt(x0) => {
                x0.write_word(env, sink);
            }
            tan(x0) => {
                x0.write_word(env, sink);
            }
            tanh(x0) => {
                x0.write_word(env, sink);
            }
            tanpi(x0) => {
                x0.write_word(env, sink);
            }
            tgamma(x0) => {
                x0.write_word(env, sink);
            }
            trunc(x0) => {
                x0.write_word(env, sink);
            }
            half_cos(x0) => {
                x0.write_word(env, sink);
            }
            half_divide(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            half_exp(x0) => {
                x0.write_word(env, sink);
            }
            half_exp2(x0) => {
                x0.write_word(env, sink);
            }
            half_exp10(x0) => {
                x0.write_word(env, sink);
            }
            half_log(x0) => {
                x0.write_word(env, sink);
            }
            half_log2(x0) => {
                x0.write_word(env, sink);
            }
            half_log10(x0) => {
                x0.write_word(env, sink);
            }
            half_powr(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            half_recip(x0) => {
                x0.write_word(env, sink);
            }
            half_rsqrt(x0) => {
                x0.write_word(env, sink);
            }
            half_sin(x0) => {
                x0.write_word(env, sink);
            }
            half_sqrt(x0) => {
                x0.write_word(env, sink);
            }
            half_tan(x0) => {
                x0.write_word(env, sink);
            }
            native_cos(x0) => {
                x0.write_word(env, sink);
            }
            native_divide(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            native_exp(x0) => {
                x0.write_word(env, sink);
            }
            native_exp2(x0) => {
                x0.write_word(env, sink);
            }
            native_exp10(x0) => {
                x0.write_word(env, sink);
            }
            native_log(x0) => {
                x0.write_word(env, sink);
            }
            native_log2(x0) => {
                x0.write_word(env, sink);
            }
            native_log10(x0) => {
                x0.write_word(env, sink);
            }
            native_powr(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            native_recip(x0) => {
                x0.write_word(env, sink);
            }
            native_rsqrt(x0) => {
                x0.write_word(env, sink);
            }
            native_sin(x0) => {
                x0.write_word(env, sink);
            }
            native_sqrt(x0) => {
                x0.write_word(env, sink);
            }
            native_tan(x0) => {
                x0.write_word(env, sink);
            }
            s_abs(x0) => {
                x0.write_word(env, sink);
            }
            s_abs_diff(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_add_sat(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_add_sat(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_hadd(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_hadd(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_rhadd(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_rhadd(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_clamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            u_clamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            clz(x0) => {
                x0.write_word(env, sink);
            }
            ctz(x0) => {
                x0.write_word(env, sink);
            }
            s_mad_hi(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            u_mad_sat(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            s_mad_sat(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            s_max(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_max(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_min(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_min(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_mul_hi(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            rotate(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_sub_sat(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_sub_sat(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_upsample(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            s_upsample(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            popcount(x0) => {
                x0.write_word(env, sink);
            }
            s_mad24(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            u_mad24(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            s_mul24(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_mul24(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_abs(x0) => {
                x0.write_word(env, sink);
            }
            u_abs_diff(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_mul_hi(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            u_mad_hi(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            fclamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            degrees(x0) => {
                x0.write_word(env, sink);
            }
            fmax_common(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            fmin_common(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            mix(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            radians(x0) => {
                x0.write_word(env, sink);
            }
            step(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            smoothstep(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            sign(x0) => {
                x0.write_word(env, sink);
            }
            cross(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            distance(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            length(x0) => {
                x0.write_word(env, sink);
            }
            normalize(x0) => {
                x0.write_word(env, sink);
            }
            fast_distance(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            fast_length(x0) => {
                x0.write_word(env, sink);
            }
            fast_normalize(x0) => {
                x0.write_word(env, sink);
            }
            bitselect(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            select(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vloadn(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstoren(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vload_half(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            vload_halfn(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstore_half(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstore_half_r(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            vstore_halfn(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstore_halfn_r(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            vloada_halfn(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstorea_halfn(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            vstorea_halfn_r(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            shuffle(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            shuffle2(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            printf(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            prefetch(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
        }
        sink[mark] |= ((sink.len() - mark) as u32) << 16;
    }
}
