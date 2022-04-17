pub use Opcode::*;
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Opcode {
    #[default]
    OpNop = 0,
    OpUndef(IdType, IdResult) = 1,
    OpSourceContinued(String) = 2,
    OpSource(SourceLanguage, u32, Option<ID>, Option<String>) = 3,
    OpSourceExtension(String) = 4,
    OpName(ID, String) = 5,
    OpMemberName(ID, u32, String) = 6,
    OpString(IdResult, String) = 7,
    OpLine(ID, u32, u32) = 8,
    OpExtension(String) = 10,
    OpExtInstImport(IdResult, String) = 11,
    OpExtInst(IdType, IdResult, ID, u32, Vec<ID>) = 12,
    OpMemoryModel(AddressingModel, MemoryModel) = 14,
    OpEntryPoint(ExecutionModel, ID, String, Vec<ID>) = 15,
    OpExecutionMode(ID, ExecutionMode) = 16,
    OpCapability(Capability) = 17,
    OpTypeVoid(IdResult) = 19,
    OpTypeBool(IdResult) = 20,
    OpTypeInt(IdResult, u32, u32) = 21,
    OpTypeFloat(IdResult, u32) = 22,
    OpTypeVector(IdResult, ID, u32) = 23,
    OpTypeMatrix(IdResult, ID, u32) = 24,
    OpTypeImage(
        IdResult,
        ID,
        Dim,
        u32,
        u32,
        u32,
        u32,
        ImageFormat,
        Option<AccessQualifier>,
    ) = 25,
    OpTypeSampler(IdResult) = 26,
    OpTypeSampledImage(IdResult, ID) = 27,
    OpTypeArray(IdResult, ID, ID) = 28,
    OpTypeRuntimeArray(IdResult, ID) = 29,
    OpTypeStruct(IdResult, Vec<ID>) = 30,
    OpTypeOpaque(IdResult, String) = 31,
    OpTypePointer(IdResult, StorageClass, ID) = 32,
    OpTypeFunction(IdResult, ID, Vec<ID>) = 33,
    OpTypeEvent(IdResult) = 34,
    OpTypeDeviceEvent(IdResult) = 35,
    OpTypeReserveId(IdResult) = 36,
    OpTypeQueue(IdResult) = 37,
    OpTypePipe(IdResult, AccessQualifier) = 38,
    OpTypeForwardPointer(ID, StorageClass) = 39,
    OpConstantTrue(IdType, IdResult) = 41,
    OpConstantFalse(IdType, IdResult) = 42,
    OpConstant(IdType, IdResult, u32) = 43,
    OpConstantComposite(IdType, IdResult, Vec<ID>) = 44,
    OpConstantSampler(
        IdType,
        IdResult,
        SamplerAddressingMode,
        u32,
        SamplerFilterMode,
    ) = 45,
    OpConstantNull(IdType, IdResult) = 46,
    OpSpecConstantTrue(IdType, IdResult) = 48,
    OpSpecConstantFalse(IdType, IdResult) = 49,
    OpSpecConstant(IdType, IdResult, u32) = 50,
    OpSpecConstantComposite(IdType, IdResult, Vec<ID>) = 51,
    OpSpecConstantOp(IdType, IdResult, Box<Opcode>) = 52,
    OpFunction(IdType, IdResult, FunctionControl, ID) = 54,
    OpFunctionParameter(IdType, IdResult) = 55,
    OpFunctionEnd = 56,
    OpFunctionCall(IdType, IdResult, ID, Vec<ID>) = 57,
    OpVariable(IdType, IdResult, StorageClass, Option<ID>) = 59,
    OpImageTexelPointer(IdType, IdResult, ID, ID, ID) = 60,
    OpLoad(IdType, IdResult, ID, Option<MemoryAccess>) = 61,
    OpStore(ID, ID, Option<MemoryAccess>) = 62,
    OpCopyMemory(ID, ID, Option<MemoryAccess>, Option<MemoryAccess>) = 63,
    OpCopyMemorySized(ID, ID, ID, Option<MemoryAccess>, Option<MemoryAccess>) = 64,
    OpAccessChain(IdType, IdResult, ID, Vec<ID>) = 65,
    OpInBoundsAccessChain(IdType, IdResult, ID, Vec<ID>) = 66,
    OpPtrAccessChain(IdType, IdResult, ID, ID, Vec<ID>) = 67,
    OpArrayLength(IdType, IdResult, ID, u32) = 68,
    OpGenericPtrMemSemantics(IdType, IdResult, ID) = 69,
    OpInBoundsPtrAccessChain(IdType, IdResult, ID, ID, Vec<ID>) = 70,
    OpDecorate(ID, Decoration) = 71,
    OpMemberDecorate(ID, u32, Decoration) = 72,
    OpDecorationGroup(IdResult) = 73,
    OpGroupDecorate(ID, Vec<ID>) = 74,
    OpGroupMemberDecorate(ID, Vec<(ID, u32)>) = 75,
    OpVectorExtractDynamic(IdType, IdResult, ID, ID) = 77,
    OpVectorInsertDynamic(IdType, IdResult, ID, ID, ID) = 78,
    OpVectorShuffle(IdType, IdResult, ID, ID, Vec<u32>) = 79,
    OpCompositeConstruct(IdType, IdResult, Vec<ID>) = 80,
    OpCompositeExtract(IdType, IdResult, ID, Vec<u32>) = 81,
    OpCompositeInsert(IdType, IdResult, ID, ID, Vec<u32>) = 82,
    OpCopyObject(IdType, IdResult, ID) = 83,
    OpTranspose(IdType, IdResult, ID) = 84,
    OpSampledImage(IdType, IdResult, ID, ID) = 86,
    OpImageSampleImplicitLod(IdType, IdResult, ID, ID, Option<ImageOperands>) = 87,
    OpImageSampleExplicitLod(IdType, IdResult, ID, ID, ImageOperands) = 88,
    OpImageSampleDrefImplicitLod(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 89,
    OpImageSampleDrefExplicitLod(IdType, IdResult, ID, ID, ID, ImageOperands) = 90,
    OpImageSampleProjImplicitLod(IdType, IdResult, ID, ID, Option<ImageOperands>) = 91,
    OpImageSampleProjExplicitLod(IdType, IdResult, ID, ID, ImageOperands) = 92,
    OpImageSampleProjDrefImplicitLod(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 93,
    OpImageSampleProjDrefExplicitLod(IdType, IdResult, ID, ID, ID, ImageOperands) = 94,
    OpImageFetch(IdType, IdResult, ID, ID, Option<ImageOperands>) = 95,
    OpImageGather(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 96,
    OpImageDrefGather(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 97,
    OpImageRead(IdType, IdResult, ID, ID, Option<ImageOperands>) = 98,
    OpImageWrite(ID, ID, ID, Option<ImageOperands>) = 99,
    OpImage(IdType, IdResult, ID) = 100,
    OpImageQueryFormat(IdType, IdResult, ID) = 101,
    OpImageQueryOrder(IdType, IdResult, ID) = 102,
    OpImageQuerySizeLod(IdType, IdResult, ID, ID) = 103,
    OpImageQuerySize(IdType, IdResult, ID) = 104,
    OpImageQueryLod(IdType, IdResult, ID, ID) = 105,
    OpImageQueryLevels(IdType, IdResult, ID) = 106,
    OpImageQuerySamples(IdType, IdResult, ID) = 107,
    OpConvertFToU(IdType, IdResult, ID) = 109,
    OpConvertFToS(IdType, IdResult, ID) = 110,
    OpConvertSToF(IdType, IdResult, ID) = 111,
    OpConvertUToF(IdType, IdResult, ID) = 112,
    OpUConvert(IdType, IdResult, ID) = 113,
    OpSConvert(IdType, IdResult, ID) = 114,
    OpFConvert(IdType, IdResult, ID) = 115,
    OpQuantizeToF16(IdType, IdResult, ID) = 116,
    OpConvertPtrToU(IdType, IdResult, ID) = 117,
    OpSatConvertSToU(IdType, IdResult, ID) = 118,
    OpSatConvertUToS(IdType, IdResult, ID) = 119,
    OpConvertUToPtr(IdType, IdResult, ID) = 120,
    OpPtrCastToGeneric(IdType, IdResult, ID) = 121,
    OpGenericCastToPtr(IdType, IdResult, ID) = 122,
    OpGenericCastToPtrExplicit(IdType, IdResult, ID, StorageClass) = 123,
    OpBitcast(IdType, IdResult, ID) = 124,
    OpSNegate(IdType, IdResult, ID) = 126,
    OpFNegate(IdType, IdResult, ID) = 127,
    OpIAdd(IdType, IdResult, ID, ID) = 128,
    OpFAdd(IdType, IdResult, ID, ID) = 129,
    OpISub(IdType, IdResult, ID, ID) = 130,
    OpFSub(IdType, IdResult, ID, ID) = 131,
    OpIMul(IdType, IdResult, ID, ID) = 132,
    OpFMul(IdType, IdResult, ID, ID) = 133,
    OpUDiv(IdType, IdResult, ID, ID) = 134,
    OpSDiv(IdType, IdResult, ID, ID) = 135,
    OpFDiv(IdType, IdResult, ID, ID) = 136,
    OpUMod(IdType, IdResult, ID, ID) = 137,
    OpSRem(IdType, IdResult, ID, ID) = 138,
    OpSMod(IdType, IdResult, ID, ID) = 139,
    OpFRem(IdType, IdResult, ID, ID) = 140,
    OpFMod(IdType, IdResult, ID, ID) = 141,
    OpVectorTimesScalar(IdType, IdResult, ID, ID) = 142,
    OpMatrixTimesScalar(IdType, IdResult, ID, ID) = 143,
    OpVectorTimesMatrix(IdType, IdResult, ID, ID) = 144,
    OpMatrixTimesVector(IdType, IdResult, ID, ID) = 145,
    OpMatrixTimesMatrix(IdType, IdResult, ID, ID) = 146,
    OpOuterProduct(IdType, IdResult, ID, ID) = 147,
    OpDot(IdType, IdResult, ID, ID) = 148,
    OpIAddCarry(IdType, IdResult, ID, ID) = 149,
    OpISubBorrow(IdType, IdResult, ID, ID) = 150,
    OpUMulExtended(IdType, IdResult, ID, ID) = 151,
    OpSMulExtended(IdType, IdResult, ID, ID) = 152,
    OpAny(IdType, IdResult, ID) = 154,
    OpAll(IdType, IdResult, ID) = 155,
    OpIsNan(IdType, IdResult, ID) = 156,
    OpIsInf(IdType, IdResult, ID) = 157,
    OpIsFinite(IdType, IdResult, ID) = 158,
    OpIsNormal(IdType, IdResult, ID) = 159,
    OpSignBitSet(IdType, IdResult, ID) = 160,
    OpLessOrGreater(IdType, IdResult, ID, ID) = 161,
    OpOrdered(IdType, IdResult, ID, ID) = 162,
    OpUnordered(IdType, IdResult, ID, ID) = 163,
    OpLogicalEqual(IdType, IdResult, ID, ID) = 164,
    OpLogicalNotEqual(IdType, IdResult, ID, ID) = 165,
    OpLogicalOr(IdType, IdResult, ID, ID) = 166,
    OpLogicalAnd(IdType, IdResult, ID, ID) = 167,
    OpLogicalNot(IdType, IdResult, ID) = 168,
    OpSelect(IdType, IdResult, ID, ID, ID) = 169,
    OpIEqual(IdType, IdResult, ID, ID) = 170,
    OpINotEqual(IdType, IdResult, ID, ID) = 171,
    OpUGreaterThan(IdType, IdResult, ID, ID) = 172,
    OpSGreaterThan(IdType, IdResult, ID, ID) = 173,
    OpUGreaterThanEqual(IdType, IdResult, ID, ID) = 174,
    OpSGreaterThanEqual(IdType, IdResult, ID, ID) = 175,
    OpULessThan(IdType, IdResult, ID, ID) = 176,
    OpSLessThan(IdType, IdResult, ID, ID) = 177,
    OpULessThanEqual(IdType, IdResult, ID, ID) = 178,
    OpSLessThanEqual(IdType, IdResult, ID, ID) = 179,
    OpFOrdEqual(IdType, IdResult, ID, ID) = 180,
    OpFUnordEqual(IdType, IdResult, ID, ID) = 181,
    OpFOrdNotEqual(IdType, IdResult, ID, ID) = 182,
    OpFUnordNotEqual(IdType, IdResult, ID, ID) = 183,
    OpFOrdLessThan(IdType, IdResult, ID, ID) = 184,
    OpFUnordLessThan(IdType, IdResult, ID, ID) = 185,
    OpFOrdGreaterThan(IdType, IdResult, ID, ID) = 186,
    OpFUnordGreaterThan(IdType, IdResult, ID, ID) = 187,
    OpFOrdLessThanEqual(IdType, IdResult, ID, ID) = 188,
    OpFUnordLessThanEqual(IdType, IdResult, ID, ID) = 189,
    OpFOrdGreaterThanEqual(IdType, IdResult, ID, ID) = 190,
    OpFUnordGreaterThanEqual(IdType, IdResult, ID, ID) = 191,
    OpShiftRightLogical(IdType, IdResult, ID, ID) = 194,
    OpShiftRightArithmetic(IdType, IdResult, ID, ID) = 195,
    OpShiftLeftLogical(IdType, IdResult, ID, ID) = 196,
    OpBitwiseOr(IdType, IdResult, ID, ID) = 197,
    OpBitwiseXor(IdType, IdResult, ID, ID) = 198,
    OpBitwiseAnd(IdType, IdResult, ID, ID) = 199,
    OpNot(IdType, IdResult, ID) = 200,
    OpBitFieldInsert(IdType, IdResult, ID, ID, ID, ID) = 201,
    OpBitFieldSExtract(IdType, IdResult, ID, ID, ID) = 202,
    OpBitFieldUExtract(IdType, IdResult, ID, ID, ID) = 203,
    OpBitReverse(IdType, IdResult, ID) = 204,
    OpBitCount(IdType, IdResult, ID) = 205,
    OpDPdx(IdType, IdResult, ID) = 207,
    OpDPdy(IdType, IdResult, ID) = 208,
    OpFwidth(IdType, IdResult, ID) = 209,
    OpDPdxFine(IdType, IdResult, ID) = 210,
    OpDPdyFine(IdType, IdResult, ID) = 211,
    OpFwidthFine(IdType, IdResult, ID) = 212,
    OpDPdxCoarse(IdType, IdResult, ID) = 213,
    OpDPdyCoarse(IdType, IdResult, ID) = 214,
    OpFwidthCoarse(IdType, IdResult, ID) = 215,
    OpEmitVertex = 218,
    OpEndPrimitive = 219,
    OpEmitStreamVertex(ID) = 220,
    OpEndStreamPrimitive(ID) = 221,
    OpControlBarrier(ID, ID, ID) = 224,
    OpMemoryBarrier(ID, ID) = 225,
    OpAtomicLoad(IdType, IdResult, ID, ID, ID) = 227,
    OpAtomicStore(ID, ID, ID, ID) = 228,
    OpAtomicExchange(IdType, IdResult, ID, ID, ID, ID) = 229,
    OpAtomicCompareExchange(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 230,
    OpAtomicCompareExchangeWeak(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 231,
    OpAtomicIIncrement(IdType, IdResult, ID, ID, ID) = 232,
    OpAtomicIDecrement(IdType, IdResult, ID, ID, ID) = 233,
    OpAtomicIAdd(IdType, IdResult, ID, ID, ID, ID) = 234,
    OpAtomicISub(IdType, IdResult, ID, ID, ID, ID) = 235,
    OpAtomicSMin(IdType, IdResult, ID, ID, ID, ID) = 236,
    OpAtomicUMin(IdType, IdResult, ID, ID, ID, ID) = 237,
    OpAtomicSMax(IdType, IdResult, ID, ID, ID, ID) = 238,
    OpAtomicUMax(IdType, IdResult, ID, ID, ID, ID) = 239,
    OpAtomicAnd(IdType, IdResult, ID, ID, ID, ID) = 240,
    OpAtomicOr(IdType, IdResult, ID, ID, ID, ID) = 241,
    OpAtomicXor(IdType, IdResult, ID, ID, ID, ID) = 242,
    OpPhi(IdType, IdResult, Vec<(ID, ID)>) = 245,
    OpLoopMerge(ID, ID, LoopControl) = 246,
    OpSelectionMerge(ID, SelectionControl) = 247,
    OpLabel(IdResult) = 248,
    OpBranch(ID) = 249,
    OpBranchConditional(ID, ID, ID, Vec<u32>) = 250,
    OpSwitch(ID, ID, Vec<(u32, ID)>) = 251,
    OpKill = 252,
    OpReturn = 253,
    OpReturnValue(ID) = 254,
    OpUnreachable = 255,
    OpLifetimeStart(ID, u32) = 256,
    OpLifetimeStop(ID, u32) = 257,
    OpGroupAsyncCopy(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 259,
    OpGroupWaitEvents(ID, ID, ID) = 260,
    OpGroupAll(IdType, IdResult, ID, ID) = 261,
    OpGroupAny(IdType, IdResult, ID, ID) = 262,
    OpGroupBroadcast(IdType, IdResult, ID, ID, ID) = 263,
    OpGroupIAdd(IdType, IdResult, ID, GroupOperation, ID) = 264,
    OpGroupFAdd(IdType, IdResult, ID, GroupOperation, ID) = 265,
    OpGroupFMin(IdType, IdResult, ID, GroupOperation, ID) = 266,
    OpGroupUMin(IdType, IdResult, ID, GroupOperation, ID) = 267,
    OpGroupSMin(IdType, IdResult, ID, GroupOperation, ID) = 268,
    OpGroupFMax(IdType, IdResult, ID, GroupOperation, ID) = 269,
    OpGroupUMax(IdType, IdResult, ID, GroupOperation, ID) = 270,
    OpGroupSMax(IdType, IdResult, ID, GroupOperation, ID) = 271,
    OpReadPipe(IdType, IdResult, ID, ID, ID, ID) = 274,
    OpWritePipe(IdType, IdResult, ID, ID, ID, ID) = 275,
    OpReservedReadPipe(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 276,
    OpReservedWritePipe(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 277,
    OpReserveReadPipePackets(IdType, IdResult, ID, ID, ID, ID) = 278,
    OpReserveWritePipePackets(IdType, IdResult, ID, ID, ID, ID) = 279,
    OpCommitReadPipe(ID, ID, ID, ID) = 280,
    OpCommitWritePipe(ID, ID, ID, ID) = 281,
    OpIsValidReserveId(IdType, IdResult, ID) = 282,
    OpGetNumPipePackets(IdType, IdResult, ID, ID, ID) = 283,
    OpGetMaxPipePackets(IdType, IdResult, ID, ID, ID) = 284,
    OpGroupReserveReadPipePackets(IdType, IdResult, ID, ID, ID, ID, ID) = 285,
    OpGroupReserveWritePipePackets(IdType, IdResult, ID, ID, ID, ID, ID) = 286,
    OpGroupCommitReadPipe(ID, ID, ID, ID, ID) = 287,
    OpGroupCommitWritePipe(ID, ID, ID, ID, ID) = 288,
    OpEnqueueMarker(IdType, IdResult, ID, ID, ID, ID) = 291,
    OpEnqueueKernel(
        IdType,
        IdResult,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        Vec<ID>,
    ) = 292,
    OpGetKernelNDrangeSubGroupCount(IdType, IdResult, ID, ID, ID, ID, ID) = 293,
    OpGetKernelNDrangeMaxSubGroupSize(IdType, IdResult, ID, ID, ID, ID, ID) = 294,
    OpGetKernelWorkGroupSize(IdType, IdResult, ID, ID, ID, ID) = 295,
    OpGetKernelPreferredWorkGroupSizeMultiple(IdType, IdResult, ID, ID, ID, ID) = 296,
    OpRetainEvent(ID) = 297,
    OpReleaseEvent(ID) = 298,
    OpCreateUserEvent(IdType, IdResult) = 299,
    OpIsValidEvent(IdType, IdResult, ID) = 300,
    OpSetUserEventStatus(ID, ID) = 301,
    OpCaptureEventProfilingInfo(ID, ID, ID) = 302,
    OpGetDefaultQueue(IdType, IdResult) = 303,
    OpBuildNDRange(IdType, IdResult, ID, ID, ID) = 304,
    OpImageSparseSampleImplicitLod(IdType, IdResult, ID, ID, Option<ImageOperands>) = 305,
    OpImageSparseSampleExplicitLod(IdType, IdResult, ID, ID, ImageOperands) = 306,
    OpImageSparseSampleDrefImplicitLod(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 307,
    OpImageSparseSampleDrefExplicitLod(IdType, IdResult, ID, ID, ID, ImageOperands) = 308,
    OpImageSparseSampleProjImplicitLod(IdType, IdResult, ID, ID, Option<ImageOperands>) = 309,
    OpImageSparseSampleProjExplicitLod(IdType, IdResult, ID, ID, ImageOperands) = 310,
    OpImageSparseSampleProjDrefImplicitLod(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) =
        311,
    OpImageSparseSampleProjDrefExplicitLod(IdType, IdResult, ID, ID, ID, ImageOperands) = 312,
    OpImageSparseFetch(IdType, IdResult, ID, ID, Option<ImageOperands>) = 313,
    OpImageSparseGather(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 314,
    OpImageSparseDrefGather(IdType, IdResult, ID, ID, ID, Option<ImageOperands>) = 315,
    OpImageSparseTexelsResident(IdType, IdResult, ID) = 316,
    OpNoLine = 317,
    OpAtomicFlagTestAndSet(IdType, IdResult, ID, ID, ID) = 318,
    OpAtomicFlagClear(ID, ID, ID) = 319,
    OpImageSparseRead(IdType, IdResult, ID, ID, Option<ImageOperands>) = 320,
    OpSizeOf(IdType, IdResult, ID) = 321,
    OpTypePipeStorage(IdResult) = 322,
    OpConstantPipeStorage(IdType, IdResult, u32, u32, u32) = 323,
    OpCreatePipeFromPipeStorage(IdType, IdResult, ID) = 324,
    OpGetKernelLocalSizeForSubgroupCount(IdType, IdResult, ID, ID, ID, ID, ID) = 325,
    OpGetKernelMaxNumSubgroups(IdType, IdResult, ID, ID, ID, ID) = 326,
    OpTypeNamedBarrier(IdResult) = 327,
    OpNamedBarrierInitialize(IdType, IdResult, ID) = 328,
    OpMemoryNamedBarrier(ID, ID, ID) = 329,
    OpModuleProcessed(String) = 330,
    OpExecutionModeId(ID, ExecutionMode) = 331,
    OpDecorateId(ID, Decoration) = 332,
    OpGroupNonUniformElect(IdType, IdResult, ID) = 333,
    OpGroupNonUniformAll(IdType, IdResult, ID, ID) = 334,
    OpGroupNonUniformAny(IdType, IdResult, ID, ID) = 335,
    OpGroupNonUniformAllEqual(IdType, IdResult, ID, ID) = 336,
    OpGroupNonUniformBroadcast(IdType, IdResult, ID, ID, ID) = 337,
    OpGroupNonUniformBroadcastFirst(IdType, IdResult, ID, ID) = 338,
    OpGroupNonUniformBallot(IdType, IdResult, ID, ID) = 339,
    OpGroupNonUniformInverseBallot(IdType, IdResult, ID, ID) = 340,
    OpGroupNonUniformBallotBitExtract(IdType, IdResult, ID, ID, ID) = 341,
    OpGroupNonUniformBallotBitCount(IdType, IdResult, ID, GroupOperation, ID) = 342,
    OpGroupNonUniformBallotFindLSB(IdType, IdResult, ID, ID) = 343,
    OpGroupNonUniformBallotFindMSB(IdType, IdResult, ID, ID) = 344,
    OpGroupNonUniformShuffle(IdType, IdResult, ID, ID, ID) = 345,
    OpGroupNonUniformShuffleXor(IdType, IdResult, ID, ID, ID) = 346,
    OpGroupNonUniformShuffleUp(IdType, IdResult, ID, ID, ID) = 347,
    OpGroupNonUniformShuffleDown(IdType, IdResult, ID, ID, ID) = 348,
    OpGroupNonUniformIAdd(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 349,
    OpGroupNonUniformFAdd(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 350,
    OpGroupNonUniformIMul(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 351,
    OpGroupNonUniformFMul(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 352,
    OpGroupNonUniformSMin(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 353,
    OpGroupNonUniformUMin(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 354,
    OpGroupNonUniformFMin(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 355,
    OpGroupNonUniformSMax(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 356,
    OpGroupNonUniformUMax(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 357,
    OpGroupNonUniformFMax(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 358,
    OpGroupNonUniformBitwiseAnd(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 359,
    OpGroupNonUniformBitwiseOr(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 360,
    OpGroupNonUniformBitwiseXor(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 361,
    OpGroupNonUniformLogicalAnd(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 362,
    OpGroupNonUniformLogicalOr(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 363,
    OpGroupNonUniformLogicalXor(IdType, IdResult, ID, GroupOperation, ID, Option<ID>) = 364,
    OpGroupNonUniformQuadBroadcast(IdType, IdResult, ID, ID, ID) = 365,
    OpGroupNonUniformQuadSwap(IdType, IdResult, ID, ID, ID) = 366,
    OpCopyLogical(IdType, IdResult, ID) = 400,
    OpPtrEqual(IdType, IdResult, ID, ID) = 401,
    OpPtrNotEqual(IdType, IdResult, ID, ID) = 402,
    OpPtrDiff(IdType, IdResult, ID, ID) = 403,
    OpTerminateInvocation = 4416,
    OpSubgroupBallotKHR(IdType, IdResult, ID) = 4421,
    OpSubgroupFirstInvocationKHR(IdType, IdResult, ID) = 4422,
    OpSubgroupAllKHR(IdType, IdResult, ID) = 4428,
    OpSubgroupAnyKHR(IdType, IdResult, ID) = 4429,
    OpSubgroupAllEqualKHR(IdType, IdResult, ID) = 4430,
    OpSubgroupReadInvocationKHR(IdType, IdResult, ID, ID) = 4432,
    OpTraceRayKHR(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 4445,
    OpExecuteCallableKHR(ID, ID) = 4446,
    OpConvertUToAccelerationStructureKHR(IdType, IdResult, ID) = 4447,
    OpIgnoreIntersectionKHR = 4448,
    OpTerminateRayKHR = 4449,
    OpSDot(IdType, IdResult, ID, ID, Option<PackedVectorFormat>) = 4450,
    OpUDot(IdType, IdResult, ID, ID, Option<PackedVectorFormat>) = 4451,
    OpSUDot(IdType, IdResult, ID, ID, Option<PackedVectorFormat>) = 4452,
    OpSDotAccSat(IdType, IdResult, ID, ID, ID, Option<PackedVectorFormat>) = 4453,
    OpUDotAccSat(IdType, IdResult, ID, ID, ID, Option<PackedVectorFormat>) = 4454,
    OpSUDotAccSat(IdType, IdResult, ID, ID, ID, Option<PackedVectorFormat>) = 4455,
    OpTypeRayQueryKHR(IdResult) = 4472,
    OpRayQueryInitializeKHR(ID, ID, ID, ID, ID, ID, ID, ID) = 4473,
    OpRayQueryTerminateKHR(ID) = 4474,
    OpRayQueryGenerateIntersectionKHR(ID, ID) = 4475,
    OpRayQueryConfirmIntersectionKHR(ID) = 4476,
    OpRayQueryProceedKHR(IdType, IdResult, ID) = 4477,
    OpRayQueryGetIntersectionTypeKHR(IdType, IdResult, ID, ID) = 4479,
    OpGroupIAddNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5000,
    OpGroupFAddNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5001,
    OpGroupFMinNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5002,
    OpGroupUMinNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5003,
    OpGroupSMinNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5004,
    OpGroupFMaxNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5005,
    OpGroupUMaxNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5006,
    OpGroupSMaxNonUniformAMD(IdType, IdResult, ID, GroupOperation, ID) = 5007,
    OpFragmentMaskFetchAMD(IdType, IdResult, ID, ID) = 5011,
    OpFragmentFetchAMD(IdType, IdResult, ID, ID, ID) = 5012,
    OpReadClockKHR(IdType, IdResult, ID) = 5056,
    OpImageSampleFootprintNV(IdType, IdResult, ID, ID, ID, ID, Option<ImageOperands>) = 5283,
    OpGroupNonUniformPartitionNV(IdType, IdResult, ID) = 5296,
    OpWritePackedPrimitiveIndices4x8NV(ID, ID) = 5299,
    OpReportIntersectionNV(IdType, IdResult, ID, ID) = 5334,
    OpIgnoreIntersectionNV = 5335,
    OpTerminateRayNV = 5336,
    OpTraceNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5337,
    OpTraceMotionNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5338,
    OpTraceRayMotionNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5339,
    OpTypeAccelerationStructureNV(IdResult) = 5341,
    OpExecuteCallableNV(ID, ID) = 5344,
    OpTypeCooperativeMatrixNV(IdResult, ID, ID, ID, ID) = 5358,
    OpCooperativeMatrixLoadNV(IdType, IdResult, ID, ID, ID, Option<MemoryAccess>) = 5359,
    OpCooperativeMatrixStoreNV(ID, ID, ID, ID, Option<MemoryAccess>) = 5360,
    OpCooperativeMatrixMulAddNV(IdType, IdResult, ID, ID, ID) = 5361,
    OpCooperativeMatrixLengthNV(IdType, IdResult, ID) = 5362,
    OpBeginInvocationInterlockEXT = 5364,
    OpEndInvocationInterlockEXT = 5365,
    OpDemoteToHelperInvocation = 5380,
    OpIsHelperInvocationEXT(IdType, IdResult) = 5381,
    OpConvertUToImageNV(IdType, IdResult, ID) = 5391,
    OpConvertUToSamplerNV(IdType, IdResult, ID) = 5392,
    OpConvertImageToUNV(IdType, IdResult, ID) = 5393,
    OpConvertSamplerToUNV(IdType, IdResult, ID) = 5394,
    OpConvertUToSampledImageNV(IdType, IdResult, ID) = 5395,
    OpConvertSampledImageToUNV(IdType, IdResult, ID) = 5396,
    OpSamplerImageAddressingModeNV(u32) = 5397,
    OpSubgroupShuffleINTEL(IdType, IdResult, ID, ID) = 5571,
    OpSubgroupShuffleDownINTEL(IdType, IdResult, ID, ID, ID) = 5572,
    OpSubgroupShuffleUpINTEL(IdType, IdResult, ID, ID, ID) = 5573,
    OpSubgroupShuffleXorINTEL(IdType, IdResult, ID, ID) = 5574,
    OpSubgroupBlockReadINTEL(IdType, IdResult, ID) = 5575,
    OpSubgroupBlockWriteINTEL(ID, ID) = 5576,
    OpSubgroupImageBlockReadINTEL(IdType, IdResult, ID, ID) = 5577,
    OpSubgroupImageBlockWriteINTEL(ID, ID, ID) = 5578,
    OpSubgroupImageMediaBlockReadINTEL(IdType, IdResult, ID, ID, ID, ID) = 5580,
    OpSubgroupImageMediaBlockWriteINTEL(ID, ID, ID, ID, ID) = 5581,
    OpUCountLeadingZerosINTEL(IdType, IdResult, ID) = 5585,
    OpUCountTrailingZerosINTEL(IdType, IdResult, ID) = 5586,
    OpAbsISubINTEL(IdType, IdResult, ID, ID) = 5587,
    OpAbsUSubINTEL(IdType, IdResult, ID, ID) = 5588,
    OpIAddSatINTEL(IdType, IdResult, ID, ID) = 5589,
    OpUAddSatINTEL(IdType, IdResult, ID, ID) = 5590,
    OpIAverageINTEL(IdType, IdResult, ID, ID) = 5591,
    OpUAverageINTEL(IdType, IdResult, ID, ID) = 5592,
    OpIAverageRoundedINTEL(IdType, IdResult, ID, ID) = 5593,
    OpUAverageRoundedINTEL(IdType, IdResult, ID, ID) = 5594,
    OpISubSatINTEL(IdType, IdResult, ID, ID) = 5595,
    OpUSubSatINTEL(IdType, IdResult, ID, ID) = 5596,
    OpIMul32x16INTEL(IdType, IdResult, ID, ID) = 5597,
    OpUMul32x16INTEL(IdType, IdResult, ID, ID) = 5598,
    OpConstantFunctionPointerINTEL(IdType, IdResult, ID) = 5600,
    OpFunctionPointerCallINTEL(IdType, IdResult, Vec<ID>) = 5601,
    OpAsmTargetINTEL(IdType, IdResult, String) = 5609,
    OpAsmINTEL(IdType, IdResult, ID, ID, String, String) = 5610,
    OpAsmCallINTEL(IdType, IdResult, ID, Vec<ID>) = 5611,
    OpAtomicFMinEXT(IdType, IdResult, ID, ID, ID, ID) = 5614,
    OpAtomicFMaxEXT(IdType, IdResult, ID, ID, ID, ID) = 5615,
    OpAssumeTrueKHR(ID) = 5630,
    OpExpectKHR(IdType, IdResult, ID, ID) = 5631,
    OpDecorateString(ID, Decoration) = 5632,
    OpMemberDecorateString(ID, u32, Decoration) = 5633,
    OpVmeImageINTEL(IdType, IdResult, ID, ID) = 5699,
    OpTypeVmeImageINTEL(IdResult, ID) = 5700,
    OpTypeAvcImePayloadINTEL(IdResult) = 5701,
    OpTypeAvcRefPayloadINTEL(IdResult) = 5702,
    OpTypeAvcSicPayloadINTEL(IdResult) = 5703,
    OpTypeAvcMcePayloadINTEL(IdResult) = 5704,
    OpTypeAvcMceResultINTEL(IdResult) = 5705,
    OpTypeAvcImeResultINTEL(IdResult) = 5706,
    OpTypeAvcImeResultSingleReferenceStreamoutINTEL(IdResult) = 5707,
    OpTypeAvcImeResultDualReferenceStreamoutINTEL(IdResult) = 5708,
    OpTypeAvcImeSingleReferenceStreaminINTEL(IdResult) = 5709,
    OpTypeAvcImeDualReferenceStreaminINTEL(IdResult) = 5710,
    OpTypeAvcRefResultINTEL(IdResult) = 5711,
    OpTypeAvcSicResultINTEL(IdResult) = 5712,
    OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(IdType, IdResult, ID, ID) = 5713,
    OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(IdType, IdResult, ID, ID) = 5714,
    OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(IdType, IdResult, ID, ID) = 5715,
    OpSubgroupAvcMceSetInterShapePenaltyINTEL(IdType, IdResult, ID, ID) = 5716,
    OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(IdType, IdResult, ID, ID) = 5717,
    OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(IdType, IdResult, ID, ID) = 5718,
    OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(IdType, IdResult, ID, ID) = 5719,
    OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(IdType, IdResult, ID, ID) = 5720,
    OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(IdType, IdResult) = 5721,
    OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(IdType, IdResult) = 5722,
    OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(IdType, IdResult) = 5723,
    OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(IdType, IdResult, ID, ID, ID, ID) = 5724,
    OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(IdType, IdResult, ID, ID) = 5725,
    OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(IdType, IdResult) = 5726,
    OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(IdType, IdResult) = 5727,
    OpSubgroupAvcMceSetAcOnlyHaarINTEL(IdType, IdResult, ID) = 5728,
    OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(IdType, IdResult, ID, ID) = 5729,
    OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(IdType, IdResult, ID, ID) = 5730,
    OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(IdType, IdResult, ID, ID, ID) =
        5731,
    OpSubgroupAvcMceConvertToImePayloadINTEL(IdType, IdResult, ID) = 5732,
    OpSubgroupAvcMceConvertToImeResultINTEL(IdType, IdResult, ID) = 5733,
    OpSubgroupAvcMceConvertToRefPayloadINTEL(IdType, IdResult, ID) = 5734,
    OpSubgroupAvcMceConvertToRefResultINTEL(IdType, IdResult, ID) = 5735,
    OpSubgroupAvcMceConvertToSicPayloadINTEL(IdType, IdResult, ID) = 5736,
    OpSubgroupAvcMceConvertToSicResultINTEL(IdType, IdResult, ID) = 5737,
    OpSubgroupAvcMceGetMotionVectorsINTEL(IdType, IdResult, ID) = 5738,
    OpSubgroupAvcMceGetInterDistortionsINTEL(IdType, IdResult, ID) = 5739,
    OpSubgroupAvcMceGetBestInterDistortionsINTEL(IdType, IdResult, ID) = 5740,
    OpSubgroupAvcMceGetInterMajorShapeINTEL(IdType, IdResult, ID) = 5741,
    OpSubgroupAvcMceGetInterMinorShapeINTEL(IdType, IdResult, ID) = 5742,
    OpSubgroupAvcMceGetInterDirectionsINTEL(IdType, IdResult, ID) = 5743,
    OpSubgroupAvcMceGetInterMotionVectorCountINTEL(IdType, IdResult, ID) = 5744,
    OpSubgroupAvcMceGetInterReferenceIdsINTEL(IdType, IdResult, ID) = 5745,
    OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(IdType, IdResult, ID, ID, ID) =
        5746,
    OpSubgroupAvcImeInitializeINTEL(IdType, IdResult, ID, ID, ID) = 5747,
    OpSubgroupAvcImeSetSingleReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5748,
    OpSubgroupAvcImeSetDualReferenceINTEL(IdType, IdResult, ID, ID, ID, ID) = 5749,
    OpSubgroupAvcImeRefWindowSizeINTEL(IdType, IdResult, ID, ID) = 5750,
    OpSubgroupAvcImeAdjustRefOffsetINTEL(IdType, IdResult, ID, ID, ID, ID) = 5751,
    OpSubgroupAvcImeConvertToMcePayloadINTEL(IdType, IdResult, ID) = 5752,
    OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(IdType, IdResult, ID, ID) = 5753,
    OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(IdType, IdResult, ID) = 5754,
    OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(IdType, IdResult, ID, ID) = 5755,
    OpSubgroupAvcImeSetWeightedSadINTEL(IdType, IdResult, ID, ID) = 5756,
    OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5757,
    OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(IdType, IdResult, ID, ID, ID, ID) = 5758,
    OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(IdType, IdResult, ID, ID, ID, ID) =
        5759,
    OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(IdType, IdResult, ID, ID, ID, ID, ID) =
        5760,
    OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(IdType, IdResult, ID, ID, ID) = 5761,
    OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(IdType, IdResult, ID, ID, ID, ID) =
        5762,
    OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(IdType, IdResult, ID, ID, ID, ID) =
        5763,
    OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(IdType, IdResult, ID, ID, ID, ID, ID) =
        5764,
    OpSubgroupAvcImeConvertToMceResultINTEL(IdType, IdResult, ID) = 5765,
    OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(IdType, IdResult, ID) = 5766,
    OpSubgroupAvcImeGetDualReferenceStreaminINTEL(IdType, IdResult, ID) = 5767,
    OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(IdType, IdResult, ID) = 5768,
    OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(IdType, IdResult, ID) = 5769,
    OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
        IdType,
        IdResult,
        ID,
        ID,
    ) = 5770,
    OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(IdType, IdResult, ID, ID) =
        5771,
    OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
        IdType,
        IdResult,
        ID,
        ID,
    ) = 5772,
    OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
        IdType,
        IdResult,
        ID,
        ID,
        ID,
    ) = 5773,
    OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
        IdType,
        IdResult,
        ID,
        ID,
        ID,
    ) = 5774,
    OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
        IdType,
        IdResult,
        ID,
        ID,
        ID,
    ) = 5775,
    OpSubgroupAvcImeGetBorderReachedINTEL(IdType, IdResult, ID, ID) = 5776,
    OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(IdType, IdResult, ID) = 5777,
    OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(IdType, IdResult, ID) = 5778,
    OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(IdType, IdResult, ID) = 5779,
    OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(IdType, IdResult, ID) = 5780,
    OpSubgroupAvcFmeInitializeINTEL(IdType, IdResult, ID, ID, ID, ID, ID, ID, ID) = 5781,
    OpSubgroupAvcBmeInitializeINTEL(IdType, IdResult, ID, ID, ID, ID, ID, ID, ID, ID) = 5782,
    OpSubgroupAvcRefConvertToMcePayloadINTEL(IdType, IdResult, ID) = 5783,
    OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(IdType, IdResult, ID) = 5784,
    OpSubgroupAvcRefSetBilinearFilterEnableINTEL(IdType, IdResult, ID) = 5785,
    OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5786,
    OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(IdType, IdResult, ID, ID, ID, ID) = 5787,
    OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5788,
    OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(IdType, IdResult, ID, ID, ID, ID) =
        5789,
    OpSubgroupAvcRefConvertToMceResultINTEL(IdType, IdResult, ID) = 5790,
    OpSubgroupAvcSicInitializeINTEL(IdType, IdResult, ID) = 5791,
    OpSubgroupAvcSicConfigureSkcINTEL(IdType, IdResult, ID, ID, ID, ID, ID, ID) = 5792,
    OpSubgroupAvcSicConfigureIpeLumaINTEL(IdType, IdResult, ID, ID, ID, ID, ID, ID, ID, ID) = 5793,
    OpSubgroupAvcSicConfigureIpeLumaChromaINTEL(
        IdType,
        IdResult,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
        ID,
    ) = 5794,
    OpSubgroupAvcSicGetMotionVectorMaskINTEL(IdType, IdResult, ID, ID) = 5795,
    OpSubgroupAvcSicConvertToMcePayloadINTEL(IdType, IdResult, ID) = 5796,
    OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(IdType, IdResult, ID, ID) = 5797,
    OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(IdType, IdResult, ID, ID, ID, ID) = 5798,
    OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(IdType, IdResult, ID, ID) = 5799,
    OpSubgroupAvcSicSetBilinearFilterEnableINTEL(IdType, IdResult, ID) = 5800,
    OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(IdType, IdResult, ID, ID) = 5801,
    OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(IdType, IdResult, ID, ID) = 5802,
    OpSubgroupAvcSicEvaluateIpeINTEL(IdType, IdResult, ID, ID) = 5803,
    OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5804,
    OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(IdType, IdResult, ID, ID, ID, ID) = 5805,
    OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(IdType, IdResult, ID, ID, ID) = 5806,
    OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(IdType, IdResult, ID, ID, ID, ID) =
        5807,
    OpSubgroupAvcSicConvertToMceResultINTEL(IdType, IdResult, ID) = 5808,
    OpSubgroupAvcSicGetIpeLumaShapeINTEL(IdType, IdResult, ID) = 5809,
    OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(IdType, IdResult, ID) = 5810,
    OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(IdType, IdResult, ID) = 5811,
    OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(IdType, IdResult, ID) = 5812,
    OpSubgroupAvcSicGetIpeChromaModeINTEL(IdType, IdResult, ID) = 5813,
    OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(IdType, IdResult, ID) = 5814,
    OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(IdType, IdResult, ID) = 5815,
    OpSubgroupAvcSicGetInterRawSadsINTEL(IdType, IdResult, ID) = 5816,
    OpVariableLengthArrayINTEL(IdType, IdResult, ID) = 5818,
    OpSaveMemoryINTEL(IdType, IdResult) = 5819,
    OpRestoreMemoryINTEL(ID) = 5820,
    OpArbitraryFloatSinCosPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32, u32) = 5840,
    OpArbitraryFloatCastINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5841,
    OpArbitraryFloatCastFromIntINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5842,
    OpArbitraryFloatCastToIntINTEL(IdType, IdResult, ID, u32, u32, u32, u32) = 5843,
    OpArbitraryFloatAddINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5846,
    OpArbitraryFloatSubINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5847,
    OpArbitraryFloatMulINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5848,
    OpArbitraryFloatDivINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5849,
    OpArbitraryFloatGTINTEL(IdType, IdResult, ID, u32, ID, u32) = 5850,
    OpArbitraryFloatGEINTEL(IdType, IdResult, ID, u32, ID, u32) = 5851,
    OpArbitraryFloatLTINTEL(IdType, IdResult, ID, u32, ID, u32) = 5852,
    OpArbitraryFloatLEINTEL(IdType, IdResult, ID, u32, ID, u32) = 5853,
    OpArbitraryFloatEQINTEL(IdType, IdResult, ID, u32, ID, u32) = 5854,
    OpArbitraryFloatRecipINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5855,
    OpArbitraryFloatRSqrtINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5856,
    OpArbitraryFloatCbrtINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5857,
    OpArbitraryFloatHypotINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5858,
    OpArbitraryFloatSqrtINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5859,
    OpArbitraryFloatLogINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5860,
    OpArbitraryFloatLog2INTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5861,
    OpArbitraryFloatLog10INTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5862,
    OpArbitraryFloatLog1pINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5863,
    OpArbitraryFloatExpINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5864,
    OpArbitraryFloatExp2INTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5865,
    OpArbitraryFloatExp10INTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5866,
    OpArbitraryFloatExpm1INTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5867,
    OpArbitraryFloatSinINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5868,
    OpArbitraryFloatCosINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5869,
    OpArbitraryFloatSinCosINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5870,
    OpArbitraryFloatSinPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5871,
    OpArbitraryFloatCosPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5872,
    OpArbitraryFloatASinINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5873,
    OpArbitraryFloatASinPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5874,
    OpArbitraryFloatACosINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5875,
    OpArbitraryFloatACosPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5876,
    OpArbitraryFloatATanINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5877,
    OpArbitraryFloatATanPiINTEL(IdType, IdResult, ID, u32, u32, u32, u32, u32) = 5878,
    OpArbitraryFloatATan2INTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5879,
    OpArbitraryFloatPowINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5880,
    OpArbitraryFloatPowRINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32, u32) = 5881,
    OpArbitraryFloatPowNINTEL(IdType, IdResult, ID, u32, ID, u32, u32, u32, u32) = 5882,
    OpLoopControlINTEL(Vec<u32>) = 5887,
    OpAliasDomainDeclINTEL(IdResult, Option<ID>) = 5911,
    OpAliasScopeDeclINTEL(IdResult, ID, Option<ID>) = 5912,
    OpAliasScopeListDeclINTEL(IdResult, Vec<ID>) = 5913,
    OpFixedSqrtINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5923,
    OpFixedRecipINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5924,
    OpFixedRsqrtINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5925,
    OpFixedSinINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5926,
    OpFixedCosINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5927,
    OpFixedSinCosINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5928,
    OpFixedSinPiINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5929,
    OpFixedCosPiINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5930,
    OpFixedSinCosPiINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5931,
    OpFixedLogINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5932,
    OpFixedExpINTEL(IdType, IdResult, ID, ID, u32, u32, u32, u32, u32) = 5933,
    OpPtrCastToCrossWorkgroupINTEL(IdType, IdResult, ID) = 5934,
    OpCrossWorkgroupCastToPtrINTEL(IdType, IdResult, ID) = 5938,
    OpReadPipeBlockingINTEL(IdType, IdResult, ID, ID) = 5946,
    OpWritePipeBlockingINTEL(IdType, IdResult, ID, ID) = 5947,
    OpFPGARegINTEL(IdType, IdResult, ID, ID) = 5949,
    OpRayQueryGetRayTMinKHR(IdType, IdResult, ID) = 6016,
    OpRayQueryGetRayFlagsKHR(IdType, IdResult, ID) = 6017,
    OpRayQueryGetIntersectionTKHR(IdType, IdResult, ID, ID) = 6018,
    OpRayQueryGetIntersectionInstanceCustomIndexKHR(IdType, IdResult, ID, ID) = 6019,
    OpRayQueryGetIntersectionInstanceIdKHR(IdType, IdResult, ID, ID) = 6020,
    OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(IdType, IdResult, ID, ID) =
        6021,
    OpRayQueryGetIntersectionGeometryIndexKHR(IdType, IdResult, ID, ID) = 6022,
    OpRayQueryGetIntersectionPrimitiveIndexKHR(IdType, IdResult, ID, ID) = 6023,
    OpRayQueryGetIntersectionBarycentricsKHR(IdType, IdResult, ID, ID) = 6024,
    OpRayQueryGetIntersectionFrontFaceKHR(IdType, IdResult, ID, ID) = 6025,
    OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(IdType, IdResult, ID) = 6026,
    OpRayQueryGetIntersectionObjectRayDirectionKHR(IdType, IdResult, ID, ID) = 6027,
    OpRayQueryGetIntersectionObjectRayOriginKHR(IdType, IdResult, ID, ID) = 6028,
    OpRayQueryGetWorldRayDirectionKHR(IdType, IdResult, ID) = 6029,
    OpRayQueryGetWorldRayOriginKHR(IdType, IdResult, ID) = 6030,
    OpRayQueryGetIntersectionObjectToWorldKHR(IdType, IdResult, ID, ID) = 6031,
    OpRayQueryGetIntersectionWorldToObjectKHR(IdType, IdResult, ID, ID) = 6032,
    OpAtomicFAddEXT(IdType, IdResult, ID, ID, ID, ID) = 6035,
    OpTypeBufferSurfaceINTEL(IdResult, AccessQualifier) = 6086,
    OpTypeStructContinuedINTEL(Vec<ID>) = 6090,
    OpConstantCompositeContinuedINTEL(Vec<ID>) = 6091,
    OpSpecConstantCompositeContinuedINTEL(Vec<ID>) = 6092,
    OpControlBarrierArriveINTEL(ID, ID, ID) = 6142,
    OpControlBarrierWaitINTEL(ID, ID, ID) = 6143,
    OpGroupIMulKHR(IdType, IdResult, ID, GroupOperation, ID) = 6401,
    OpGroupFMulKHR(IdType, IdResult, ID, GroupOperation, ID) = 6402,
    OpGroupBitwiseAndKHR(IdType, IdResult, ID, GroupOperation, ID) = 6403,
    OpGroupBitwiseOrKHR(IdType, IdResult, ID, GroupOperation, ID) = 6404,
    OpGroupBitwiseXorKHR(IdType, IdResult, ID, GroupOperation, ID) = 6405,
    OpGroupLogicalAndKHR(IdType, IdResult, ID, GroupOperation, ID) = 6406,
    OpGroupLogicalOrKHR(IdType, IdResult, ID, GroupOperation, ID) = 6407,
    OpGroupLogicalXorKHR(IdType, IdResult, ID, GroupOperation, ID) = 6408,
}
pub type ImageOperands = BitEnum<ImageOperandsBits>;
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ImageOperandsBits {
    #[default]
    None = 0,
    Bias(ID) = 1,
    Lod(ID) = 2,
    Grad(ID, ID) = 4,
    ConstOffset(ID) = 8,
    Offset(ID) = 16,
    ConstOffsets(ID) = 32,
    Sample(ID) = 64,
    MinLod(ID) = 128,
    MakeTexelAvailable(ID) = 256,
    MakeTexelVisible(ID) = 512,
    NonPrivateTexel = 1024,
    VolatileTexel = 2048,
    SignExtend = 4096,
    ZeroExtend = 8192,
    Nontemporal = 16384,
    Offsets(ID) = 65536,
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct FPFastMathMode(pub u32);
impl FPFastMathMode {
    pub const None: Self = Self(0);
    pub const NotNaN: Self = Self(1);
    pub const NotInf: Self = Self(2);
    pub const NSZ: Self = Self(4);
    pub const AllowRecip: Self = Self(8);
    pub const Fast: Self = Self(16);
    pub const AllowContractFastINTEL: Self = Self(65536);
    pub const AllowReassocINTEL: Self = Self(131072);
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct SelectionControl(pub u32);
impl SelectionControl {
    pub const None: Self = Self(0);
    pub const Flatten: Self = Self(1);
    pub const DontFlatten: Self = Self(2);
}
pub type LoopControl = BitEnum<LoopControlBits>;
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum LoopControlBits {
    #[default]
    None = 0,
    Unroll = 1,
    DontUnroll = 2,
    DependencyInfinite = 4,
    DependencyLength(u32) = 8,
    MinIterations(u32) = 16,
    MaxIterations(u32) = 32,
    IterationMultiple(u32) = 64,
    PeelCount(u32) = 128,
    PartialCount(u32) = 256,
    InitiationIntervalINTEL(u32) = 65536,
    MaxConcurrencyINTEL(u32) = 131072,
    DependencyArrayINTEL(u32) = 262144,
    PipelineEnableINTEL(u32) = 524288,
    LoopCoalesceINTEL(u32) = 1048576,
    MaxInterleavingINTEL(u32) = 2097152,
    SpeculatedIterationsINTEL(u32) = 4194304,
    NoFusionINTEL(u32) = 8388608,
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct FunctionControl(pub u32);
impl FunctionControl {
    pub const None: Self = Self(0);
    pub const Inline: Self = Self(1);
    pub const DontInline: Self = Self(2);
    pub const Pure: Self = Self(4);
    pub const Const: Self = Self(8);
    pub const OptNoneINTEL: Self = Self(65536);
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct MemorySemantics(pub u32);
impl MemorySemantics {
    pub const Relaxed: Self = Self(0);
    pub const Acquire: Self = Self(2);
    pub const Release: Self = Self(4);
    pub const AcquireRelease: Self = Self(8);
    pub const SequentiallyConsistent: Self = Self(16);
    pub const UniformMemory: Self = Self(64);
    pub const SubgroupMemory: Self = Self(128);
    pub const WorkgroupMemory: Self = Self(256);
    pub const CrossWorkgroupMemory: Self = Self(512);
    pub const AtomicCounterMemory: Self = Self(1024);
    pub const ImageMemory: Self = Self(2048);
    pub const OutputMemory: Self = Self(4096);
    pub const MakeAvailable: Self = Self(8192);
    pub const MakeVisible: Self = Self(16384);
    pub const Volatile: Self = Self(32768);
}
pub type MemoryAccess = BitEnum<MemoryAccessBits>;
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum MemoryAccessBits {
    #[default]
    None = 0,
    Volatile = 1,
    Aligned(u32) = 2,
    Nontemporal = 4,
    MakePointerAvailable(ID) = 8,
    MakePointerVisible(ID) = 16,
    NonPrivatePointer = 32,
    AliasScopeINTELMask(ID) = 65536,
    NoAliasINTELMask(ID) = 131072,
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct KernelProfilingInfo(pub u32);
impl KernelProfilingInfo {
    pub const None: Self = Self(0);
    pub const CmdExecTime: Self = Self(1);
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct RayFlags(pub u32);
impl RayFlags {
    pub const NoneKHR: Self = Self(0);
    pub const OpaqueKHR: Self = Self(1);
    pub const NoOpaqueKHR: Self = Self(2);
    pub const TerminateOnFirstHitKHR: Self = Self(4);
    pub const SkipClosestHitShaderKHR: Self = Self(8);
    pub const CullBackFacingTrianglesKHR: Self = Self(16);
    pub const CullFrontFacingTrianglesKHR: Self = Self(32);
    pub const CullOpaqueKHR: Self = Self(64);
    pub const CullNoOpaqueKHR: Self = Self(128);
    pub const SkipTrianglesKHR: Self = Self(256);
    pub const SkipAABBsKHR: Self = Self(512);
}
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct FragmentShadingRate(pub u32);
impl FragmentShadingRate {
    pub const Vertical2Pixels: Self = Self(1);
    pub const Vertical4Pixels: Self = Self(2);
    pub const Horizontal2Pixels: Self = Self(4);
    pub const Horizontal4Pixels: Self = Self(8);
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum SourceLanguage {
    Unknown = 0,
    ESSL = 1,
    GLSL = 2,
    OpenCL_C = 3,
    OpenCL_CPP = 4,
    HLSL = 5,
    CPP_for_OpenCL = 6,
    SYCL = 7,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ExecutionModel {
    Vertex = 0,
    TessellationControl = 1,
    TessellationEvaluation = 2,
    Geometry = 3,
    Fragment = 4,
    GLCompute = 5,
    Kernel = 6,
    TaskNV = 5267,
    MeshNV = 5268,
    RayGenerationNV = 5313,
    IntersectionNV = 5314,
    AnyHitNV = 5315,
    ClosestHitNV = 5316,
    MissNV = 5317,
    CallableNV = 5318,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum AddressingModel {
    Logical = 0,
    Physical32 = 1,
    Physical64 = 2,
    PhysicalStorageBuffer64 = 5348,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum MemoryModel {
    Simple = 0,
    GLSL450 = 1,
    OpenCL = 2,
    Vulkan = 3,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ExecutionMode {
    Invocations(u32) = 0,
    SpacingEqual = 1,
    SpacingFractionalEven = 2,
    SpacingFractionalOdd = 3,
    VertexOrderCw = 4,
    VertexOrderCcw = 5,
    PixelCenterInteger = 6,
    OriginUpperLeft = 7,
    OriginLowerLeft = 8,
    EarlyFragmentTests = 9,
    PointMode = 10,
    Xfb = 11,
    DepthReplacing = 12,
    DepthGreater = 14,
    DepthLess = 15,
    DepthUnchanged = 16,
    LocalSize(u32, u32, u32) = 17,
    LocalSizeHint(u32, u32, u32) = 18,
    InputPoints = 19,
    InputLines = 20,
    InputLinesAdjacency = 21,
    Triangles = 22,
    InputTrianglesAdjacency = 23,
    Quads = 24,
    Isolines = 25,
    OutputVertices(u32) = 26,
    OutputPoints = 27,
    OutputLineStrip = 28,
    OutputTriangleStrip = 29,
    VecTypeHint(u32) = 30,
    ContractionOff = 31,
    Initializer = 33,
    Finalizer = 34,
    SubgroupSize(u32) = 35,
    SubgroupsPerWorkgroup(u32) = 36,
    SubgroupsPerWorkgroupId(ID) = 37,
    LocalSizeId(ID, ID, ID) = 38,
    LocalSizeHintId(ID, ID, ID) = 39,
    SubgroupUniformControlFlowKHR = 4421,
    PostDepthCoverage = 4446,
    DenormPreserve(u32) = 4459,
    DenormFlushToZero(u32) = 4460,
    SignedZeroInfNanPreserve(u32) = 4461,
    RoundingModeRTE(u32) = 4462,
    RoundingModeRTZ(u32) = 4463,
    StencilRefReplacingEXT = 5027,
    OutputLinesNV = 5269,
    OutputPrimitivesNV(u32) = 5270,
    DerivativeGroupQuadsNV = 5289,
    DerivativeGroupLinearNV = 5290,
    OutputTrianglesNV = 5298,
    PixelInterlockOrderedEXT = 5366,
    PixelInterlockUnorderedEXT = 5367,
    SampleInterlockOrderedEXT = 5368,
    SampleInterlockUnorderedEXT = 5369,
    ShadingRateInterlockOrderedEXT = 5370,
    ShadingRateInterlockUnorderedEXT = 5371,
    SharedLocalMemorySizeINTEL(u32) = 5618,
    RoundingModeRTPINTEL(u32) = 5620,
    RoundingModeRTNINTEL(u32) = 5621,
    FloatingPointModeALTINTEL(u32) = 5622,
    FloatingPointModeIEEEINTEL(u32) = 5623,
    MaxWorkgroupSizeINTEL(u32, u32, u32) = 5893,
    MaxWorkDimINTEL(u32) = 5894,
    NoGlobalOffsetINTEL = 5895,
    NumSIMDWorkitemsINTEL(u32) = 5896,
    SchedulerTargetFmaxMhzINTEL(u32) = 5903,
    NamedBarrierCountINTEL(u32) = 6417,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum StorageClass {
    UniformConstant = 0,
    Input = 1,
    Uniform = 2,
    Output = 3,
    Workgroup = 4,
    CrossWorkgroup = 5,
    Private = 6,
    Function = 7,
    Generic = 8,
    PushConstant = 9,
    AtomicCounter = 10,
    Image = 11,
    StorageBuffer = 12,
    CallableDataNV = 5328,
    IncomingCallableDataNV = 5329,
    RayPayloadNV = 5338,
    HitAttributeNV = 5339,
    IncomingRayPayloadNV = 5342,
    ShaderRecordBufferNV = 5343,
    PhysicalStorageBuffer = 5349,
    CodeSectionINTEL = 5605,
    DeviceOnlyINTEL = 5936,
    HostOnlyINTEL = 5937,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Dim {
    _1D = 0,
    _2D = 1,
    _3D = 2,
    Cube = 3,
    Rect = 4,
    Buffer = 5,
    SubpassData = 6,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum SamplerAddressingMode {
    None = 0,
    ClampToEdge = 1,
    Clamp = 2,
    Repeat = 3,
    RepeatMirrored = 4,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum SamplerFilterMode {
    Nearest = 0,
    Linear = 1,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ImageFormat {
    Unknown = 0,
    Rgba32f = 1,
    Rgba16f = 2,
    R32f = 3,
    Rgba8 = 4,
    Rgba8Snorm = 5,
    Rg32f = 6,
    Rg16f = 7,
    R11fG11fB10f = 8,
    R16f = 9,
    Rgba16 = 10,
    Rgb10A2 = 11,
    Rg16 = 12,
    Rg8 = 13,
    R16 = 14,
    R8 = 15,
    Rgba16Snorm = 16,
    Rg16Snorm = 17,
    Rg8Snorm = 18,
    R16Snorm = 19,
    R8Snorm = 20,
    Rgba32i = 21,
    Rgba16i = 22,
    Rgba8i = 23,
    R32i = 24,
    Rg32i = 25,
    Rg16i = 26,
    Rg8i = 27,
    R16i = 28,
    R8i = 29,
    Rgba32ui = 30,
    Rgba16ui = 31,
    Rgba8ui = 32,
    R32ui = 33,
    Rgb10a2ui = 34,
    Rg32ui = 35,
    Rg16ui = 36,
    Rg8ui = 37,
    R16ui = 38,
    R8ui = 39,
    R64ui = 40,
    R64i = 41,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ImageChannelOrder {
    R = 0,
    A = 1,
    RG = 2,
    RA = 3,
    RGB = 4,
    RGBA = 5,
    BGRA = 6,
    ARGB = 7,
    Intensity = 8,
    Luminance = 9,
    Rx = 10,
    RGx = 11,
    RGBx = 12,
    Depth = 13,
    DepthStencil = 14,
    sRGB = 15,
    sRGBx = 16,
    sRGBA = 17,
    sBGRA = 18,
    ABGR = 19,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum ImageChannelDataType {
    SnormInt8 = 0,
    SnormInt16 = 1,
    UnormInt8 = 2,
    UnormInt16 = 3,
    UnormShort565 = 4,
    UnormShort555 = 5,
    UnormInt101010 = 6,
    SignedInt8 = 7,
    SignedInt16 = 8,
    SignedInt32 = 9,
    UnsignedInt8 = 10,
    UnsignedInt16 = 11,
    UnsignedInt32 = 12,
    HalfFloat = 13,
    Float = 14,
    UnormInt24 = 15,
    UnormInt101010_2 = 16,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum FPRoundingMode {
    RTE = 0,
    RTZ = 1,
    RTP = 2,
    RTN = 3,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum FPDenormMode {
    Preserve = 0,
    FlushToZero = 1,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum QuantizationModes {
    TRN = 0,
    TRN_ZERO = 1,
    RND = 2,
    RND_ZERO = 3,
    RND_INF = 4,
    RND_MIN_INF = 5,
    RND_CONV = 6,
    RND_CONV_ODD = 7,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum FPOperationMode {
    IEEE = 0,
    ALT = 1,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum OverflowModes {
    WRAP = 0,
    SAT = 1,
    SAT_ZERO = 2,
    SAT_SYM = 3,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum LinkageType {
    Export = 0,
    Import = 1,
    LinkOnceODR = 2,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum AccessQualifier {
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum FunctionParameterAttribute {
    Zext = 0,
    Sext = 1,
    ByVal = 2,
    Sret = 3,
    NoAlias = 4,
    NoCapture = 5,
    NoWrite = 6,
    NoReadWrite = 7,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Decoration {
    RelaxedPrecision = 0,
    SpecId(u32) = 1,
    Block = 2,
    BufferBlock = 3,
    RowMajor = 4,
    ColMajor = 5,
    ArrayStride(u32) = 6,
    MatrixStride(u32) = 7,
    GLSLShared = 8,
    GLSLPacked = 9,
    CPacked = 10,
    BuiltIn(BuiltIn) = 11,
    NoPerspective = 13,
    Flat = 14,
    Patch = 15,
    Centroid = 16,
    Sample = 17,
    Invariant = 18,
    Restrict = 19,
    Aliased = 20,
    Volatile = 21,
    Constant = 22,
    Coherent = 23,
    NonWritable = 24,
    NonReadable = 25,
    Uniform = 26,
    UniformId(ID) = 27,
    SaturatedConversion = 28,
    Stream(u32) = 29,
    Location(u32) = 30,
    Component(u32) = 31,
    Index(u32) = 32,
    Binding(u32) = 33,
    DescriptorSet(u32) = 34,
    Offset(u32) = 35,
    XfbBuffer(u32) = 36,
    XfbStride(u32) = 37,
    FuncParamAttr(FunctionParameterAttribute) = 38,
    FPRoundingMode(FPRoundingMode) = 39,
    FPFastMathMode(FPFastMathMode) = 40,
    LinkageAttributes(String, LinkageType) = 41,
    NoContraction = 42,
    InputAttachmentIndex(u32) = 43,
    Alignment(u32) = 44,
    MaxByteOffset(u32) = 45,
    AlignmentId(ID) = 46,
    MaxByteOffsetId(ID) = 47,
    NoSignedWrap = 4469,
    NoUnsignedWrap = 4470,
    ExplicitInterpAMD = 4999,
    OverrideCoverageNV = 5248,
    PassthroughNV = 5250,
    ViewportRelativeNV = 5252,
    SecondaryViewportRelativeNV(u32) = 5256,
    PerPrimitiveNV = 5271,
    PerViewNV = 5272,
    PerTaskNV = 5273,
    PerVertexKHR = 5285,
    NonUniform = 5300,
    RestrictPointer = 5355,
    AliasedPointer = 5356,
    BindlessSamplerNV = 5398,
    BindlessImageNV = 5399,
    BoundSamplerNV = 5400,
    BoundImageNV = 5401,
    SIMTCallINTEL(u32) = 5599,
    ReferencedIndirectlyINTEL = 5602,
    ClobberINTEL(String) = 5607,
    SideEffectsINTEL = 5608,
    VectorComputeVariableINTEL = 5624,
    FuncParamIOKindINTEL(u32) = 5625,
    VectorComputeFunctionINTEL = 5626,
    StackCallINTEL = 5627,
    GlobalVariableOffsetINTEL(u32) = 5628,
    CounterBuffer(ID) = 5634,
    UserSemantic(String) = 5635,
    UserTypeGOOGLE(String) = 5636,
    FunctionRoundingModeINTEL(u32, FPRoundingMode) = 5822,
    FunctionDenormModeINTEL(u32, FPDenormMode) = 5823,
    RegisterINTEL = 5825,
    MemoryINTEL(String) = 5826,
    NumbanksINTEL(u32) = 5827,
    BankwidthINTEL(u32) = 5828,
    MaxPrivateCopiesINTEL(u32) = 5829,
    SinglepumpINTEL = 5830,
    DoublepumpINTEL = 5831,
    MaxReplicatesINTEL(u32) = 5832,
    SimpleDualPortINTEL = 5833,
    MergeINTEL(String, String) = 5834,
    BankBitsINTEL(u32) = 5835,
    ForcePow2DepthINTEL(u32) = 5836,
    BurstCoalesceINTEL = 5899,
    CacheSizeINTEL(u32) = 5900,
    DontStaticallyCoalesceINTEL = 5901,
    PrefetchINTEL(u32) = 5902,
    StallEnableINTEL = 5905,
    FuseLoopsInFunctionINTEL = 5907,
    AliasScopeINTEL(ID) = 5914,
    NoAliasINTEL(ID) = 5915,
    BufferLocationINTEL(u32) = 5921,
    IOPipeStorageINTEL(u32) = 5944,
    FunctionFloatingPointModeINTEL(u32, FPOperationMode) = 6080,
    SingleElementVectorINTEL = 6085,
    VectorComputeCallableFunctionINTEL = 6087,
    MediaBlockIOINTEL = 6140,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum BuiltIn {
    Position = 0,
    PointSize = 1,
    ClipDistance = 3,
    CullDistance = 4,
    VertexId = 5,
    InstanceId = 6,
    PrimitiveId = 7,
    InvocationId = 8,
    Layer = 9,
    ViewportIndex = 10,
    TessLevelOuter = 11,
    TessLevelInner = 12,
    TessCoord = 13,
    PatchVertices = 14,
    FragCoord = 15,
    PointCoord = 16,
    FrontFacing = 17,
    SampleId = 18,
    SamplePosition = 19,
    SampleMask = 20,
    FragDepth = 22,
    HelperInvocation = 23,
    NumWorkgroups = 24,
    WorkgroupSize = 25,
    WorkgroupId = 26,
    LocalInvocationId = 27,
    GlobalInvocationId = 28,
    LocalInvocationIndex = 29,
    WorkDim = 30,
    GlobalSize = 31,
    EnqueuedWorkgroupSize = 32,
    GlobalOffset = 33,
    GlobalLinearId = 34,
    SubgroupSize = 36,
    SubgroupMaxSize = 37,
    NumSubgroups = 38,
    NumEnqueuedSubgroups = 39,
    SubgroupId = 40,
    SubgroupLocalInvocationId = 41,
    VertexIndex = 42,
    InstanceIndex = 43,
    SubgroupEqMask = 4416,
    SubgroupGeMask = 4417,
    SubgroupGtMask = 4418,
    SubgroupLeMask = 4419,
    SubgroupLtMask = 4420,
    BaseVertex = 4424,
    BaseInstance = 4425,
    DrawIndex = 4426,
    PrimitiveShadingRateKHR = 4432,
    DeviceIndex = 4438,
    ViewIndex = 4440,
    ShadingRateKHR = 4444,
    BaryCoordNoPerspAMD = 4992,
    BaryCoordNoPerspCentroidAMD = 4993,
    BaryCoordNoPerspSampleAMD = 4994,
    BaryCoordSmoothAMD = 4995,
    BaryCoordSmoothCentroidAMD = 4996,
    BaryCoordSmoothSampleAMD = 4997,
    BaryCoordPullModelAMD = 4998,
    FragStencilRefEXT = 5014,
    ViewportMaskNV = 5253,
    SecondaryPositionNV = 5257,
    SecondaryViewportMaskNV = 5258,
    PositionPerViewNV = 5261,
    ViewportMaskPerViewNV = 5262,
    FullyCoveredEXT = 5264,
    TaskCountNV = 5274,
    PrimitiveCountNV = 5275,
    PrimitiveIndicesNV = 5276,
    ClipDistancePerViewNV = 5277,
    CullDistancePerViewNV = 5278,
    LayerPerViewNV = 5279,
    MeshViewCountNV = 5280,
    MeshViewIndicesNV = 5281,
    BaryCoordKHR = 5286,
    BaryCoordNoPerspKHR = 5287,
    FragSizeEXT = 5292,
    FragInvocationCountEXT = 5293,
    LaunchIdNV = 5319,
    LaunchSizeNV = 5320,
    WorldRayOriginNV = 5321,
    WorldRayDirectionNV = 5322,
    ObjectRayOriginNV = 5323,
    ObjectRayDirectionNV = 5324,
    RayTminNV = 5325,
    RayTmaxNV = 5326,
    InstanceCustomIndexNV = 5327,
    ObjectToWorldNV = 5330,
    WorldToObjectNV = 5331,
    HitTNV = 5332,
    HitKindNV = 5333,
    CurrentRayTimeNV = 5334,
    IncomingRayFlagsNV = 5351,
    RayGeometryIndexKHR = 5352,
    WarpsPerSMNV = 5374,
    SMCountNV = 5375,
    WarpIDNV = 5376,
    SMIDNV = 5377,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Scope {
    CrossDevice = 0,
    Device = 1,
    Workgroup = 2,
    Subgroup = 3,
    Invocation = 4,
    QueueFamily = 5,
    ShaderCallKHR = 6,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum GroupOperation {
    Reduce = 0,
    InclusiveScan = 1,
    ExclusiveScan = 2,
    ClusteredReduce = 3,
    PartitionedReduceNV = 6,
    PartitionedInclusiveScanNV = 7,
    PartitionedExclusiveScanNV = 8,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum KernelEnqueueFlags {
    NoWait = 0,
    WaitKernel = 1,
    WaitWorkGroup = 2,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum Capability {
    Matrix = 0,
    Shader = 1,
    Geometry = 2,
    Tessellation = 3,
    Addresses = 4,
    Linkage = 5,
    Kernel = 6,
    Vector16 = 7,
    Float16Buffer = 8,
    Float16 = 9,
    Float64 = 10,
    Int64 = 11,
    Int64Atomics = 12,
    ImageBasic = 13,
    ImageReadWrite = 14,
    ImageMipmap = 15,
    Pipes = 17,
    Groups = 18,
    DeviceEnqueue = 19,
    LiteralSampler = 20,
    AtomicStorage = 21,
    Int16 = 22,
    TessellationPointSize = 23,
    GeometryPointSize = 24,
    ImageGatherExtended = 25,
    StorageImageMultisample = 27,
    UniformBufferArrayDynamicIndexing = 28,
    SampledImageArrayDynamicIndexing = 29,
    StorageBufferArrayDynamicIndexing = 30,
    StorageImageArrayDynamicIndexing = 31,
    ClipDistance = 32,
    CullDistance = 33,
    ImageCubeArray = 34,
    SampleRateShading = 35,
    ImageRect = 36,
    SampledRect = 37,
    GenericPointer = 38,
    Int8 = 39,
    InputAttachment = 40,
    SparseResidency = 41,
    MinLod = 42,
    Sampled1D = 43,
    Image1D = 44,
    SampledCubeArray = 45,
    SampledBuffer = 46,
    ImageBuffer = 47,
    ImageMSArray = 48,
    StorageImageExtendedFormats = 49,
    ImageQuery = 50,
    DerivativeControl = 51,
    InterpolationFunction = 52,
    TransformFeedback = 53,
    GeometryStreams = 54,
    StorageImageReadWithoutFormat = 55,
    StorageImageWriteWithoutFormat = 56,
    MultiViewport = 57,
    SubgroupDispatch = 58,
    NamedBarrier = 59,
    PipeStorage = 60,
    GroupNonUniform = 61,
    GroupNonUniformVote = 62,
    GroupNonUniformArithmetic = 63,
    GroupNonUniformBallot = 64,
    GroupNonUniformShuffle = 65,
    GroupNonUniformShuffleRelative = 66,
    GroupNonUniformClustered = 67,
    GroupNonUniformQuad = 68,
    ShaderLayer = 69,
    ShaderViewportIndex = 70,
    UniformDecoration = 71,
    FragmentShadingRateKHR = 4422,
    SubgroupBallotKHR = 4423,
    DrawParameters = 4427,
    WorkgroupMemoryExplicitLayoutKHR = 4428,
    WorkgroupMemoryExplicitLayout8BitAccessKHR = 4429,
    WorkgroupMemoryExplicitLayout16BitAccessKHR = 4430,
    SubgroupVoteKHR = 4431,
    StorageBuffer16BitAccess = 4433,
    UniformAndStorageBuffer16BitAccess = 4434,
    StoragePushConstant16 = 4435,
    StorageInputOutput16 = 4436,
    DeviceGroup = 4437,
    MultiView = 4439,
    VariablePointersStorageBuffer = 4441,
    VariablePointers = 4442,
    AtomicStorageOps = 4445,
    SampleMaskPostDepthCoverage = 4447,
    StorageBuffer8BitAccess = 4448,
    UniformAndStorageBuffer8BitAccess = 4449,
    StoragePushConstant8 = 4450,
    DenormPreserve = 4464,
    DenormFlushToZero = 4465,
    SignedZeroInfNanPreserve = 4466,
    RoundingModeRTE = 4467,
    RoundingModeRTZ = 4468,
    RayQueryProvisionalKHR = 4471,
    RayQueryKHR = 4472,
    RayTraversalPrimitiveCullingKHR = 4478,
    RayTracingKHR = 4479,
    Float16ImageAMD = 5008,
    ImageGatherBiasLodAMD = 5009,
    FragmentMaskAMD = 5010,
    StencilExportEXT = 5013,
    ImageReadWriteLodAMD = 5015,
    Int64ImageEXT = 5016,
    ShaderClockKHR = 5055,
    SampleMaskOverrideCoverageNV = 5249,
    GeometryShaderPassthroughNV = 5251,
    ShaderViewportIndexLayerEXT = 5254,
    ShaderViewportMaskNV = 5255,
    ShaderStereoViewNV = 5259,
    PerViewAttributesNV = 5260,
    FragmentFullyCoveredEXT = 5265,
    MeshShadingNV = 5266,
    ImageFootprintNV = 5282,
    FragmentBarycentricKHR = 5284,
    ComputeDerivativeGroupQuadsNV = 5288,
    FragmentDensityEXT = 5291,
    GroupNonUniformPartitionedNV = 5297,
    ShaderNonUniform = 5301,
    RuntimeDescriptorArray = 5302,
    InputAttachmentArrayDynamicIndexing = 5303,
    UniformTexelBufferArrayDynamicIndexing = 5304,
    StorageTexelBufferArrayDynamicIndexing = 5305,
    UniformBufferArrayNonUniformIndexing = 5306,
    SampledImageArrayNonUniformIndexing = 5307,
    StorageBufferArrayNonUniformIndexing = 5308,
    StorageImageArrayNonUniformIndexing = 5309,
    InputAttachmentArrayNonUniformIndexing = 5310,
    UniformTexelBufferArrayNonUniformIndexing = 5311,
    StorageTexelBufferArrayNonUniformIndexing = 5312,
    RayTracingNV = 5340,
    RayTracingMotionBlurNV = 5341,
    VulkanMemoryModel = 5345,
    VulkanMemoryModelDeviceScope = 5346,
    PhysicalStorageBufferAddresses = 5347,
    ComputeDerivativeGroupLinearNV = 5350,
    RayTracingProvisionalKHR = 5353,
    CooperativeMatrixNV = 5357,
    FragmentShaderSampleInterlockEXT = 5363,
    FragmentShaderShadingRateInterlockEXT = 5372,
    ShaderSMBuiltinsNV = 5373,
    FragmentShaderPixelInterlockEXT = 5378,
    DemoteToHelperInvocation = 5379,
    BindlessTextureNV = 5390,
    SubgroupShuffleINTEL = 5568,
    SubgroupBufferBlockIOINTEL = 5569,
    SubgroupImageBlockIOINTEL = 5570,
    SubgroupImageMediaBlockIOINTEL = 5579,
    RoundToInfinityINTEL = 5582,
    FloatingPointModeINTEL = 5583,
    IntegerFunctions2INTEL = 5584,
    FunctionPointersINTEL = 5603,
    IndirectReferencesINTEL = 5604,
    AsmINTEL = 5606,
    AtomicFloat32MinMaxEXT = 5612,
    AtomicFloat64MinMaxEXT = 5613,
    AtomicFloat16MinMaxEXT = 5616,
    VectorComputeINTEL = 5617,
    VectorAnyINTEL = 5619,
    ExpectAssumeKHR = 5629,
    SubgroupAvcMotionEstimationINTEL = 5696,
    SubgroupAvcMotionEstimationIntraINTEL = 5697,
    SubgroupAvcMotionEstimationChromaINTEL = 5698,
    VariableLengthArrayINTEL = 5817,
    FunctionFloatControlINTEL = 5821,
    FPGAMemoryAttributesINTEL = 5824,
    FPFastMathModeINTEL = 5837,
    ArbitraryPrecisionIntegersINTEL = 5844,
    ArbitraryPrecisionFloatingPointINTEL = 5845,
    UnstructuredLoopControlsINTEL = 5886,
    FPGALoopControlsINTEL = 5888,
    KernelAttributesINTEL = 5892,
    FPGAKernelAttributesINTEL = 5897,
    FPGAMemoryAccessesINTEL = 5898,
    FPGAClusterAttributesINTEL = 5904,
    LoopFuseINTEL = 5906,
    MemoryAccessAliasingINTEL = 5910,
    FPGABufferLocationINTEL = 5920,
    ArbitraryPrecisionFixedPointINTEL = 5922,
    USMStorageClassesINTEL = 5935,
    IOPipesINTEL = 5943,
    BlockingPipesINTEL = 5945,
    FPGARegINTEL = 5948,
    DotProductInputAll = 6016,
    DotProductInput4x8Bit = 6017,
    DotProductInput4x8BitPacked = 6018,
    DotProduct = 6019,
    BitInstructions = 6025,
    AtomicFloat32AddEXT = 6033,
    AtomicFloat64AddEXT = 6034,
    LongConstantCompositeINTEL = 6089,
    OptNoneINTEL = 6094,
    AtomicFloat16AddEXT = 6095,
    DebugInfoModuleINTEL = 6114,
    SplitBarrierINTEL = 6141,
    GroupUniformArithmeticKHR = 6400,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum RayQueryIntersection {
    RayQueryCandidateIntersectionKHR = 0,
    RayQueryCommittedIntersectionKHR = 1,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum RayQueryCommittedIntersectionType {
    RayQueryCommittedIntersectionNoneKHR = 0,
    RayQueryCommittedIntersectionTriangleKHR = 1,
    RayQueryCommittedIntersectionGeneratedKHR = 2,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum RayQueryCandidateIntersectionType {
    RayQueryCandidateIntersectionTriangleKHR = 0,
    RayQueryCandidateIntersectionAABBKHR = 1,
    #[default]
    INVALID = u32::MAX,
}
#[repr(u32)]
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum PackedVectorFormat {
    PackedVectorFormat4x8Bit = 0,
    #[default]
    INVALID = u32::MAX,
}
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ID(pub u32);
pub type IdResult = ID;
pub type IdType = ID;
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub struct BitEnum<T: BitField> {
    fields: Vec<T>,
}
impl<T: BitField> Asm for BitEnum<T> {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        let opc_idx = sink.len();
        sink.push(0);
        let mut fields = self.fields.iter().collect::<Vec<&T>>();
        fields.sort_by(|a, b| a.opcode().cmp(&b.opcode()));
        for field in &self.fields {
            field.write_word(opc_idx, sink, req);
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let mut opc = chunk[*idx];
        *idx += 1;
        let mut fields = vec![];
        while opc != 0 {
            fields.push(T::read_word(chunk, &mut opc, idx))
        }
        Self { fields }
    }
}
pub trait BitField {
    fn opcode(&self) -> u32;
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>, req: &mut ModuleRequirements);
    fn read_word(chunk: &[u32], opc: &mut u32, idx: &mut usize) -> Self;
}
pub trait Asm {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements);
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self;
}
impl Opcode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    fn read_as_spec_op(ty: IdType, id: IdResult, chunk: &[u32], idx: &mut usize) -> Self {
        use Opcode::*;
        let i = *idx as usize;
        let mask = u16::MAX as u32;
        let opc = chunk[i] & mask;
        *idx += 1;
        match opc {
            1 => OpUndef(ty, id),
            12 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpExtInst(ty, id, x2, x3, x4)
            }
            41 => OpConstantTrue(ty, id),
            42 => OpConstantFalse(ty, id),
            43 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConstant(ty, id, x2)
            }
            44 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConstantComposite(ty, id, x2)
            }
            45 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpConstantSampler(ty, id, x2, x3, x4)
            }
            46 => OpConstantNull(ty, id),
            48 => OpSpecConstantTrue(ty, id),
            49 => OpSpecConstantFalse(ty, id),
            50 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSpecConstant(ty, id, x2)
            }
            51 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSpecConstantComposite(ty, id, x2)
            }
            52 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSpecConstantOp(ty, id, x2)
            }
            54 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFunction(ty, id, x2, x3)
            }
            55 => OpFunctionParameter(ty, id),
            57 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFunctionCall(ty, id, x2, x3)
            }
            59 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVariable(ty, id, x2, x3)
            }
            60 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageTexelPointer(ty, id, x2, x3, x4)
            }
            61 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLoad(ty, id, x2, x3)
            }
            65 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAccessChain(ty, id, x2, x3)
            }
            66 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpInBoundsAccessChain(ty, id, x2, x3)
            }
            67 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpPtrAccessChain(ty, id, x2, x3, x4)
            }
            68 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpArrayLength(ty, id, x2, x3)
            }
            69 => {
                let x2 = Asm::read_word(chunk, idx);
                OpGenericPtrMemSemantics(ty, id, x2)
            }
            70 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpInBoundsPtrAccessChain(ty, id, x2, x3, x4)
            }
            77 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorExtractDynamic(ty, id, x2, x3)
            }
            78 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpVectorInsertDynamic(ty, id, x2, x3, x4)
            }
            79 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpVectorShuffle(ty, id, x2, x3, x4)
            }
            80 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCompositeConstruct(ty, id, x2)
            }
            81 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpCompositeExtract(ty, id, x2, x3)
            }
            82 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCompositeInsert(ty, id, x2, x3, x4)
            }
            83 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCopyObject(ty, id, x2)
            }
            84 => {
                let x2 = Asm::read_word(chunk, idx);
                OpTranspose(ty, id, x2)
            }
            86 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSampledImage(ty, id, x2, x3)
            }
            87 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleImplicitLod(ty, id, x2, x3, x4)
            }
            88 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleExplicitLod(ty, id, x2, x3, x4)
            }
            89 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleDrefImplicitLod(ty, id, x2, x3, x4, x5)
            }
            90 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleDrefExplicitLod(ty, id, x2, x3, x4, x5)
            }
            91 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleProjImplicitLod(ty, id, x2, x3, x4)
            }
            92 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleProjExplicitLod(ty, id, x2, x3, x4)
            }
            93 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleProjDrefImplicitLod(ty, id, x2, x3, x4, x5)
            }
            94 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleProjDrefExplicitLod(ty, id, x2, x3, x4, x5)
            }
            95 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageFetch(ty, id, x2, x3, x4)
            }
            96 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageGather(ty, id, x2, x3, x4, x5)
            }
            97 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageDrefGather(ty, id, x2, x3, x4, x5)
            }
            98 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageRead(ty, id, x2, x3, x4)
            }
            100 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImage(ty, id, x2)
            }
            101 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryFormat(ty, id, x2)
            }
            102 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryOrder(ty, id, x2)
            }
            103 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpImageQuerySizeLod(ty, id, x2, x3)
            }
            104 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageQuerySize(ty, id, x2)
            }
            105 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpImageQueryLod(ty, id, x2, x3)
            }
            106 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryLevels(ty, id, x2)
            }
            107 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageQuerySamples(ty, id, x2)
            }
            109 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertFToU(ty, id, x2)
            }
            110 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertFToS(ty, id, x2)
            }
            111 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSToF(ty, id, x2)
            }
            112 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToF(ty, id, x2)
            }
            113 => {
                let x2 = Asm::read_word(chunk, idx);
                OpUConvert(ty, id, x2)
            }
            114 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSConvert(ty, id, x2)
            }
            115 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFConvert(ty, id, x2)
            }
            116 => {
                let x2 = Asm::read_word(chunk, idx);
                OpQuantizeToF16(ty, id, x2)
            }
            117 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertPtrToU(ty, id, x2)
            }
            118 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSatConvertSToU(ty, id, x2)
            }
            119 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSatConvertUToS(ty, id, x2)
            }
            120 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToPtr(ty, id, x2)
            }
            121 => {
                let x2 = Asm::read_word(chunk, idx);
                OpPtrCastToGeneric(ty, id, x2)
            }
            122 => {
                let x2 = Asm::read_word(chunk, idx);
                OpGenericCastToPtr(ty, id, x2)
            }
            123 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGenericCastToPtrExplicit(ty, id, x2, x3)
            }
            124 => {
                let x2 = Asm::read_word(chunk, idx);
                OpBitcast(ty, id, x2)
            }
            126 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSNegate(ty, id, x2)
            }
            127 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFNegate(ty, id, x2)
            }
            128 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAdd(ty, id, x2, x3)
            }
            129 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFAdd(ty, id, x2, x3)
            }
            130 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISub(ty, id, x2, x3)
            }
            131 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFSub(ty, id, x2, x3)
            }
            132 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIMul(ty, id, x2, x3)
            }
            133 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFMul(ty, id, x2, x3)
            }
            134 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUDiv(ty, id, x2, x3)
            }
            135 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSDiv(ty, id, x2, x3)
            }
            136 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFDiv(ty, id, x2, x3)
            }
            137 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMod(ty, id, x2, x3)
            }
            138 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSRem(ty, id, x2, x3)
            }
            139 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSMod(ty, id, x2, x3)
            }
            140 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFRem(ty, id, x2, x3)
            }
            141 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFMod(ty, id, x2, x3)
            }
            142 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorTimesScalar(ty, id, x2, x3)
            }
            143 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesScalar(ty, id, x2, x3)
            }
            144 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorTimesMatrix(ty, id, x2, x3)
            }
            145 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesVector(ty, id, x2, x3)
            }
            146 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesMatrix(ty, id, x2, x3)
            }
            147 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpOuterProduct(ty, id, x2, x3)
            }
            148 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpDot(ty, id, x2, x3)
            }
            149 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAddCarry(ty, id, x2, x3)
            }
            150 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISubBorrow(ty, id, x2, x3)
            }
            151 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMulExtended(ty, id, x2, x3)
            }
            152 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSMulExtended(ty, id, x2, x3)
            }
            154 => {
                let x2 = Asm::read_word(chunk, idx);
                OpAny(ty, id, x2)
            }
            155 => {
                let x2 = Asm::read_word(chunk, idx);
                OpAll(ty, id, x2)
            }
            156 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsNan(ty, id, x2)
            }
            157 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsInf(ty, id, x2)
            }
            158 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsFinite(ty, id, x2)
            }
            159 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsNormal(ty, id, x2)
            }
            160 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSignBitSet(ty, id, x2)
            }
            161 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLessOrGreater(ty, id, x2, x3)
            }
            162 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpOrdered(ty, id, x2, x3)
            }
            163 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUnordered(ty, id, x2, x3)
            }
            164 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalEqual(ty, id, x2, x3)
            }
            165 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalNotEqual(ty, id, x2, x3)
            }
            166 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalOr(ty, id, x2, x3)
            }
            167 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalAnd(ty, id, x2, x3)
            }
            168 => {
                let x2 = Asm::read_word(chunk, idx);
                OpLogicalNot(ty, id, x2)
            }
            169 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSelect(ty, id, x2, x3, x4)
            }
            170 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIEqual(ty, id, x2, x3)
            }
            171 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpINotEqual(ty, id, x2, x3)
            }
            172 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUGreaterThan(ty, id, x2, x3)
            }
            173 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSGreaterThan(ty, id, x2, x3)
            }
            174 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUGreaterThanEqual(ty, id, x2, x3)
            }
            175 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSGreaterThanEqual(ty, id, x2, x3)
            }
            176 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpULessThan(ty, id, x2, x3)
            }
            177 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSLessThan(ty, id, x2, x3)
            }
            178 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpULessThanEqual(ty, id, x2, x3)
            }
            179 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSLessThanEqual(ty, id, x2, x3)
            }
            180 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdEqual(ty, id, x2, x3)
            }
            181 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordEqual(ty, id, x2, x3)
            }
            182 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdNotEqual(ty, id, x2, x3)
            }
            183 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordNotEqual(ty, id, x2, x3)
            }
            184 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdLessThan(ty, id, x2, x3)
            }
            185 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordLessThan(ty, id, x2, x3)
            }
            186 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdGreaterThan(ty, id, x2, x3)
            }
            187 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordGreaterThan(ty, id, x2, x3)
            }
            188 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdLessThanEqual(ty, id, x2, x3)
            }
            189 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordLessThanEqual(ty, id, x2, x3)
            }
            190 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdGreaterThanEqual(ty, id, x2, x3)
            }
            191 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordGreaterThanEqual(ty, id, x2, x3)
            }
            194 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftRightLogical(ty, id, x2, x3)
            }
            195 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftRightArithmetic(ty, id, x2, x3)
            }
            196 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftLeftLogical(ty, id, x2, x3)
            }
            197 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseOr(ty, id, x2, x3)
            }
            198 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseXor(ty, id, x2, x3)
            }
            199 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseAnd(ty, id, x2, x3)
            }
            200 => {
                let x2 = Asm::read_word(chunk, idx);
                OpNot(ty, id, x2)
            }
            201 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpBitFieldInsert(ty, id, x2, x3, x4, x5)
            }
            202 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBitFieldSExtract(ty, id, x2, x3, x4)
            }
            203 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBitFieldUExtract(ty, id, x2, x3, x4)
            }
            204 => {
                let x2 = Asm::read_word(chunk, idx);
                OpBitReverse(ty, id, x2)
            }
            205 => {
                let x2 = Asm::read_word(chunk, idx);
                OpBitCount(ty, id, x2)
            }
            207 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdx(ty, id, x2)
            }
            208 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdy(ty, id, x2)
            }
            209 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFwidth(ty, id, x2)
            }
            210 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdxFine(ty, id, x2)
            }
            211 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdyFine(ty, id, x2)
            }
            212 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFwidthFine(ty, id, x2)
            }
            213 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdxCoarse(ty, id, x2)
            }
            214 => {
                let x2 = Asm::read_word(chunk, idx);
                OpDPdyCoarse(ty, id, x2)
            }
            215 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFwidthCoarse(ty, id, x2)
            }
            227 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicLoad(ty, id, x2, x3, x4)
            }
            229 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicExchange(ty, id, x2, x3, x4, x5)
            }
            230 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpAtomicCompareExchange(ty, id, x2, x3, x4, x5, x6, x7)
            }
            231 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpAtomicCompareExchangeWeak(ty, id, x2, x3, x4, x5, x6, x7)
            }
            232 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicIIncrement(ty, id, x2, x3, x4)
            }
            233 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicIDecrement(ty, id, x2, x3, x4)
            }
            234 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicIAdd(ty, id, x2, x3, x4, x5)
            }
            235 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicISub(ty, id, x2, x3, x4, x5)
            }
            236 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicSMin(ty, id, x2, x3, x4, x5)
            }
            237 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicUMin(ty, id, x2, x3, x4, x5)
            }
            238 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicSMax(ty, id, x2, x3, x4, x5)
            }
            239 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicUMax(ty, id, x2, x3, x4, x5)
            }
            240 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicAnd(ty, id, x2, x3, x4, x5)
            }
            241 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicOr(ty, id, x2, x3, x4, x5)
            }
            242 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicXor(ty, id, x2, x3, x4, x5)
            }
            245 => {
                let x2 = Asm::read_word(chunk, idx);
                OpPhi(ty, id, x2)
            }
            259 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpGroupAsyncCopy(ty, id, x2, x3, x4, x5, x6, x7)
            }
            261 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupAll(ty, id, x2, x3)
            }
            262 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupAny(ty, id, x2, x3)
            }
            263 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBroadcast(ty, id, x2, x3, x4)
            }
            264 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIAdd(ty, id, x2, x3, x4)
            }
            265 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFAdd(ty, id, x2, x3, x4)
            }
            266 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMin(ty, id, x2, x3, x4)
            }
            267 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMin(ty, id, x2, x3, x4)
            }
            268 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMin(ty, id, x2, x3, x4)
            }
            269 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMax(ty, id, x2, x3, x4)
            }
            270 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMax(ty, id, x2, x3, x4)
            }
            271 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMax(ty, id, x2, x3, x4)
            }
            274 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReadPipe(ty, id, x2, x3, x4, x5)
            }
            275 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpWritePipe(ty, id, x2, x3, x4, x5)
            }
            276 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpReservedReadPipe(ty, id, x2, x3, x4, x5, x6, x7)
            }
            277 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpReservedWritePipe(ty, id, x2, x3, x4, x5, x6, x7)
            }
            278 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReserveReadPipePackets(ty, id, x2, x3, x4, x5)
            }
            279 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReserveWritePipePackets(ty, id, x2, x3, x4, x5)
            }
            282 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsValidReserveId(ty, id, x2)
            }
            283 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGetNumPipePackets(ty, id, x2, x3, x4)
            }
            284 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGetMaxPipePackets(ty, id, x2, x3, x4)
            }
            285 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGroupReserveReadPipePackets(ty, id, x2, x3, x4, x5, x6)
            }
            286 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGroupReserveWritePipePackets(ty, id, x2, x3, x4, x5, x6)
            }
            291 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpEnqueueMarker(ty, id, x2, x3, x4, x5)
            }
            292 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                let x12 = Asm::read_word(chunk, idx);
                OpEnqueueKernel(ty, id, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12)
            }
            293 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelNDrangeSubGroupCount(ty, id, x2, x3, x4, x5, x6)
            }
            294 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelNDrangeMaxSubGroupSize(ty, id, x2, x3, x4, x5, x6)
            }
            295 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelWorkGroupSize(ty, id, x2, x3, x4, x5)
            }
            296 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelPreferredWorkGroupSizeMultiple(ty, id, x2, x3, x4, x5)
            }
            299 => OpCreateUserEvent(ty, id),
            300 => {
                let x2 = Asm::read_word(chunk, idx);
                OpIsValidEvent(ty, id, x2)
            }
            303 => OpGetDefaultQueue(ty, id),
            304 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBuildNDRange(ty, id, x2, x3, x4)
            }
            305 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleImplicitLod(ty, id, x2, x3, x4)
            }
            306 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleExplicitLod(ty, id, x2, x3, x4)
            }
            307 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleDrefImplicitLod(ty, id, x2, x3, x4, x5)
            }
            308 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleDrefExplicitLod(ty, id, x2, x3, x4, x5)
            }
            309 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjImplicitLod(ty, id, x2, x3, x4)
            }
            310 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjExplicitLod(ty, id, x2, x3, x4)
            }
            311 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjDrefImplicitLod(ty, id, x2, x3, x4, x5)
            }
            312 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjDrefExplicitLod(ty, id, x2, x3, x4, x5)
            }
            313 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseFetch(ty, id, x2, x3, x4)
            }
            314 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseGather(ty, id, x2, x3, x4, x5)
            }
            315 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseDrefGather(ty, id, x2, x3, x4, x5)
            }
            316 => {
                let x2 = Asm::read_word(chunk, idx);
                OpImageSparseTexelsResident(ty, id, x2)
            }
            318 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicFlagTestAndSet(ty, id, x2, x3, x4)
            }
            320 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseRead(ty, id, x2, x3, x4)
            }
            321 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSizeOf(ty, id, x2)
            }
            323 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpConstantPipeStorage(ty, id, x2, x3, x4)
            }
            324 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCreatePipeFromPipeStorage(ty, id, x2)
            }
            325 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelLocalSizeForSubgroupCount(ty, id, x2, x3, x4, x5, x6)
            }
            326 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelMaxNumSubgroups(ty, id, x2, x3, x4, x5)
            }
            328 => {
                let x2 = Asm::read_word(chunk, idx);
                OpNamedBarrierInitialize(ty, id, x2)
            }
            333 => {
                let x2 = Asm::read_word(chunk, idx);
                OpGroupNonUniformElect(ty, id, x2)
            }
            334 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAll(ty, id, x2, x3)
            }
            335 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAny(ty, id, x2, x3)
            }
            336 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAllEqual(ty, id, x2, x3)
            }
            337 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBroadcast(ty, id, x2, x3, x4)
            }
            338 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBroadcastFirst(ty, id, x2, x3)
            }
            339 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallot(ty, id, x2, x3)
            }
            340 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformInverseBallot(ty, id, x2, x3)
            }
            341 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotBitExtract(ty, id, x2, x3, x4)
            }
            342 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotBitCount(ty, id, x2, x3, x4)
            }
            343 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotFindLSB(ty, id, x2, x3)
            }
            344 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotFindMSB(ty, id, x2, x3)
            }
            345 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffle(ty, id, x2, x3, x4)
            }
            346 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleXor(ty, id, x2, x3, x4)
            }
            347 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleUp(ty, id, x2, x3, x4)
            }
            348 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleDown(ty, id, x2, x3, x4)
            }
            349 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformIAdd(ty, id, x2, x3, x4, x5)
            }
            350 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFAdd(ty, id, x2, x3, x4, x5)
            }
            351 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformIMul(ty, id, x2, x3, x4, x5)
            }
            352 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMul(ty, id, x2, x3, x4, x5)
            }
            353 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformSMin(ty, id, x2, x3, x4, x5)
            }
            354 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformUMin(ty, id, x2, x3, x4, x5)
            }
            355 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMin(ty, id, x2, x3, x4, x5)
            }
            356 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformSMax(ty, id, x2, x3, x4, x5)
            }
            357 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformUMax(ty, id, x2, x3, x4, x5)
            }
            358 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMax(ty, id, x2, x3, x4, x5)
            }
            359 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseAnd(ty, id, x2, x3, x4, x5)
            }
            360 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseOr(ty, id, x2, x3, x4, x5)
            }
            361 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseXor(ty, id, x2, x3, x4, x5)
            }
            362 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalAnd(ty, id, x2, x3, x4, x5)
            }
            363 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalOr(ty, id, x2, x3, x4, x5)
            }
            364 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalXor(ty, id, x2, x3, x4, x5)
            }
            365 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformQuadBroadcast(ty, id, x2, x3, x4)
            }
            366 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformQuadSwap(ty, id, x2, x3, x4)
            }
            400 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCopyLogical(ty, id, x2)
            }
            401 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrEqual(ty, id, x2, x3)
            }
            402 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrNotEqual(ty, id, x2, x3)
            }
            403 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrDiff(ty, id, x2, x3)
            }
            4421 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupBallotKHR(ty, id, x2)
            }
            4422 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupFirstInvocationKHR(ty, id, x2)
            }
            4428 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAllKHR(ty, id, x2)
            }
            4429 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAnyKHR(ty, id, x2)
            }
            4430 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAllEqualKHR(ty, id, x2)
            }
            4432 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupReadInvocationKHR(ty, id, x2, x3)
            }
            4447 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToAccelerationStructureKHR(ty, id, x2)
            }
            4450 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSDot(ty, id, x2, x3, x4)
            }
            4451 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpUDot(ty, id, x2, x3, x4)
            }
            4452 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSUDot(ty, id, x2, x3, x4)
            }
            4453 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSDotAccSat(ty, id, x2, x3, x4, x5)
            }
            4454 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpUDotAccSat(ty, id, x2, x3, x4, x5)
            }
            4455 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSUDotAccSat(ty, id, x2, x3, x4, x5)
            }
            4477 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryProceedKHR(ty, id, x2)
            }
            4479 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionTypeKHR(ty, id, x2, x3)
            }
            5000 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIAddNonUniformAMD(ty, id, x2, x3, x4)
            }
            5001 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFAddNonUniformAMD(ty, id, x2, x3, x4)
            }
            5002 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMinNonUniformAMD(ty, id, x2, x3, x4)
            }
            5003 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMinNonUniformAMD(ty, id, x2, x3, x4)
            }
            5004 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMinNonUniformAMD(ty, id, x2, x3, x4)
            }
            5005 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMaxNonUniformAMD(ty, id, x2, x3, x4)
            }
            5006 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMaxNonUniformAMD(ty, id, x2, x3, x4)
            }
            5007 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMaxNonUniformAMD(ty, id, x2, x3, x4)
            }
            5011 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFragmentMaskFetchAMD(ty, id, x2, x3)
            }
            5012 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpFragmentFetchAMD(ty, id, x2, x3, x4)
            }
            5056 => {
                let x2 = Asm::read_word(chunk, idx);
                OpReadClockKHR(ty, id, x2)
            }
            5283 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpImageSampleFootprintNV(ty, id, x2, x3, x4, x5, x6)
            }
            5296 => {
                let x2 = Asm::read_word(chunk, idx);
                OpGroupNonUniformPartitionNV(ty, id, x2)
            }
            5334 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpReportIntersectionNV(ty, id, x2, x3)
            }
            5359 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixLoadNV(ty, id, x2, x3, x4, x5)
            }
            5361 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixMulAddNV(ty, id, x2, x3, x4)
            }
            5362 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixLengthNV(ty, id, x2)
            }
            5381 => OpIsHelperInvocationEXT(ty, id),
            5391 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToImageNV(ty, id, x2)
            }
            5392 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToSamplerNV(ty, id, x2)
            }
            5393 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertImageToUNV(ty, id, x2)
            }
            5394 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSamplerToUNV(ty, id, x2)
            }
            5395 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToSampledImageNV(ty, id, x2)
            }
            5396 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSampledImageToUNV(ty, id, x2)
            }
            5571 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleINTEL(ty, id, x2, x3)
            }
            5572 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleDownINTEL(ty, id, x2, x3, x4)
            }
            5573 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleUpINTEL(ty, id, x2, x3, x4)
            }
            5574 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleXorINTEL(ty, id, x2, x3)
            }
            5575 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupBlockReadINTEL(ty, id, x2)
            }
            5577 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupImageBlockReadINTEL(ty, id, x2, x3)
            }
            5580 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupImageMediaBlockReadINTEL(ty, id, x2, x3, x4, x5)
            }
            5585 => {
                let x2 = Asm::read_word(chunk, idx);
                OpUCountLeadingZerosINTEL(ty, id, x2)
            }
            5586 => {
                let x2 = Asm::read_word(chunk, idx);
                OpUCountTrailingZerosINTEL(ty, id, x2)
            }
            5587 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAbsISubINTEL(ty, id, x2, x3)
            }
            5588 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAbsUSubINTEL(ty, id, x2, x3)
            }
            5589 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAddSatINTEL(ty, id, x2, x3)
            }
            5590 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAddSatINTEL(ty, id, x2, x3)
            }
            5591 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAverageINTEL(ty, id, x2, x3)
            }
            5592 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAverageINTEL(ty, id, x2, x3)
            }
            5593 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAverageRoundedINTEL(ty, id, x2, x3)
            }
            5594 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAverageRoundedINTEL(ty, id, x2, x3)
            }
            5595 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISubSatINTEL(ty, id, x2, x3)
            }
            5596 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUSubSatINTEL(ty, id, x2, x3)
            }
            5597 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIMul32x16INTEL(ty, id, x2, x3)
            }
            5598 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMul32x16INTEL(ty, id, x2, x3)
            }
            5600 => {
                let x2 = Asm::read_word(chunk, idx);
                OpConstantFunctionPointerINTEL(ty, id, x2)
            }
            5601 => {
                let x2 = Asm::read_word(chunk, idx);
                OpFunctionPointerCallINTEL(ty, id, x2)
            }
            5609 => {
                let x2 = Asm::read_word(chunk, idx);
                OpAsmTargetINTEL(ty, id, x2)
            }
            5610 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAsmINTEL(ty, id, x2, x3, x4, x5)
            }
            5611 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAsmCallINTEL(ty, id, x2, x3)
            }
            5614 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFMinEXT(ty, id, x2, x3, x4, x5)
            }
            5615 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFMaxEXT(ty, id, x2, x3, x4, x5)
            }
            5631 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpExpectKHR(ty, id, x2, x3)
            }
            5699 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVmeImageINTEL(ty, id, x2, x3)
            }
            5713 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(ty, id, x2, x3)
            }
            5714 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(ty, id, x2, x3)
            }
            5715 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(ty, id, x2, x3)
            }
            5716 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterShapePenaltyINTEL(ty, id, x2, x3)
            }
            5717 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(ty, id, x2, x3)
            }
            5718 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(ty, id, x2, x3)
            }
            5719 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(ty, id, x2, x3)
            }
            5720 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(ty, id, x2, x3)
            }
            5721 => OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(ty, id),
            5722 => OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(ty, id),
            5723 => OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(ty, id),
            5724 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(ty, id, x2, x3, x4, x5)
            }
            5725 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(ty, id, x2, x3)
            }
            5726 => OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(ty, id),
            5727 => OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(ty, id),
            5728 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetAcOnlyHaarINTEL(ty, id, x2)
            }
            5729 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(ty, id, x2, x3)
            }
            5730 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(ty, id, x2, x3)
            }
            5731 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(ty, id, x2, x3, x4)
            }
            5732 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToImePayloadINTEL(ty, id, x2)
            }
            5733 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToImeResultINTEL(ty, id, x2)
            }
            5734 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToRefPayloadINTEL(ty, id, x2)
            }
            5735 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToRefResultINTEL(ty, id, x2)
            }
            5736 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToSicPayloadINTEL(ty, id, x2)
            }
            5737 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToSicResultINTEL(ty, id, x2)
            }
            5738 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetMotionVectorsINTEL(ty, id, x2)
            }
            5739 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterDistortionsINTEL(ty, id, x2)
            }
            5740 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetBestInterDistortionsINTEL(ty, id, x2)
            }
            5741 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMajorShapeINTEL(ty, id, x2)
            }
            5742 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMinorShapeINTEL(ty, id, x2)
            }
            5743 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterDirectionsINTEL(ty, id, x2)
            }
            5744 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMotionVectorCountINTEL(ty, id, x2)
            }
            5745 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterReferenceIdsINTEL(ty, id, x2)
            }
            5746 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(ty, id, x2, x3, x4)
            }
            5747 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeInitializeINTEL(ty, id, x2, x3, x4)
            }
            5748 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetSingleReferenceINTEL(ty, id, x2, x3, x4)
            }
            5749 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetDualReferenceINTEL(ty, id, x2, x3, x4, x5)
            }
            5750 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeRefWindowSizeINTEL(ty, id, x2, x3)
            }
            5751 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeAdjustRefOffsetINTEL(ty, id, x2, x3, x4, x5)
            }
            5752 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeConvertToMcePayloadINTEL(ty, id, x2)
            }
            5753 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(ty, id, x2, x3)
            }
            5754 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(ty, id, x2)
            }
            5755 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(ty, id, x2, x3)
            }
            5756 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetWeightedSadINTEL(ty, id, x2, x3)
            }
            5757 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(ty, id, x2, x3, x4)
            }
            5758 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(ty, id, x2, x3, x4, x5)
            }
            5759 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(ty, id, x2, x3, x4, x5)
            }
            5760 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(ty, id, x2, x3, x4, x5, x6)
            }
            5761 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(ty, id, x2, x3, x4)
            }
            5762 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(ty, id, x2, x3, x4, x5)
            }
            5763 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(ty, id, x2, x3, x4, x5)
            }
            5764 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                    ty, id, x2, x3, x4, x5, x6,
                )
            }
            5765 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeConvertToMceResultINTEL(ty, id, x2)
            }
            5766 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(ty, id, x2)
            }
            5767 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetDualReferenceStreaminINTEL(ty, id, x2)
            }
            5768 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(ty, id, x2)
            }
            5769 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(ty, id, x2)
            }
            5770 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                    ty, id, x2, x3,
                )
            }
            5771 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                    ty, id, x2, x3,
                )
            }
            5772 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                    ty, id, x2, x3,
                )
            }
            5773 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                    ty, id, x2, x3, x4,
                )
            }
            5774 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                    ty, id, x2, x3, x4,
                )
            }
            5775 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                    ty, id, x2, x3, x4,
                )
            }
            5776 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetBorderReachedINTEL(ty, id, x2, x3)
            }
            5777 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(ty, id, x2)
            }
            5778 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(ty, id, x2)
            }
            5779 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(ty, id, x2)
            }
            5780 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(ty, id, x2)
            }
            5781 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpSubgroupAvcFmeInitializeINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5782 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpSubgroupAvcBmeInitializeINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5783 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefConvertToMcePayloadINTEL(ty, id, x2)
            }
            5784 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(ty, id, x2)
            }
            5785 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefSetBilinearFilterEnableINTEL(ty, id, x2)
            }
            5786 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(ty, id, x2, x3, x4)
            }
            5787 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(ty, id, x2, x3, x4, x5)
            }
            5788 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(ty, id, x2, x3, x4)
            }
            5789 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(ty, id, x2, x3, x4, x5)
            }
            5790 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefConvertToMceResultINTEL(ty, id, x2)
            }
            5791 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicInitializeINTEL(ty, id, x2)
            }
            5792 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureSkcINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5793 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureIpeLumaINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5794 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                let x12 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureIpeLumaChromaINTEL(
                    ty, id, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
                )
            }
            5795 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetMotionVectorMaskINTEL(ty, id, x2, x3)
            }
            5796 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConvertToMcePayloadINTEL(ty, id, x2)
            }
            5797 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(ty, id, x2, x3)
            }
            5798 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(ty, id, x2, x3, x4, x5)
            }
            5799 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(ty, id, x2, x3)
            }
            5800 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetBilinearFilterEnableINTEL(ty, id, x2)
            }
            5801 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(ty, id, x2, x3)
            }
            5802 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(ty, id, x2, x3)
            }
            5803 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateIpeINTEL(ty, id, x2, x3)
            }
            5804 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(ty, id, x2, x3, x4)
            }
            5805 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(ty, id, x2, x3, x4, x5)
            }
            5806 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(ty, id, x2, x3, x4)
            }
            5807 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(ty, id, x2, x3, x4, x5)
            }
            5808 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConvertToMceResultINTEL(ty, id, x2)
            }
            5809 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetIpeLumaShapeINTEL(ty, id, x2)
            }
            5810 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(ty, id, x2)
            }
            5811 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(ty, id, x2)
            }
            5812 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(ty, id, x2)
            }
            5813 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetIpeChromaModeINTEL(ty, id, x2)
            }
            5814 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(ty, id, x2)
            }
            5815 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(ty, id, x2)
            }
            5816 => {
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetInterRawSadsINTEL(ty, id, x2)
            }
            5818 => {
                let x2 = Asm::read_word(chunk, idx);
                OpVariableLengthArrayINTEL(ty, id, x2)
            }
            5819 => OpSaveMemoryINTEL(ty, id),
            5840 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinCosPiINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5841 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5842 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastFromIntINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5843 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastToIntINTEL(ty, id, x2, x3, x4, x5, x6)
            }
            5846 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatAddINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5847 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSubINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5848 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatMulINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5849 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatDivINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5850 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatGTINTEL(ty, id, x2, x3, x4, x5)
            }
            5851 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatGEINTEL(ty, id, x2, x3, x4, x5)
            }
            5852 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLTINTEL(ty, id, x2, x3, x4, x5)
            }
            5853 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLEINTEL(ty, id, x2, x3, x4, x5)
            }
            5854 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatEQINTEL(ty, id, x2, x3, x4, x5)
            }
            5855 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatRecipINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5856 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatRSqrtINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5857 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCbrtINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5858 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatHypotINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5859 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSqrtINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5860 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLogINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5861 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog2INTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5862 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog10INTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5863 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog1pINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5864 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExpINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5865 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExp2INTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5866 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExp10INTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5867 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExpm1INTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5868 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5869 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCosINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5870 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinCosINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5871 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinPiINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5872 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCosPiINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5873 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatASinINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5874 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatASinPiINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5875 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatACosINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5876 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatACosPiINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5877 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATanINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5878 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATanPiINTEL(ty, id, x2, x3, x4, x5, x6, x7)
            }
            5879 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATan2INTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5880 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5881 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowRINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5882 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowNINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5923 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSqrtINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5924 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedRecipINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5925 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedRsqrtINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5926 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5927 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedCosINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5928 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinCosINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5929 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinPiINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5930 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedCosPiINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5931 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinCosPiINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5932 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedLogINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5933 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedExpINTEL(ty, id, x2, x3, x4, x5, x6, x7, x8)
            }
            5934 => {
                let x2 = Asm::read_word(chunk, idx);
                OpPtrCastToCrossWorkgroupINTEL(ty, id, x2)
            }
            5938 => {
                let x2 = Asm::read_word(chunk, idx);
                OpCrossWorkgroupCastToPtrINTEL(ty, id, x2)
            }
            5946 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpReadPipeBlockingINTEL(ty, id, x2, x3)
            }
            5947 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpWritePipeBlockingINTEL(ty, id, x2, x3)
            }
            5949 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFPGARegINTEL(ty, id, x2, x3)
            }
            6016 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetRayTMinKHR(ty, id, x2)
            }
            6017 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetRayFlagsKHR(ty, id, x2)
            }
            6018 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionTKHR(ty, id, x2, x3)
            }
            6019 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceCustomIndexKHR(ty, id, x2, x3)
            }
            6020 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceIdKHR(ty, id, x2, x3)
            }
            6021 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(ty, id, x2, x3)
            }
            6022 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionGeometryIndexKHR(ty, id, x2, x3)
            }
            6023 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionPrimitiveIndexKHR(ty, id, x2, x3)
            }
            6024 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionBarycentricsKHR(ty, id, x2, x3)
            }
            6025 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionFrontFaceKHR(ty, id, x2, x3)
            }
            6026 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(ty, id, x2)
            }
            6027 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectRayDirectionKHR(ty, id, x2, x3)
            }
            6028 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectRayOriginKHR(ty, id, x2, x3)
            }
            6029 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetWorldRayDirectionKHR(ty, id, x2)
            }
            6030 => {
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetWorldRayOriginKHR(ty, id, x2)
            }
            6031 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectToWorldKHR(ty, id, x2, x3)
            }
            6032 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionWorldToObjectKHR(ty, id, x2, x3)
            }
            6035 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFAddEXT(ty, id, x2, x3, x4, x5)
            }
            6401 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIMulKHR(ty, id, x2, x3, x4)
            }
            6402 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMulKHR(ty, id, x2, x3, x4)
            }
            6403 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseAndKHR(ty, id, x2, x3, x4)
            }
            6404 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseOrKHR(ty, id, x2, x3, x4)
            }
            6405 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseXorKHR(ty, id, x2, x3, x4)
            }
            6406 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalAndKHR(ty, id, x2, x3, x4)
            }
            6407 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalOrKHR(ty, id, x2, x3, x4)
            }
            6408 => {
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalXorKHR(ty, id, x2, x3, x4)
            }
            what => panic!("{:?}", what),
        }
    }
}
impl Asm for Opcode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            OpNop => (),
            OpUndef(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSourceContinued(x0) => {
                x0.write_word(sink, req);
            }
            OpSource(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSourceExtension(x0) => {
                x0.write_word(sink, req);
            }
            OpName(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpMemberName(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpString(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpLine(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpExtension(x0) => {
                x0.write_word(sink, req);
            }
            OpExtInstImport(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpExtInst(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpMemoryModel(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpEntryPoint(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpExecutionMode(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpCapability(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeVoid(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeBool(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeInt(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeFloat(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeVector(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeMatrix(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpTypeSampler(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeSampledImage(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeArray(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeRuntimeArray(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeStruct(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeOpaque(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypePointer(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeFunction(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypeEvent(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeDeviceEvent(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeReserveId(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeQueue(x0) => {
                x0.write_word(sink, req);
            }
            OpTypePipe(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeForwardPointer(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpConstantTrue(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpConstantFalse(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpConstant(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConstantComposite(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConstantSampler(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpConstantNull(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSpecConstantTrue(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSpecConstantFalse(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSpecConstant(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSpecConstantComposite(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSpecConstantOp(_, _, specop) => specop.write_word(sink, req),
            OpFunction(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFunctionParameter(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpFunctionEnd => (),
            OpFunctionCall(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpVariable(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpImageTexelPointer(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpLoad(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpStore(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpCopyMemory(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpCopyMemorySized(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpAccessChain(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpInBoundsAccessChain(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpPtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpArrayLength(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGenericPtrMemSemantics(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpInBoundsPtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpDecorate(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpMemberDecorate(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDecorationGroup(x0) => {
                x0.write_word(sink, req);
            }
            OpGroupDecorate(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpGroupMemberDecorate(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpVectorExtractDynamic(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpVectorInsertDynamic(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpVectorShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCompositeConstruct(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpCompositeExtract(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpCompositeInsert(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCopyObject(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTranspose(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSampledImage(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpImageSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageRead(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageWrite(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpImage(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageQueryFormat(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageQueryOrder(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageQuerySizeLod(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpImageQuerySize(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageQueryLod(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpImageQueryLevels(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageQuerySamples(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertFToU(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertFToS(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertSToF(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertUToF(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpUConvert(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSConvert(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFConvert(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpQuantizeToF16(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertPtrToU(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSatConvertSToU(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSatConvertUToS(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertUToPtr(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpPtrCastToGeneric(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGenericCastToPtr(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGenericCastToPtrExplicit(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpBitcast(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSNegate(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFNegate(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIAdd(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFAdd(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpISub(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFSub(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIMul(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFMul(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUDiv(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSDiv(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFDiv(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUMod(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSRem(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSMod(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFRem(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFMod(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpVectorTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpMatrixTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpVectorTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpMatrixTimesVector(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpMatrixTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpOuterProduct(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpDot(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIAddCarry(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpISubBorrow(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUMulExtended(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSMulExtended(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpAny(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpAll(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIsNan(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIsInf(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIsFinite(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIsNormal(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSignBitSet(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpLessOrGreater(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpOrdered(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUnordered(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpLogicalEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpLogicalNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpLogicalOr(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpLogicalAnd(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpLogicalNot(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSelect(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpIEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpINotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpULessThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpULessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFOrdGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFUnordGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpShiftRightLogical(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpShiftRightArithmetic(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpShiftLeftLogical(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpBitwiseOr(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpBitwiseXor(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpBitwiseAnd(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpNot(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpBitFieldInsert(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpBitFieldSExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpBitFieldUExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpBitReverse(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpBitCount(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdx(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdy(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFwidth(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdxFine(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdyFine(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFwidthFine(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdxCoarse(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpDPdyCoarse(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFwidthCoarse(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpEmitVertex => (),
            OpEndPrimitive => (),
            OpEmitStreamVertex(x0) => {
                x0.write_word(sink, req);
            }
            OpEndStreamPrimitive(x0) => {
                x0.write_word(sink, req);
            }
            OpControlBarrier(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpMemoryBarrier(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpAtomicLoad(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpAtomicStore(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpAtomicExchange(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpAtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpAtomicIIncrement(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpAtomicIDecrement(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpAtomicIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicISub(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpPhi(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpLoopMerge(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSelectionMerge(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpLabel(x0) => {
                x0.write_word(sink, req);
            }
            OpBranch(x0) => {
                x0.write_word(sink, req);
            }
            OpBranchConditional(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSwitch(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpKill => (),
            OpReturn => (),
            OpReturnValue(x0) => {
                x0.write_word(sink, req);
            }
            OpUnreachable => (),
            OpLifetimeStart(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpLifetimeStop(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpGroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpGroupWaitEvents(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGroupAll(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupAny(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupIAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupUMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupSMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupUMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupSMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpReadPipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpWritePipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpReserveReadPipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpReserveWritePipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpCommitReadPipe(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpCommitWritePipe(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIsValidReserveId(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGetNumPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGetMaxPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGroupCommitReadPipe(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupCommitWritePipe(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpEnqueueMarker(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpEnqueueKernel(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
                x11.write_word(sink, req);
                x12.write_word(sink, req);
            }
            OpGetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGetKernelPreferredWorkGroupSizeMultiple(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpRetainEvent(x0) => {
                x0.write_word(sink, req);
            }
            OpReleaseEvent(x0) => {
                x0.write_word(sink, req);
            }
            OpCreateUserEvent(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpIsValidEvent(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSetUserEventStatus(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpCaptureEventProfilingInfo(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGetDefaultQueue(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpBuildNDRange(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpImageSparseGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpImageSparseTexelsResident(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpNoLine => (),
            OpAtomicFlagTestAndSet(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpAtomicFlagClear(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageSparseRead(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSizeOf(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpTypePipeStorage(x0) => {
                x0.write_word(sink, req);
            }
            OpConstantPipeStorage(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCreatePipeFromPipeStorage(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGetKernelLocalSizeForSubgroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpTypeNamedBarrier(x0) => {
                x0.write_word(sink, req);
            }
            OpNamedBarrierInitialize(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpMemoryNamedBarrier(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpModuleProcessed(x0) => {
                x0.write_word(sink, req);
            }
            OpExecutionModeId(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpDecorateId(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpGroupNonUniformElect(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGroupNonUniformAll(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformAny(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformAllEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformBroadcastFirst(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformBallot(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformInverseBallot(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformBallotBitCount(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformBallotFindLSB(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformBallotFindMSB(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupNonUniformShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformShuffleXor(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformShuffleUp(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformShuffleDown(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformFAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformIMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformFMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformFMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformFMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpGroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupNonUniformQuadSwap(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCopyLogical(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpPtrEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpPtrNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpPtrDiff(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpTerminateInvocation => (),
            OpSubgroupBallotKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupFirstInvocationKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAllKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAnyKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAllEqualKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupReadInvocationKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpTraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
            }
            OpExecuteCallableKHR(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpConvertUToAccelerationStructureKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpIgnoreIntersectionKHR => (),
            OpTerminateRayKHR => (),
            OpSDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpUDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSUDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpUDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSUDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpTypeRayQueryKHR(x0) => {
                x0.write_word(sink, req);
            }
            OpRayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpRayQueryTerminateKHR(x0) => {
                x0.write_word(sink, req);
            }
            OpRayQueryGenerateIntersectionKHR(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpRayQueryConfirmIntersectionKHR(x0) => {
                x0.write_word(sink, req);
            }
            OpRayQueryProceedKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetIntersectionTypeKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpGroupIAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupUMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupSMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupUMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupSMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpFragmentMaskFetchAMD(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFragmentFetchAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpReadClockKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpGroupNonUniformPartitionNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpWritePackedPrimitiveIndices4x8NV(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpReportIntersectionNV(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIgnoreIntersectionNV => (),
            OpTerminateRayNV => (),
            OpTraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
            }
            OpTraceMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
                x11.write_word(sink, req);
            }
            OpTraceRayMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
                x11.write_word(sink, req);
            }
            OpTypeAccelerationStructureNV(x0) => {
                x0.write_word(sink, req);
            }
            OpExecuteCallableNV(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeCooperativeMatrixNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpCooperativeMatrixStoreNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCooperativeMatrixMulAddNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpCooperativeMatrixLengthNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpBeginInvocationInterlockEXT => (),
            OpEndInvocationInterlockEXT => (),
            OpDemoteToHelperInvocation => (),
            OpIsHelperInvocationEXT(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpConvertUToImageNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertUToSamplerNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertImageToUNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertSamplerToUNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertUToSampledImageNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpConvertSampledImageToUNV(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSamplerImageAddressingModeNV(x0) => {
                x0.write_word(sink, req);
            }
            OpSubgroupShuffleINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupShuffleDownINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupShuffleUpINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupShuffleXorINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupBlockReadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupBlockWriteINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupImageBlockReadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupImageBlockWriteINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpUCountLeadingZerosINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpUCountTrailingZerosINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpAbsISubINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpAbsUSubINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpISubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUSubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpIMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpUMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpConstantFunctionPointerINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpFunctionPointerCallINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpAsmTargetINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpAsmINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAsmCallINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpAtomicFMinEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAtomicFMaxEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpAssumeTrueKHR(x0) => {
                x0.write_word(sink, req);
            }
            OpExpectKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpDecorateString(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpMemberDecorateString(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpVmeImageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpTypeVmeImageINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeAvcImePayloadINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcRefPayloadINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcSicPayloadINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcMcePayloadINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcMceResultINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcImeResultINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcImeResultSingleReferenceStreamoutINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcImeResultDualReferenceStreamoutINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcImeSingleReferenceStreaminINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcImeDualReferenceStreaminINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcRefResultINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpTypeAvcSicResultINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpSubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
                x5,
                x6,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpSubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcFmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpSubgroupAvcBmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpSubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicInitializeINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicConfigureSkcINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpSubgroupAvcSicConfigureIpeLumaINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpSubgroupAvcSicConfigureIpeLumaChromaINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
                x5,
                x6,
                x7,
                x8,
                x9,
                x10,
                x11,
                x12,
            ) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
                x10.write_word(sink, req);
                x11.write_word(sink, req);
                x12.write_word(sink, req);
            }
            OpSubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpSubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpVariableLengthArrayINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpSaveMemoryINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpRestoreMemoryINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpArbitraryFloatSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatCastFromIntINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
            }
            OpArbitraryFloatAddINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatSubINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatMulINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatDivINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatHypotINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
            }
            OpArbitraryFloatATan2INTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatPowINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatPowRINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
                x9.write_word(sink, req);
            }
            OpArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpLoopControlINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpAliasDomainDeclINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpAliasScopeDeclINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpAliasScopeListDeclINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpFixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpFixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
                x6.write_word(sink, req);
                x7.write_word(sink, req);
                x8.write_word(sink, req);
            }
            OpPtrCastToCrossWorkgroupINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpCrossWorkgroupCastToPtrINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpReadPipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpWritePipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpFPGARegINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetRayTMinKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetRayFlagsKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetIntersectionTKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionInstanceCustomIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetWorldRayDirectionKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetWorldRayOriginKHR(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpRayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpRayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
            }
            OpAtomicFAddEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
                x5.write_word(sink, req);
            }
            OpTypeBufferSurfaceINTEL(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            OpTypeStructContinuedINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpSpecConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(sink, req);
            }
            OpControlBarrierArriveINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpControlBarrierWaitINTEL(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            OpGroupIMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupFMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupBitwiseAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupBitwiseOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupBitwiseXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupLogicalAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupLogicalOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            OpGroupLogicalXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
                x3.write_word(sink, req);
                x4.write_word(sink, req);
            }
            what => panic!("{:?}", what),
        }
        sink[mark] |= ((sink.len() - mark) as u32) << 16;
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        use Opcode::*;
        let len = (chunk[*idx] >> 16) as usize;
        let opc = chunk[*idx] & 0xffff;
        let chunk = &chunk[..*idx + len];
        let mark = *idx;
        *idx += 1;
        let re = match opc {
            0 => OpNop,
            1 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpUndef(x0, x1)
            }
            2 => {
                let x0 = Asm::read_word(chunk, idx);
                OpSourceContinued(x0)
            }
            3 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSource(x0, x1, x2, x3)
            }
            4 => {
                let x0 = Asm::read_word(chunk, idx);
                OpSourceExtension(x0)
            }
            5 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpName(x0, x1)
            }
            6 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpMemberName(x0, x1, x2)
            }
            7 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpString(x0, x1)
            }
            8 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpLine(x0, x1, x2)
            }
            10 => {
                let x0 = Asm::read_word(chunk, idx);
                OpExtension(x0)
            }
            11 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpExtInstImport(x0, x1)
            }
            12 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpExtInst(x0, x1, x2, x3, x4)
            }
            14 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpMemoryModel(x0, x1)
            }
            15 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpEntryPoint(x0, x1, x2, x3)
            }
            16 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpExecutionMode(x0, x1)
            }
            17 => {
                let x0 = Asm::read_word(chunk, idx);
                OpCapability(x0)
            }
            19 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeVoid(x0)
            }
            20 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeBool(x0)
            }
            21 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypeInt(x0, x1, x2)
            }
            22 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeFloat(x0, x1)
            }
            23 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypeVector(x0, x1, x2)
            }
            24 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypeMatrix(x0, x1, x2)
            }
            25 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpTypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            26 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeSampler(x0)
            }
            27 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeSampledImage(x0, x1)
            }
            28 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypeArray(x0, x1, x2)
            }
            29 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeRuntimeArray(x0, x1)
            }
            30 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeStruct(x0, x1)
            }
            31 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeOpaque(x0, x1)
            }
            32 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypePointer(x0, x1, x2)
            }
            33 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTypeFunction(x0, x1, x2)
            }
            34 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeEvent(x0)
            }
            35 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeDeviceEvent(x0)
            }
            36 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeReserveId(x0)
            }
            37 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeQueue(x0)
            }
            38 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypePipe(x0, x1)
            }
            39 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeForwardPointer(x0, x1)
            }
            41 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpConstantTrue(x0, x1)
            }
            42 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpConstantFalse(x0, x1)
            }
            43 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConstant(x0, x1, x2)
            }
            44 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConstantComposite(x0, x1, x2)
            }
            45 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpConstantSampler(x0, x1, x2, x3, x4)
            }
            46 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpConstantNull(x0, x1)
            }
            48 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSpecConstantTrue(x0, x1)
            }
            49 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSpecConstantFalse(x0, x1)
            }
            50 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSpecConstant(x0, x1, x2)
            }
            51 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSpecConstantComposite(x0, x1, x2)
            }
            52 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Opcode::read_as_spec_op(x0, x1, chunk, idx);
                OpSpecConstantOp(x0, x1, Box::new(x2))
            }
            54 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFunction(x0, x1, x2, x3)
            }
            55 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpFunctionParameter(x0, x1)
            }
            56 => OpFunctionEnd,
            57 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFunctionCall(x0, x1, x2, x3)
            }
            59 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVariable(x0, x1, x2, x3)
            }
            60 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageTexelPointer(x0, x1, x2, x3, x4)
            }
            61 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLoad(x0, x1, x2, x3)
            }
            62 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpStore(x0, x1, x2)
            }
            63 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpCopyMemory(x0, x1, x2, x3)
            }
            64 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCopyMemorySized(x0, x1, x2, x3, x4)
            }
            65 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAccessChain(x0, x1, x2, x3)
            }
            66 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpInBoundsAccessChain(x0, x1, x2, x3)
            }
            67 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpPtrAccessChain(x0, x1, x2, x3, x4)
            }
            68 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpArrayLength(x0, x1, x2, x3)
            }
            69 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpGenericPtrMemSemantics(x0, x1, x2)
            }
            70 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpInBoundsPtrAccessChain(x0, x1, x2, x3, x4)
            }
            71 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpDecorate(x0, x1)
            }
            72 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpMemberDecorate(x0, x1, x2)
            }
            73 => {
                let x0 = Asm::read_word(chunk, idx);
                OpDecorationGroup(x0)
            }
            74 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpGroupDecorate(x0, x1)
            }
            75 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpGroupMemberDecorate(x0, x1)
            }
            77 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorExtractDynamic(x0, x1, x2, x3)
            }
            78 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpVectorInsertDynamic(x0, x1, x2, x3, x4)
            }
            79 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpVectorShuffle(x0, x1, x2, x3, x4)
            }
            80 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCompositeConstruct(x0, x1, x2)
            }
            81 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpCompositeExtract(x0, x1, x2, x3)
            }
            82 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCompositeInsert(x0, x1, x2, x3, x4)
            }
            83 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCopyObject(x0, x1, x2)
            }
            84 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpTranspose(x0, x1, x2)
            }
            86 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSampledImage(x0, x1, x2, x3)
            }
            87 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleImplicitLod(x0, x1, x2, x3, x4)
            }
            88 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleExplicitLod(x0, x1, x2, x3, x4)
            }
            89 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5)
            }
            90 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5)
            }
            91 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleProjImplicitLod(x0, x1, x2, x3, x4)
            }
            92 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSampleProjExplicitLod(x0, x1, x2, x3, x4)
            }
            93 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5)
            }
            94 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5)
            }
            95 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageFetch(x0, x1, x2, x3, x4)
            }
            96 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageGather(x0, x1, x2, x3, x4, x5)
            }
            97 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageDrefGather(x0, x1, x2, x3, x4, x5)
            }
            98 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageRead(x0, x1, x2, x3, x4)
            }
            99 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpImageWrite(x0, x1, x2, x3)
            }
            100 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImage(x0, x1, x2)
            }
            101 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryFormat(x0, x1, x2)
            }
            102 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryOrder(x0, x1, x2)
            }
            103 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpImageQuerySizeLod(x0, x1, x2, x3)
            }
            104 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageQuerySize(x0, x1, x2)
            }
            105 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpImageQueryLod(x0, x1, x2, x3)
            }
            106 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageQueryLevels(x0, x1, x2)
            }
            107 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageQuerySamples(x0, x1, x2)
            }
            109 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertFToU(x0, x1, x2)
            }
            110 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertFToS(x0, x1, x2)
            }
            111 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSToF(x0, x1, x2)
            }
            112 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToF(x0, x1, x2)
            }
            113 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpUConvert(x0, x1, x2)
            }
            114 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSConvert(x0, x1, x2)
            }
            115 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFConvert(x0, x1, x2)
            }
            116 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpQuantizeToF16(x0, x1, x2)
            }
            117 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertPtrToU(x0, x1, x2)
            }
            118 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSatConvertSToU(x0, x1, x2)
            }
            119 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSatConvertUToS(x0, x1, x2)
            }
            120 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToPtr(x0, x1, x2)
            }
            121 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpPtrCastToGeneric(x0, x1, x2)
            }
            122 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpGenericCastToPtr(x0, x1, x2)
            }
            123 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGenericCastToPtrExplicit(x0, x1, x2, x3)
            }
            124 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpBitcast(x0, x1, x2)
            }
            126 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSNegate(x0, x1, x2)
            }
            127 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFNegate(x0, x1, x2)
            }
            128 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAdd(x0, x1, x2, x3)
            }
            129 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFAdd(x0, x1, x2, x3)
            }
            130 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISub(x0, x1, x2, x3)
            }
            131 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFSub(x0, x1, x2, x3)
            }
            132 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIMul(x0, x1, x2, x3)
            }
            133 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFMul(x0, x1, x2, x3)
            }
            134 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUDiv(x0, x1, x2, x3)
            }
            135 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSDiv(x0, x1, x2, x3)
            }
            136 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFDiv(x0, x1, x2, x3)
            }
            137 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMod(x0, x1, x2, x3)
            }
            138 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSRem(x0, x1, x2, x3)
            }
            139 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSMod(x0, x1, x2, x3)
            }
            140 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFRem(x0, x1, x2, x3)
            }
            141 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFMod(x0, x1, x2, x3)
            }
            142 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorTimesScalar(x0, x1, x2, x3)
            }
            143 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesScalar(x0, x1, x2, x3)
            }
            144 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVectorTimesMatrix(x0, x1, x2, x3)
            }
            145 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesVector(x0, x1, x2, x3)
            }
            146 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpMatrixTimesMatrix(x0, x1, x2, x3)
            }
            147 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpOuterProduct(x0, x1, x2, x3)
            }
            148 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpDot(x0, x1, x2, x3)
            }
            149 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAddCarry(x0, x1, x2, x3)
            }
            150 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISubBorrow(x0, x1, x2, x3)
            }
            151 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMulExtended(x0, x1, x2, x3)
            }
            152 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSMulExtended(x0, x1, x2, x3)
            }
            154 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpAny(x0, x1, x2)
            }
            155 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpAll(x0, x1, x2)
            }
            156 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsNan(x0, x1, x2)
            }
            157 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsInf(x0, x1, x2)
            }
            158 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsFinite(x0, x1, x2)
            }
            159 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsNormal(x0, x1, x2)
            }
            160 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSignBitSet(x0, x1, x2)
            }
            161 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLessOrGreater(x0, x1, x2, x3)
            }
            162 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpOrdered(x0, x1, x2, x3)
            }
            163 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUnordered(x0, x1, x2, x3)
            }
            164 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalEqual(x0, x1, x2, x3)
            }
            165 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalNotEqual(x0, x1, x2, x3)
            }
            166 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalOr(x0, x1, x2, x3)
            }
            167 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpLogicalAnd(x0, x1, x2, x3)
            }
            168 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpLogicalNot(x0, x1, x2)
            }
            169 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSelect(x0, x1, x2, x3, x4)
            }
            170 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIEqual(x0, x1, x2, x3)
            }
            171 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpINotEqual(x0, x1, x2, x3)
            }
            172 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUGreaterThan(x0, x1, x2, x3)
            }
            173 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSGreaterThan(x0, x1, x2, x3)
            }
            174 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUGreaterThanEqual(x0, x1, x2, x3)
            }
            175 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSGreaterThanEqual(x0, x1, x2, x3)
            }
            176 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpULessThan(x0, x1, x2, x3)
            }
            177 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSLessThan(x0, x1, x2, x3)
            }
            178 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpULessThanEqual(x0, x1, x2, x3)
            }
            179 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSLessThanEqual(x0, x1, x2, x3)
            }
            180 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdEqual(x0, x1, x2, x3)
            }
            181 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordEqual(x0, x1, x2, x3)
            }
            182 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdNotEqual(x0, x1, x2, x3)
            }
            183 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordNotEqual(x0, x1, x2, x3)
            }
            184 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdLessThan(x0, x1, x2, x3)
            }
            185 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordLessThan(x0, x1, x2, x3)
            }
            186 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdGreaterThan(x0, x1, x2, x3)
            }
            187 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordGreaterThan(x0, x1, x2, x3)
            }
            188 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdLessThanEqual(x0, x1, x2, x3)
            }
            189 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordLessThanEqual(x0, x1, x2, x3)
            }
            190 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFOrdGreaterThanEqual(x0, x1, x2, x3)
            }
            191 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFUnordGreaterThanEqual(x0, x1, x2, x3)
            }
            194 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftRightLogical(x0, x1, x2, x3)
            }
            195 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftRightArithmetic(x0, x1, x2, x3)
            }
            196 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpShiftLeftLogical(x0, x1, x2, x3)
            }
            197 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseOr(x0, x1, x2, x3)
            }
            198 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseXor(x0, x1, x2, x3)
            }
            199 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBitwiseAnd(x0, x1, x2, x3)
            }
            200 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpNot(x0, x1, x2)
            }
            201 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpBitFieldInsert(x0, x1, x2, x3, x4, x5)
            }
            202 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBitFieldSExtract(x0, x1, x2, x3, x4)
            }
            203 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBitFieldUExtract(x0, x1, x2, x3, x4)
            }
            204 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpBitReverse(x0, x1, x2)
            }
            205 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpBitCount(x0, x1, x2)
            }
            207 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdx(x0, x1, x2)
            }
            208 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdy(x0, x1, x2)
            }
            209 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFwidth(x0, x1, x2)
            }
            210 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdxFine(x0, x1, x2)
            }
            211 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdyFine(x0, x1, x2)
            }
            212 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFwidthFine(x0, x1, x2)
            }
            213 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdxCoarse(x0, x1, x2)
            }
            214 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpDPdyCoarse(x0, x1, x2)
            }
            215 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFwidthCoarse(x0, x1, x2)
            }
            218 => OpEmitVertex,
            219 => OpEndPrimitive,
            220 => {
                let x0 = Asm::read_word(chunk, idx);
                OpEmitStreamVertex(x0)
            }
            221 => {
                let x0 = Asm::read_word(chunk, idx);
                OpEndStreamPrimitive(x0)
            }
            224 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpControlBarrier(x0, x1, x2)
            }
            225 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpMemoryBarrier(x0, x1)
            }
            227 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicLoad(x0, x1, x2, x3, x4)
            }
            228 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAtomicStore(x0, x1, x2, x3)
            }
            229 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicExchange(x0, x1, x2, x3, x4, x5)
            }
            230 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpAtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            231 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpAtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            232 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicIIncrement(x0, x1, x2, x3, x4)
            }
            233 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicIDecrement(x0, x1, x2, x3, x4)
            }
            234 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicIAdd(x0, x1, x2, x3, x4, x5)
            }
            235 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicISub(x0, x1, x2, x3, x4, x5)
            }
            236 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicSMin(x0, x1, x2, x3, x4, x5)
            }
            237 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicUMin(x0, x1, x2, x3, x4, x5)
            }
            238 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicSMax(x0, x1, x2, x3, x4, x5)
            }
            239 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicUMax(x0, x1, x2, x3, x4, x5)
            }
            240 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicAnd(x0, x1, x2, x3, x4, x5)
            }
            241 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicOr(x0, x1, x2, x3, x4, x5)
            }
            242 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicXor(x0, x1, x2, x3, x4, x5)
            }
            245 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpPhi(x0, x1, x2)
            }
            246 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpLoopMerge(x0, x1, x2)
            }
            247 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSelectionMerge(x0, x1)
            }
            248 => {
                let x0 = Asm::read_word(chunk, idx);
                OpLabel(x0)
            }
            249 => {
                let x0 = Asm::read_word(chunk, idx);
                OpBranch(x0)
            }
            250 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpBranchConditional(x0, x1, x2, x3)
            }
            251 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSwitch(x0, x1, x2)
            }
            252 => OpKill,
            253 => OpReturn,
            254 => {
                let x0 = Asm::read_word(chunk, idx);
                OpReturnValue(x0)
            }
            255 => OpUnreachable,
            256 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpLifetimeStart(x0, x1)
            }
            257 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpLifetimeStop(x0, x1)
            }
            259 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpGroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            260 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpGroupWaitEvents(x0, x1, x2)
            }
            261 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupAll(x0, x1, x2, x3)
            }
            262 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupAny(x0, x1, x2, x3)
            }
            263 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBroadcast(x0, x1, x2, x3, x4)
            }
            264 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIAdd(x0, x1, x2, x3, x4)
            }
            265 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFAdd(x0, x1, x2, x3, x4)
            }
            266 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMin(x0, x1, x2, x3, x4)
            }
            267 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMin(x0, x1, x2, x3, x4)
            }
            268 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMin(x0, x1, x2, x3, x4)
            }
            269 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMax(x0, x1, x2, x3, x4)
            }
            270 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMax(x0, x1, x2, x3, x4)
            }
            271 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMax(x0, x1, x2, x3, x4)
            }
            274 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReadPipe(x0, x1, x2, x3, x4, x5)
            }
            275 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpWritePipe(x0, x1, x2, x3, x4, x5)
            }
            276 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            277 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            278 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReserveReadPipePackets(x0, x1, x2, x3, x4, x5)
            }
            279 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpReserveWritePipePackets(x0, x1, x2, x3, x4, x5)
            }
            280 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpCommitReadPipe(x0, x1, x2, x3)
            }
            281 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpCommitWritePipe(x0, x1, x2, x3)
            }
            282 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsValidReserveId(x0, x1, x2)
            }
            283 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGetNumPipePackets(x0, x1, x2, x3, x4)
            }
            284 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGetMaxPipePackets(x0, x1, x2, x3, x4)
            }
            285 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6)
            }
            286 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6)
            }
            287 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupCommitReadPipe(x0, x1, x2, x3, x4)
            }
            288 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupCommitWritePipe(x0, x1, x2, x3, x4)
            }
            291 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpEnqueueMarker(x0, x1, x2, x3, x4, x5)
            }
            292 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                let x12 = Asm::read_word(chunk, idx);
                OpEnqueueKernel(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12)
            }
            293 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6)
            }
            294 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6)
            }
            295 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5)
            }
            296 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelPreferredWorkGroupSizeMultiple(x0, x1, x2, x3, x4, x5)
            }
            297 => {
                let x0 = Asm::read_word(chunk, idx);
                OpRetainEvent(x0)
            }
            298 => {
                let x0 = Asm::read_word(chunk, idx);
                OpReleaseEvent(x0)
            }
            299 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpCreateUserEvent(x0, x1)
            }
            300 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpIsValidEvent(x0, x1, x2)
            }
            301 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSetUserEventStatus(x0, x1)
            }
            302 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCaptureEventProfilingInfo(x0, x1, x2)
            }
            303 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpGetDefaultQueue(x0, x1)
            }
            304 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpBuildNDRange(x0, x1, x2, x3, x4)
            }
            305 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleImplicitLod(x0, x1, x2, x3, x4)
            }
            306 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleExplicitLod(x0, x1, x2, x3, x4)
            }
            307 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5)
            }
            308 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5)
            }
            309 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4)
            }
            310 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4)
            }
            311 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5)
            }
            312 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5)
            }
            313 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseFetch(x0, x1, x2, x3, x4)
            }
            314 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseGather(x0, x1, x2, x3, x4, x5)
            }
            315 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpImageSparseDrefGather(x0, x1, x2, x3, x4, x5)
            }
            316 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpImageSparseTexelsResident(x0, x1, x2)
            }
            317 => OpNoLine,
            318 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpAtomicFlagTestAndSet(x0, x1, x2, x3, x4)
            }
            319 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpAtomicFlagClear(x0, x1, x2)
            }
            320 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpImageSparseRead(x0, x1, x2, x3, x4)
            }
            321 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSizeOf(x0, x1, x2)
            }
            322 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypePipeStorage(x0)
            }
            323 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpConstantPipeStorage(x0, x1, x2, x3, x4)
            }
            324 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCreatePipeFromPipeStorage(x0, x1, x2)
            }
            325 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpGetKernelLocalSizeForSubgroupCount(x0, x1, x2, x3, x4, x5, x6)
            }
            326 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5)
            }
            327 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeNamedBarrier(x0)
            }
            328 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpNamedBarrierInitialize(x0, x1, x2)
            }
            329 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpMemoryNamedBarrier(x0, x1, x2)
            }
            330 => {
                let x0 = Asm::read_word(chunk, idx);
                OpModuleProcessed(x0)
            }
            331 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpExecutionModeId(x0, x1)
            }
            332 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpDecorateId(x0, x1)
            }
            333 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpGroupNonUniformElect(x0, x1, x2)
            }
            334 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAll(x0, x1, x2, x3)
            }
            335 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAny(x0, x1, x2, x3)
            }
            336 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformAllEqual(x0, x1, x2, x3)
            }
            337 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBroadcast(x0, x1, x2, x3, x4)
            }
            338 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBroadcastFirst(x0, x1, x2, x3)
            }
            339 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallot(x0, x1, x2, x3)
            }
            340 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformInverseBallot(x0, x1, x2, x3)
            }
            341 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4)
            }
            342 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotBitCount(x0, x1, x2, x3, x4)
            }
            343 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotFindLSB(x0, x1, x2, x3)
            }
            344 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBallotFindMSB(x0, x1, x2, x3)
            }
            345 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffle(x0, x1, x2, x3, x4)
            }
            346 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleXor(x0, x1, x2, x3, x4)
            }
            347 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleUp(x0, x1, x2, x3, x4)
            }
            348 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformShuffleDown(x0, x1, x2, x3, x4)
            }
            349 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformIAdd(x0, x1, x2, x3, x4, x5)
            }
            350 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFAdd(x0, x1, x2, x3, x4, x5)
            }
            351 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformIMul(x0, x1, x2, x3, x4, x5)
            }
            352 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMul(x0, x1, x2, x3, x4, x5)
            }
            353 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformSMin(x0, x1, x2, x3, x4, x5)
            }
            354 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformUMin(x0, x1, x2, x3, x4, x5)
            }
            355 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMin(x0, x1, x2, x3, x4, x5)
            }
            356 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformSMax(x0, x1, x2, x3, x4, x5)
            }
            357 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformUMax(x0, x1, x2, x3, x4, x5)
            }
            358 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformFMax(x0, x1, x2, x3, x4, x5)
            }
            359 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5)
            }
            360 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5)
            }
            361 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5)
            }
            362 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5)
            }
            363 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5)
            }
            364 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpGroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5)
            }
            365 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4)
            }
            366 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupNonUniformQuadSwap(x0, x1, x2, x3, x4)
            }
            400 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCopyLogical(x0, x1, x2)
            }
            401 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrEqual(x0, x1, x2, x3)
            }
            402 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrNotEqual(x0, x1, x2, x3)
            }
            403 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpPtrDiff(x0, x1, x2, x3)
            }
            4416 => OpTerminateInvocation,
            4421 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupBallotKHR(x0, x1, x2)
            }
            4422 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupFirstInvocationKHR(x0, x1, x2)
            }
            4428 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAllKHR(x0, x1, x2)
            }
            4429 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAnyKHR(x0, x1, x2)
            }
            4430 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAllEqualKHR(x0, x1, x2)
            }
            4432 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupReadInvocationKHR(x0, x1, x2, x3)
            }
            4445 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                OpTraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10)
            }
            4446 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpExecuteCallableKHR(x0, x1)
            }
            4447 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToAccelerationStructureKHR(x0, x1, x2)
            }
            4448 => OpIgnoreIntersectionKHR,
            4449 => OpTerminateRayKHR,
            4450 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSDot(x0, x1, x2, x3, x4)
            }
            4451 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpUDot(x0, x1, x2, x3, x4)
            }
            4452 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSUDot(x0, x1, x2, x3, x4)
            }
            4453 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSDotAccSat(x0, x1, x2, x3, x4, x5)
            }
            4454 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpUDotAccSat(x0, x1, x2, x3, x4, x5)
            }
            4455 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSUDotAccSat(x0, x1, x2, x3, x4, x5)
            }
            4472 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeRayQueryKHR(x0)
            }
            4473 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpRayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            4474 => {
                let x0 = Asm::read_word(chunk, idx);
                OpRayQueryTerminateKHR(x0)
            }
            4475 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpRayQueryGenerateIntersectionKHR(x0, x1)
            }
            4476 => {
                let x0 = Asm::read_word(chunk, idx);
                OpRayQueryConfirmIntersectionKHR(x0)
            }
            4477 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryProceedKHR(x0, x1, x2)
            }
            4479 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionTypeKHR(x0, x1, x2, x3)
            }
            5000 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIAddNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5001 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFAddNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5002 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMinNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5003 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMinNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5004 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMinNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5005 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMaxNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5006 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupUMaxNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5007 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupSMaxNonUniformAMD(x0, x1, x2, x3, x4)
            }
            5011 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFragmentMaskFetchAMD(x0, x1, x2, x3)
            }
            5012 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpFragmentFetchAMD(x0, x1, x2, x3, x4)
            }
            5056 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpReadClockKHR(x0, x1, x2)
            }
            5283 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6)
            }
            5296 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpGroupNonUniformPartitionNV(x0, x1, x2)
            }
            5299 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpWritePackedPrimitiveIndices4x8NV(x0, x1)
            }
            5334 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpReportIntersectionNV(x0, x1, x2, x3)
            }
            5335 => OpIgnoreIntersectionNV,
            5336 => OpTerminateRayNV,
            5337 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                OpTraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10)
            }
            5338 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                OpTraceMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11)
            }
            5339 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                OpTraceRayMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11)
            }
            5341 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAccelerationStructureNV(x0)
            }
            5344 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpExecuteCallableNV(x0, x1)
            }
            5358 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpTypeCooperativeMatrixNV(x0, x1, x2, x3, x4)
            }
            5359 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5)
            }
            5360 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixStoreNV(x0, x1, x2, x3, x4)
            }
            5361 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixMulAddNV(x0, x1, x2, x3, x4)
            }
            5362 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCooperativeMatrixLengthNV(x0, x1, x2)
            }
            5364 => OpBeginInvocationInterlockEXT,
            5365 => OpEndInvocationInterlockEXT,
            5380 => OpDemoteToHelperInvocation,
            5381 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpIsHelperInvocationEXT(x0, x1)
            }
            5391 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToImageNV(x0, x1, x2)
            }
            5392 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToSamplerNV(x0, x1, x2)
            }
            5393 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertImageToUNV(x0, x1, x2)
            }
            5394 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSamplerToUNV(x0, x1, x2)
            }
            5395 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertUToSampledImageNV(x0, x1, x2)
            }
            5396 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConvertSampledImageToUNV(x0, x1, x2)
            }
            5397 => {
                let x0 = Asm::read_word(chunk, idx);
                OpSamplerImageAddressingModeNV(x0)
            }
            5571 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleINTEL(x0, x1, x2, x3)
            }
            5572 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleDownINTEL(x0, x1, x2, x3, x4)
            }
            5573 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleUpINTEL(x0, x1, x2, x3, x4)
            }
            5574 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupShuffleXorINTEL(x0, x1, x2, x3)
            }
            5575 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupBlockReadINTEL(x0, x1, x2)
            }
            5576 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupBlockWriteINTEL(x0, x1)
            }
            5577 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupImageBlockReadINTEL(x0, x1, x2, x3)
            }
            5578 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupImageBlockWriteINTEL(x0, x1, x2)
            }
            5580 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5)
            }
            5581 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4)
            }
            5585 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpUCountLeadingZerosINTEL(x0, x1, x2)
            }
            5586 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpUCountTrailingZerosINTEL(x0, x1, x2)
            }
            5587 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAbsISubINTEL(x0, x1, x2, x3)
            }
            5588 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAbsUSubINTEL(x0, x1, x2, x3)
            }
            5589 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAddSatINTEL(x0, x1, x2, x3)
            }
            5590 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAddSatINTEL(x0, x1, x2, x3)
            }
            5591 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAverageINTEL(x0, x1, x2, x3)
            }
            5592 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAverageINTEL(x0, x1, x2, x3)
            }
            5593 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIAverageRoundedINTEL(x0, x1, x2, x3)
            }
            5594 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUAverageRoundedINTEL(x0, x1, x2, x3)
            }
            5595 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpISubSatINTEL(x0, x1, x2, x3)
            }
            5596 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUSubSatINTEL(x0, x1, x2, x3)
            }
            5597 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpIMul32x16INTEL(x0, x1, x2, x3)
            }
            5598 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpUMul32x16INTEL(x0, x1, x2, x3)
            }
            5600 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpConstantFunctionPointerINTEL(x0, x1, x2)
            }
            5601 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpFunctionPointerCallINTEL(x0, x1, x2)
            }
            5609 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpAsmTargetINTEL(x0, x1, x2)
            }
            5610 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAsmINTEL(x0, x1, x2, x3, x4, x5)
            }
            5611 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpAsmCallINTEL(x0, x1, x2, x3)
            }
            5614 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFMinEXT(x0, x1, x2, x3, x4, x5)
            }
            5615 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFMaxEXT(x0, x1, x2, x3, x4, x5)
            }
            5630 => {
                let x0 = Asm::read_word(chunk, idx);
                OpAssumeTrueKHR(x0)
            }
            5631 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpExpectKHR(x0, x1, x2, x3)
            }
            5632 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpDecorateString(x0, x1)
            }
            5633 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpMemberDecorateString(x0, x1, x2)
            }
            5699 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpVmeImageINTEL(x0, x1, x2, x3)
            }
            5700 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeVmeImageINTEL(x0, x1)
            }
            5701 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImePayloadINTEL(x0)
            }
            5702 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcRefPayloadINTEL(x0)
            }
            5703 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcSicPayloadINTEL(x0)
            }
            5704 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcMcePayloadINTEL(x0)
            }
            5705 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcMceResultINTEL(x0)
            }
            5706 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImeResultINTEL(x0)
            }
            5707 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImeResultSingleReferenceStreamoutINTEL(x0)
            }
            5708 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImeResultDualReferenceStreamoutINTEL(x0)
            }
            5709 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImeSingleReferenceStreaminINTEL(x0)
            }
            5710 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcImeDualReferenceStreaminINTEL(x0)
            }
            5711 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcRefResultINTEL(x0)
            }
            5712 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeAvcSicResultINTEL(x0)
            }
            5713 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3)
            }
            5714 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3)
            }
            5715 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x0, x1, x2, x3)
            }
            5716 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3)
            }
            5717 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x0, x1, x2, x3)
            }
            5718 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3)
            }
            5719 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x0, x1, x2, x3)
            }
            5720 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(x0, x1, x2, x3)
            }
            5721 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1)
            }
            5722 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1)
            }
            5723 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1)
            }
            5724 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(x0, x1, x2, x3, x4, x5)
            }
            5725 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x0, x1, x2, x3)
            }
            5726 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1)
            }
            5727 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x0, x1)
            }
            5728 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2)
            }
            5729 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(x0, x1, x2, x3)
            }
            5730 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3)
            }
            5731 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4)
            }
            5732 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2)
            }
            5733 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2)
            }
            5734 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2)
            }
            5735 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2)
            }
            5736 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2)
            }
            5737 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2)
            }
            5738 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2)
            }
            5739 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2)
            }
            5740 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2)
            }
            5741 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2)
            }
            5742 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2)
            }
            5743 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2)
            }
            5744 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2)
            }
            5745 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2)
            }
            5746 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4)
            }
            5747 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4)
            }
            5748 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5749 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5)
            }
            5750 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3)
            }
            5751 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5)
            }
            5752 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2)
            }
            5753 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3)
            }
            5754 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2)
            }
            5755 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(x0, x1, x2, x3)
            }
            5756 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3)
            }
            5757 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5758 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5)
            }
            5759 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5)
            }
            5760 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5, x6)
            }
            5761 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(x0, x1, x2, x3, x4)
            }
            5762 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(x0, x1, x2, x3, x4, x5)
            }
            5763 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5)
            }
            5764 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                    x0, x1, x2, x3, x4, x5, x6,
                )
            }
            5765 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2)
            }
            5766 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2)
            }
            5767 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2)
            }
            5768 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2)
            }
            5769 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2)
            }
            5770 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                    x0, x1, x2, x3,
                )
            }
            5771 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                    x0, x1, x2, x3,
                )
            }
            5772 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                    x0, x1, x2, x3,
                )
            }
            5773 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                    x0, x1, x2, x3, x4,
                )
            }
            5774 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                    x0, x1, x2, x3, x4,
                )
            }
            5775 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                    x0, x1, x2, x3, x4,
                )
            }
            5776 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3)
            }
            5777 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2)
            }
            5778 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(x0, x1, x2)
            }
            5779 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(x0, x1, x2)
            }
            5780 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x0, x1, x2)
            }
            5781 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpSubgroupAvcFmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5782 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpSubgroupAvcBmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5783 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2)
            }
            5784 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2)
            }
            5785 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2)
            }
            5786 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5787 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5)
            }
            5788 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5789 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5)
            }
            5790 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2)
            }
            5791 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicInitializeINTEL(x0, x1, x2)
            }
            5792 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureSkcINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5793 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureIpeLumaINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5794 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                let x10 = Asm::read_word(chunk, idx);
                let x11 = Asm::read_word(chunk, idx);
                let x12 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConfigureIpeLumaChromaINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
                )
            }
            5795 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3)
            }
            5796 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2)
            }
            5797 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3)
            }
            5798 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(x0, x1, x2, x3, x4, x5)
            }
            5799 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x0, x1, x2, x3)
            }
            5800 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2)
            }
            5801 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(x0, x1, x2, x3)
            }
            5802 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3)
            }
            5803 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3)
            }
            5804 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5805 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5)
            }
            5806 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4)
            }
            5807 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5)
            }
            5808 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2)
            }
            5809 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2)
            }
            5810 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2)
            }
            5811 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2)
            }
            5812 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2)
            }
            5813 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2)
            }
            5814 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x0, x1, x2)
            }
            5815 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2)
            }
            5816 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpSubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2)
            }
            5818 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpVariableLengthArrayINTEL(x0, x1, x2)
            }
            5819 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpSaveMemoryINTEL(x0, x1)
            }
            5820 => {
                let x0 = Asm::read_word(chunk, idx);
                OpRestoreMemoryINTEL(x0)
            }
            5840 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5841 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5842 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastFromIntINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5843 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6)
            }
            5846 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatAddINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5847 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSubINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5848 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatMulINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5849 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatDivINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5850 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5)
            }
            5851 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5)
            }
            5852 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5)
            }
            5853 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5)
            }
            5854 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5)
            }
            5855 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5856 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5857 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5858 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatHypotINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5859 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5860 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5861 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5862 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5863 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5864 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5865 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5866 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5867 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5868 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5869 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5870 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5871 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5872 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5873 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5874 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5875 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5876 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5877 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5878 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7)
            }
            5879 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatATan2INTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5880 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5881 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                let x9 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowRINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9)
            }
            5882 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5887 => {
                let x0 = Asm::read_word(chunk, idx);
                OpLoopControlINTEL(x0)
            }
            5911 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpAliasDomainDeclINTEL(x0, x1)
            }
            5912 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpAliasScopeDeclINTEL(x0, x1, x2)
            }
            5913 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpAliasScopeListDeclINTEL(x0, x1)
            }
            5923 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5924 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5925 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5926 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5927 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5928 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5929 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5930 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5931 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5932 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5933 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                let x6 = Asm::read_word(chunk, idx);
                let x7 = Asm::read_word(chunk, idx);
                let x8 = Asm::read_word(chunk, idx);
                OpFixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8)
            }
            5934 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpPtrCastToCrossWorkgroupINTEL(x0, x1, x2)
            }
            5938 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpCrossWorkgroupCastToPtrINTEL(x0, x1, x2)
            }
            5946 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpReadPipeBlockingINTEL(x0, x1, x2, x3)
            }
            5947 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpWritePipeBlockingINTEL(x0, x1, x2, x3)
            }
            5949 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpFPGARegINTEL(x0, x1, x2, x3)
            }
            6016 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetRayTMinKHR(x0, x1, x2)
            }
            6017 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetRayFlagsKHR(x0, x1, x2)
            }
            6018 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionTKHR(x0, x1, x2, x3)
            }
            6019 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceCustomIndexKHR(x0, x1, x2, x3)
            }
            6020 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3)
            }
            6021 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(x0, x1, x2, x3)
            }
            6022 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3)
            }
            6023 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3)
            }
            6024 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3)
            }
            6025 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3)
            }
            6026 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2)
            }
            6027 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3)
            }
            6028 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3)
            }
            6029 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetWorldRayDirectionKHR(x0, x1, x2)
            }
            6030 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpRayQueryGetWorldRayOriginKHR(x0, x1, x2)
            }
            6031 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3)
            }
            6032 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                OpRayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3)
            }
            6035 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                let x5 = Asm::read_word(chunk, idx);
                OpAtomicFAddEXT(x0, x1, x2, x3, x4, x5)
            }
            6086 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                OpTypeBufferSurfaceINTEL(x0, x1)
            }
            6090 => {
                let x0 = Asm::read_word(chunk, idx);
                OpTypeStructContinuedINTEL(x0)
            }
            6091 => {
                let x0 = Asm::read_word(chunk, idx);
                OpConstantCompositeContinuedINTEL(x0)
            }
            6092 => {
                let x0 = Asm::read_word(chunk, idx);
                OpSpecConstantCompositeContinuedINTEL(x0)
            }
            6142 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpControlBarrierArriveINTEL(x0, x1, x2)
            }
            6143 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                OpControlBarrierWaitINTEL(x0, x1, x2)
            }
            6401 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupIMulKHR(x0, x1, x2, x3, x4)
            }
            6402 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupFMulKHR(x0, x1, x2, x3, x4)
            }
            6403 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseAndKHR(x0, x1, x2, x3, x4)
            }
            6404 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseOrKHR(x0, x1, x2, x3, x4)
            }
            6405 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupBitwiseXorKHR(x0, x1, x2, x3, x4)
            }
            6406 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalAndKHR(x0, x1, x2, x3, x4)
            }
            6407 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalOrKHR(x0, x1, x2, x3, x4)
            }
            6408 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                let x3 = Asm::read_word(chunk, idx);
                let x4 = Asm::read_word(chunk, idx);
                OpGroupLogicalXorKHR(x0, x1, x2, x3, x4)
            }
            what => panic!("{:?}", what),
        };
        assert_eq!(*idx - mark, len);
        re
    }
}
impl BitField for ImageOperandsBits {
    fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        use ImageOperandsBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => {}
            Bias(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            Lod(x0) => {
                x0.write_word(sink, req);
            }
            Grad(x0, x1) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            ConstOffset(x0) => {
                x0.write_word(sink, req);
            }
            Offset(x0) => {
                req.add_cap(Capability::ImageGatherExtended);
                x0.write_word(sink, req);
            }
            ConstOffsets(x0) => {
                req.add_cap(Capability::ImageGatherExtended);
                x0.write_word(sink, req);
            }
            Sample(x0) => {
                x0.write_word(sink, req);
            }
            MinLod(x0) => {
                req.add_cap(Capability::MinLod);
                x0.write_word(sink, req);
            }
            MakeTexelAvailable(x0) => {
                req.add_cap(Capability::VulkanMemoryModel);
                x0.write_word(sink, req);
            }
            MakeTexelVisible(x0) => {
                req.add_cap(Capability::VulkanMemoryModel);
                x0.write_word(sink, req);
            }
            NonPrivateTexel => {
                req.add_cap(Capability::VulkanMemoryModel);
            }
            VolatileTexel => {
                req.add_cap(Capability::VulkanMemoryModel);
            }
            SignExtend => {}
            ZeroExtend => {}
            Nontemporal => {}
            Offsets(x0) => {
                x0.write_word(sink, req);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], opc: &mut u32, idx: &mut usize) -> Self {
        use ImageOperandsBits::*;
        let this = 1 << opc.trailing_zeros();
        *opc &= !this;
        match this {
            0 => None,
            1 => {
                let x0 = Asm::read_word(chunk, idx);
                Bias(x0)
            }
            2 => {
                let x0 = Asm::read_word(chunk, idx);
                Lod(x0)
            }
            4 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                Grad(x0, x1)
            }
            8 => {
                let x0 = Asm::read_word(chunk, idx);
                ConstOffset(x0)
            }
            16 => {
                let x0 = Asm::read_word(chunk, idx);
                Offset(x0)
            }
            32 => {
                let x0 = Asm::read_word(chunk, idx);
                ConstOffsets(x0)
            }
            64 => {
                let x0 = Asm::read_word(chunk, idx);
                Sample(x0)
            }
            128 => {
                let x0 = Asm::read_word(chunk, idx);
                MinLod(x0)
            }
            256 => {
                let x0 = Asm::read_word(chunk, idx);
                MakeTexelAvailable(x0)
            }
            512 => {
                let x0 = Asm::read_word(chunk, idx);
                MakeTexelVisible(x0)
            }
            1024 => NonPrivateTexel,
            2048 => VolatileTexel,
            4096 => SignExtend,
            8192 => ZeroExtend,
            16384 => Nontemporal,
            65536 => {
                let x0 = Asm::read_word(chunk, idx);
                Offsets(x0)
            }
            what => panic!("{:?}", what),
        }
    }
}
impl FPFastMathMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<FPFastMathMode> for FPFastMathMode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<FPFastMathMode> for FPFastMathMode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for FPFastMathMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl SelectionControl {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<SelectionControl> for SelectionControl {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<SelectionControl> for SelectionControl {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for SelectionControl {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl BitField for LoopControlBits {
    fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        use LoopControlBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => {}
            Unroll => {}
            DontUnroll => {}
            DependencyInfinite => {}
            DependencyLength(x0) => {
                x0.write_word(sink, req);
            }
            MinIterations(x0) => {
                x0.write_word(sink, req);
            }
            MaxIterations(x0) => {
                x0.write_word(sink, req);
            }
            IterationMultiple(x0) => {
                x0.write_word(sink, req);
            }
            PeelCount(x0) => {
                x0.write_word(sink, req);
            }
            PartialCount(x0) => {
                x0.write_word(sink, req);
            }
            InitiationIntervalINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            MaxConcurrencyINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            DependencyArrayINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            PipelineEnableINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            LoopCoalesceINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            MaxInterleavingINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            SpeculatedIterationsINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            NoFusionINTEL(x0) => {
                req.add_cap(Capability::FPGALoopControlsINTEL);
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
                x0.write_word(sink, req);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], opc: &mut u32, idx: &mut usize) -> Self {
        use LoopControlBits::*;
        let this = 1 << opc.trailing_zeros();
        *opc &= !this;
        match this {
            0 => None,
            1 => Unroll,
            2 => DontUnroll,
            4 => DependencyInfinite,
            8 => {
                let x0 = Asm::read_word(chunk, idx);
                DependencyLength(x0)
            }
            16 => {
                let x0 = Asm::read_word(chunk, idx);
                MinIterations(x0)
            }
            32 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxIterations(x0)
            }
            64 => {
                let x0 = Asm::read_word(chunk, idx);
                IterationMultiple(x0)
            }
            128 => {
                let x0 = Asm::read_word(chunk, idx);
                PeelCount(x0)
            }
            256 => {
                let x0 = Asm::read_word(chunk, idx);
                PartialCount(x0)
            }
            65536 => {
                let x0 = Asm::read_word(chunk, idx);
                InitiationIntervalINTEL(x0)
            }
            131072 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxConcurrencyINTEL(x0)
            }
            262144 => {
                let x0 = Asm::read_word(chunk, idx);
                DependencyArrayINTEL(x0)
            }
            524288 => {
                let x0 = Asm::read_word(chunk, idx);
                PipelineEnableINTEL(x0)
            }
            1048576 => {
                let x0 = Asm::read_word(chunk, idx);
                LoopCoalesceINTEL(x0)
            }
            2097152 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxInterleavingINTEL(x0)
            }
            4194304 => {
                let x0 = Asm::read_word(chunk, idx);
                SpeculatedIterationsINTEL(x0)
            }
            8388608 => {
                let x0 = Asm::read_word(chunk, idx);
                NoFusionINTEL(x0)
            }
            what => panic!("{:?}", what),
        }
    }
}
impl FunctionControl {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<FunctionControl> for FunctionControl {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<FunctionControl> for FunctionControl {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for FunctionControl {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl MemorySemantics {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<MemorySemantics> for MemorySemantics {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<MemorySemantics> for MemorySemantics {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for MemorySemantics {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl BitField for MemoryAccessBits {
    fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        use MemoryAccessBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => {}
            Volatile => {}
            Aligned(x0) => {
                x0.write_word(sink, req);
            }
            Nontemporal => {}
            MakePointerAvailable(x0) => {
                req.add_cap(Capability::VulkanMemoryModel);
                x0.write_word(sink, req);
            }
            MakePointerVisible(x0) => {
                req.add_cap(Capability::VulkanMemoryModel);
                x0.write_word(sink, req);
            }
            NonPrivatePointer => {
                req.add_cap(Capability::VulkanMemoryModel);
            }
            AliasScopeINTELMask(x0) => {
                req.add_cap(Capability::MemoryAccessAliasingINTEL);
                req.add_ext("SPV_INTEL_memory_access_aliasing".to_string());
                x0.write_word(sink, req);
            }
            NoAliasINTELMask(x0) => {
                req.add_cap(Capability::MemoryAccessAliasingINTEL);
                req.add_ext("SPV_INTEL_memory_access_aliasing".to_string());
                x0.write_word(sink, req);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], opc: &mut u32, idx: &mut usize) -> Self {
        use MemoryAccessBits::*;
        let this = 1 << opc.trailing_zeros();
        *opc &= !this;
        match this {
            0 => None,
            1 => Volatile,
            2 => {
                let x0 = Asm::read_word(chunk, idx);
                Aligned(x0)
            }
            4 => Nontemporal,
            8 => {
                let x0 = Asm::read_word(chunk, idx);
                MakePointerAvailable(x0)
            }
            16 => {
                let x0 = Asm::read_word(chunk, idx);
                MakePointerVisible(x0)
            }
            32 => NonPrivatePointer,
            65536 => {
                let x0 = Asm::read_word(chunk, idx);
                AliasScopeINTELMask(x0)
            }
            131072 => {
                let x0 = Asm::read_word(chunk, idx);
                NoAliasINTELMask(x0)
            }
            what => panic!("{:?}", what),
        }
    }
}
impl KernelProfilingInfo {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<KernelProfilingInfo> for KernelProfilingInfo {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<KernelProfilingInfo> for KernelProfilingInfo {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for KernelProfilingInfo {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl RayFlags {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<RayFlags> for RayFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<RayFlags> for RayFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for RayFlags {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl FragmentShadingRate {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl std::ops::BitOr<FragmentShadingRate> for FragmentShadingRate {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitAnd<FragmentShadingRate> for FragmentShadingRate {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Asm for FragmentShadingRate {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        unsafe { std::mem::transmute_copy(&chunk[*idx as usize - 1]) }
    }
}
impl SourceLanguage {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for SourceLanguage {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use SourceLanguage::*;
        match self {
            Unknown => {}
            ESSL => {}
            GLSL => {}
            OpenCL_C => {}
            OpenCL_CPP => {}
            HLSL => {}
            CPP_for_OpenCL => {}
            SYCL => {}
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use SourceLanguage::*;
        match chunk[*idx as usize - 1] {
            0 => Unknown,
            1 => ESSL,
            2 => GLSL,
            3 => OpenCL_C,
            4 => OpenCL_CPP,
            5 => HLSL,
            6 => CPP_for_OpenCL,
            7 => SYCL,
            what => panic!("{:?}", what),
        }
    }
}
impl ExecutionModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for ExecutionModel {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use ExecutionModel::*;
        match self {
            Vertex => {
                req.add_cap(Capability::Shader);
            }
            TessellationControl => {
                req.add_cap(Capability::Tessellation);
            }
            TessellationEvaluation => {
                req.add_cap(Capability::Tessellation);
            }
            Geometry => {
                req.add_cap(Capability::Geometry);
            }
            Fragment => {
                req.add_cap(Capability::Shader);
            }
            GLCompute => {
                req.add_cap(Capability::Shader);
            }
            Kernel => {
                req.add_cap(Capability::Kernel);
            }
            TaskNV => {
                req.add_cap(Capability::MeshShadingNV);
            }
            MeshNV => {
                req.add_cap(Capability::MeshShadingNV);
            }
            RayGenerationNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            IntersectionNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            AnyHitNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            ClosestHitNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            MissNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            CallableNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use ExecutionModel::*;
        match chunk[*idx as usize - 1] {
            0 => Vertex,
            1 => TessellationControl,
            2 => TessellationEvaluation,
            3 => Geometry,
            4 => Fragment,
            5 => GLCompute,
            6 => Kernel,
            5267 => TaskNV,
            5268 => MeshNV,
            5313 => RayGenerationNV,
            5314 => IntersectionNV,
            5315 => AnyHitNV,
            5316 => ClosestHitNV,
            5317 => MissNV,
            5318 => CallableNV,
            what => panic!("{:?}", what),
        }
    }
}
impl AddressingModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for AddressingModel {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use AddressingModel::*;
        match self {
            Logical => {}
            Physical32 => {
                req.add_cap(Capability::Addresses);
            }
            Physical64 => {
                req.add_cap(Capability::Addresses);
            }
            PhysicalStorageBuffer64 => {
                req.add_cap(Capability::PhysicalStorageBufferAddresses);
                req.add_ext("SPV_EXT_physical_storage_buffer".to_string());
                req.add_ext("SPV_KHR_physical_storage_buffer".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use AddressingModel::*;
        match chunk[*idx as usize - 1] {
            0 => Logical,
            1 => Physical32,
            2 => Physical64,
            5348 => PhysicalStorageBuffer64,
            what => panic!("{:?}", what),
        }
    }
}
impl MemoryModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for MemoryModel {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use MemoryModel::*;
        match self {
            Simple => {
                req.add_cap(Capability::Shader);
            }
            GLSL450 => {
                req.add_cap(Capability::Shader);
            }
            OpenCL => {
                req.add_cap(Capability::Kernel);
            }
            Vulkan => {
                req.add_cap(Capability::VulkanMemoryModel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use MemoryModel::*;
        match chunk[*idx as usize - 1] {
            0 => Simple,
            1 => GLSL450,
            2 => OpenCL,
            3 => Vulkan,
            what => panic!("{:?}", what),
        }
    }
}
impl ExecutionMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for ExecutionMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use ExecutionMode::*;
        match self {
            Invocations(x0) => {
                req.add_cap(Capability::Geometry);
                x0.write_word(sink, req);
            }
            SpacingEqual => {
                req.add_cap(Capability::Tessellation);
            }
            SpacingFractionalEven => {
                req.add_cap(Capability::Tessellation);
            }
            SpacingFractionalOdd => {
                req.add_cap(Capability::Tessellation);
            }
            VertexOrderCw => {
                req.add_cap(Capability::Tessellation);
            }
            VertexOrderCcw => {
                req.add_cap(Capability::Tessellation);
            }
            PixelCenterInteger => {
                req.add_cap(Capability::Shader);
            }
            OriginUpperLeft => {
                req.add_cap(Capability::Shader);
            }
            OriginLowerLeft => {
                req.add_cap(Capability::Shader);
            }
            EarlyFragmentTests => {
                req.add_cap(Capability::Shader);
            }
            PointMode => {
                req.add_cap(Capability::Tessellation);
            }
            Xfb => {
                req.add_cap(Capability::TransformFeedback);
            }
            DepthReplacing => {
                req.add_cap(Capability::Shader);
            }
            DepthGreater => {
                req.add_cap(Capability::Shader);
            }
            DepthLess => {
                req.add_cap(Capability::Shader);
            }
            DepthUnchanged => {
                req.add_cap(Capability::Shader);
            }
            LocalSize(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            LocalSizeHint(x0, x1, x2) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            InputPoints => {
                req.add_cap(Capability::Geometry);
            }
            InputLines => {
                req.add_cap(Capability::Geometry);
            }
            InputLinesAdjacency => {
                req.add_cap(Capability::Geometry);
            }
            Triangles => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::Tessellation);
            }
            InputTrianglesAdjacency => {
                req.add_cap(Capability::Geometry);
            }
            Quads => {
                req.add_cap(Capability::Tessellation);
            }
            Isolines => {
                req.add_cap(Capability::Tessellation);
            }
            OutputVertices(x0) => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::Tessellation);
                req.add_cap(Capability::MeshShadingNV);
                x0.write_word(sink, req);
            }
            OutputPoints => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::MeshShadingNV);
            }
            OutputLineStrip => {
                req.add_cap(Capability::Geometry);
            }
            OutputTriangleStrip => {
                req.add_cap(Capability::Geometry);
            }
            VecTypeHint(x0) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            ContractionOff => {
                req.add_cap(Capability::Kernel);
            }
            Initializer => {
                req.add_cap(Capability::Kernel);
            }
            Finalizer => {
                req.add_cap(Capability::Kernel);
            }
            SubgroupSize(x0) => {
                req.add_cap(Capability::SubgroupDispatch);
                x0.write_word(sink, req);
            }
            SubgroupsPerWorkgroup(x0) => {
                req.add_cap(Capability::SubgroupDispatch);
                x0.write_word(sink, req);
            }
            SubgroupsPerWorkgroupId(x0) => {
                req.add_cap(Capability::SubgroupDispatch);
                x0.write_word(sink, req);
            }
            LocalSizeId(x0, x1, x2) => {
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            LocalSizeHintId(x0, x1, x2) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            SubgroupUniformControlFlowKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_subgroup_uniform_control_flow".to_string());
            }
            PostDepthCoverage => {
                req.add_cap(Capability::SampleMaskPostDepthCoverage);
                req.add_ext("SPV_KHR_post_depth_coverage".to_string());
            }
            DenormPreserve(x0) => {
                req.add_cap(Capability::DenormPreserve);
                req.add_ext("SPV_KHR_float_controls".to_string());
                x0.write_word(sink, req);
            }
            DenormFlushToZero(x0) => {
                req.add_cap(Capability::DenormFlushToZero);
                req.add_ext("SPV_KHR_float_controls".to_string());
                x0.write_word(sink, req);
            }
            SignedZeroInfNanPreserve(x0) => {
                req.add_cap(Capability::SignedZeroInfNanPreserve);
                req.add_ext("SPV_KHR_float_controls".to_string());
                x0.write_word(sink, req);
            }
            RoundingModeRTE(x0) => {
                req.add_cap(Capability::RoundingModeRTE);
                req.add_ext("SPV_KHR_float_controls".to_string());
                x0.write_word(sink, req);
            }
            RoundingModeRTZ(x0) => {
                req.add_cap(Capability::RoundingModeRTZ);
                req.add_ext("SPV_KHR_float_controls".to_string());
                x0.write_word(sink, req);
            }
            StencilRefReplacingEXT => {
                req.add_cap(Capability::StencilExportEXT);
                req.add_ext("SPV_EXT_shader_stencil_export".to_string());
            }
            OutputLinesNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            OutputPrimitivesNV(x0) => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
                x0.write_word(sink, req);
            }
            DerivativeGroupQuadsNV => {
                req.add_cap(Capability::ComputeDerivativeGroupQuadsNV);
                req.add_ext("SPV_NV_compute_shader_derivatives".to_string());
            }
            DerivativeGroupLinearNV => {
                req.add_cap(Capability::ComputeDerivativeGroupLinearNV);
                req.add_ext("SPV_NV_compute_shader_derivatives".to_string());
            }
            OutputTrianglesNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PixelInterlockOrderedEXT => {
                req.add_cap(Capability::FragmentShaderPixelInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            PixelInterlockUnorderedEXT => {
                req.add_cap(Capability::FragmentShaderPixelInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            SampleInterlockOrderedEXT => {
                req.add_cap(Capability::FragmentShaderSampleInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            SampleInterlockUnorderedEXT => {
                req.add_cap(Capability::FragmentShaderSampleInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            ShadingRateInterlockOrderedEXT => {
                req.add_cap(Capability::FragmentShaderShadingRateInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            ShadingRateInterlockUnorderedEXT => {
                req.add_cap(Capability::FragmentShaderShadingRateInterlockEXT);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            SharedLocalMemorySizeINTEL(x0) => {
                req.add_cap(Capability::VectorComputeINTEL);
                x0.write_word(sink, req);
            }
            RoundingModeRTPINTEL(x0) => {
                req.add_cap(Capability::RoundToInfinityINTEL);
                x0.write_word(sink, req);
            }
            RoundingModeRTNINTEL(x0) => {
                req.add_cap(Capability::RoundToInfinityINTEL);
                x0.write_word(sink, req);
            }
            FloatingPointModeALTINTEL(x0) => {
                req.add_cap(Capability::RoundToInfinityINTEL);
                x0.write_word(sink, req);
            }
            FloatingPointModeIEEEINTEL(x0) => {
                req.add_cap(Capability::RoundToInfinityINTEL);
                x0.write_word(sink, req);
            }
            MaxWorkgroupSizeINTEL(x0, x1, x2) => {
                req.add_cap(Capability::KernelAttributesINTEL);
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
                x0.write_word(sink, req);
                x1.write_word(sink, req);
                x2.write_word(sink, req);
            }
            MaxWorkDimINTEL(x0) => {
                req.add_cap(Capability::KernelAttributesINTEL);
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
                x0.write_word(sink, req);
            }
            NoGlobalOffsetINTEL => {
                req.add_cap(Capability::KernelAttributesINTEL);
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
            }
            NumSIMDWorkitemsINTEL(x0) => {
                req.add_cap(Capability::FPGAKernelAttributesINTEL);
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
                x0.write_word(sink, req);
            }
            SchedulerTargetFmaxMhzINTEL(x0) => {
                req.add_cap(Capability::FPGAKernelAttributesINTEL);
                x0.write_word(sink, req);
            }
            NamedBarrierCountINTEL(x0) => {
                req.add_cap(Capability::VectorComputeINTEL);
                x0.write_word(sink, req);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use ExecutionMode::*;
        match chunk[*idx as usize - 1] {
            0 => {
                let x0 = Asm::read_word(chunk, idx);
                Invocations(x0)
            }
            1 => SpacingEqual,
            2 => SpacingFractionalEven,
            3 => SpacingFractionalOdd,
            4 => VertexOrderCw,
            5 => VertexOrderCcw,
            6 => PixelCenterInteger,
            7 => OriginUpperLeft,
            8 => OriginLowerLeft,
            9 => EarlyFragmentTests,
            10 => PointMode,
            11 => Xfb,
            12 => DepthReplacing,
            14 => DepthGreater,
            15 => DepthLess,
            16 => DepthUnchanged,
            17 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                LocalSize(x0, x1, x2)
            }
            18 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                LocalSizeHint(x0, x1, x2)
            }
            19 => InputPoints,
            20 => InputLines,
            21 => InputLinesAdjacency,
            22 => Triangles,
            23 => InputTrianglesAdjacency,
            24 => Quads,
            25 => Isolines,
            26 => {
                let x0 = Asm::read_word(chunk, idx);
                OutputVertices(x0)
            }
            27 => OutputPoints,
            28 => OutputLineStrip,
            29 => OutputTriangleStrip,
            30 => {
                let x0 = Asm::read_word(chunk, idx);
                VecTypeHint(x0)
            }
            31 => ContractionOff,
            33 => Initializer,
            34 => Finalizer,
            35 => {
                let x0 = Asm::read_word(chunk, idx);
                SubgroupSize(x0)
            }
            36 => {
                let x0 = Asm::read_word(chunk, idx);
                SubgroupsPerWorkgroup(x0)
            }
            37 => {
                let x0 = Asm::read_word(chunk, idx);
                SubgroupsPerWorkgroupId(x0)
            }
            38 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                LocalSizeId(x0, x1, x2)
            }
            39 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                LocalSizeHintId(x0, x1, x2)
            }
            4421 => SubgroupUniformControlFlowKHR,
            4446 => PostDepthCoverage,
            4459 => {
                let x0 = Asm::read_word(chunk, idx);
                DenormPreserve(x0)
            }
            4460 => {
                let x0 = Asm::read_word(chunk, idx);
                DenormFlushToZero(x0)
            }
            4461 => {
                let x0 = Asm::read_word(chunk, idx);
                SignedZeroInfNanPreserve(x0)
            }
            4462 => {
                let x0 = Asm::read_word(chunk, idx);
                RoundingModeRTE(x0)
            }
            4463 => {
                let x0 = Asm::read_word(chunk, idx);
                RoundingModeRTZ(x0)
            }
            5027 => StencilRefReplacingEXT,
            5269 => OutputLinesNV,
            5270 => {
                let x0 = Asm::read_word(chunk, idx);
                OutputPrimitivesNV(x0)
            }
            5289 => DerivativeGroupQuadsNV,
            5290 => DerivativeGroupLinearNV,
            5298 => OutputTrianglesNV,
            5366 => PixelInterlockOrderedEXT,
            5367 => PixelInterlockUnorderedEXT,
            5368 => SampleInterlockOrderedEXT,
            5369 => SampleInterlockUnorderedEXT,
            5370 => ShadingRateInterlockOrderedEXT,
            5371 => ShadingRateInterlockUnorderedEXT,
            5618 => {
                let x0 = Asm::read_word(chunk, idx);
                SharedLocalMemorySizeINTEL(x0)
            }
            5620 => {
                let x0 = Asm::read_word(chunk, idx);
                RoundingModeRTPINTEL(x0)
            }
            5621 => {
                let x0 = Asm::read_word(chunk, idx);
                RoundingModeRTNINTEL(x0)
            }
            5622 => {
                let x0 = Asm::read_word(chunk, idx);
                FloatingPointModeALTINTEL(x0)
            }
            5623 => {
                let x0 = Asm::read_word(chunk, idx);
                FloatingPointModeIEEEINTEL(x0)
            }
            5893 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                let x2 = Asm::read_word(chunk, idx);
                MaxWorkgroupSizeINTEL(x0, x1, x2)
            }
            5894 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxWorkDimINTEL(x0)
            }
            5895 => NoGlobalOffsetINTEL,
            5896 => {
                let x0 = Asm::read_word(chunk, idx);
                NumSIMDWorkitemsINTEL(x0)
            }
            5903 => {
                let x0 = Asm::read_word(chunk, idx);
                SchedulerTargetFmaxMhzINTEL(x0)
            }
            6417 => {
                let x0 = Asm::read_word(chunk, idx);
                NamedBarrierCountINTEL(x0)
            }
            what => panic!("{:?}", what),
        }
    }
}
impl StorageClass {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for StorageClass {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use StorageClass::*;
        match self {
            UniformConstant => {}
            Input => {}
            Uniform => {
                req.add_cap(Capability::Shader);
            }
            Output => {
                req.add_cap(Capability::Shader);
            }
            Workgroup => {}
            CrossWorkgroup => {}
            Private => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::VectorComputeINTEL);
            }
            Function => {}
            Generic => {
                req.add_cap(Capability::GenericPointer);
            }
            PushConstant => {
                req.add_cap(Capability::Shader);
            }
            AtomicCounter => {
                req.add_cap(Capability::AtomicStorage);
            }
            Image => {}
            StorageBuffer => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_storage_buffer_storage_class".to_string());
                req.add_ext("SPV_KHR_variable_pointers".to_string());
            }
            CallableDataNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            IncomingCallableDataNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            RayPayloadNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            HitAttributeNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            IncomingRayPayloadNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            ShaderRecordBufferNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            PhysicalStorageBuffer => {
                req.add_cap(Capability::PhysicalStorageBufferAddresses);
                req.add_ext("SPV_EXT_physical_storage_buffer".to_string());
                req.add_ext("SPV_KHR_physical_storage_buffer".to_string());
            }
            CodeSectionINTEL => {
                req.add_cap(Capability::FunctionPointersINTEL);
                req.add_ext("SPV_INTEL_function_pointers".to_string());
            }
            DeviceOnlyINTEL => {
                req.add_cap(Capability::USMStorageClassesINTEL);
                req.add_ext("SPV_INTEL_usm_storage_classes".to_string());
            }
            HostOnlyINTEL => {
                req.add_cap(Capability::USMStorageClassesINTEL);
                req.add_ext("SPV_INTEL_usm_storage_classes".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use StorageClass::*;
        match chunk[*idx as usize - 1] {
            0 => UniformConstant,
            1 => Input,
            2 => Uniform,
            3 => Output,
            4 => Workgroup,
            5 => CrossWorkgroup,
            6 => Private,
            7 => Function,
            8 => Generic,
            9 => PushConstant,
            10 => AtomicCounter,
            11 => Image,
            12 => StorageBuffer,
            5328 => CallableDataNV,
            5329 => IncomingCallableDataNV,
            5338 => RayPayloadNV,
            5339 => HitAttributeNV,
            5342 => IncomingRayPayloadNV,
            5343 => ShaderRecordBufferNV,
            5349 => PhysicalStorageBuffer,
            5605 => CodeSectionINTEL,
            5936 => DeviceOnlyINTEL,
            5937 => HostOnlyINTEL,
            what => panic!("{:?}", what),
        }
    }
}
impl Dim {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for Dim {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use Dim::*;
        match self {
            _1D => {
                req.add_cap(Capability::Sampled1D);
                req.add_cap(Capability::Image1D);
            }
            _2D => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::ImageMSArray);
            }
            _3D => {}
            Cube => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::ImageCubeArray);
            }
            Rect => {
                req.add_cap(Capability::SampledRect);
                req.add_cap(Capability::ImageRect);
            }
            Buffer => {
                req.add_cap(Capability::SampledBuffer);
                req.add_cap(Capability::ImageBuffer);
            }
            SubpassData => {
                req.add_cap(Capability::InputAttachment);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use Dim::*;
        match chunk[*idx as usize - 1] {
            0 => _1D,
            1 => _2D,
            2 => _3D,
            3 => Cube,
            4 => Rect,
            5 => Buffer,
            6 => SubpassData,
            what => panic!("{:?}", what),
        }
    }
}
impl SamplerAddressingMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for SamplerAddressingMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use SamplerAddressingMode::*;
        match self {
            None => {
                req.add_cap(Capability::Kernel);
            }
            ClampToEdge => {
                req.add_cap(Capability::Kernel);
            }
            Clamp => {
                req.add_cap(Capability::Kernel);
            }
            Repeat => {
                req.add_cap(Capability::Kernel);
            }
            RepeatMirrored => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use SamplerAddressingMode::*;
        match chunk[*idx as usize - 1] {
            0 => None,
            1 => ClampToEdge,
            2 => Clamp,
            3 => Repeat,
            4 => RepeatMirrored,
            what => panic!("{:?}", what),
        }
    }
}
impl SamplerFilterMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for SamplerFilterMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use SamplerFilterMode::*;
        match self {
            Nearest => {
                req.add_cap(Capability::Kernel);
            }
            Linear => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use SamplerFilterMode::*;
        match chunk[*idx as usize - 1] {
            0 => Nearest,
            1 => Linear,
            what => panic!("{:?}", what),
        }
    }
}
impl ImageFormat {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for ImageFormat {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use ImageFormat::*;
        match self {
            Unknown => {}
            Rgba32f => {
                req.add_cap(Capability::Shader);
            }
            Rgba16f => {
                req.add_cap(Capability::Shader);
            }
            R32f => {
                req.add_cap(Capability::Shader);
            }
            Rgba8 => {
                req.add_cap(Capability::Shader);
            }
            Rgba8Snorm => {
                req.add_cap(Capability::Shader);
            }
            Rg32f => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg16f => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R11fG11fB10f => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R16f => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rgba16 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rgb10A2 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg16 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg8 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R16 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R8 => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rgba16Snorm => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg16Snorm => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg8Snorm => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R16Snorm => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R8Snorm => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rgba32i => {
                req.add_cap(Capability::Shader);
            }
            Rgba16i => {
                req.add_cap(Capability::Shader);
            }
            Rgba8i => {
                req.add_cap(Capability::Shader);
            }
            R32i => {
                req.add_cap(Capability::Shader);
            }
            Rg32i => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg16i => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg8i => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R16i => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R8i => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rgba32ui => {
                req.add_cap(Capability::Shader);
            }
            Rgba16ui => {
                req.add_cap(Capability::Shader);
            }
            Rgba8ui => {
                req.add_cap(Capability::Shader);
            }
            R32ui => {
                req.add_cap(Capability::Shader);
            }
            Rgb10a2ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg32ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg16ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            Rg8ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R16ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R8ui => {
                req.add_cap(Capability::StorageImageExtendedFormats);
            }
            R64ui => {
                req.add_cap(Capability::Int64ImageEXT);
            }
            R64i => {
                req.add_cap(Capability::Int64ImageEXT);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use ImageFormat::*;
        match chunk[*idx as usize - 1] {
            0 => Unknown,
            1 => Rgba32f,
            2 => Rgba16f,
            3 => R32f,
            4 => Rgba8,
            5 => Rgba8Snorm,
            6 => Rg32f,
            7 => Rg16f,
            8 => R11fG11fB10f,
            9 => R16f,
            10 => Rgba16,
            11 => Rgb10A2,
            12 => Rg16,
            13 => Rg8,
            14 => R16,
            15 => R8,
            16 => Rgba16Snorm,
            17 => Rg16Snorm,
            18 => Rg8Snorm,
            19 => R16Snorm,
            20 => R8Snorm,
            21 => Rgba32i,
            22 => Rgba16i,
            23 => Rgba8i,
            24 => R32i,
            25 => Rg32i,
            26 => Rg16i,
            27 => Rg8i,
            28 => R16i,
            29 => R8i,
            30 => Rgba32ui,
            31 => Rgba16ui,
            32 => Rgba8ui,
            33 => R32ui,
            34 => Rgb10a2ui,
            35 => Rg32ui,
            36 => Rg16ui,
            37 => Rg8ui,
            38 => R16ui,
            39 => R8ui,
            40 => R64ui,
            41 => R64i,
            what => panic!("{:?}", what),
        }
    }
}
impl ImageChannelOrder {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for ImageChannelOrder {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use ImageChannelOrder::*;
        match self {
            R => {
                req.add_cap(Capability::Kernel);
            }
            A => {
                req.add_cap(Capability::Kernel);
            }
            RG => {
                req.add_cap(Capability::Kernel);
            }
            RA => {
                req.add_cap(Capability::Kernel);
            }
            RGB => {
                req.add_cap(Capability::Kernel);
            }
            RGBA => {
                req.add_cap(Capability::Kernel);
            }
            BGRA => {
                req.add_cap(Capability::Kernel);
            }
            ARGB => {
                req.add_cap(Capability::Kernel);
            }
            Intensity => {
                req.add_cap(Capability::Kernel);
            }
            Luminance => {
                req.add_cap(Capability::Kernel);
            }
            Rx => {
                req.add_cap(Capability::Kernel);
            }
            RGx => {
                req.add_cap(Capability::Kernel);
            }
            RGBx => {
                req.add_cap(Capability::Kernel);
            }
            Depth => {
                req.add_cap(Capability::Kernel);
            }
            DepthStencil => {
                req.add_cap(Capability::Kernel);
            }
            sRGB => {
                req.add_cap(Capability::Kernel);
            }
            sRGBx => {
                req.add_cap(Capability::Kernel);
            }
            sRGBA => {
                req.add_cap(Capability::Kernel);
            }
            sBGRA => {
                req.add_cap(Capability::Kernel);
            }
            ABGR => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use ImageChannelOrder::*;
        match chunk[*idx as usize - 1] {
            0 => R,
            1 => A,
            2 => RG,
            3 => RA,
            4 => RGB,
            5 => RGBA,
            6 => BGRA,
            7 => ARGB,
            8 => Intensity,
            9 => Luminance,
            10 => Rx,
            11 => RGx,
            12 => RGBx,
            13 => Depth,
            14 => DepthStencil,
            15 => sRGB,
            16 => sRGBx,
            17 => sRGBA,
            18 => sBGRA,
            19 => ABGR,
            what => panic!("{:?}", what),
        }
    }
}
impl ImageChannelDataType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for ImageChannelDataType {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use ImageChannelDataType::*;
        match self {
            SnormInt8 => {
                req.add_cap(Capability::Kernel);
            }
            SnormInt16 => {
                req.add_cap(Capability::Kernel);
            }
            UnormInt8 => {
                req.add_cap(Capability::Kernel);
            }
            UnormInt16 => {
                req.add_cap(Capability::Kernel);
            }
            UnormShort565 => {
                req.add_cap(Capability::Kernel);
            }
            UnormShort555 => {
                req.add_cap(Capability::Kernel);
            }
            UnormInt101010 => {
                req.add_cap(Capability::Kernel);
            }
            SignedInt8 => {
                req.add_cap(Capability::Kernel);
            }
            SignedInt16 => {
                req.add_cap(Capability::Kernel);
            }
            SignedInt32 => {
                req.add_cap(Capability::Kernel);
            }
            UnsignedInt8 => {
                req.add_cap(Capability::Kernel);
            }
            UnsignedInt16 => {
                req.add_cap(Capability::Kernel);
            }
            UnsignedInt32 => {
                req.add_cap(Capability::Kernel);
            }
            HalfFloat => {
                req.add_cap(Capability::Kernel);
            }
            Float => {
                req.add_cap(Capability::Kernel);
            }
            UnormInt24 => {
                req.add_cap(Capability::Kernel);
            }
            UnormInt101010_2 => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use ImageChannelDataType::*;
        match chunk[*idx as usize - 1] {
            0 => SnormInt8,
            1 => SnormInt16,
            2 => UnormInt8,
            3 => UnormInt16,
            4 => UnormShort565,
            5 => UnormShort555,
            6 => UnormInt101010,
            7 => SignedInt8,
            8 => SignedInt16,
            9 => SignedInt32,
            10 => UnsignedInt8,
            11 => UnsignedInt16,
            12 => UnsignedInt32,
            13 => HalfFloat,
            14 => Float,
            15 => UnormInt24,
            16 => UnormInt101010_2,
            what => panic!("{:?}", what),
        }
    }
}
impl FPRoundingMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for FPRoundingMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use FPRoundingMode::*;
        match self {
            RTE => {}
            RTZ => {}
            RTP => {}
            RTN => {}
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use FPRoundingMode::*;
        match chunk[*idx as usize - 1] {
            0 => RTE,
            1 => RTZ,
            2 => RTP,
            3 => RTN,
            what => panic!("{:?}", what),
        }
    }
}
impl FPDenormMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for FPDenormMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use FPDenormMode::*;
        match self {
            Preserve => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
            }
            FlushToZero => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use FPDenormMode::*;
        match chunk[*idx as usize - 1] {
            0 => Preserve,
            1 => FlushToZero,
            what => panic!("{:?}", what),
        }
    }
}
impl QuantizationModes {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for QuantizationModes {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use QuantizationModes::*;
        match self {
            TRN => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            TRN_ZERO => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND_ZERO => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND_INF => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND_MIN_INF => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND_CONV => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            RND_CONV_ODD => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use QuantizationModes::*;
        match chunk[*idx as usize - 1] {
            0 => TRN,
            1 => TRN_ZERO,
            2 => RND,
            3 => RND_ZERO,
            4 => RND_INF,
            5 => RND_MIN_INF,
            6 => RND_CONV,
            7 => RND_CONV_ODD,
            what => panic!("{:?}", what),
        }
    }
}
impl FPOperationMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for FPOperationMode {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use FPOperationMode::*;
        match self {
            IEEE => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
            }
            ALT => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use FPOperationMode::*;
        match chunk[*idx as usize - 1] {
            0 => IEEE,
            1 => ALT,
            what => panic!("{:?}", what),
        }
    }
}
impl OverflowModes {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for OverflowModes {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use OverflowModes::*;
        match self {
            WRAP => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            SAT => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            SAT_ZERO => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            SAT_SYM => {
                req.add_cap(Capability::ArbitraryPrecisionFixedPointINTEL);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use OverflowModes::*;
        match chunk[*idx as usize - 1] {
            0 => WRAP,
            1 => SAT,
            2 => SAT_ZERO,
            3 => SAT_SYM,
            what => panic!("{:?}", what),
        }
    }
}
impl LinkageType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for LinkageType {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use LinkageType::*;
        match self {
            Export => {
                req.add_cap(Capability::Linkage);
            }
            Import => {
                req.add_cap(Capability::Linkage);
            }
            LinkOnceODR => {
                req.add_cap(Capability::Linkage);
                req.add_ext("SPV_KHR_linkonce_odr".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use LinkageType::*;
        match chunk[*idx as usize - 1] {
            0 => Export,
            1 => Import,
            2 => LinkOnceODR,
            what => panic!("{:?}", what),
        }
    }
}
impl AccessQualifier {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for AccessQualifier {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use AccessQualifier::*;
        match self {
            ReadOnly => {
                req.add_cap(Capability::Kernel);
            }
            WriteOnly => {
                req.add_cap(Capability::Kernel);
            }
            ReadWrite => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use AccessQualifier::*;
        match chunk[*idx as usize - 1] {
            0 => ReadOnly,
            1 => WriteOnly,
            2 => ReadWrite,
            what => panic!("{:?}", what),
        }
    }
}
impl FunctionParameterAttribute {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for FunctionParameterAttribute {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use FunctionParameterAttribute::*;
        match self {
            Zext => {
                req.add_cap(Capability::Kernel);
            }
            Sext => {
                req.add_cap(Capability::Kernel);
            }
            ByVal => {
                req.add_cap(Capability::Kernel);
            }
            Sret => {
                req.add_cap(Capability::Kernel);
            }
            NoAlias => {
                req.add_cap(Capability::Kernel);
            }
            NoCapture => {
                req.add_cap(Capability::Kernel);
            }
            NoWrite => {
                req.add_cap(Capability::Kernel);
            }
            NoReadWrite => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use FunctionParameterAttribute::*;
        match chunk[*idx as usize - 1] {
            0 => Zext,
            1 => Sext,
            2 => ByVal,
            3 => Sret,
            4 => NoAlias,
            5 => NoCapture,
            6 => NoWrite,
            7 => NoReadWrite,
            what => panic!("{:?}", what),
        }
    }
}
impl Decoration {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for Decoration {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use Decoration::*;
        match self {
            RelaxedPrecision => {
                req.add_cap(Capability::Shader);
            }
            SpecId(x0) => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            Block => {
                req.add_cap(Capability::Shader);
            }
            BufferBlock => {
                req.add_cap(Capability::Shader);
            }
            RowMajor => {
                req.add_cap(Capability::Matrix);
            }
            ColMajor => {
                req.add_cap(Capability::Matrix);
            }
            ArrayStride(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            MatrixStride(x0) => {
                req.add_cap(Capability::Matrix);
                x0.write_word(sink, req);
            }
            GLSLShared => {
                req.add_cap(Capability::Shader);
            }
            GLSLPacked => {
                req.add_cap(Capability::Shader);
            }
            CPacked => {
                req.add_cap(Capability::Kernel);
            }
            BuiltIn(x0) => {
                x0.write_word(sink, req);
            }
            NoPerspective => {
                req.add_cap(Capability::Shader);
            }
            Flat => {
                req.add_cap(Capability::Shader);
            }
            Patch => {
                req.add_cap(Capability::Tessellation);
            }
            Centroid => {
                req.add_cap(Capability::Shader);
            }
            Sample => {
                req.add_cap(Capability::SampleRateShading);
            }
            Invariant => {
                req.add_cap(Capability::Shader);
            }
            Restrict => {}
            Aliased => {}
            Volatile => {}
            Constant => {
                req.add_cap(Capability::Kernel);
            }
            Coherent => {}
            NonWritable => {}
            NonReadable => {}
            Uniform => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::UniformDecoration);
            }
            UniformId(x0) => {
                req.add_cap(Capability::Shader);
                req.add_cap(Capability::UniformDecoration);
                x0.write_word(sink, req);
            }
            SaturatedConversion => {
                req.add_cap(Capability::Kernel);
            }
            Stream(x0) => {
                req.add_cap(Capability::GeometryStreams);
                x0.write_word(sink, req);
            }
            Location(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            Component(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            Index(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            Binding(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            DescriptorSet(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            Offset(x0) => {
                req.add_cap(Capability::Shader);
                x0.write_word(sink, req);
            }
            XfbBuffer(x0) => {
                req.add_cap(Capability::TransformFeedback);
                x0.write_word(sink, req);
            }
            XfbStride(x0) => {
                req.add_cap(Capability::TransformFeedback);
                x0.write_word(sink, req);
            }
            FuncParamAttr(x0) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            FPRoundingMode(x0) => {
                x0.write_word(sink, req);
            }
            FPFastMathMode(x0) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            LinkageAttributes(x0, x1) => {
                req.add_cap(Capability::Linkage);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            NoContraction => {
                req.add_cap(Capability::Shader);
            }
            InputAttachmentIndex(x0) => {
                req.add_cap(Capability::InputAttachment);
                x0.write_word(sink, req);
            }
            Alignment(x0) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            MaxByteOffset(x0) => {
                req.add_cap(Capability::Addresses);
                x0.write_word(sink, req);
            }
            AlignmentId(x0) => {
                req.add_cap(Capability::Kernel);
                x0.write_word(sink, req);
            }
            MaxByteOffsetId(x0) => {
                req.add_cap(Capability::Addresses);
                x0.write_word(sink, req);
            }
            NoSignedWrap => {
                req.add_ext("SPV_KHR_no_integer_wrap_decoration".to_string());
            }
            NoUnsignedWrap => {
                req.add_ext("SPV_KHR_no_integer_wrap_decoration".to_string());
            }
            ExplicitInterpAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            OverrideCoverageNV => {
                req.add_cap(Capability::SampleMaskOverrideCoverageNV);
                req.add_ext("SPV_NV_sample_mask_override_coverage".to_string());
            }
            PassthroughNV => {
                req.add_cap(Capability::GeometryShaderPassthroughNV);
                req.add_ext("SPV_NV_geometry_shader_passthrough".to_string());
            }
            ViewportRelativeNV => {
                req.add_cap(Capability::ShaderViewportMaskNV);
            }
            SecondaryViewportRelativeNV(x0) => {
                req.add_cap(Capability::ShaderStereoViewNV);
                req.add_ext("SPV_NV_stereo_view_rendering".to_string());
                x0.write_word(sink, req);
            }
            PerPrimitiveNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PerViewNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PerTaskNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PerVertexKHR => {
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_ext("SPV_NV_fragment_shader_barycentric".to_string());
                req.add_ext("SPV_KHR_fragment_shader_barycentric".to_string());
            }
            NonUniform => {
                req.add_cap(Capability::ShaderNonUniform);
            }
            RestrictPointer => {
                req.add_cap(Capability::PhysicalStorageBufferAddresses);
                req.add_ext("SPV_EXT_physical_storage_buffer".to_string());
                req.add_ext("SPV_KHR_physical_storage_buffer".to_string());
            }
            AliasedPointer => {
                req.add_cap(Capability::PhysicalStorageBufferAddresses);
                req.add_ext("SPV_EXT_physical_storage_buffer".to_string());
                req.add_ext("SPV_KHR_physical_storage_buffer".to_string());
            }
            BindlessSamplerNV => {
                req.add_cap(Capability::BindlessTextureNV);
            }
            BindlessImageNV => {
                req.add_cap(Capability::BindlessTextureNV);
            }
            BoundSamplerNV => {
                req.add_cap(Capability::BindlessTextureNV);
            }
            BoundImageNV => {
                req.add_cap(Capability::BindlessTextureNV);
            }
            SIMTCallINTEL(x0) => {
                req.add_cap(Capability::VectorComputeINTEL);
                x0.write_word(sink, req);
            }
            ReferencedIndirectlyINTEL => {
                req.add_cap(Capability::IndirectReferencesINTEL);
                req.add_ext("SPV_INTEL_function_pointers".to_string());
            }
            ClobberINTEL(x0) => {
                req.add_cap(Capability::AsmINTEL);
                x0.write_word(sink, req);
            }
            SideEffectsINTEL => {
                req.add_cap(Capability::AsmINTEL);
            }
            VectorComputeVariableINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            FuncParamIOKindINTEL(x0) => {
                req.add_cap(Capability::VectorComputeINTEL);
                x0.write_word(sink, req);
            }
            VectorComputeFunctionINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            StackCallINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            GlobalVariableOffsetINTEL(x0) => {
                req.add_cap(Capability::VectorComputeINTEL);
                x0.write_word(sink, req);
            }
            CounterBuffer(x0) => {
                x0.write_word(sink, req);
            }
            UserSemantic(x0) => {
                x0.write_word(sink, req);
            }
            UserTypeGOOGLE(x0) => {
                req.add_ext("SPV_GOOGLE_user_type".to_string());
                x0.write_word(sink, req);
            }
            FunctionRoundingModeINTEL(x0, x1) => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            FunctionDenormModeINTEL(x0, x1) => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            RegisterINTEL => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
            }
            MemoryINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            NumbanksINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            BankwidthINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            MaxPrivateCopiesINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            SinglepumpINTEL => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
            }
            DoublepumpINTEL => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
            }
            MaxReplicatesINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            SimpleDualPortINTEL => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
            }
            MergeINTEL(x0, x1) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            BankBitsINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            ForcePow2DepthINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAttributesINTEL);
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
                x0.write_word(sink, req);
            }
            BurstCoalesceINTEL => {
                req.add_cap(Capability::FPGAMemoryAccessesINTEL);
            }
            CacheSizeINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAccessesINTEL);
                x0.write_word(sink, req);
            }
            DontStaticallyCoalesceINTEL => {
                req.add_cap(Capability::FPGAMemoryAccessesINTEL);
            }
            PrefetchINTEL(x0) => {
                req.add_cap(Capability::FPGAMemoryAccessesINTEL);
                x0.write_word(sink, req);
            }
            StallEnableINTEL => {
                req.add_cap(Capability::FPGAClusterAttributesINTEL);
            }
            FuseLoopsInFunctionINTEL => {
                req.add_cap(Capability::LoopFuseINTEL);
            }
            AliasScopeINTEL(x0) => {
                req.add_cap(Capability::MemoryAccessAliasingINTEL);
                x0.write_word(sink, req);
            }
            NoAliasINTEL(x0) => {
                req.add_cap(Capability::MemoryAccessAliasingINTEL);
                x0.write_word(sink, req);
            }
            BufferLocationINTEL(x0) => {
                req.add_cap(Capability::FPGABufferLocationINTEL);
                x0.write_word(sink, req);
            }
            IOPipeStorageINTEL(x0) => {
                req.add_cap(Capability::IOPipesINTEL);
                x0.write_word(sink, req);
            }
            FunctionFloatingPointModeINTEL(x0, x1) => {
                req.add_cap(Capability::FunctionFloatControlINTEL);
                x0.write_word(sink, req);
                x1.write_word(sink, req);
            }
            SingleElementVectorINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            VectorComputeCallableFunctionINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            MediaBlockIOINTEL => {
                req.add_cap(Capability::VectorComputeINTEL);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use Decoration::*;
        match chunk[*idx as usize - 1] {
            0 => RelaxedPrecision,
            1 => {
                let x0 = Asm::read_word(chunk, idx);
                SpecId(x0)
            }
            2 => Block,
            3 => BufferBlock,
            4 => RowMajor,
            5 => ColMajor,
            6 => {
                let x0 = Asm::read_word(chunk, idx);
                ArrayStride(x0)
            }
            7 => {
                let x0 = Asm::read_word(chunk, idx);
                MatrixStride(x0)
            }
            8 => GLSLShared,
            9 => GLSLPacked,
            10 => CPacked,
            11 => {
                let x0 = Asm::read_word(chunk, idx);
                BuiltIn(x0)
            }
            13 => NoPerspective,
            14 => Flat,
            15 => Patch,
            16 => Centroid,
            17 => Sample,
            18 => Invariant,
            19 => Restrict,
            20 => Aliased,
            21 => Volatile,
            22 => Constant,
            23 => Coherent,
            24 => NonWritable,
            25 => NonReadable,
            26 => Uniform,
            27 => {
                let x0 = Asm::read_word(chunk, idx);
                UniformId(x0)
            }
            28 => SaturatedConversion,
            29 => {
                let x0 = Asm::read_word(chunk, idx);
                Stream(x0)
            }
            30 => {
                let x0 = Asm::read_word(chunk, idx);
                Location(x0)
            }
            31 => {
                let x0 = Asm::read_word(chunk, idx);
                Component(x0)
            }
            32 => {
                let x0 = Asm::read_word(chunk, idx);
                Index(x0)
            }
            33 => {
                let x0 = Asm::read_word(chunk, idx);
                Binding(x0)
            }
            34 => {
                let x0 = Asm::read_word(chunk, idx);
                DescriptorSet(x0)
            }
            35 => {
                let x0 = Asm::read_word(chunk, idx);
                Offset(x0)
            }
            36 => {
                let x0 = Asm::read_word(chunk, idx);
                XfbBuffer(x0)
            }
            37 => {
                let x0 = Asm::read_word(chunk, idx);
                XfbStride(x0)
            }
            38 => {
                let x0 = Asm::read_word(chunk, idx);
                FuncParamAttr(x0)
            }
            39 => {
                let x0 = Asm::read_word(chunk, idx);
                FPRoundingMode(x0)
            }
            40 => {
                let x0 = Asm::read_word(chunk, idx);
                FPFastMathMode(x0)
            }
            41 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                LinkageAttributes(x0, x1)
            }
            42 => NoContraction,
            43 => {
                let x0 = Asm::read_word(chunk, idx);
                InputAttachmentIndex(x0)
            }
            44 => {
                let x0 = Asm::read_word(chunk, idx);
                Alignment(x0)
            }
            45 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxByteOffset(x0)
            }
            46 => {
                let x0 = Asm::read_word(chunk, idx);
                AlignmentId(x0)
            }
            47 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxByteOffsetId(x0)
            }
            4469 => NoSignedWrap,
            4470 => NoUnsignedWrap,
            4999 => ExplicitInterpAMD,
            5248 => OverrideCoverageNV,
            5250 => PassthroughNV,
            5252 => ViewportRelativeNV,
            5256 => {
                let x0 = Asm::read_word(chunk, idx);
                SecondaryViewportRelativeNV(x0)
            }
            5271 => PerPrimitiveNV,
            5272 => PerViewNV,
            5273 => PerTaskNV,
            5285 => PerVertexKHR,
            5300 => NonUniform,
            5355 => RestrictPointer,
            5356 => AliasedPointer,
            5398 => BindlessSamplerNV,
            5399 => BindlessImageNV,
            5400 => BoundSamplerNV,
            5401 => BoundImageNV,
            5599 => {
                let x0 = Asm::read_word(chunk, idx);
                SIMTCallINTEL(x0)
            }
            5602 => ReferencedIndirectlyINTEL,
            5607 => {
                let x0 = Asm::read_word(chunk, idx);
                ClobberINTEL(x0)
            }
            5608 => SideEffectsINTEL,
            5624 => VectorComputeVariableINTEL,
            5625 => {
                let x0 = Asm::read_word(chunk, idx);
                FuncParamIOKindINTEL(x0)
            }
            5626 => VectorComputeFunctionINTEL,
            5627 => StackCallINTEL,
            5628 => {
                let x0 = Asm::read_word(chunk, idx);
                GlobalVariableOffsetINTEL(x0)
            }
            5634 => {
                let x0 = Asm::read_word(chunk, idx);
                CounterBuffer(x0)
            }
            5635 => {
                let x0 = Asm::read_word(chunk, idx);
                UserSemantic(x0)
            }
            5636 => {
                let x0 = Asm::read_word(chunk, idx);
                UserTypeGOOGLE(x0)
            }
            5822 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                FunctionRoundingModeINTEL(x0, x1)
            }
            5823 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                FunctionDenormModeINTEL(x0, x1)
            }
            5825 => RegisterINTEL,
            5826 => {
                let x0 = Asm::read_word(chunk, idx);
                MemoryINTEL(x0)
            }
            5827 => {
                let x0 = Asm::read_word(chunk, idx);
                NumbanksINTEL(x0)
            }
            5828 => {
                let x0 = Asm::read_word(chunk, idx);
                BankwidthINTEL(x0)
            }
            5829 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxPrivateCopiesINTEL(x0)
            }
            5830 => SinglepumpINTEL,
            5831 => DoublepumpINTEL,
            5832 => {
                let x0 = Asm::read_word(chunk, idx);
                MaxReplicatesINTEL(x0)
            }
            5833 => SimpleDualPortINTEL,
            5834 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                MergeINTEL(x0, x1)
            }
            5835 => {
                let x0 = Asm::read_word(chunk, idx);
                BankBitsINTEL(x0)
            }
            5836 => {
                let x0 = Asm::read_word(chunk, idx);
                ForcePow2DepthINTEL(x0)
            }
            5899 => BurstCoalesceINTEL,
            5900 => {
                let x0 = Asm::read_word(chunk, idx);
                CacheSizeINTEL(x0)
            }
            5901 => DontStaticallyCoalesceINTEL,
            5902 => {
                let x0 = Asm::read_word(chunk, idx);
                PrefetchINTEL(x0)
            }
            5905 => StallEnableINTEL,
            5907 => FuseLoopsInFunctionINTEL,
            5914 => {
                let x0 = Asm::read_word(chunk, idx);
                AliasScopeINTEL(x0)
            }
            5915 => {
                let x0 = Asm::read_word(chunk, idx);
                NoAliasINTEL(x0)
            }
            5921 => {
                let x0 = Asm::read_word(chunk, idx);
                BufferLocationINTEL(x0)
            }
            5944 => {
                let x0 = Asm::read_word(chunk, idx);
                IOPipeStorageINTEL(x0)
            }
            6080 => {
                let x0 = Asm::read_word(chunk, idx);
                let x1 = Asm::read_word(chunk, idx);
                FunctionFloatingPointModeINTEL(x0, x1)
            }
            6085 => SingleElementVectorINTEL,
            6087 => VectorComputeCallableFunctionINTEL,
            6140 => MediaBlockIOINTEL,
            what => panic!("{:?}", what),
        }
    }
}
impl BuiltIn {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for BuiltIn {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use BuiltIn::*;
        match self {
            Position => {
                req.add_cap(Capability::Shader);
            }
            PointSize => {
                req.add_cap(Capability::Shader);
            }
            ClipDistance => {
                req.add_cap(Capability::ClipDistance);
            }
            CullDistance => {
                req.add_cap(Capability::CullDistance);
            }
            VertexId => {
                req.add_cap(Capability::Shader);
            }
            InstanceId => {
                req.add_cap(Capability::Shader);
            }
            PrimitiveId => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::Tessellation);
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_cap(Capability::MeshShadingNV);
            }
            InvocationId => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::Tessellation);
            }
            Layer => {
                req.add_cap(Capability::Geometry);
                req.add_cap(Capability::ShaderLayer);
                req.add_cap(Capability::ShaderViewportIndexLayerEXT);
                req.add_cap(Capability::MeshShadingNV);
            }
            ViewportIndex => {
                req.add_cap(Capability::MultiViewport);
                req.add_cap(Capability::ShaderViewportIndex);
                req.add_cap(Capability::ShaderViewportIndexLayerEXT);
                req.add_cap(Capability::MeshShadingNV);
            }
            TessLevelOuter => {
                req.add_cap(Capability::Tessellation);
            }
            TessLevelInner => {
                req.add_cap(Capability::Tessellation);
            }
            TessCoord => {
                req.add_cap(Capability::Tessellation);
            }
            PatchVertices => {
                req.add_cap(Capability::Tessellation);
            }
            FragCoord => {
                req.add_cap(Capability::Shader);
            }
            PointCoord => {
                req.add_cap(Capability::Shader);
            }
            FrontFacing => {
                req.add_cap(Capability::Shader);
            }
            SampleId => {
                req.add_cap(Capability::SampleRateShading);
            }
            SamplePosition => {
                req.add_cap(Capability::SampleRateShading);
            }
            SampleMask => {
                req.add_cap(Capability::Shader);
            }
            FragDepth => {
                req.add_cap(Capability::Shader);
            }
            HelperInvocation => {
                req.add_cap(Capability::Shader);
            }
            NumWorkgroups => {}
            WorkgroupSize => {}
            WorkgroupId => {}
            LocalInvocationId => {}
            GlobalInvocationId => {}
            LocalInvocationIndex => {}
            WorkDim => {
                req.add_cap(Capability::Kernel);
            }
            GlobalSize => {
                req.add_cap(Capability::Kernel);
            }
            EnqueuedWorkgroupSize => {
                req.add_cap(Capability::Kernel);
            }
            GlobalOffset => {
                req.add_cap(Capability::Kernel);
            }
            GlobalLinearId => {
                req.add_cap(Capability::Kernel);
            }
            SubgroupSize => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniform);
                req.add_cap(Capability::SubgroupBallotKHR);
            }
            SubgroupMaxSize => {
                req.add_cap(Capability::Kernel);
            }
            NumSubgroups => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniform);
            }
            NumEnqueuedSubgroups => {
                req.add_cap(Capability::Kernel);
            }
            SubgroupId => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniform);
            }
            SubgroupLocalInvocationId => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniform);
                req.add_cap(Capability::SubgroupBallotKHR);
            }
            VertexIndex => {
                req.add_cap(Capability::Shader);
            }
            InstanceIndex => {
                req.add_cap(Capability::Shader);
            }
            SubgroupEqMask => {
                req.add_cap(Capability::SubgroupBallotKHR);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            SubgroupGeMask => {
                req.add_cap(Capability::SubgroupBallotKHR);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            SubgroupGtMask => {
                req.add_cap(Capability::SubgroupBallotKHR);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            SubgroupLeMask => {
                req.add_cap(Capability::SubgroupBallotKHR);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            SubgroupLtMask => {
                req.add_cap(Capability::SubgroupBallotKHR);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            BaseVertex => {
                req.add_cap(Capability::DrawParameters);
                req.add_ext("SPV_KHR_shader_draw_parameters".to_string());
            }
            BaseInstance => {
                req.add_cap(Capability::DrawParameters);
                req.add_ext("SPV_KHR_shader_draw_parameters".to_string());
            }
            DrawIndex => {
                req.add_cap(Capability::DrawParameters);
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_KHR_shader_draw_parameters".to_string());
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PrimitiveShadingRateKHR => {
                req.add_cap(Capability::FragmentShadingRateKHR);
                req.add_ext("SPV_KHR_fragment_shading_rate".to_string());
            }
            DeviceIndex => {
                req.add_cap(Capability::DeviceGroup);
                req.add_ext("SPV_KHR_device_group".to_string());
            }
            ViewIndex => {
                req.add_cap(Capability::MultiView);
                req.add_ext("SPV_KHR_multiview".to_string());
            }
            ShadingRateKHR => {
                req.add_cap(Capability::FragmentShadingRateKHR);
                req.add_ext("SPV_KHR_fragment_shading_rate".to_string());
            }
            BaryCoordNoPerspAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordNoPerspCentroidAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordNoPerspSampleAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordSmoothAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordSmoothCentroidAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordSmoothSampleAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            BaryCoordPullModelAMD => {
                req.add_ext("SPV_AMD_shader_explicit_vertex_parameter".to_string());
            }
            FragStencilRefEXT => {
                req.add_cap(Capability::StencilExportEXT);
                req.add_ext("SPV_EXT_shader_stencil_export".to_string());
            }
            ViewportMaskNV => {
                req.add_cap(Capability::ShaderViewportMaskNV);
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_viewport_array2".to_string());
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            SecondaryPositionNV => {
                req.add_cap(Capability::ShaderStereoViewNV);
                req.add_ext("SPV_NV_stereo_view_rendering".to_string());
            }
            SecondaryViewportMaskNV => {
                req.add_cap(Capability::ShaderStereoViewNV);
                req.add_ext("SPV_NV_stereo_view_rendering".to_string());
            }
            PositionPerViewNV => {
                req.add_cap(Capability::PerViewAttributesNV);
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NVX_multiview_per_view_attributes".to_string());
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            ViewportMaskPerViewNV => {
                req.add_cap(Capability::PerViewAttributesNV);
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NVX_multiview_per_view_attributes".to_string());
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            FullyCoveredEXT => {
                req.add_cap(Capability::FragmentFullyCoveredEXT);
                req.add_ext("SPV_EXT_fragment_fully_covered".to_string());
            }
            TaskCountNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PrimitiveCountNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            PrimitiveIndicesNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            ClipDistancePerViewNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            CullDistancePerViewNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            LayerPerViewNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            MeshViewCountNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            MeshViewIndicesNV => {
                req.add_cap(Capability::MeshShadingNV);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            BaryCoordKHR => {
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_ext("SPV_NV_fragment_shader_barycentric".to_string());
                req.add_ext("SPV_KHR_fragment_shader_barycentric".to_string());
            }
            BaryCoordNoPerspKHR => {
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_cap(Capability::FragmentBarycentricKHR);
                req.add_ext("SPV_NV_fragment_shader_barycentric".to_string());
                req.add_ext("SPV_KHR_fragment_shader_barycentric".to_string());
            }
            FragSizeEXT => {
                req.add_cap(Capability::FragmentDensityEXT);
                req.add_cap(Capability::FragmentDensityEXT);
                req.add_ext("SPV_EXT_fragment_invocation_density".to_string());
                req.add_ext("SPV_NV_shading_rate".to_string());
            }
            FragInvocationCountEXT => {
                req.add_cap(Capability::FragmentDensityEXT);
                req.add_cap(Capability::FragmentDensityEXT);
                req.add_ext("SPV_EXT_fragment_invocation_density".to_string());
                req.add_ext("SPV_NV_shading_rate".to_string());
            }
            LaunchIdNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            LaunchSizeNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            WorldRayOriginNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            WorldRayDirectionNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            ObjectRayOriginNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            ObjectRayDirectionNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            RayTminNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            RayTmaxNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            InstanceCustomIndexNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            ObjectToWorldNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            WorldToObjectNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            HitTNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_ext("SPV_NV_ray_tracing".to_string());
            }
            HitKindNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            CurrentRayTimeNV => {
                req.add_cap(Capability::RayTracingMotionBlurNV);
                req.add_ext("SPV_NV_ray_tracing_motion_blur".to_string());
            }
            IncomingRayFlagsNV => {
                req.add_cap(Capability::RayTracingNV);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_NV_ray_tracing".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            RayGeometryIndexKHR => {
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            WarpsPerSMNV => {
                req.add_cap(Capability::ShaderSMBuiltinsNV);
                req.add_ext("SPV_NV_shader_sm_builtins".to_string());
            }
            SMCountNV => {
                req.add_cap(Capability::ShaderSMBuiltinsNV);
                req.add_ext("SPV_NV_shader_sm_builtins".to_string());
            }
            WarpIDNV => {
                req.add_cap(Capability::ShaderSMBuiltinsNV);
                req.add_ext("SPV_NV_shader_sm_builtins".to_string());
            }
            SMIDNV => {
                req.add_cap(Capability::ShaderSMBuiltinsNV);
                req.add_ext("SPV_NV_shader_sm_builtins".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use BuiltIn::*;
        match chunk[*idx as usize - 1] {
            0 => Position,
            1 => PointSize,
            3 => ClipDistance,
            4 => CullDistance,
            5 => VertexId,
            6 => InstanceId,
            7 => PrimitiveId,
            8 => InvocationId,
            9 => Layer,
            10 => ViewportIndex,
            11 => TessLevelOuter,
            12 => TessLevelInner,
            13 => TessCoord,
            14 => PatchVertices,
            15 => FragCoord,
            16 => PointCoord,
            17 => FrontFacing,
            18 => SampleId,
            19 => SamplePosition,
            20 => SampleMask,
            22 => FragDepth,
            23 => HelperInvocation,
            24 => NumWorkgroups,
            25 => WorkgroupSize,
            26 => WorkgroupId,
            27 => LocalInvocationId,
            28 => GlobalInvocationId,
            29 => LocalInvocationIndex,
            30 => WorkDim,
            31 => GlobalSize,
            32 => EnqueuedWorkgroupSize,
            33 => GlobalOffset,
            34 => GlobalLinearId,
            36 => SubgroupSize,
            37 => SubgroupMaxSize,
            38 => NumSubgroups,
            39 => NumEnqueuedSubgroups,
            40 => SubgroupId,
            41 => SubgroupLocalInvocationId,
            42 => VertexIndex,
            43 => InstanceIndex,
            4416 => SubgroupEqMask,
            4417 => SubgroupGeMask,
            4418 => SubgroupGtMask,
            4419 => SubgroupLeMask,
            4420 => SubgroupLtMask,
            4424 => BaseVertex,
            4425 => BaseInstance,
            4426 => DrawIndex,
            4432 => PrimitiveShadingRateKHR,
            4438 => DeviceIndex,
            4440 => ViewIndex,
            4444 => ShadingRateKHR,
            4992 => BaryCoordNoPerspAMD,
            4993 => BaryCoordNoPerspCentroidAMD,
            4994 => BaryCoordNoPerspSampleAMD,
            4995 => BaryCoordSmoothAMD,
            4996 => BaryCoordSmoothCentroidAMD,
            4997 => BaryCoordSmoothSampleAMD,
            4998 => BaryCoordPullModelAMD,
            5014 => FragStencilRefEXT,
            5253 => ViewportMaskNV,
            5257 => SecondaryPositionNV,
            5258 => SecondaryViewportMaskNV,
            5261 => PositionPerViewNV,
            5262 => ViewportMaskPerViewNV,
            5264 => FullyCoveredEXT,
            5274 => TaskCountNV,
            5275 => PrimitiveCountNV,
            5276 => PrimitiveIndicesNV,
            5277 => ClipDistancePerViewNV,
            5278 => CullDistancePerViewNV,
            5279 => LayerPerViewNV,
            5280 => MeshViewCountNV,
            5281 => MeshViewIndicesNV,
            5286 => BaryCoordKHR,
            5287 => BaryCoordNoPerspKHR,
            5292 => FragSizeEXT,
            5293 => FragInvocationCountEXT,
            5319 => LaunchIdNV,
            5320 => LaunchSizeNV,
            5321 => WorldRayOriginNV,
            5322 => WorldRayDirectionNV,
            5323 => ObjectRayOriginNV,
            5324 => ObjectRayDirectionNV,
            5325 => RayTminNV,
            5326 => RayTmaxNV,
            5327 => InstanceCustomIndexNV,
            5330 => ObjectToWorldNV,
            5331 => WorldToObjectNV,
            5332 => HitTNV,
            5333 => HitKindNV,
            5334 => CurrentRayTimeNV,
            5351 => IncomingRayFlagsNV,
            5352 => RayGeometryIndexKHR,
            5374 => WarpsPerSMNV,
            5375 => SMCountNV,
            5376 => WarpIDNV,
            5377 => SMIDNV,
            what => panic!("{:?}", what),
        }
    }
}
impl Scope {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for Scope {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use Scope::*;
        match self {
            CrossDevice => {}
            Device => {}
            Workgroup => {}
            Subgroup => {}
            Invocation => {}
            QueueFamily => {
                req.add_cap(Capability::VulkanMemoryModel);
            }
            ShaderCallKHR => {
                req.add_cap(Capability::RayTracingKHR);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use Scope::*;
        match chunk[*idx as usize - 1] {
            0 => CrossDevice,
            1 => Device,
            2 => Workgroup,
            3 => Subgroup,
            4 => Invocation,
            5 => QueueFamily,
            6 => ShaderCallKHR,
            what => panic!("{:?}", what),
        }
    }
}
impl GroupOperation {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for GroupOperation {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use GroupOperation::*;
        match self {
            Reduce => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniformArithmetic);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            InclusiveScan => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniformArithmetic);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            ExclusiveScan => {
                req.add_cap(Capability::Kernel);
                req.add_cap(Capability::GroupNonUniformArithmetic);
                req.add_cap(Capability::GroupNonUniformBallot);
            }
            ClusteredReduce => {
                req.add_cap(Capability::GroupNonUniformClustered);
            }
            PartitionedReduceNV => {
                req.add_cap(Capability::GroupNonUniformPartitionedNV);
                req.add_ext("SPV_NV_shader_subgroup_partitioned".to_string());
            }
            PartitionedInclusiveScanNV => {
                req.add_cap(Capability::GroupNonUniformPartitionedNV);
                req.add_ext("SPV_NV_shader_subgroup_partitioned".to_string());
            }
            PartitionedExclusiveScanNV => {
                req.add_cap(Capability::GroupNonUniformPartitionedNV);
                req.add_ext("SPV_NV_shader_subgroup_partitioned".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use GroupOperation::*;
        match chunk[*idx as usize - 1] {
            0 => Reduce,
            1 => InclusiveScan,
            2 => ExclusiveScan,
            3 => ClusteredReduce,
            6 => PartitionedReduceNV,
            7 => PartitionedInclusiveScanNV,
            8 => PartitionedExclusiveScanNV,
            what => panic!("{:?}", what),
        }
    }
}
impl KernelEnqueueFlags {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for KernelEnqueueFlags {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use KernelEnqueueFlags::*;
        match self {
            NoWait => {
                req.add_cap(Capability::Kernel);
            }
            WaitKernel => {
                req.add_cap(Capability::Kernel);
            }
            WaitWorkGroup => {
                req.add_cap(Capability::Kernel);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use KernelEnqueueFlags::*;
        match chunk[*idx as usize - 1] {
            0 => NoWait,
            1 => WaitKernel,
            2 => WaitWorkGroup,
            what => panic!("{:?}", what),
        }
    }
}
impl Capability {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for Capability {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use Capability::*;
        match self {
            Matrix => {}
            Shader => {
                req.add_cap(Capability::Matrix);
            }
            Geometry => {
                req.add_cap(Capability::Shader);
            }
            Tessellation => {
                req.add_cap(Capability::Shader);
            }
            Addresses => {}
            Linkage => {}
            Kernel => {}
            Vector16 => {
                req.add_cap(Capability::Kernel);
            }
            Float16Buffer => {
                req.add_cap(Capability::Kernel);
            }
            Float16 => {}
            Float64 => {}
            Int64 => {}
            Int64Atomics => {
                req.add_cap(Capability::Int64);
            }
            ImageBasic => {
                req.add_cap(Capability::Kernel);
            }
            ImageReadWrite => {
                req.add_cap(Capability::ImageBasic);
            }
            ImageMipmap => {
                req.add_cap(Capability::ImageBasic);
            }
            Pipes => {
                req.add_cap(Capability::Kernel);
            }
            Groups => {
                req.add_ext("SPV_AMD_shader_ballot".to_string());
            }
            DeviceEnqueue => {
                req.add_cap(Capability::Kernel);
            }
            LiteralSampler => {
                req.add_cap(Capability::Kernel);
            }
            AtomicStorage => {
                req.add_cap(Capability::Shader);
            }
            Int16 => {}
            TessellationPointSize => {
                req.add_cap(Capability::Tessellation);
            }
            GeometryPointSize => {
                req.add_cap(Capability::Geometry);
            }
            ImageGatherExtended => {
                req.add_cap(Capability::Shader);
            }
            StorageImageMultisample => {
                req.add_cap(Capability::Shader);
            }
            UniformBufferArrayDynamicIndexing => {
                req.add_cap(Capability::Shader);
            }
            SampledImageArrayDynamicIndexing => {
                req.add_cap(Capability::Shader);
            }
            StorageBufferArrayDynamicIndexing => {
                req.add_cap(Capability::Shader);
            }
            StorageImageArrayDynamicIndexing => {
                req.add_cap(Capability::Shader);
            }
            ClipDistance => {
                req.add_cap(Capability::Shader);
            }
            CullDistance => {
                req.add_cap(Capability::Shader);
            }
            ImageCubeArray => {
                req.add_cap(Capability::SampledCubeArray);
            }
            SampleRateShading => {
                req.add_cap(Capability::Shader);
            }
            ImageRect => {
                req.add_cap(Capability::SampledRect);
            }
            SampledRect => {
                req.add_cap(Capability::Shader);
            }
            GenericPointer => {
                req.add_cap(Capability::Addresses);
            }
            Int8 => {}
            InputAttachment => {
                req.add_cap(Capability::Shader);
            }
            SparseResidency => {
                req.add_cap(Capability::Shader);
            }
            MinLod => {
                req.add_cap(Capability::Shader);
            }
            Sampled1D => {}
            Image1D => {
                req.add_cap(Capability::Sampled1D);
            }
            SampledCubeArray => {
                req.add_cap(Capability::Shader);
            }
            SampledBuffer => {}
            ImageBuffer => {
                req.add_cap(Capability::SampledBuffer);
            }
            ImageMSArray => {
                req.add_cap(Capability::Shader);
            }
            StorageImageExtendedFormats => {
                req.add_cap(Capability::Shader);
            }
            ImageQuery => {
                req.add_cap(Capability::Shader);
            }
            DerivativeControl => {
                req.add_cap(Capability::Shader);
            }
            InterpolationFunction => {
                req.add_cap(Capability::Shader);
            }
            TransformFeedback => {
                req.add_cap(Capability::Shader);
            }
            GeometryStreams => {
                req.add_cap(Capability::Geometry);
            }
            StorageImageReadWithoutFormat => {
                req.add_cap(Capability::Shader);
            }
            StorageImageWriteWithoutFormat => {
                req.add_cap(Capability::Shader);
            }
            MultiViewport => {
                req.add_cap(Capability::Geometry);
            }
            SubgroupDispatch => {
                req.add_cap(Capability::DeviceEnqueue);
            }
            NamedBarrier => {
                req.add_cap(Capability::Kernel);
            }
            PipeStorage => {
                req.add_cap(Capability::Pipes);
            }
            GroupNonUniform => {}
            GroupNonUniformVote => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformArithmetic => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformBallot => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformShuffle => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformShuffleRelative => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformClustered => {
                req.add_cap(Capability::GroupNonUniform);
            }
            GroupNonUniformQuad => {
                req.add_cap(Capability::GroupNonUniform);
            }
            ShaderLayer => {}
            ShaderViewportIndex => {}
            UniformDecoration => {}
            FragmentShadingRateKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_fragment_shading_rate".to_string());
            }
            SubgroupBallotKHR => {
                req.add_ext("SPV_KHR_shader_ballot".to_string());
            }
            DrawParameters => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_shader_draw_parameters".to_string());
            }
            WorkgroupMemoryExplicitLayoutKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_workgroup_memory_explicit_layout".to_string());
            }
            WorkgroupMemoryExplicitLayout8BitAccessKHR => {
                req.add_cap(Capability::WorkgroupMemoryExplicitLayoutKHR);
                req.add_ext("SPV_KHR_workgroup_memory_explicit_layout".to_string());
            }
            WorkgroupMemoryExplicitLayout16BitAccessKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_workgroup_memory_explicit_layout".to_string());
            }
            SubgroupVoteKHR => {
                req.add_ext("SPV_KHR_subgroup_vote".to_string());
            }
            StorageBuffer16BitAccess => {
                req.add_ext("SPV_KHR_16bit_storage".to_string());
            }
            UniformAndStorageBuffer16BitAccess => {
                req.add_cap(Capability::StorageBuffer16BitAccess);
                req.add_cap(Capability::StorageBuffer16BitAccess);
                req.add_ext("SPV_KHR_16bit_storage".to_string());
            }
            StoragePushConstant16 => {
                req.add_ext("SPV_KHR_16bit_storage".to_string());
            }
            StorageInputOutput16 => {
                req.add_ext("SPV_KHR_16bit_storage".to_string());
            }
            DeviceGroup => {
                req.add_ext("SPV_KHR_device_group".to_string());
            }
            MultiView => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_multiview".to_string());
            }
            VariablePointersStorageBuffer => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_variable_pointers".to_string());
            }
            VariablePointers => {
                req.add_cap(Capability::VariablePointersStorageBuffer);
                req.add_ext("SPV_KHR_variable_pointers".to_string());
            }
            AtomicStorageOps => {
                req.add_ext("SPV_KHR_shader_atomic_counter_ops".to_string());
            }
            SampleMaskPostDepthCoverage => {
                req.add_ext("SPV_KHR_post_depth_coverage".to_string());
            }
            StorageBuffer8BitAccess => {
                req.add_ext("SPV_KHR_8bit_storage".to_string());
            }
            UniformAndStorageBuffer8BitAccess => {
                req.add_cap(Capability::StorageBuffer8BitAccess);
                req.add_ext("SPV_KHR_8bit_storage".to_string());
            }
            StoragePushConstant8 => {
                req.add_ext("SPV_KHR_8bit_storage".to_string());
            }
            DenormPreserve => {
                req.add_ext("SPV_KHR_float_controls".to_string());
            }
            DenormFlushToZero => {
                req.add_ext("SPV_KHR_float_controls".to_string());
            }
            SignedZeroInfNanPreserve => {
                req.add_ext("SPV_KHR_float_controls".to_string());
            }
            RoundingModeRTE => {
                req.add_ext("SPV_KHR_float_controls".to_string());
            }
            RoundingModeRTZ => {
                req.add_ext("SPV_KHR_float_controls".to_string());
            }
            RayQueryProvisionalKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_ray_query".to_string());
            }
            RayQueryKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_ray_query".to_string());
            }
            RayTraversalPrimitiveCullingKHR => {
                req.add_cap(Capability::RayQueryKHR);
                req.add_cap(Capability::RayTracingKHR);
                req.add_ext("SPV_KHR_ray_query".to_string());
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            RayTracingKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            Float16ImageAMD => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_AMD_gpu_shader_half_float_fetch".to_string());
            }
            ImageGatherBiasLodAMD => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_AMD_texture_gather_bias_lod".to_string());
            }
            FragmentMaskAMD => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_AMD_shader_fragment_mask".to_string());
            }
            StencilExportEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_shader_stencil_export".to_string());
            }
            ImageReadWriteLodAMD => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_AMD_shader_image_load_store_lod".to_string());
            }
            Int64ImageEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_shader_image_int64".to_string());
            }
            ShaderClockKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_shader_clock".to_string());
            }
            SampleMaskOverrideCoverageNV => {
                req.add_cap(Capability::SampleRateShading);
                req.add_ext("SPV_NV_sample_mask_override_coverage".to_string());
            }
            GeometryShaderPassthroughNV => {
                req.add_cap(Capability::Geometry);
                req.add_ext("SPV_NV_geometry_shader_passthrough".to_string());
            }
            ShaderViewportIndexLayerEXT => {
                req.add_cap(Capability::MultiViewport);
                req.add_ext("SPV_EXT_shader_viewport_index_layer".to_string());
            }
            ShaderViewportMaskNV => {
                req.add_cap(Capability::ShaderViewportIndexLayerEXT);
                req.add_ext("SPV_NV_viewport_array2".to_string());
            }
            ShaderStereoViewNV => {
                req.add_cap(Capability::ShaderViewportMaskNV);
                req.add_ext("SPV_NV_stereo_view_rendering".to_string());
            }
            PerViewAttributesNV => {
                req.add_cap(Capability::MultiView);
                req.add_ext("SPV_NVX_multiview_per_view_attributes".to_string());
            }
            FragmentFullyCoveredEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_fragment_fully_covered".to_string());
            }
            MeshShadingNV => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_NV_mesh_shader".to_string());
            }
            ImageFootprintNV => {
                req.add_ext("SPV_NV_shader_image_footprint".to_string());
            }
            FragmentBarycentricKHR => {
                req.add_ext("SPV_NV_fragment_shader_barycentric".to_string());
                req.add_ext("SPV_KHR_fragment_shader_barycentric".to_string());
            }
            ComputeDerivativeGroupQuadsNV => {
                req.add_ext("SPV_NV_compute_shader_derivatives".to_string());
            }
            FragmentDensityEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_fragment_invocation_density".to_string());
                req.add_ext("SPV_NV_shading_rate".to_string());
            }
            GroupNonUniformPartitionedNV => {
                req.add_ext("SPV_NV_shader_subgroup_partitioned".to_string());
            }
            ShaderNonUniform => {
                req.add_cap(Capability::Shader);
            }
            RuntimeDescriptorArray => {
                req.add_cap(Capability::Shader);
            }
            InputAttachmentArrayDynamicIndexing => {
                req.add_cap(Capability::InputAttachment);
            }
            UniformTexelBufferArrayDynamicIndexing => {
                req.add_cap(Capability::SampledBuffer);
            }
            StorageTexelBufferArrayDynamicIndexing => {
                req.add_cap(Capability::ImageBuffer);
            }
            UniformBufferArrayNonUniformIndexing => {
                req.add_cap(Capability::ShaderNonUniform);
            }
            SampledImageArrayNonUniformIndexing => {
                req.add_cap(Capability::ShaderNonUniform);
            }
            StorageBufferArrayNonUniformIndexing => {
                req.add_cap(Capability::ShaderNonUniform);
            }
            StorageImageArrayNonUniformIndexing => {
                req.add_cap(Capability::ShaderNonUniform);
            }
            InputAttachmentArrayNonUniformIndexing => {
                req.add_cap(Capability::InputAttachment);
                req.add_cap(Capability::ShaderNonUniform);
            }
            UniformTexelBufferArrayNonUniformIndexing => {
                req.add_cap(Capability::SampledBuffer);
                req.add_cap(Capability::ShaderNonUniform);
            }
            StorageTexelBufferArrayNonUniformIndexing => {
                req.add_cap(Capability::ImageBuffer);
                req.add_cap(Capability::ShaderNonUniform);
            }
            RayTracingNV => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_NV_ray_tracing".to_string());
            }
            RayTracingMotionBlurNV => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_NV_ray_tracing_motion_blur".to_string());
            }
            VulkanMemoryModel => {}
            VulkanMemoryModelDeviceScope => {}
            PhysicalStorageBufferAddresses => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_physical_storage_buffer".to_string());
                req.add_ext("SPV_KHR_physical_storage_buffer".to_string());
            }
            ComputeDerivativeGroupLinearNV => {
                req.add_ext("SPV_NV_compute_shader_derivatives".to_string());
            }
            RayTracingProvisionalKHR => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_KHR_ray_tracing".to_string());
            }
            CooperativeMatrixNV => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_NV_cooperative_matrix".to_string());
            }
            FragmentShaderSampleInterlockEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            FragmentShaderShadingRateInterlockEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            ShaderSMBuiltinsNV => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_NV_shader_sm_builtins".to_string());
            }
            FragmentShaderPixelInterlockEXT => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_EXT_fragment_shader_interlock".to_string());
            }
            DemoteToHelperInvocation => {
                req.add_cap(Capability::Shader);
            }
            BindlessTextureNV => {
                req.add_ext("SPV_NV_bindless_texture".to_string());
            }
            SubgroupShuffleINTEL => {
                req.add_ext("SPV_INTEL_subgroups".to_string());
            }
            SubgroupBufferBlockIOINTEL => {
                req.add_ext("SPV_INTEL_subgroups".to_string());
            }
            SubgroupImageBlockIOINTEL => {
                req.add_ext("SPV_INTEL_subgroups".to_string());
            }
            SubgroupImageMediaBlockIOINTEL => {
                req.add_ext("SPV_INTEL_media_block_io".to_string());
            }
            RoundToInfinityINTEL => {
                req.add_ext("SPV_INTEL_float_controls2".to_string());
            }
            FloatingPointModeINTEL => {
                req.add_ext("SPV_INTEL_float_controls2".to_string());
            }
            IntegerFunctions2INTEL => {
                req.add_cap(Capability::Shader);
                req.add_ext("SPV_INTEL_shader_integer_functions2".to_string());
            }
            FunctionPointersINTEL => {
                req.add_ext("SPV_INTEL_function_pointers".to_string());
            }
            IndirectReferencesINTEL => {
                req.add_ext("SPV_INTEL_function_pointers".to_string());
            }
            AsmINTEL => {
                req.add_ext("SPV_INTEL_inline_assembly".to_string());
            }
            AtomicFloat32MinMaxEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float_min_max".to_string());
            }
            AtomicFloat64MinMaxEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float_min_max".to_string());
            }
            AtomicFloat16MinMaxEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float_min_max".to_string());
            }
            VectorComputeINTEL => {
                req.add_cap(Capability::VectorAnyINTEL);
                req.add_ext("SPV_INTEL_vector_compute".to_string());
            }
            VectorAnyINTEL => {
                req.add_ext("SPV_INTEL_vector_compute".to_string());
            }
            ExpectAssumeKHR => {
                req.add_ext("SPV_KHR_expect_assume".to_string());
            }
            SubgroupAvcMotionEstimationINTEL => {
                req.add_ext("SPV_INTEL_device_side_avc_motion_estimation".to_string());
            }
            SubgroupAvcMotionEstimationIntraINTEL => {
                req.add_ext("SPV_INTEL_device_side_avc_motion_estimation".to_string());
            }
            SubgroupAvcMotionEstimationChromaINTEL => {
                req.add_ext("SPV_INTEL_device_side_avc_motion_estimation".to_string());
            }
            VariableLengthArrayINTEL => {
                req.add_ext("SPV_INTEL_variable_length_array".to_string());
            }
            FunctionFloatControlINTEL => {
                req.add_ext("SPV_INTEL_float_controls2".to_string());
            }
            FPGAMemoryAttributesINTEL => {
                req.add_ext("SPV_INTEL_fpga_memory_attributes".to_string());
            }
            FPFastMathModeINTEL => {
                req.add_cap(Capability::Kernel);
                req.add_ext("SPV_INTEL_fp_fast_math_mode".to_string());
            }
            ArbitraryPrecisionIntegersINTEL => {
                req.add_ext("SPV_INTEL_arbitrary_precision_integers".to_string());
            }
            ArbitraryPrecisionFloatingPointINTEL => {
                req.add_ext("SPV_INTEL_arbitrary_precision_floating_point".to_string());
            }
            UnstructuredLoopControlsINTEL => {
                req.add_ext("SPV_INTEL_unstructured_loop_controls".to_string());
            }
            FPGALoopControlsINTEL => {
                req.add_ext("SPV_INTEL_fpga_loop_controls".to_string());
            }
            KernelAttributesINTEL => {
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
            }
            FPGAKernelAttributesINTEL => {
                req.add_ext("SPV_INTEL_kernel_attributes".to_string());
            }
            FPGAMemoryAccessesINTEL => {
                req.add_ext("SPV_INTEL_fpga_memory_accesses".to_string());
            }
            FPGAClusterAttributesINTEL => {
                req.add_ext("SPV_INTEL_fpga_cluster_attributes".to_string());
            }
            LoopFuseINTEL => {
                req.add_ext("SPV_INTEL_loop_fuse".to_string());
            }
            MemoryAccessAliasingINTEL => {
                req.add_ext("SPV_INTEL_memory_access_aliasing".to_string());
            }
            FPGABufferLocationINTEL => {
                req.add_ext("SPV_INTEL_fpga_buffer_location".to_string());
            }
            ArbitraryPrecisionFixedPointINTEL => {
                req.add_ext("SPV_INTEL_arbitrary_precision_fixed_point".to_string());
            }
            USMStorageClassesINTEL => {
                req.add_ext("SPV_INTEL_usm_storage_classes".to_string());
            }
            IOPipesINTEL => {
                req.add_ext("SPV_INTEL_io_pipes".to_string());
            }
            BlockingPipesINTEL => {
                req.add_ext("SPV_INTEL_blocking_pipes".to_string());
            }
            FPGARegINTEL => {
                req.add_ext("SPV_INTEL_fpga_reg".to_string());
            }
            DotProductInputAll => {}
            DotProductInput4x8Bit => {
                req.add_cap(Capability::Int8);
            }
            DotProductInput4x8BitPacked => {}
            DotProduct => {}
            BitInstructions => {
                req.add_ext("SPV_KHR_bit_instructions".to_string());
            }
            AtomicFloat32AddEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float_add".to_string());
            }
            AtomicFloat64AddEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float_add".to_string());
            }
            LongConstantCompositeINTEL => {
                req.add_ext("SPV_INTEL_long_constant_composite".to_string());
            }
            OptNoneINTEL => {
                req.add_ext("SPV_INTEL_optnone".to_string());
            }
            AtomicFloat16AddEXT => {
                req.add_ext("SPV_EXT_shader_atomic_float16_add".to_string());
            }
            DebugInfoModuleINTEL => {
                req.add_ext("SPV_INTEL_debug_module".to_string());
            }
            SplitBarrierINTEL => {
                req.add_ext("SPV_INTEL_split_barrier".to_string());
            }
            GroupUniformArithmeticKHR => {
                req.add_ext("SPV_KHR_uniform_group_instructions".to_string());
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use Capability::*;
        match chunk[*idx as usize - 1] {
            0 => Matrix,
            1 => Shader,
            2 => Geometry,
            3 => Tessellation,
            4 => Addresses,
            5 => Linkage,
            6 => Kernel,
            7 => Vector16,
            8 => Float16Buffer,
            9 => Float16,
            10 => Float64,
            11 => Int64,
            12 => Int64Atomics,
            13 => ImageBasic,
            14 => ImageReadWrite,
            15 => ImageMipmap,
            17 => Pipes,
            18 => Groups,
            19 => DeviceEnqueue,
            20 => LiteralSampler,
            21 => AtomicStorage,
            22 => Int16,
            23 => TessellationPointSize,
            24 => GeometryPointSize,
            25 => ImageGatherExtended,
            27 => StorageImageMultisample,
            28 => UniformBufferArrayDynamicIndexing,
            29 => SampledImageArrayDynamicIndexing,
            30 => StorageBufferArrayDynamicIndexing,
            31 => StorageImageArrayDynamicIndexing,
            32 => ClipDistance,
            33 => CullDistance,
            34 => ImageCubeArray,
            35 => SampleRateShading,
            36 => ImageRect,
            37 => SampledRect,
            38 => GenericPointer,
            39 => Int8,
            40 => InputAttachment,
            41 => SparseResidency,
            42 => MinLod,
            43 => Sampled1D,
            44 => Image1D,
            45 => SampledCubeArray,
            46 => SampledBuffer,
            47 => ImageBuffer,
            48 => ImageMSArray,
            49 => StorageImageExtendedFormats,
            50 => ImageQuery,
            51 => DerivativeControl,
            52 => InterpolationFunction,
            53 => TransformFeedback,
            54 => GeometryStreams,
            55 => StorageImageReadWithoutFormat,
            56 => StorageImageWriteWithoutFormat,
            57 => MultiViewport,
            58 => SubgroupDispatch,
            59 => NamedBarrier,
            60 => PipeStorage,
            61 => GroupNonUniform,
            62 => GroupNonUniformVote,
            63 => GroupNonUniformArithmetic,
            64 => GroupNonUniformBallot,
            65 => GroupNonUniformShuffle,
            66 => GroupNonUniformShuffleRelative,
            67 => GroupNonUniformClustered,
            68 => GroupNonUniformQuad,
            69 => ShaderLayer,
            70 => ShaderViewportIndex,
            71 => UniformDecoration,
            4422 => FragmentShadingRateKHR,
            4423 => SubgroupBallotKHR,
            4427 => DrawParameters,
            4428 => WorkgroupMemoryExplicitLayoutKHR,
            4429 => WorkgroupMemoryExplicitLayout8BitAccessKHR,
            4430 => WorkgroupMemoryExplicitLayout16BitAccessKHR,
            4431 => SubgroupVoteKHR,
            4433 => StorageBuffer16BitAccess,
            4434 => UniformAndStorageBuffer16BitAccess,
            4435 => StoragePushConstant16,
            4436 => StorageInputOutput16,
            4437 => DeviceGroup,
            4439 => MultiView,
            4441 => VariablePointersStorageBuffer,
            4442 => VariablePointers,
            4445 => AtomicStorageOps,
            4447 => SampleMaskPostDepthCoverage,
            4448 => StorageBuffer8BitAccess,
            4449 => UniformAndStorageBuffer8BitAccess,
            4450 => StoragePushConstant8,
            4464 => DenormPreserve,
            4465 => DenormFlushToZero,
            4466 => SignedZeroInfNanPreserve,
            4467 => RoundingModeRTE,
            4468 => RoundingModeRTZ,
            4471 => RayQueryProvisionalKHR,
            4472 => RayQueryKHR,
            4478 => RayTraversalPrimitiveCullingKHR,
            4479 => RayTracingKHR,
            5008 => Float16ImageAMD,
            5009 => ImageGatherBiasLodAMD,
            5010 => FragmentMaskAMD,
            5013 => StencilExportEXT,
            5015 => ImageReadWriteLodAMD,
            5016 => Int64ImageEXT,
            5055 => ShaderClockKHR,
            5249 => SampleMaskOverrideCoverageNV,
            5251 => GeometryShaderPassthroughNV,
            5254 => ShaderViewportIndexLayerEXT,
            5255 => ShaderViewportMaskNV,
            5259 => ShaderStereoViewNV,
            5260 => PerViewAttributesNV,
            5265 => FragmentFullyCoveredEXT,
            5266 => MeshShadingNV,
            5282 => ImageFootprintNV,
            5284 => FragmentBarycentricKHR,
            5288 => ComputeDerivativeGroupQuadsNV,
            5291 => FragmentDensityEXT,
            5297 => GroupNonUniformPartitionedNV,
            5301 => ShaderNonUniform,
            5302 => RuntimeDescriptorArray,
            5303 => InputAttachmentArrayDynamicIndexing,
            5304 => UniformTexelBufferArrayDynamicIndexing,
            5305 => StorageTexelBufferArrayDynamicIndexing,
            5306 => UniformBufferArrayNonUniformIndexing,
            5307 => SampledImageArrayNonUniformIndexing,
            5308 => StorageBufferArrayNonUniformIndexing,
            5309 => StorageImageArrayNonUniformIndexing,
            5310 => InputAttachmentArrayNonUniformIndexing,
            5311 => UniformTexelBufferArrayNonUniformIndexing,
            5312 => StorageTexelBufferArrayNonUniformIndexing,
            5340 => RayTracingNV,
            5341 => RayTracingMotionBlurNV,
            5345 => VulkanMemoryModel,
            5346 => VulkanMemoryModelDeviceScope,
            5347 => PhysicalStorageBufferAddresses,
            5350 => ComputeDerivativeGroupLinearNV,
            5353 => RayTracingProvisionalKHR,
            5357 => CooperativeMatrixNV,
            5363 => FragmentShaderSampleInterlockEXT,
            5372 => FragmentShaderShadingRateInterlockEXT,
            5373 => ShaderSMBuiltinsNV,
            5378 => FragmentShaderPixelInterlockEXT,
            5379 => DemoteToHelperInvocation,
            5390 => BindlessTextureNV,
            5568 => SubgroupShuffleINTEL,
            5569 => SubgroupBufferBlockIOINTEL,
            5570 => SubgroupImageBlockIOINTEL,
            5579 => SubgroupImageMediaBlockIOINTEL,
            5582 => RoundToInfinityINTEL,
            5583 => FloatingPointModeINTEL,
            5584 => IntegerFunctions2INTEL,
            5603 => FunctionPointersINTEL,
            5604 => IndirectReferencesINTEL,
            5606 => AsmINTEL,
            5612 => AtomicFloat32MinMaxEXT,
            5613 => AtomicFloat64MinMaxEXT,
            5616 => AtomicFloat16MinMaxEXT,
            5617 => VectorComputeINTEL,
            5619 => VectorAnyINTEL,
            5629 => ExpectAssumeKHR,
            5696 => SubgroupAvcMotionEstimationINTEL,
            5697 => SubgroupAvcMotionEstimationIntraINTEL,
            5698 => SubgroupAvcMotionEstimationChromaINTEL,
            5817 => VariableLengthArrayINTEL,
            5821 => FunctionFloatControlINTEL,
            5824 => FPGAMemoryAttributesINTEL,
            5837 => FPFastMathModeINTEL,
            5844 => ArbitraryPrecisionIntegersINTEL,
            5845 => ArbitraryPrecisionFloatingPointINTEL,
            5886 => UnstructuredLoopControlsINTEL,
            5888 => FPGALoopControlsINTEL,
            5892 => KernelAttributesINTEL,
            5897 => FPGAKernelAttributesINTEL,
            5898 => FPGAMemoryAccessesINTEL,
            5904 => FPGAClusterAttributesINTEL,
            5906 => LoopFuseINTEL,
            5910 => MemoryAccessAliasingINTEL,
            5920 => FPGABufferLocationINTEL,
            5922 => ArbitraryPrecisionFixedPointINTEL,
            5935 => USMStorageClassesINTEL,
            5943 => IOPipesINTEL,
            5945 => BlockingPipesINTEL,
            5948 => FPGARegINTEL,
            6016 => DotProductInputAll,
            6017 => DotProductInput4x8Bit,
            6018 => DotProductInput4x8BitPacked,
            6019 => DotProduct,
            6025 => BitInstructions,
            6033 => AtomicFloat32AddEXT,
            6034 => AtomicFloat64AddEXT,
            6089 => LongConstantCompositeINTEL,
            6094 => OptNoneINTEL,
            6095 => AtomicFloat16AddEXT,
            6114 => DebugInfoModuleINTEL,
            6141 => SplitBarrierINTEL,
            6400 => GroupUniformArithmeticKHR,
            what => panic!("{:?}", what),
        }
    }
}
impl RayQueryIntersection {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for RayQueryIntersection {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use RayQueryIntersection::*;
        match self {
            RayQueryCandidateIntersectionKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            RayQueryCommittedIntersectionKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use RayQueryIntersection::*;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCandidateIntersectionKHR,
            1 => RayQueryCommittedIntersectionKHR,
            what => panic!("{:?}", what),
        }
    }
}
impl RayQueryCommittedIntersectionType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for RayQueryCommittedIntersectionType {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use RayQueryCommittedIntersectionType::*;
        match self {
            RayQueryCommittedIntersectionNoneKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            RayQueryCommittedIntersectionTriangleKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            RayQueryCommittedIntersectionGeneratedKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use RayQueryCommittedIntersectionType::*;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCommittedIntersectionNoneKHR,
            1 => RayQueryCommittedIntersectionTriangleKHR,
            2 => RayQueryCommittedIntersectionGeneratedKHR,
            what => panic!("{:?}", what),
        }
    }
}
impl RayQueryCandidateIntersectionType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for RayQueryCandidateIntersectionType {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use RayQueryCandidateIntersectionType::*;
        match self {
            RayQueryCandidateIntersectionTriangleKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            RayQueryCandidateIntersectionAABBKHR => {
                req.add_cap(Capability::RayQueryKHR);
            }
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use RayQueryCandidateIntersectionType::*;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCandidateIntersectionTriangleKHR,
            1 => RayQueryCandidateIntersectionAABBKHR,
            what => panic!("{:?}", what),
        }
    }
}
impl PackedVectorFormat {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl Asm for PackedVectorFormat {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.opcode());
        use PackedVectorFormat::*;
        match self {
            PackedVectorFormat4x8Bit => {}
            what => panic!("{:?}", what),
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        use PackedVectorFormat::*;
        match chunk[*idx as usize - 1] {
            0 => PackedVectorFormat4x8Bit,
            what => panic!("{:?}", what),
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct ModuleRequirements {
    pub cap: std::collections::HashSet<Capability>,
    pub ext: std::collections::HashSet<String>,
}
impl ModuleRequirements {
    pub fn add_cap(&mut self, cap: Capability) {
        self.cap.insert(cap);
    }
    pub fn add_ext(&mut self, ext: String) {
        self.ext.insert(ext);
    }
}
impl<T: Asm> Asm for Option<T> {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        self.as_ref().map(|t| t.write_word(sink, req));
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        if *idx < chunk.len() {
            Some(T::read_word(chunk, idx))
        } else {
            None
        }
    }
}
impl<T: Asm, U: Asm> Asm for (T, U) {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        self.0.write_word(sink, req);
        self.1.write_word(sink, req);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let t = T::read_word(chunk, idx);
        let u = U::read_word(chunk, idx);
        (t, u)
    }
}
impl<T: Asm> Asm for Box<T> {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        self.as_ref().write_word(sink, req);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        Box::new(T::read_word(chunk, idx))
    }
}
impl<T: Asm> Asm for Vec<T> {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        self.iter().for_each(|t| t.write_word(sink, req));
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let mut re = vec![];
        while *idx < chunk.len() {
            re.push(T::read_word(chunk, idx));
        }
        re
    }
}
impl Asm for u32 {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(*self);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        chunk[*idx as usize - 1]
    }
}
impl Asm for String {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        let mark = sink.len();
        let strlen = (self.len() >> 2) + 1;
        sink.resize(mark + strlen, 0);
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.as_ptr(),
                sink.as_mut_ptr().offset(mark as isize) as *mut u8,
                self.len(),
            );
        }
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let chunk = &chunk[*idx as usize..];
        let mut len = 0;
        'outer: for u in chunk {
            for u in u.to_le_bytes().iter() {
                if *u == 0 {
                    break 'outer;
                }
                len += 1;
            }
        }
        let offset = (len >> 2) + 1;
        *idx += offset;
        unsafe {
            let s = std::slice::from_raw_parts((chunk.as_ptr() as *const u8), len as usize);
            std::str::from_utf8(s).unwrap().to_owned()
        }
    }
}
impl Asm for ID {
    fn write_word(&self, sink: &mut Vec<u32>, req: &mut ModuleRequirements) {
        sink.push(self.0);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let id = ID(chunk[*idx as usize]);
        *idx += 1;
        id
    }
}
