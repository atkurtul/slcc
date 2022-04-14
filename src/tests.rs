use crate::opcode::*;
#[test]
pub fn ___test___Opcode() {
    use std::mem::*;
    use Opcode::*;
    unsafe {
        let variant = OpNop;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpUndef(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpSourceContinued(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSource(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpSourceExtension(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpName(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpMemberName(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpString(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpLine(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpExtension(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpExtInstImport(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpExtInst(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpMemoryModel(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpEntryPoint(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpExecutionMode(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpCapability(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeVoid(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeBool(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypeInt(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 21);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeFloat(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypeVector(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypeMatrix(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpTypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeSampler(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 26);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeSampledImage(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypeArray(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeRuntimeArray(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeStruct(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeOpaque(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypePointer(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTypeFunction(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeEvent(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeDeviceEvent(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 35);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeReserveId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeQueue(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypePipe(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeForwardPointer(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpConstantTrue(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 41);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpConstantFalse(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 42);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConstant(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 43);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConstantComposite(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 44);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpConstantSampler(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 45);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpConstantNull(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 46);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSpecConstantTrue(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 48);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSpecConstantFalse(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 49);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSpecConstant(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 50);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSpecConstantComposite(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 51);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSpecConstantOp(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 52);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFunction(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 54);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpFunctionParameter(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 55);
    }
    unsafe {
        let variant = OpFunctionEnd;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 56);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFunctionCall(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 57);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpVariable(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 59);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageTexelPointer(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 60);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLoad(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 61);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpStore(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 62);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpCopyMemory(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 63);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpCopyMemorySized(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpAccessChain(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpInBoundsAccessChain(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 66);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpPtrAccessChain(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 67);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpArrayLength(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 68);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpGenericPtrMemSemantics(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 69);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpInBoundsPtrAccessChain(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 70);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpDecorate(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 71);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpMemberDecorate(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 72);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpDecorationGroup(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 73);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpGroupDecorate(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 74);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpGroupMemberDecorate(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 75);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpVectorExtractDynamic(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 77);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpVectorInsertDynamic(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 78);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpVectorShuffle(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 79);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCompositeConstruct(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 80);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpCompositeExtract(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 81);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpCompositeInsert(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 82);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCopyObject(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 83);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpTranspose(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 84);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSampledImage(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 86);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSampleImplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 87);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSampleExplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 88);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 89);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 90);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSampleProjImplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 91);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSampleProjExplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 92);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 93);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 94);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageFetch(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 95);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageGather(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 96);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageDrefGather(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 97);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageRead(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 98);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpImageWrite(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 99);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImage(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 100);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageQueryFormat(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 101);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageQueryOrder(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 102);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpImageQuerySizeLod(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 103);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageQuerySize(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 104);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpImageQueryLod(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 105);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageQueryLevels(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 106);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageQuerySamples(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 107);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertFToU(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 109);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertFToS(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 110);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertSToF(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 111);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToF(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 112);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpUConvert(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 113);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSConvert(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 114);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFConvert(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 115);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpQuantizeToF16(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 116);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertPtrToU(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 117);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSatConvertSToU(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 118);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSatConvertUToS(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 119);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToPtr(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 120);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpPtrCastToGeneric(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 121);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpGenericCastToPtr(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 122);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGenericCastToPtrExplicit(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 123);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpBitcast(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 124);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSNegate(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 126);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFNegate(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 127);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIAdd(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 128);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFAdd(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 129);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpISub(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 130);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFSub(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 131);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIMul(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 132);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFMul(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 133);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUDiv(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 134);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSDiv(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 135);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFDiv(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 136);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUMod(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 137);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSRem(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 138);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSMod(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 139);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFRem(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 140);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFMod(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 141);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpVectorTimesScalar(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 142);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpMatrixTimesScalar(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 143);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpVectorTimesMatrix(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 144);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpMatrixTimesVector(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 145);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpMatrixTimesMatrix(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 146);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpOuterProduct(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 147);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpDot(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 148);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIAddCarry(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 149);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpISubBorrow(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 150);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUMulExtended(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 151);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSMulExtended(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 152);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpAny(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 154);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpAll(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 155);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsNan(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 156);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsInf(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 157);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsFinite(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 158);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsNormal(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 159);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSignBitSet(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 160);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLessOrGreater(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 161);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpOrdered(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 162);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUnordered(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 163);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLogicalEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 164);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLogicalNotEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 165);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLogicalOr(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 166);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpLogicalAnd(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 167);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpLogicalNot(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 168);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSelect(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 169);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 170);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpINotEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 171);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUGreaterThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 172);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSGreaterThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 173);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUGreaterThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 174);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSGreaterThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 175);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpULessThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 176);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSLessThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 177);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpULessThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 178);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSLessThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 179);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 180);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 181);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdNotEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 182);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordNotEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 183);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdLessThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 184);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordLessThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 185);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdGreaterThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 186);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordGreaterThan(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 187);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdLessThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 188);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordLessThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 189);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFOrdGreaterThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 190);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFUnordGreaterThanEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 191);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpShiftRightLogical(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 194);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpShiftRightArithmetic(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 195);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpShiftLeftLogical(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 196);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpBitwiseOr(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 197);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpBitwiseXor(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 198);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpBitwiseAnd(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 199);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpNot(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 200);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpBitFieldInsert(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 201);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpBitFieldSExtract(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 202);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpBitFieldUExtract(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 203);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpBitReverse(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 204);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpBitCount(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 205);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdx(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 207);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdy(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 208);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFwidth(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 209);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdxFine(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 210);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdyFine(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 211);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFwidthFine(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 212);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdxCoarse(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 213);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpDPdyCoarse(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 214);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFwidthCoarse(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 215);
    }
    unsafe {
        let variant = OpEmitVertex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 218);
    }
    unsafe {
        let variant = OpEndPrimitive;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 219);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpEmitStreamVertex(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 220);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpEndStreamPrimitive(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 221);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpControlBarrier(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 224);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpMemoryBarrier(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 225);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpAtomicLoad(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 227);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpAtomicStore(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 228);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicExchange(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 229);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpAtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 230);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpAtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 231);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpAtomicIIncrement(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 232);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpAtomicIDecrement(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 233);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicIAdd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 234);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicISub(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 235);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicSMin(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 236);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicUMin(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 237);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicSMax(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 238);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicUMax(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 239);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicAnd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 240);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicOr(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 241);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicXor(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 242);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpPhi(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 245);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpLoopMerge(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 246);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSelectionMerge(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 247);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpLabel(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 248);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpBranch(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 249);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpBranchConditional(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 250);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSwitch(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 251);
    }
    unsafe {
        let variant = OpKill;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 252);
    }
    unsafe {
        let variant = OpReturn;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 253);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpReturnValue(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 254);
    }
    unsafe {
        let variant = OpUnreachable;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 255);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpLifetimeStart(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 256);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpLifetimeStop(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 257);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpGroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 259);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpGroupWaitEvents(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 260);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupAll(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 261);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupAny(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 262);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupBroadcast(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 263);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupIAdd(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 264);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFAdd(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 265);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFMin(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 266);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupUMin(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 267);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupSMin(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 268);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFMax(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 269);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupUMax(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 270);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupSMax(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 271);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpReadPipe(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 274);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpWritePipe(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 275);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 276);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 277);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpReserveReadPipePackets(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 278);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpReserveWritePipePackets(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 279);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpCommitReadPipe(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 280);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpCommitWritePipe(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 281);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsValidReserveId(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 282);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGetNumPipePackets(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 283);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGetMaxPipePackets(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 284);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpGroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 285);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpGroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 286);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupCommitReadPipe(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 287);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupCommitWritePipe(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 288);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpEnqueueMarker(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 291);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let x11 = <_>::default();
        let x12 = <_>::default();
        let variant = OpEnqueueKernel(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 292);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpGetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 293);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpGetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 294);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 295);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGetKernelPreferredWorkGroupSizeMultiple(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 296);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpRetainEvent(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 297);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpReleaseEvent(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 298);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpCreateUserEvent(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 299);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpIsValidEvent(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 300);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSetUserEventStatus(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 301);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCaptureEventProfilingInfo(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 302);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpGetDefaultQueue(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 303);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpBuildNDRange(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 304);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseSampleImplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 305);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseSampleExplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 306);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 307);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 308);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 309);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 310);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 311);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 312);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseFetch(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 313);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseGather(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 314);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpImageSparseDrefGather(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 315);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpImageSparseTexelsResident(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 316);
    }
    unsafe {
        let variant = OpNoLine;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 317);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpAtomicFlagTestAndSet(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 318);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpAtomicFlagClear(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 319);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpImageSparseRead(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 320);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSizeOf(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 321);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypePipeStorage(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 322);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpConstantPipeStorage(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 323);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCreatePipeFromPipeStorage(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 324);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpGetKernelLocalSizeForSubgroupCount(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 325);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 326);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeNamedBarrier(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 327);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpNamedBarrierInitialize(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 328);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpMemoryNamedBarrier(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 329);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpModuleProcessed(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 330);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpExecutionModeId(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 331);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpDecorateId(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 332);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpGroupNonUniformElect(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 333);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformAll(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 334);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformAny(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 335);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformAllEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 336);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformBroadcast(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 337);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformBroadcastFirst(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 338);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformBallot(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 339);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformInverseBallot(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 340);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 341);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformBallotBitCount(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 342);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformBallotFindLSB(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 343);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpGroupNonUniformBallotFindMSB(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 344);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformShuffle(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 345);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformShuffleXor(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 346);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformShuffleUp(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 347);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformShuffleDown(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 348);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformIAdd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 349);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformFAdd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 350);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformIMul(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 351);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformFMul(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 352);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformSMin(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 353);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformUMin(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 354);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformFMin(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 355);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformSMax(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 356);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformUMax(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 357);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformFMax(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 358);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 359);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 360);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 361);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 362);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 363);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpGroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 364);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 365);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupNonUniformQuadSwap(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 366);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCopyLogical(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 400);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpPtrEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 401);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpPtrNotEqual(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 402);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpPtrDiff(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 403);
    }
    unsafe {
        let variant = OpTerminateInvocation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4416);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupBallotKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4421);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupFirstInvocationKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4422);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAllKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4428);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAnyKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4429);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAllEqualKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4430);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupReadInvocationKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4432);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let variant = OpTraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4445);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpExecuteCallableKHR(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4446);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToAccelerationStructureKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4447);
    }
    unsafe {
        let variant = OpIgnoreIntersectionKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4448);
    }
    unsafe {
        let variant = OpTerminateRayKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4449);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSDot(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4450);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpUDot(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4451);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSUDot(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4452);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSDotAccSat(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4453);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpUDotAccSat(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4454);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSUDotAccSat(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4455);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeRayQueryKHR(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4472);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpRayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4473);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpRayQueryTerminateKHR(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4474);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpRayQueryGenerateIntersectionKHR(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4475);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpRayQueryConfirmIntersectionKHR(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4476);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryProceedKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4477);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionTypeKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4479);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupIAddNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5000);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFAddNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5001);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFMinNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5002);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupUMinNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5003);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupSMinNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5004);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFMaxNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5005);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupUMaxNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5006);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupSMaxNonUniformAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5007);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFragmentMaskFetchAMD(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5011);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpFragmentFetchAMD(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5012);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpReadClockKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5056);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5283);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpGroupNonUniformPartitionNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5296);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpWritePackedPrimitiveIndices4x8NV(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5299);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpReportIntersectionNV(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5334);
    }
    unsafe {
        let variant = OpIgnoreIntersectionNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5335);
    }
    unsafe {
        let variant = OpTerminateRayNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5336);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let variant = OpTraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5337);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let x11 = <_>::default();
        let variant = OpTraceMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5338);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let x11 = <_>::default();
        let variant = OpTraceRayMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5339);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAccelerationStructureNV(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5341);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpExecuteCallableNV(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5344);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpTypeCooperativeMatrixNV(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5358);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpCooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5359);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpCooperativeMatrixStoreNV(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5360);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpCooperativeMatrixMulAddNV(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5361);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCooperativeMatrixLengthNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5362);
    }
    unsafe {
        let variant = OpBeginInvocationInterlockEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5364);
    }
    unsafe {
        let variant = OpEndInvocationInterlockEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5365);
    }
    unsafe {
        let variant = OpDemoteToHelperInvocation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5380);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpIsHelperInvocationEXT(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5381);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToImageNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5391);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToSamplerNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5392);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertImageToUNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5393);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertSamplerToUNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5394);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertUToSampledImageNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5395);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConvertSampledImageToUNV(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5396);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpSamplerImageAddressingModeNV(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5397);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupShuffleINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5571);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupShuffleDownINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5572);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupShuffleUpINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5573);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupShuffleXorINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5574);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupBlockReadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5575);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupBlockWriteINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5576);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupImageBlockReadINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5577);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupImageBlockWriteINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5578);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5580);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5581);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpUCountLeadingZerosINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5585);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpUCountTrailingZerosINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5586);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpAbsISubINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5587);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpAbsUSubINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5588);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIAddSatINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5589);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUAddSatINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5590);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIAverageINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5591);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUAverageINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5592);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIAverageRoundedINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5593);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUAverageRoundedINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5594);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpISubSatINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5595);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUSubSatINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5596);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpIMul32x16INTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5597);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpUMul32x16INTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5598);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpConstantFunctionPointerINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5600);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpFunctionPointerCallINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5601);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpAsmTargetINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5609);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAsmINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5610);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpAsmCallINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5611);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicFMinEXT(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5614);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicFMaxEXT(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5615);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpAssumeTrueKHR(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5630);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpExpectKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5631);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpDecorateString(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5632);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpMemberDecorateString(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5633);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpVmeImageINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5699);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeVmeImageINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5700);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImePayloadINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5701);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcRefPayloadINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5702);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcSicPayloadINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5703);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcMcePayloadINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5704);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcMceResultINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5705);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImeResultINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5706);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImeResultSingleReferenceStreamoutINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5707);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImeResultDualReferenceStreamoutINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5708);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImeSingleReferenceStreaminINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5709);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcImeDualReferenceStreaminINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5710);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcRefResultINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5711);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeAvcSicResultINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5712);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5713);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5714);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5715);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5716);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5717);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5718);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5719);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5720);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5721);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5722);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5723);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5724);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5725);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5726);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5727);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5728);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5729);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant =
            OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5730);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant =
            OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5731);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5732);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5733);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5734);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5735);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5736);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5737);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5738);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5739);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5740);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5741);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5742);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5743);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5744);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5745);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant =
            OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5746);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5747);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5748);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5749);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5750);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5751);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5752);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5753);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5754);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5755);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5756);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5757);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5758);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant =
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5759);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant =
            OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5760);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5761);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant =
            OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5762);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant =
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5763);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant =
            OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5764);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5765);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5766);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5767);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5768);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5769);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant =
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5770);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant =
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5771);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant =
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5772);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
            x0, x1, x2, x3, x4,
        );
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5773);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant =
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5774);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
            x0, x1, x2, x3, x4,
        );
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5775);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5776);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5777);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5778);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5779);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5780);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpSubgroupAvcFmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5781);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpSubgroupAvcBmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5782);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5783);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5784);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5785);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5786);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5787);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5788);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant =
            OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5789);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5790);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicInitializeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5791);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpSubgroupAvcSicConfigureSkcINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5792);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpSubgroupAvcSicConfigureIpeLumaINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5793);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let x10 = <_>::default();
        let x11 = <_>::default();
        let x12 = <_>::default();
        let variant = OpSubgroupAvcSicConfigureIpeLumaChromaINTEL(
            x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
        );
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5794);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5795);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5796);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5797);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5798);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5799);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5800);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5801);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5802);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpSubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5803);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5804);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5805);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5806);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant =
            OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5807);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5808);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5809);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5810);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5811);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5812);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5813);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5814);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5815);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpSubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5816);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpVariableLengthArrayINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5818);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpSaveMemoryINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5819);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpRestoreMemoryINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5820);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpArbitraryFloatSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5840);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5841);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatCastFromIntINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5842);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let variant = OpArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5843);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatAddINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5846);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatSubINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5847);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatMulINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5848);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatDivINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5849);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5850);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5851);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5852);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5853);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5854);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5855);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5856);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5857);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatHypotINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5858);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5859);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5860);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5861);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5862);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5863);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5864);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5865);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5866);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5867);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5868);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5869);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5870);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5871);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5872);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5873);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5874);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5875);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5876);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5877);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let variant = OpArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5878);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatATan2INTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5879);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatPowINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5880);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let x9 = <_>::default();
        let variant = OpArbitraryFloatPowRINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5881);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5882);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpLoopControlINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5887);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpAliasDomainDeclINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5911);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpAliasScopeDeclINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5912);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpAliasScopeListDeclINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5913);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5923);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5924);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5925);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5926);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5927);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5928);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5929);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5930);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5931);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5932);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let x6 = <_>::default();
        let x7 = <_>::default();
        let x8 = <_>::default();
        let variant = OpFixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5933);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpPtrCastToCrossWorkgroupINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5934);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpCrossWorkgroupCastToPtrINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5938);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpReadPipeBlockingINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5946);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpWritePipeBlockingINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5947);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpFPGARegINTEL(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5949);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryGetRayTMinKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6016);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryGetRayFlagsKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6017);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionTKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6018);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionInstanceCustomIndexKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6019);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6020);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant =
            OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6021);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6022);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6023);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6024);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6025);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6026);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6027);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6028);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryGetWorldRayDirectionKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6029);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpRayQueryGetWorldRayOriginKHR(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6030);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6031);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let variant = OpRayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6032);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let x5 = <_>::default();
        let variant = OpAtomicFAddEXT(x0, x1, x2, x3, x4, x5);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6035);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = OpTypeBufferSurfaceINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6086);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpTypeStructContinuedINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6090);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpConstantCompositeContinuedINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6091);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OpSpecConstantCompositeContinuedINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6092);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpControlBarrierArriveINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6142);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = OpControlBarrierWaitINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6143);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupIMulKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6401);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupFMulKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6402);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupBitwiseAndKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6403);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupBitwiseOrKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6404);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupBitwiseXorKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6405);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupLogicalAndKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6406);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupLogicalOrKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6407);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let x3 = <_>::default();
        let x4 = <_>::default();
        let variant = OpGroupLogicalXorKHR(x0, x1, x2, x3, x4);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6408);
    }
}
#[test]
pub fn ___test___ImageOperands() {
    use std::mem::*;
    use ImageOperands::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Bias(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Lod(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = Grad(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = ConstOffset(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Offset(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = ConstOffsets(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Sample(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MinLod(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 128);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MakeTexelAvailable(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 256);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MakeTexelVisible(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 512);
    }
    unsafe {
        let variant = NonPrivateTexel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1024);
    }
    unsafe {
        let variant = VolatileTexel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2048);
    }
    unsafe {
        let variant = SignExtend;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4096);
    }
    unsafe {
        let variant = ZeroExtend;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8192);
    }
    unsafe {
        let variant = Nontemporal;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16384);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Offsets(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65536);
    }
}
#[test]
pub fn ___test___FPFastMathMode() {
    use std::mem::*;
    use FPFastMathMode::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = NotNaN;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = NotInf;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = NSZ;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = AllowRecip;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = Fast;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = AllowContractFastINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65536);
    }
    unsafe {
        let variant = AllowReassocINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 131072);
    }
}
#[test]
pub fn ___test___SelectionControl() {
    use std::mem::*;
    use SelectionControl::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Flatten;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = DontFlatten;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
}
#[test]
pub fn ___test___LoopControl() {
    use std::mem::*;
    use LoopControl::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Unroll;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = DontUnroll;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = DependencyInfinite;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = DependencyLength(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MinIterations(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxIterations(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = IterationMultiple(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = PeelCount(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 128);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = PartialCount(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 256);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = InitiationIntervalINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65536);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxConcurrencyINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 131072);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = DependencyArrayINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 262144);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = PipelineEnableINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 524288);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = LoopCoalesceINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1048576);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxInterleavingINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2097152);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SpeculatedIterationsINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4194304);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NoFusionINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8388608);
    }
}
#[test]
pub fn ___test___FunctionControl() {
    use std::mem::*;
    use FunctionControl::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Inline;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = DontInline;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Pure;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = Const;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = OptNoneINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65536);
    }
}
#[test]
pub fn ___test___MemorySemantics() {
    use std::mem::*;
    use MemorySemantics::*;
    unsafe {
        let variant = Relaxed;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Acquire;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Release;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = AcquireRelease;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = SequentiallyConsistent;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = UniformMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let variant = SubgroupMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 128);
    }
    unsafe {
        let variant = WorkgroupMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 256);
    }
    unsafe {
        let variant = CrossWorkgroupMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 512);
    }
    unsafe {
        let variant = AtomicCounterMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1024);
    }
    unsafe {
        let variant = ImageMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2048);
    }
    unsafe {
        let variant = OutputMemory;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4096);
    }
    unsafe {
        let variant = MakeAvailable;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8192);
    }
    unsafe {
        let variant = MakeVisible;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16384);
    }
    unsafe {
        let variant = Volatile;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32768);
    }
}
#[test]
pub fn ___test___MemoryAccess() {
    use std::mem::*;
    use MemoryAccess::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Volatile;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Aligned(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Nontemporal;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MakePointerAvailable(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MakePointerVisible(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = NonPrivatePointer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = AliasScopeINTELMask(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65536);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NoAliasINTELMask(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 131072);
    }
}
#[test]
pub fn ___test___KernelProfilingInfo() {
    use std::mem::*;
    use KernelProfilingInfo::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = CmdExecTime;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___RayFlags() {
    use std::mem::*;
    use RayFlags::*;
    unsafe {
        let variant = NoneKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = OpaqueKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = NoOpaqueKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = TerminateOnFirstHitKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = SkipClosestHitShaderKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = CullBackFacingTrianglesKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = CullFrontFacingTrianglesKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let variant = CullOpaqueKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let variant = CullNoOpaqueKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 128);
    }
    unsafe {
        let variant = SkipTrianglesKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 256);
    }
    unsafe {
        let variant = SkipAABBsKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 512);
    }
}
#[test]
pub fn ___test___FragmentShadingRate() {
    use std::mem::*;
    use FragmentShadingRate::*;
    unsafe {
        let variant = Vertical2Pixels;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Vertical4Pixels;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Horizontal2Pixels;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = Horizontal4Pixels;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
}
#[test]
pub fn ___test___SourceLanguage() {
    use std::mem::*;
    use SourceLanguage::*;
    unsafe {
        let variant = Unknown;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = ESSL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = GLSL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = OpenCL_C;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = OpenCL_CPP;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = HLSL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = CPP_for_OpenCL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = SYCL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
}
#[test]
pub fn ___test___ExecutionModel() {
    use std::mem::*;
    use ExecutionModel::*;
    unsafe {
        let variant = Vertex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = TessellationControl;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = TessellationEvaluation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Geometry;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Fragment;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = GLCompute;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = Kernel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = TaskNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5267);
    }
    unsafe {
        let variant = MeshNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5268);
    }
    unsafe {
        let variant = RayGenerationNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5313);
    }
    unsafe {
        let variant = IntersectionNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5314);
    }
    unsafe {
        let variant = AnyHitNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5315);
    }
    unsafe {
        let variant = ClosestHitNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5316);
    }
    unsafe {
        let variant = MissNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5317);
    }
    unsafe {
        let variant = CallableNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5318);
    }
}
#[test]
pub fn ___test___AddressingModel() {
    use std::mem::*;
    use AddressingModel::*;
    unsafe {
        let variant = Logical;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Physical32;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Physical64;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = PhysicalStorageBuffer64;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5348);
    }
}
#[test]
pub fn ___test___MemoryModel() {
    use std::mem::*;
    use MemoryModel::*;
    unsafe {
        let variant = Simple;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = GLSL450;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = OpenCL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Vulkan;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
}
#[test]
pub fn ___test___ExecutionMode() {
    use std::mem::*;
    use ExecutionMode::*;
    unsafe {
        let x0 = <_>::default();
        let variant = Invocations(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = SpacingEqual;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = SpacingFractionalEven;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = SpacingFractionalOdd;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = VertexOrderCw;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = VertexOrderCcw;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = PixelCenterInteger;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = OriginUpperLeft;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = OriginLowerLeft;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = EarlyFragmentTests;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = PointMode;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = Xfb;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = DepthReplacing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = DepthGreater;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = DepthLess;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = DepthUnchanged;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = LocalSize(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = LocalSizeHint(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = InputPoints;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let variant = InputLines;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let variant = InputLinesAdjacency;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 21);
    }
    unsafe {
        let variant = Triangles;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let variant = InputTrianglesAdjacency;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let variant = Quads;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let variant = Isolines;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OutputVertices(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 26);
    }
    unsafe {
        let variant = OutputPoints;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let variant = OutputLineStrip;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let variant = OutputTriangleStrip;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = VecTypeHint(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let variant = ContractionOff;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let variant = Initializer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let variant = Finalizer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SubgroupSize(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 35);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SubgroupsPerWorkgroup(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SubgroupsPerWorkgroupId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = LocalSizeId(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = LocalSizeHintId(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let variant = SubgroupUniformControlFlowKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4421);
    }
    unsafe {
        let variant = PostDepthCoverage;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4446);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = DenormPreserve(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4459);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = DenormFlushToZero(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4460);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SignedZeroInfNanPreserve(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4461);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = RoundingModeRTE(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4462);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = RoundingModeRTZ(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4463);
    }
    unsafe {
        let variant = StencilRefReplacingEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5027);
    }
    unsafe {
        let variant = OutputLinesNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5269);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = OutputPrimitivesNV(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5270);
    }
    unsafe {
        let variant = DerivativeGroupQuadsNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5289);
    }
    unsafe {
        let variant = DerivativeGroupLinearNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5290);
    }
    unsafe {
        let variant = OutputTrianglesNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5298);
    }
    unsafe {
        let variant = PixelInterlockOrderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5366);
    }
    unsafe {
        let variant = PixelInterlockUnorderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5367);
    }
    unsafe {
        let variant = SampleInterlockOrderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5368);
    }
    unsafe {
        let variant = SampleInterlockUnorderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5369);
    }
    unsafe {
        let variant = ShadingRateInterlockOrderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5370);
    }
    unsafe {
        let variant = ShadingRateInterlockUnorderedEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5371);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SharedLocalMemorySizeINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5618);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = RoundingModeRTPINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5620);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = RoundingModeRTNINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5621);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FloatingPointModeALTINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5622);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FloatingPointModeIEEEINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5623);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let x2 = <_>::default();
        let variant = MaxWorkgroupSizeINTEL(x0, x1, x2);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5893);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxWorkDimINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5894);
    }
    unsafe {
        let variant = NoGlobalOffsetINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5895);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NumSIMDWorkitemsINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5896);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SchedulerTargetFmaxMhzINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5903);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NamedBarrierCountINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6417);
    }
}
#[test]
pub fn ___test___StorageClass() {
    use std::mem::*;
    use StorageClass::*;
    unsafe {
        let variant = UniformConstant;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Input;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Uniform;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Output;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Workgroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = CrossWorkgroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = Private;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = Function;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = Generic;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = PushConstant;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = AtomicCounter;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = Image;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = StorageBuffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = CallableDataNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5328);
    }
    unsafe {
        let variant = IncomingCallableDataNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5329);
    }
    unsafe {
        let variant = RayPayloadNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5338);
    }
    unsafe {
        let variant = HitAttributeNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5339);
    }
    unsafe {
        let variant = IncomingRayPayloadNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5342);
    }
    unsafe {
        let variant = ShaderRecordBufferNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5343);
    }
    unsafe {
        let variant = PhysicalStorageBuffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5349);
    }
    unsafe {
        let variant = CodeSectionINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5605);
    }
    unsafe {
        let variant = DeviceOnlyINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5936);
    }
    unsafe {
        let variant = HostOnlyINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5937);
    }
}
#[test]
pub fn ___test___Dim() {
    use std::mem::*;
    use Dim::*;
    unsafe {
        let variant = _1D;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = _2D;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = _3D;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Cube;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Rect;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = Buffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = SubpassData;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
}
#[test]
pub fn ___test___SamplerAddressingMode() {
    use std::mem::*;
    use SamplerAddressingMode::*;
    unsafe {
        let variant = None;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = ClampToEdge;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Clamp;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Repeat;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = RepeatMirrored;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
}
#[test]
pub fn ___test___SamplerFilterMode() {
    use std::mem::*;
    use SamplerFilterMode::*;
    unsafe {
        let variant = Nearest;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Linear;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___ImageFormat() {
    use std::mem::*;
    use ImageFormat::*;
    unsafe {
        let variant = Unknown;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Rgba32f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Rgba16f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = R32f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Rgba8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = Rgba8Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = Rg32f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = Rg16f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = R11fG11fB10f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = R16f;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = Rgba16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = Rgb10A2;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = Rg16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = Rg8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = R16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = R8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = Rgba16Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = Rg16Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let variant = Rg8Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = R16Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let variant = R8Snorm;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let variant = Rgba32i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 21);
    }
    unsafe {
        let variant = Rgba16i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let variant = Rgba8i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let variant = R32i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let variant = Rg32i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let variant = Rg16i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 26);
    }
    unsafe {
        let variant = Rg8i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let variant = R16i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let variant = R8i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let variant = Rgba32ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let variant = Rgba16ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let variant = Rgba8ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let variant = R32ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let variant = Rgb10a2ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let variant = Rg32ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 35);
    }
    unsafe {
        let variant = Rg16ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let variant = Rg8ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let variant = R16ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let variant = R8ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let variant = R64ui;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 40);
    }
    unsafe {
        let variant = R64i;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 41);
    }
}
#[test]
pub fn ___test___ImageChannelOrder() {
    use std::mem::*;
    use ImageChannelOrder::*;
    unsafe {
        let variant = R;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = A;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = RG;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = RA;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = RGB;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = RGBA;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = BGRA;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = ARGB;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = Intensity;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = Luminance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = Rx;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = RGx;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = RGBx;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = Depth;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = DepthStencil;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = sRGB;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = sRGBx;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = sRGBA;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let variant = sBGRA;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = ABGR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
}
#[test]
pub fn ___test___ImageChannelDataType() {
    use std::mem::*;
    use ImageChannelDataType::*;
    unsafe {
        let variant = SnormInt8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = SnormInt16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = UnormInt8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = UnormInt16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = UnormShort565;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = UnormShort555;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = UnormInt101010;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = SignedInt8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = SignedInt16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = SignedInt32;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = UnsignedInt8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = UnsignedInt16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = UnsignedInt32;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = HalfFloat;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = Float;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = UnormInt24;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = UnormInt101010_2;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
}
#[test]
pub fn ___test___FPRoundingMode() {
    use std::mem::*;
    use FPRoundingMode::*;
    unsafe {
        let variant = RTE;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = RTZ;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = RTP;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = RTN;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
}
#[test]
pub fn ___test___FPDenormMode() {
    use std::mem::*;
    use FPDenormMode::*;
    unsafe {
        let variant = Preserve;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = FlushToZero;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___QuantizationModes() {
    use std::mem::*;
    use QuantizationModes::*;
    unsafe {
        let variant = TRN;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = TRN_ZERO;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = RND;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = RND_ZERO;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = RND_INF;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = RND_MIN_INF;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = RND_CONV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = RND_CONV_ODD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
}
#[test]
pub fn ___test___FPOperationMode() {
    use std::mem::*;
    use FPOperationMode::*;
    unsafe {
        let variant = IEEE;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = ALT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___OverflowModes() {
    use std::mem::*;
    use OverflowModes::*;
    unsafe {
        let variant = WRAP;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = SAT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = SAT_ZERO;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = SAT_SYM;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
}
#[test]
pub fn ___test___LinkageType() {
    use std::mem::*;
    use LinkageType::*;
    unsafe {
        let variant = Export;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Import;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = LinkOnceODR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
}
#[test]
pub fn ___test___AccessQualifier() {
    use std::mem::*;
    use AccessQualifier::*;
    unsafe {
        let variant = ReadOnly;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = WriteOnly;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = ReadWrite;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
}
#[test]
pub fn ___test___FunctionParameterAttribute() {
    use std::mem::*;
    use FunctionParameterAttribute::*;
    unsafe {
        let variant = Zext;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Sext;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = ByVal;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Sret;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = NoAlias;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = NoCapture;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = NoWrite;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = NoReadWrite;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
}
#[test]
pub fn ___test___Decoration() {
    use std::mem::*;
    use Decoration::*;
    unsafe {
        let variant = RelaxedPrecision;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SpecId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Block;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = BufferBlock;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = RowMajor;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = ColMajor;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = ArrayStride(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MatrixStride(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = GLSLShared;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = GLSLPacked;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = CPacked;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = BuiltIn(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = NoPerspective;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = Flat;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = Patch;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = Centroid;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = Sample;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let variant = Invariant;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = Restrict;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let variant = Aliased;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let variant = Volatile;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 21);
    }
    unsafe {
        let variant = Constant;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let variant = Coherent;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let variant = NonWritable;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let variant = NonReadable;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let variant = Uniform;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 26);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = UniformId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let variant = SaturatedConversion;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Stream(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Location(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Component(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Index(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Binding(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = DescriptorSet(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Offset(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 35);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = XfbBuffer(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = XfbStride(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FuncParamAttr(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FPRoundingMode(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FPFastMathMode(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 40);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = LinkageAttributes(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 41);
    }
    unsafe {
        let variant = NoContraction;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 42);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = InputAttachmentIndex(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 43);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = Alignment(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 44);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxByteOffset(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 45);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = AlignmentId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 46);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxByteOffsetId(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 47);
    }
    unsafe {
        let variant = NoSignedWrap;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4469);
    }
    unsafe {
        let variant = NoUnsignedWrap;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4470);
    }
    unsafe {
        let variant = ExplicitInterpAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4999);
    }
    unsafe {
        let variant = OverrideCoverageNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5248);
    }
    unsafe {
        let variant = PassthroughNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5250);
    }
    unsafe {
        let variant = ViewportRelativeNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5252);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SecondaryViewportRelativeNV(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5256);
    }
    unsafe {
        let variant = PerPrimitiveNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5271);
    }
    unsafe {
        let variant = PerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5272);
    }
    unsafe {
        let variant = PerTaskNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5273);
    }
    unsafe {
        let variant = PerVertexKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5285);
    }
    unsafe {
        let variant = NonUniform;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5300);
    }
    unsafe {
        let variant = RestrictPointer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5355);
    }
    unsafe {
        let variant = AliasedPointer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5356);
    }
    unsafe {
        let variant = BindlessSamplerNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5398);
    }
    unsafe {
        let variant = BindlessImageNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5399);
    }
    unsafe {
        let variant = BoundSamplerNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5400);
    }
    unsafe {
        let variant = BoundImageNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5401);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = SIMTCallINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5599);
    }
    unsafe {
        let variant = ReferencedIndirectlyINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5602);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = ClobberINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5607);
    }
    unsafe {
        let variant = SideEffectsINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5608);
    }
    unsafe {
        let variant = VectorComputeVariableINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5624);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = FuncParamIOKindINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5625);
    }
    unsafe {
        let variant = VectorComputeFunctionINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5626);
    }
    unsafe {
        let variant = StackCallINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5627);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = GlobalVariableOffsetINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5628);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = CounterBuffer(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5634);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = UserSemantic(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5635);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = UserTypeGOOGLE(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5636);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = FunctionRoundingModeINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5822);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = FunctionDenormModeINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5823);
    }
    unsafe {
        let variant = RegisterINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5825);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MemoryINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5826);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NumbanksINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5827);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = BankwidthINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5828);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxPrivateCopiesINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5829);
    }
    unsafe {
        let variant = SinglepumpINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5830);
    }
    unsafe {
        let variant = DoublepumpINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5831);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = MaxReplicatesINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5832);
    }
    unsafe {
        let variant = SimpleDualPortINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5833);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = MergeINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5834);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = BankBitsINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5835);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = ForcePow2DepthINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5836);
    }
    unsafe {
        let variant = BurstCoalesceINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5899);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = CacheSizeINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5900);
    }
    unsafe {
        let variant = DontStaticallyCoalesceINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5901);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = PrefetchINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5902);
    }
    unsafe {
        let variant = StallEnableINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5905);
    }
    unsafe {
        let variant = FuseLoopsInFunctionINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5907);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = AliasScopeINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5914);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = NoAliasINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5915);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = BufferLocationINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5921);
    }
    unsafe {
        let x0 = <_>::default();
        let variant = IOPipeStorageINTEL(x0);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5944);
    }
    unsafe {
        let x0 = <_>::default();
        let x1 = <_>::default();
        let variant = FunctionFloatingPointModeINTEL(x0, x1);
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6080);
    }
    unsafe {
        let variant = SingleElementVectorINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6085);
    }
    unsafe {
        let variant = VectorComputeCallableFunctionINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6087);
    }
    unsafe {
        let variant = MediaBlockIOINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6140);
    }
}
#[test]
pub fn ___test___BuiltIn() {
    use std::mem::*;
    use BuiltIn::*;
    unsafe {
        let variant = Position;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = PointSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = ClipDistance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = CullDistance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = VertexId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = InstanceId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = PrimitiveId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = InvocationId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = Layer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = ViewportIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = TessLevelOuter;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = TessLevelInner;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = TessCoord;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = PatchVertices;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = FragCoord;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = PointCoord;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 16);
    }
    unsafe {
        let variant = FrontFacing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let variant = SampleId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = SamplePosition;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let variant = SampleMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let variant = FragDepth;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let variant = HelperInvocation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let variant = NumWorkgroups;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let variant = WorkgroupSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let variant = WorkgroupId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 26);
    }
    unsafe {
        let variant = LocalInvocationId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let variant = GlobalInvocationId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let variant = LocalInvocationIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let variant = WorkDim;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let variant = GlobalSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let variant = EnqueuedWorkgroupSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let variant = GlobalOffset;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let variant = GlobalLinearId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let variant = SubgroupSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let variant = SubgroupMaxSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let variant = NumSubgroups;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let variant = NumEnqueuedSubgroups;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let variant = SubgroupId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 40);
    }
    unsafe {
        let variant = SubgroupLocalInvocationId;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 41);
    }
    unsafe {
        let variant = VertexIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 42);
    }
    unsafe {
        let variant = InstanceIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 43);
    }
    unsafe {
        let variant = SubgroupEqMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4416);
    }
    unsafe {
        let variant = SubgroupGeMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4417);
    }
    unsafe {
        let variant = SubgroupGtMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4418);
    }
    unsafe {
        let variant = SubgroupLeMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4419);
    }
    unsafe {
        let variant = SubgroupLtMask;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4420);
    }
    unsafe {
        let variant = BaseVertex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4424);
    }
    unsafe {
        let variant = BaseInstance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4425);
    }
    unsafe {
        let variant = DrawIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4426);
    }
    unsafe {
        let variant = PrimitiveShadingRateKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4432);
    }
    unsafe {
        let variant = DeviceIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4438);
    }
    unsafe {
        let variant = ViewIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4440);
    }
    unsafe {
        let variant = ShadingRateKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4444);
    }
    unsafe {
        let variant = BaryCoordNoPerspAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4992);
    }
    unsafe {
        let variant = BaryCoordNoPerspCentroidAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4993);
    }
    unsafe {
        let variant = BaryCoordNoPerspSampleAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4994);
    }
    unsafe {
        let variant = BaryCoordSmoothAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4995);
    }
    unsafe {
        let variant = BaryCoordSmoothCentroidAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4996);
    }
    unsafe {
        let variant = BaryCoordSmoothSampleAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4997);
    }
    unsafe {
        let variant = BaryCoordPullModelAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4998);
    }
    unsafe {
        let variant = FragStencilRefEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5014);
    }
    unsafe {
        let variant = ViewportMaskNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5253);
    }
    unsafe {
        let variant = SecondaryPositionNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5257);
    }
    unsafe {
        let variant = SecondaryViewportMaskNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5258);
    }
    unsafe {
        let variant = PositionPerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5261);
    }
    unsafe {
        let variant = ViewportMaskPerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5262);
    }
    unsafe {
        let variant = FullyCoveredEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5264);
    }
    unsafe {
        let variant = TaskCountNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5274);
    }
    unsafe {
        let variant = PrimitiveCountNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5275);
    }
    unsafe {
        let variant = PrimitiveIndicesNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5276);
    }
    unsafe {
        let variant = ClipDistancePerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5277);
    }
    unsafe {
        let variant = CullDistancePerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5278);
    }
    unsafe {
        let variant = LayerPerViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5279);
    }
    unsafe {
        let variant = MeshViewCountNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5280);
    }
    unsafe {
        let variant = MeshViewIndicesNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5281);
    }
    unsafe {
        let variant = BaryCoordKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5286);
    }
    unsafe {
        let variant = BaryCoordNoPerspKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5287);
    }
    unsafe {
        let variant = FragSizeEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5292);
    }
    unsafe {
        let variant = FragInvocationCountEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5293);
    }
    unsafe {
        let variant = LaunchIdNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5319);
    }
    unsafe {
        let variant = LaunchSizeNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5320);
    }
    unsafe {
        let variant = WorldRayOriginNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5321);
    }
    unsafe {
        let variant = WorldRayDirectionNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5322);
    }
    unsafe {
        let variant = ObjectRayOriginNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5323);
    }
    unsafe {
        let variant = ObjectRayDirectionNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5324);
    }
    unsafe {
        let variant = RayTminNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5325);
    }
    unsafe {
        let variant = RayTmaxNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5326);
    }
    unsafe {
        let variant = InstanceCustomIndexNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5327);
    }
    unsafe {
        let variant = ObjectToWorldNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5330);
    }
    unsafe {
        let variant = WorldToObjectNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5331);
    }
    unsafe {
        let variant = HitTNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5332);
    }
    unsafe {
        let variant = HitKindNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5333);
    }
    unsafe {
        let variant = CurrentRayTimeNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5334);
    }
    unsafe {
        let variant = IncomingRayFlagsNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5351);
    }
    unsafe {
        let variant = RayGeometryIndexKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5352);
    }
    unsafe {
        let variant = WarpsPerSMNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5374);
    }
    unsafe {
        let variant = SMCountNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5375);
    }
    unsafe {
        let variant = WarpIDNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5376);
    }
    unsafe {
        let variant = SMIDNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5377);
    }
}
#[test]
pub fn ___test___Scope() {
    use std::mem::*;
    use Scope::*;
    unsafe {
        let variant = CrossDevice;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Device;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Workgroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Subgroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Invocation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = QueueFamily;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = ShaderCallKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
}
#[test]
pub fn ___test___GroupOperation() {
    use std::mem::*;
    use GroupOperation::*;
    unsafe {
        let variant = Reduce;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = InclusiveScan;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = ExclusiveScan;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = ClusteredReduce;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = PartitionedReduceNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = PartitionedInclusiveScanNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = PartitionedExclusiveScanNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
}
#[test]
pub fn ___test___KernelEnqueueFlags() {
    use std::mem::*;
    use KernelEnqueueFlags::*;
    unsafe {
        let variant = NoWait;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = WaitKernel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = WaitWorkGroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
}
#[test]
pub fn ___test___Capability() {
    use std::mem::*;
    use Capability::*;
    unsafe {
        let variant = Matrix;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = Shader;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = Geometry;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
    unsafe {
        let variant = Tessellation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 3);
    }
    unsafe {
        let variant = Addresses;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4);
    }
    unsafe {
        let variant = Linkage;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5);
    }
    unsafe {
        let variant = Kernel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6);
    }
    unsafe {
        let variant = Vector16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 7);
    }
    unsafe {
        let variant = Float16Buffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 8);
    }
    unsafe {
        let variant = Float16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 9);
    }
    unsafe {
        let variant = Float64;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 10);
    }
    unsafe {
        let variant = Int64;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 11);
    }
    unsafe {
        let variant = Int64Atomics;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 12);
    }
    unsafe {
        let variant = ImageBasic;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 13);
    }
    unsafe {
        let variant = ImageReadWrite;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 14);
    }
    unsafe {
        let variant = ImageMipmap;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 15);
    }
    unsafe {
        let variant = Pipes;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 17);
    }
    unsafe {
        let variant = Groups;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 18);
    }
    unsafe {
        let variant = DeviceEnqueue;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 19);
    }
    unsafe {
        let variant = LiteralSampler;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 20);
    }
    unsafe {
        let variant = AtomicStorage;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 21);
    }
    unsafe {
        let variant = Int16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 22);
    }
    unsafe {
        let variant = TessellationPointSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 23);
    }
    unsafe {
        let variant = GeometryPointSize;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 24);
    }
    unsafe {
        let variant = ImageGatherExtended;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 25);
    }
    unsafe {
        let variant = StorageImageMultisample;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 27);
    }
    unsafe {
        let variant = UniformBufferArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 28);
    }
    unsafe {
        let variant = SampledImageArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 29);
    }
    unsafe {
        let variant = StorageBufferArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 30);
    }
    unsafe {
        let variant = StorageImageArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 31);
    }
    unsafe {
        let variant = ClipDistance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 32);
    }
    unsafe {
        let variant = CullDistance;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 33);
    }
    unsafe {
        let variant = ImageCubeArray;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 34);
    }
    unsafe {
        let variant = SampleRateShading;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 35);
    }
    unsafe {
        let variant = ImageRect;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 36);
    }
    unsafe {
        let variant = SampledRect;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 37);
    }
    unsafe {
        let variant = GenericPointer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 38);
    }
    unsafe {
        let variant = Int8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 39);
    }
    unsafe {
        let variant = InputAttachment;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 40);
    }
    unsafe {
        let variant = SparseResidency;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 41);
    }
    unsafe {
        let variant = MinLod;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 42);
    }
    unsafe {
        let variant = Sampled1D;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 43);
    }
    unsafe {
        let variant = Image1D;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 44);
    }
    unsafe {
        let variant = SampledCubeArray;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 45);
    }
    unsafe {
        let variant = SampledBuffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 46);
    }
    unsafe {
        let variant = ImageBuffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 47);
    }
    unsafe {
        let variant = ImageMSArray;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 48);
    }
    unsafe {
        let variant = StorageImageExtendedFormats;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 49);
    }
    unsafe {
        let variant = ImageQuery;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 50);
    }
    unsafe {
        let variant = DerivativeControl;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 51);
    }
    unsafe {
        let variant = InterpolationFunction;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 52);
    }
    unsafe {
        let variant = TransformFeedback;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 53);
    }
    unsafe {
        let variant = GeometryStreams;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 54);
    }
    unsafe {
        let variant = StorageImageReadWithoutFormat;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 55);
    }
    unsafe {
        let variant = StorageImageWriteWithoutFormat;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 56);
    }
    unsafe {
        let variant = MultiViewport;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 57);
    }
    unsafe {
        let variant = SubgroupDispatch;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 58);
    }
    unsafe {
        let variant = NamedBarrier;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 59);
    }
    unsafe {
        let variant = PipeStorage;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 60);
    }
    unsafe {
        let variant = GroupNonUniform;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 61);
    }
    unsafe {
        let variant = GroupNonUniformVote;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 62);
    }
    unsafe {
        let variant = GroupNonUniformArithmetic;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 63);
    }
    unsafe {
        let variant = GroupNonUniformBallot;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 64);
    }
    unsafe {
        let variant = GroupNonUniformShuffle;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 65);
    }
    unsafe {
        let variant = GroupNonUniformShuffleRelative;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 66);
    }
    unsafe {
        let variant = GroupNonUniformClustered;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 67);
    }
    unsafe {
        let variant = GroupNonUniformQuad;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 68);
    }
    unsafe {
        let variant = ShaderLayer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 69);
    }
    unsafe {
        let variant = ShaderViewportIndex;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 70);
    }
    unsafe {
        let variant = UniformDecoration;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 71);
    }
    unsafe {
        let variant = FragmentShadingRateKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4422);
    }
    unsafe {
        let variant = SubgroupBallotKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4423);
    }
    unsafe {
        let variant = DrawParameters;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4427);
    }
    unsafe {
        let variant = WorkgroupMemoryExplicitLayoutKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4428);
    }
    unsafe {
        let variant = WorkgroupMemoryExplicitLayout8BitAccessKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4429);
    }
    unsafe {
        let variant = WorkgroupMemoryExplicitLayout16BitAccessKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4430);
    }
    unsafe {
        let variant = SubgroupVoteKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4431);
    }
    unsafe {
        let variant = StorageBuffer16BitAccess;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4433);
    }
    unsafe {
        let variant = UniformAndStorageBuffer16BitAccess;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4434);
    }
    unsafe {
        let variant = StoragePushConstant16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4435);
    }
    unsafe {
        let variant = StorageInputOutput16;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4436);
    }
    unsafe {
        let variant = DeviceGroup;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4437);
    }
    unsafe {
        let variant = MultiView;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4439);
    }
    unsafe {
        let variant = VariablePointersStorageBuffer;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4441);
    }
    unsafe {
        let variant = VariablePointers;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4442);
    }
    unsafe {
        let variant = AtomicStorageOps;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4445);
    }
    unsafe {
        let variant = SampleMaskPostDepthCoverage;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4447);
    }
    unsafe {
        let variant = StorageBuffer8BitAccess;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4448);
    }
    unsafe {
        let variant = UniformAndStorageBuffer8BitAccess;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4449);
    }
    unsafe {
        let variant = StoragePushConstant8;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4450);
    }
    unsafe {
        let variant = DenormPreserve;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4464);
    }
    unsafe {
        let variant = DenormFlushToZero;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4465);
    }
    unsafe {
        let variant = SignedZeroInfNanPreserve;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4466);
    }
    unsafe {
        let variant = RoundingModeRTE;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4467);
    }
    unsafe {
        let variant = RoundingModeRTZ;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4468);
    }
    unsafe {
        let variant = RayQueryProvisionalKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4471);
    }
    unsafe {
        let variant = RayQueryKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4472);
    }
    unsafe {
        let variant = RayTraversalPrimitiveCullingKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4478);
    }
    unsafe {
        let variant = RayTracingKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 4479);
    }
    unsafe {
        let variant = Float16ImageAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5008);
    }
    unsafe {
        let variant = ImageGatherBiasLodAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5009);
    }
    unsafe {
        let variant = FragmentMaskAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5010);
    }
    unsafe {
        let variant = StencilExportEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5013);
    }
    unsafe {
        let variant = ImageReadWriteLodAMD;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5015);
    }
    unsafe {
        let variant = Int64ImageEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5016);
    }
    unsafe {
        let variant = ShaderClockKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5055);
    }
    unsafe {
        let variant = SampleMaskOverrideCoverageNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5249);
    }
    unsafe {
        let variant = GeometryShaderPassthroughNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5251);
    }
    unsafe {
        let variant = ShaderViewportIndexLayerEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5254);
    }
    unsafe {
        let variant = ShaderViewportMaskNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5255);
    }
    unsafe {
        let variant = ShaderStereoViewNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5259);
    }
    unsafe {
        let variant = PerViewAttributesNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5260);
    }
    unsafe {
        let variant = FragmentFullyCoveredEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5265);
    }
    unsafe {
        let variant = MeshShadingNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5266);
    }
    unsafe {
        let variant = ImageFootprintNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5282);
    }
    unsafe {
        let variant = FragmentBarycentricKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5284);
    }
    unsafe {
        let variant = ComputeDerivativeGroupQuadsNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5288);
    }
    unsafe {
        let variant = FragmentDensityEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5291);
    }
    unsafe {
        let variant = GroupNonUniformPartitionedNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5297);
    }
    unsafe {
        let variant = ShaderNonUniform;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5301);
    }
    unsafe {
        let variant = RuntimeDescriptorArray;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5302);
    }
    unsafe {
        let variant = InputAttachmentArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5303);
    }
    unsafe {
        let variant = UniformTexelBufferArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5304);
    }
    unsafe {
        let variant = StorageTexelBufferArrayDynamicIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5305);
    }
    unsafe {
        let variant = UniformBufferArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5306);
    }
    unsafe {
        let variant = SampledImageArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5307);
    }
    unsafe {
        let variant = StorageBufferArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5308);
    }
    unsafe {
        let variant = StorageImageArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5309);
    }
    unsafe {
        let variant = InputAttachmentArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5310);
    }
    unsafe {
        let variant = UniformTexelBufferArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5311);
    }
    unsafe {
        let variant = StorageTexelBufferArrayNonUniformIndexing;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5312);
    }
    unsafe {
        let variant = RayTracingNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5340);
    }
    unsafe {
        let variant = RayTracingMotionBlurNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5341);
    }
    unsafe {
        let variant = VulkanMemoryModel;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5345);
    }
    unsafe {
        let variant = VulkanMemoryModelDeviceScope;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5346);
    }
    unsafe {
        let variant = PhysicalStorageBufferAddresses;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5347);
    }
    unsafe {
        let variant = ComputeDerivativeGroupLinearNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5350);
    }
    unsafe {
        let variant = RayTracingProvisionalKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5353);
    }
    unsafe {
        let variant = CooperativeMatrixNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5357);
    }
    unsafe {
        let variant = FragmentShaderSampleInterlockEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5363);
    }
    unsafe {
        let variant = FragmentShaderShadingRateInterlockEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5372);
    }
    unsafe {
        let variant = ShaderSMBuiltinsNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5373);
    }
    unsafe {
        let variant = FragmentShaderPixelInterlockEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5378);
    }
    unsafe {
        let variant = DemoteToHelperInvocation;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5379);
    }
    unsafe {
        let variant = BindlessTextureNV;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5390);
    }
    unsafe {
        let variant = SubgroupShuffleINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5568);
    }
    unsafe {
        let variant = SubgroupBufferBlockIOINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5569);
    }
    unsafe {
        let variant = SubgroupImageBlockIOINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5570);
    }
    unsafe {
        let variant = SubgroupImageMediaBlockIOINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5579);
    }
    unsafe {
        let variant = RoundToInfinityINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5582);
    }
    unsafe {
        let variant = FloatingPointModeINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5583);
    }
    unsafe {
        let variant = IntegerFunctions2INTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5584);
    }
    unsafe {
        let variant = FunctionPointersINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5603);
    }
    unsafe {
        let variant = IndirectReferencesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5604);
    }
    unsafe {
        let variant = AsmINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5606);
    }
    unsafe {
        let variant = AtomicFloat32MinMaxEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5612);
    }
    unsafe {
        let variant = AtomicFloat64MinMaxEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5613);
    }
    unsafe {
        let variant = AtomicFloat16MinMaxEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5616);
    }
    unsafe {
        let variant = VectorComputeINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5617);
    }
    unsafe {
        let variant = VectorAnyINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5619);
    }
    unsafe {
        let variant = ExpectAssumeKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5629);
    }
    unsafe {
        let variant = SubgroupAvcMotionEstimationINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5696);
    }
    unsafe {
        let variant = SubgroupAvcMotionEstimationIntraINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5697);
    }
    unsafe {
        let variant = SubgroupAvcMotionEstimationChromaINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5698);
    }
    unsafe {
        let variant = VariableLengthArrayINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5817);
    }
    unsafe {
        let variant = FunctionFloatControlINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5821);
    }
    unsafe {
        let variant = FPGAMemoryAttributesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5824);
    }
    unsafe {
        let variant = FPFastMathModeINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5837);
    }
    unsafe {
        let variant = ArbitraryPrecisionIntegersINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5844);
    }
    unsafe {
        let variant = ArbitraryPrecisionFloatingPointINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5845);
    }
    unsafe {
        let variant = UnstructuredLoopControlsINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5886);
    }
    unsafe {
        let variant = FPGALoopControlsINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5888);
    }
    unsafe {
        let variant = KernelAttributesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5892);
    }
    unsafe {
        let variant = FPGAKernelAttributesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5897);
    }
    unsafe {
        let variant = FPGAMemoryAccessesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5898);
    }
    unsafe {
        let variant = FPGAClusterAttributesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5904);
    }
    unsafe {
        let variant = LoopFuseINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5906);
    }
    unsafe {
        let variant = MemoryAccessAliasingINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5910);
    }
    unsafe {
        let variant = FPGABufferLocationINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5920);
    }
    unsafe {
        let variant = ArbitraryPrecisionFixedPointINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5922);
    }
    unsafe {
        let variant = USMStorageClassesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5935);
    }
    unsafe {
        let variant = IOPipesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5943);
    }
    unsafe {
        let variant = BlockingPipesINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5945);
    }
    unsafe {
        let variant = FPGARegINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 5948);
    }
    unsafe {
        let variant = DotProductInputAll;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6016);
    }
    unsafe {
        let variant = DotProductInput4x8Bit;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6017);
    }
    unsafe {
        let variant = DotProductInput4x8BitPacked;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6018);
    }
    unsafe {
        let variant = DotProduct;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6019);
    }
    unsafe {
        let variant = BitInstructions;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6025);
    }
    unsafe {
        let variant = AtomicFloat32AddEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6033);
    }
    unsafe {
        let variant = AtomicFloat64AddEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6034);
    }
    unsafe {
        let variant = LongConstantCompositeINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6089);
    }
    unsafe {
        let variant = OptNoneINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6094);
    }
    unsafe {
        let variant = AtomicFloat16AddEXT;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6095);
    }
    unsafe {
        let variant = DebugInfoModuleINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6114);
    }
    unsafe {
        let variant = SplitBarrierINTEL;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6141);
    }
    unsafe {
        let variant = GroupUniformArithmeticKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 6400);
    }
}
#[test]
pub fn ___test___RayQueryIntersection() {
    use std::mem::*;
    use RayQueryIntersection::*;
    unsafe {
        let variant = RayQueryCandidateIntersectionKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = RayQueryCommittedIntersectionKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___RayQueryCommittedIntersectionType() {
    use std::mem::*;
    use RayQueryCommittedIntersectionType::*;
    unsafe {
        let variant = RayQueryCommittedIntersectionNoneKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = RayQueryCommittedIntersectionTriangleKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
    unsafe {
        let variant = RayQueryCommittedIntersectionGeneratedKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 2);
    }
}
#[test]
pub fn ___test___RayQueryCandidateIntersectionType() {
    use std::mem::*;
    use RayQueryCandidateIntersectionType::*;
    unsafe {
        let variant = RayQueryCandidateIntersectionTriangleKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
    unsafe {
        let variant = RayQueryCandidateIntersectionAABBKHR;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 1);
    }
}
#[test]
pub fn ___test___PackedVectorFormat() {
    use std::mem::*;
    use PackedVectorFormat::*;
    unsafe {
        let variant = PackedVectorFormat4x8Bit;
        let disc: u32 = std::mem::transmute(std::mem::discriminant(&variant));
        assert_eq!(disc, 0);
    }
}
