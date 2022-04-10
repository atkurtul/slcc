pub use crate::*;
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GLSL450 {
    Round(ID) = 1,
    RoundEven(ID) = 2,
    Trunc(ID) = 3,
    FAbs(ID) = 4,
    SAbs(ID) = 5,
    FSign(ID) = 6,
    SSign(ID) = 7,
    Floor(ID) = 8,
    Ceil(ID) = 9,
    Fract(ID) = 10,
    Radians(ID) = 11,
    Degrees(ID) = 12,
    Sin(ID) = 13,
    Cos(ID) = 14,
    Tan(ID) = 15,
    Asin(ID) = 16,
    Acos(ID) = 17,
    Atan(ID) = 18,
    Sinh(ID) = 19,
    Cosh(ID) = 20,
    Tanh(ID) = 21,
    Asinh(ID) = 22,
    Acosh(ID) = 23,
    Atanh(ID) = 24,
    Atan2(ID, ID) = 25,
    Pow(ID, ID) = 26,
    Exp(ID) = 27,
    Log(ID) = 28,
    Exp2(ID) = 29,
    Log2(ID) = 30,
    Sqrt(ID) = 31,
    InverseSqrt(ID) = 32,
    Determinant(ID) = 33,
    MatrixInverse(ID) = 34,
    Modf(ID, ID) = 35,
    ModfStruct(ID) = 36,
    FMin(ID, ID) = 37,
    UMin(ID, ID) = 38,
    SMin(ID, ID) = 39,
    FMax(ID, ID) = 40,
    UMax(ID, ID) = 41,
    SMax(ID, ID) = 42,
    FClamp(ID, ID, ID) = 43,
    UClamp(ID, ID, ID) = 44,
    SClamp(ID, ID, ID) = 45,
    FMix(ID, ID, ID) = 46,
    IMix(ID, ID, ID) = 47,
    Step(ID, ID) = 48,
    SmoothStep(ID, ID, ID) = 49,
    Fma(ID, ID, ID) = 50,
    Frexp(ID, ID) = 51,
    FrexpStruct(ID) = 52,
    Ldexp(ID, ID) = 53,
    PackSnorm4x8(ID) = 54,
    PackUnorm4x8(ID) = 55,
    PackSnorm2x16(ID) = 56,
    PackUnorm2x16(ID) = 57,
    PackHalf2x16(ID) = 58,
    PackDouble2x32(ID) = 59,
    UnpackSnorm2x16(ID) = 60,
    UnpackUnorm2x16(ID) = 61,
    UnpackHalf2x16(ID) = 62,
    UnpackSnorm4x8(ID) = 63,
    UnpackUnorm4x8(ID) = 64,
    UnpackDouble2x32(ID) = 65,
    Length(ID) = 66,
    Distance(ID, ID) = 67,
    Cross(ID, ID) = 68,
    Normalize(ID) = 69,
    FaceForward(ID, ID, ID) = 70,
    Reflect(ID, ID) = 71,
    Refract(ID, ID, ID) = 72,
    FindILsb(ID) = 73,
    FindSMsb(ID) = 74,
    FindUMsb(ID) = 75,
    InterpolateAtCentroid(ID) = 76,
    InterpolateAtSample(ID, ID) = 77,
    InterpolateAtOffset(ID, ID) = 78,
    NMin(ID, ID) = 79,
    NMax(ID, ID) = 80,
    NClamp(ID, ID, ID) = 81,
}
impl GLSL450 {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    pub fn read_word<Env: Environ + std::fmt::Debug>(
        chunk: &[u32],
        env: &mut Env,
        idx: &mut usize,
    ) -> Self {
        use GLSL450::*;
        let mask = u16::MAX as usize;
        let len = (chunk[*idx] >> 16) as usize & mask;
        let opc = chunk[*idx] as usize & mask;
        let chunk = &chunk[..*idx + len];
        let mark = *idx;
        *idx += 1;
        let re = match opc {
            1 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Round(x0)
            }
            2 => {
                let x0 = Writer::read_word(chunk, env, idx);
                RoundEven(x0)
            }
            3 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Trunc(x0)
            }
            4 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FAbs(x0)
            }
            5 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SAbs(x0)
            }
            6 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FSign(x0)
            }
            7 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SSign(x0)
            }
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Floor(x0)
            }
            9 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Ceil(x0)
            }
            10 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Fract(x0)
            }
            11 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Radians(x0)
            }
            12 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Degrees(x0)
            }
            13 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Sin(x0)
            }
            14 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Cos(x0)
            }
            15 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Tan(x0)
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Asin(x0)
            }
            17 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Acos(x0)
            }
            18 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Atan(x0)
            }
            19 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Sinh(x0)
            }
            20 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Cosh(x0)
            }
            21 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Tanh(x0)
            }
            22 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Asinh(x0)
            }
            23 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Acosh(x0)
            }
            24 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Atanh(x0)
            }
            25 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Atan2(x0, x1)
            }
            26 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Pow(x0, x1)
            }
            27 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Exp(x0)
            }
            28 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Log(x0)
            }
            29 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Exp2(x0)
            }
            30 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Log2(x0)
            }
            31 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Sqrt(x0)
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                InverseSqrt(x0)
            }
            33 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Determinant(x0)
            }
            34 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MatrixInverse(x0)
            }
            35 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Modf(x0, x1)
            }
            36 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ModfStruct(x0)
            }
            37 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FMin(x0, x1)
            }
            38 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UMin(x0, x1)
            }
            39 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SMin(x0, x1)
            }
            40 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FMax(x0, x1)
            }
            41 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UMax(x0, x1)
            }
            42 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SMax(x0, x1)
            }
            43 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                FClamp(x0, x1, x2)
            }
            44 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                UClamp(x0, x1, x2)
            }
            45 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                SClamp(x0, x1, x2)
            }
            46 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                FMix(x0, x1, x2)
            }
            47 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                IMix(x0, x1, x2)
            }
            48 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Step(x0, x1)
            }
            49 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                SmoothStep(x0, x1, x2)
            }
            50 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                Fma(x0, x1, x2)
            }
            51 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Frexp(x0, x1)
            }
            52 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FrexpStruct(x0)
            }
            53 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Ldexp(x0, x1)
            }
            54 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackSnorm4x8(x0)
            }
            55 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackUnorm4x8(x0)
            }
            56 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackSnorm2x16(x0)
            }
            57 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackUnorm2x16(x0)
            }
            58 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackHalf2x16(x0)
            }
            59 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PackDouble2x32(x0)
            }
            60 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackSnorm2x16(x0)
            }
            61 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackUnorm2x16(x0)
            }
            62 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackHalf2x16(x0)
            }
            63 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackSnorm4x8(x0)
            }
            64 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackUnorm4x8(x0)
            }
            65 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UnpackDouble2x32(x0)
            }
            66 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Length(x0)
            }
            67 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Distance(x0, x1)
            }
            68 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Cross(x0, x1)
            }
            69 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Normalize(x0)
            }
            70 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                FaceForward(x0, x1, x2)
            }
            71 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Reflect(x0, x1)
            }
            72 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                Refract(x0, x1, x2)
            }
            73 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FindILsb(x0)
            }
            74 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FindSMsb(x0)
            }
            75 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FindUMsb(x0)
            }
            76 => {
                let x0 = Writer::read_word(chunk, env, idx);
                InterpolateAtCentroid(x0)
            }
            77 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                InterpolateAtSample(x0, x1)
            }
            78 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                InterpolateAtOffset(x0, x1)
            }
            79 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                NMin(x0, x1)
            }
            80 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                NMax(x0, x1)
            }
            81 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                NClamp(x0, x1, x2)
            }
            wtf => panic!("{}", wtf),
        };
        assert_eq!(*idx - mark, len);
        re
    }
    pub fn write_word<Env: Environ>(&self, env: &Env, sink: &mut Vec<u32>) {
        use GLSL450::*;
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            Round(x0) => {
                x0.write_word(env, sink);
            }
            RoundEven(x0) => {
                x0.write_word(env, sink);
            }
            Trunc(x0) => {
                x0.write_word(env, sink);
            }
            FAbs(x0) => {
                x0.write_word(env, sink);
            }
            SAbs(x0) => {
                x0.write_word(env, sink);
            }
            FSign(x0) => {
                x0.write_word(env, sink);
            }
            SSign(x0) => {
                x0.write_word(env, sink);
            }
            Floor(x0) => {
                x0.write_word(env, sink);
            }
            Ceil(x0) => {
                x0.write_word(env, sink);
            }
            Fract(x0) => {
                x0.write_word(env, sink);
            }
            Radians(x0) => {
                x0.write_word(env, sink);
            }
            Degrees(x0) => {
                x0.write_word(env, sink);
            }
            Sin(x0) => {
                x0.write_word(env, sink);
            }
            Cos(x0) => {
                x0.write_word(env, sink);
            }
            Tan(x0) => {
                x0.write_word(env, sink);
            }
            Asin(x0) => {
                x0.write_word(env, sink);
            }
            Acos(x0) => {
                x0.write_word(env, sink);
            }
            Atan(x0) => {
                x0.write_word(env, sink);
            }
            Sinh(x0) => {
                x0.write_word(env, sink);
            }
            Cosh(x0) => {
                x0.write_word(env, sink);
            }
            Tanh(x0) => {
                x0.write_word(env, sink);
            }
            Asinh(x0) => {
                x0.write_word(env, sink);
            }
            Acosh(x0) => {
                x0.write_word(env, sink);
            }
            Atanh(x0) => {
                x0.write_word(env, sink);
            }
            Atan2(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Pow(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Exp(x0) => {
                x0.write_word(env, sink);
            }
            Log(x0) => {
                x0.write_word(env, sink);
            }
            Exp2(x0) => {
                x0.write_word(env, sink);
            }
            Log2(x0) => {
                x0.write_word(env, sink);
            }
            Sqrt(x0) => {
                x0.write_word(env, sink);
            }
            InverseSqrt(x0) => {
                x0.write_word(env, sink);
            }
            Determinant(x0) => {
                x0.write_word(env, sink);
            }
            MatrixInverse(x0) => {
                x0.write_word(env, sink);
            }
            Modf(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ModfStruct(x0) => {
                x0.write_word(env, sink);
            }
            FMin(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UMin(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SMin(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FMax(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UMax(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SMax(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FClamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            UClamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SClamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FMix(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IMix(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Step(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SmoothStep(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Fma(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Frexp(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FrexpStruct(x0) => {
                x0.write_word(env, sink);
            }
            Ldexp(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            PackSnorm4x8(x0) => {
                x0.write_word(env, sink);
            }
            PackUnorm4x8(x0) => {
                x0.write_word(env, sink);
            }
            PackSnorm2x16(x0) => {
                x0.write_word(env, sink);
            }
            PackUnorm2x16(x0) => {
                x0.write_word(env, sink);
            }
            PackHalf2x16(x0) => {
                x0.write_word(env, sink);
            }
            PackDouble2x32(x0) => {
                x0.write_word(env, sink);
            }
            UnpackSnorm2x16(x0) => {
                x0.write_word(env, sink);
            }
            UnpackUnorm2x16(x0) => {
                x0.write_word(env, sink);
            }
            UnpackHalf2x16(x0) => {
                x0.write_word(env, sink);
            }
            UnpackSnorm4x8(x0) => {
                x0.write_word(env, sink);
            }
            UnpackUnorm4x8(x0) => {
                x0.write_word(env, sink);
            }
            UnpackDouble2x32(x0) => {
                x0.write_word(env, sink);
            }
            Length(x0) => {
                x0.write_word(env, sink);
            }
            Distance(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Cross(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Normalize(x0) => {
                x0.write_word(env, sink);
            }
            FaceForward(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Reflect(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Refract(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FindILsb(x0) => {
                x0.write_word(env, sink);
            }
            FindSMsb(x0) => {
                x0.write_word(env, sink);
            }
            FindUMsb(x0) => {
                x0.write_word(env, sink);
            }
            InterpolateAtCentroid(x0) => {
                x0.write_word(env, sink);
            }
            InterpolateAtSample(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            InterpolateAtOffset(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            NMin(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            NMax(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            NClamp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
        }
        sink[mark] |= ((sink.len() - mark) as u32) << 16;
    }
}
