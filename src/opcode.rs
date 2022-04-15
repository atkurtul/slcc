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
    fn write_word(&self, sink: &mut Vec<u32>) {
        let opc_idx = sink.len();
        sink.push(0);
        let mut fields = self.fields.iter().collect::<Vec<&T>>();
        fields.sort_by(|a, b| a.opcode().cmp(&b.opcode()));
        for field in &self.fields {
            field.write_word(opc_idx, sink);
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
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>);
    fn read_word(chunk: &[u32], opc: &mut u32, idx: &mut usize) -> Self;
}
pub trait Asm {
    fn write_word(&self, sink: &mut Vec<u32>);
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            OpNop => (),
            OpUndef(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSourceContinued(x0) => {
                x0.write_word(sink);
            }
            OpSource(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSourceExtension(x0) => {
                x0.write_word(sink);
            }
            OpName(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpMemberName(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpString(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpLine(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpExtension(x0) => {
                x0.write_word(sink);
            }
            OpExtInstImport(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpExtInst(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpMemoryModel(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpEntryPoint(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpExecutionMode(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpCapability(x0) => {
                x0.write_word(sink);
            }
            OpTypeVoid(x0) => {
                x0.write_word(sink);
            }
            OpTypeBool(x0) => {
                x0.write_word(sink);
            }
            OpTypeInt(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeFloat(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeVector(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeMatrix(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpTypeSampler(x0) => {
                x0.write_word(sink);
            }
            OpTypeSampledImage(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeArray(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeRuntimeArray(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeStruct(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeOpaque(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypePointer(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeFunction(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypeEvent(x0) => {
                x0.write_word(sink);
            }
            OpTypeDeviceEvent(x0) => {
                x0.write_word(sink);
            }
            OpTypeReserveId(x0) => {
                x0.write_word(sink);
            }
            OpTypeQueue(x0) => {
                x0.write_word(sink);
            }
            OpTypePipe(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeForwardPointer(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpConstantTrue(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpConstantFalse(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpConstant(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConstantComposite(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConstantSampler(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpConstantNull(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSpecConstantTrue(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSpecConstantFalse(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSpecConstant(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSpecConstantComposite(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSpecConstantOp(_, _, specop) => specop.write_word(sink),
            OpFunction(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFunctionParameter(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpFunctionEnd => (),
            OpFunctionCall(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpVariable(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpImageTexelPointer(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpLoad(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpStore(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpCopyMemory(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpCopyMemorySized(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpAccessChain(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpInBoundsAccessChain(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpPtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpArrayLength(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGenericPtrMemSemantics(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpInBoundsPtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpDecorate(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpMemberDecorate(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDecorationGroup(x0) => {
                x0.write_word(sink);
            }
            OpGroupDecorate(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpGroupMemberDecorate(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpVectorExtractDynamic(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpVectorInsertDynamic(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpVectorShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCompositeConstruct(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpCompositeExtract(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpCompositeInsert(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCopyObject(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTranspose(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSampledImage(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpImageSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageRead(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageWrite(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpImage(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageQueryFormat(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageQueryOrder(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageQuerySizeLod(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpImageQuerySize(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageQueryLod(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpImageQueryLevels(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageQuerySamples(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertFToU(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertFToS(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertSToF(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertUToF(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpUConvert(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSConvert(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFConvert(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpQuantizeToF16(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertPtrToU(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSatConvertSToU(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSatConvertUToS(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertUToPtr(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpPtrCastToGeneric(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGenericCastToPtr(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGenericCastToPtrExplicit(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpBitcast(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSNegate(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFNegate(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIAdd(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFAdd(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpISub(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFSub(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIMul(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFMul(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUDiv(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSDiv(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFDiv(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUMod(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSRem(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSMod(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFRem(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFMod(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpVectorTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpMatrixTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpVectorTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpMatrixTimesVector(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpMatrixTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpOuterProduct(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpDot(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIAddCarry(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpISubBorrow(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUMulExtended(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSMulExtended(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpAny(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpAll(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIsNan(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIsInf(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIsFinite(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIsNormal(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSignBitSet(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpLessOrGreater(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpOrdered(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUnordered(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpLogicalEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpLogicalNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpLogicalOr(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpLogicalAnd(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpLogicalNot(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSelect(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpIEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpINotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpULessThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpULessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordLessThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFOrdGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFUnordGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpShiftRightLogical(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpShiftRightArithmetic(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpShiftLeftLogical(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpBitwiseOr(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpBitwiseXor(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpBitwiseAnd(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpNot(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpBitFieldInsert(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpBitFieldSExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpBitFieldUExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpBitReverse(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpBitCount(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdx(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdy(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFwidth(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdxFine(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdyFine(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFwidthFine(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdxCoarse(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpDPdyCoarse(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFwidthCoarse(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpEmitVertex => (),
            OpEndPrimitive => (),
            OpEmitStreamVertex(x0) => {
                x0.write_word(sink);
            }
            OpEndStreamPrimitive(x0) => {
                x0.write_word(sink);
            }
            OpControlBarrier(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpMemoryBarrier(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpAtomicLoad(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpAtomicStore(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpAtomicExchange(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpAtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpAtomicIIncrement(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpAtomicIDecrement(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpAtomicIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicISub(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpPhi(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpLoopMerge(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSelectionMerge(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpLabel(x0) => {
                x0.write_word(sink);
            }
            OpBranch(x0) => {
                x0.write_word(sink);
            }
            OpBranchConditional(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSwitch(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpKill => (),
            OpReturn => (),
            OpReturnValue(x0) => {
                x0.write_word(sink);
            }
            OpUnreachable => (),
            OpLifetimeStart(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpLifetimeStop(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpGroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpGroupWaitEvents(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGroupAll(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupAny(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupIAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupUMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupSMin(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupUMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupSMax(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpReadPipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpWritePipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpReserveReadPipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpReserveWritePipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpCommitReadPipe(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpCommitWritePipe(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIsValidReserveId(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGetNumPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGetMaxPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGroupCommitReadPipe(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupCommitWritePipe(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpEnqueueMarker(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpEnqueueKernel(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
                x11.write_word(sink);
                x12.write_word(sink);
            }
            OpGetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGetKernelPreferredWorkGroupSizeMultiple(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpRetainEvent(x0) => {
                x0.write_word(sink);
            }
            OpReleaseEvent(x0) => {
                x0.write_word(sink);
            }
            OpCreateUserEvent(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpIsValidEvent(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSetUserEventStatus(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpCaptureEventProfilingInfo(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGetDefaultQueue(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpBuildNDRange(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpImageSparseGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpImageSparseTexelsResident(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpNoLine => (),
            OpAtomicFlagTestAndSet(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpAtomicFlagClear(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageSparseRead(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSizeOf(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpTypePipeStorage(x0) => {
                x0.write_word(sink);
            }
            OpConstantPipeStorage(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCreatePipeFromPipeStorage(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGetKernelLocalSizeForSubgroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpTypeNamedBarrier(x0) => {
                x0.write_word(sink);
            }
            OpNamedBarrierInitialize(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpMemoryNamedBarrier(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpModuleProcessed(x0) => {
                x0.write_word(sink);
            }
            OpExecutionModeId(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpDecorateId(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpGroupNonUniformElect(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGroupNonUniformAll(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformAny(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformAllEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformBroadcastFirst(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformBallot(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformInverseBallot(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformBallotBitCount(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformBallotFindLSB(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformBallotFindMSB(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupNonUniformShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformShuffleXor(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformShuffleUp(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformShuffleDown(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformFAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformIMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformFMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformFMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformFMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpGroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupNonUniformQuadSwap(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCopyLogical(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpPtrEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpPtrNotEqual(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpPtrDiff(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpTerminateInvocation => (),
            OpSubgroupBallotKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupFirstInvocationKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAllKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAnyKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAllEqualKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupReadInvocationKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpTraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
            }
            OpExecuteCallableKHR(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpConvertUToAccelerationStructureKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpIgnoreIntersectionKHR => (),
            OpTerminateRayKHR => (),
            OpSDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpUDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSUDot(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpUDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSUDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpTypeRayQueryKHR(x0) => {
                x0.write_word(sink);
            }
            OpRayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpRayQueryTerminateKHR(x0) => {
                x0.write_word(sink);
            }
            OpRayQueryGenerateIntersectionKHR(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpRayQueryConfirmIntersectionKHR(x0) => {
                x0.write_word(sink);
            }
            OpRayQueryProceedKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetIntersectionTypeKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpGroupIAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupUMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupSMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupUMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupSMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpFragmentMaskFetchAMD(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFragmentFetchAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpReadClockKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpGroupNonUniformPartitionNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpWritePackedPrimitiveIndices4x8NV(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpReportIntersectionNV(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIgnoreIntersectionNV => (),
            OpTerminateRayNV => (),
            OpTraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
            }
            OpTraceMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
                x11.write_word(sink);
            }
            OpTraceRayMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
                x11.write_word(sink);
            }
            OpTypeAccelerationStructureNV(x0) => {
                x0.write_word(sink);
            }
            OpExecuteCallableNV(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeCooperativeMatrixNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpCooperativeMatrixStoreNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCooperativeMatrixMulAddNV(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpCooperativeMatrixLengthNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpBeginInvocationInterlockEXT => (),
            OpEndInvocationInterlockEXT => (),
            OpDemoteToHelperInvocation => (),
            OpIsHelperInvocationEXT(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpConvertUToImageNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertUToSamplerNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertImageToUNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertSamplerToUNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertUToSampledImageNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpConvertSampledImageToUNV(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSamplerImageAddressingModeNV(x0) => {
                x0.write_word(sink);
            }
            OpSubgroupShuffleINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupShuffleDownINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupShuffleUpINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupShuffleXorINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupBlockReadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupBlockWriteINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupImageBlockReadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupImageBlockWriteINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpUCountLeadingZerosINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpUCountTrailingZerosINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpAbsISubINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpAbsUSubINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpISubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUSubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpIMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpUMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpConstantFunctionPointerINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpFunctionPointerCallINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpAsmTargetINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpAsmINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAsmCallINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpAtomicFMinEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAtomicFMaxEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpAssumeTrueKHR(x0) => {
                x0.write_word(sink);
            }
            OpExpectKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpDecorateString(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpMemberDecorateString(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpVmeImageINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpTypeVmeImageINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeAvcImePayloadINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcRefPayloadINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcSicPayloadINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcMcePayloadINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcMceResultINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcImeResultINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcImeResultSingleReferenceStreamoutINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcImeResultDualReferenceStreamoutINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcImeSingleReferenceStreaminINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcImeDualReferenceStreaminINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcRefResultINTEL(x0) => {
                x0.write_word(sink);
            }
            OpTypeAvcSicResultINTEL(x0) => {
                x0.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpSubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
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
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpSubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcFmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpSubgroupAvcBmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpSubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcRefEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicInitializeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicConfigureSkcINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpSubgroupAvcSicConfigureIpeLumaINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
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
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
                x10.write_word(sink);
                x11.write_word(sink);
                x12.write_word(sink);
            }
            OpSubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcSicEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpSubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpVariableLengthArrayINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpSaveMemoryINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpRestoreMemoryINTEL(x0) => {
                x0.write_word(sink);
            }
            OpArbitraryFloatSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatCastFromIntINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
            }
            OpArbitraryFloatAddINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatSubINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatMulINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatDivINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatHypotINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
            }
            OpArbitraryFloatATan2INTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatPowINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatPowRINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
                x9.write_word(sink);
            }
            OpArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpLoopControlINTEL(x0) => {
                x0.write_word(sink);
            }
            OpAliasDomainDeclINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpAliasScopeDeclINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpAliasScopeListDeclINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpFixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpFixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
                x6.write_word(sink);
                x7.write_word(sink);
                x8.write_word(sink);
            }
            OpPtrCastToCrossWorkgroupINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpCrossWorkgroupCastToPtrINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpReadPipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpWritePipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpFPGARegINTEL(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetRayTMinKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetRayFlagsKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetIntersectionTKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionInstanceCustomIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetWorldRayDirectionKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetWorldRayOriginKHR(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpRayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpRayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
            }
            OpAtomicFAddEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
                x5.write_word(sink);
            }
            OpTypeBufferSurfaceINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            OpTypeStructContinuedINTEL(x0) => {
                x0.write_word(sink);
            }
            OpConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(sink);
            }
            OpSpecConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(sink);
            }
            OpControlBarrierArriveINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpControlBarrierWaitINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            OpGroupIMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupFMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupBitwiseAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupBitwiseOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupBitwiseXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupLogicalAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupLogicalOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
            }
            OpGroupLogicalXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
                x3.write_word(sink);
                x4.write_word(sink);
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
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>) {
        use ImageOperandsBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => (),
            Bias(x0) => {
                x0.write_word(sink);
            }
            Lod(x0) => {
                x0.write_word(sink);
            }
            Grad(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            ConstOffset(x0) => {
                x0.write_word(sink);
            }
            Offset(x0) => {
                x0.write_word(sink);
            }
            ConstOffsets(x0) => {
                x0.write_word(sink);
            }
            Sample(x0) => {
                x0.write_word(sink);
            }
            MinLod(x0) => {
                x0.write_word(sink);
            }
            MakeTexelAvailable(x0) => {
                x0.write_word(sink);
            }
            MakeTexelVisible(x0) => {
                x0.write_word(sink);
            }
            NonPrivateTexel => (),
            VolatileTexel => (),
            SignExtend => (),
            ZeroExtend => (),
            Nontemporal => (),
            Offsets(x0) => {
                x0.write_word(sink);
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>) {
        use LoopControlBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => (),
            Unroll => (),
            DontUnroll => (),
            DependencyInfinite => (),
            DependencyLength(x0) => {
                x0.write_word(sink);
            }
            MinIterations(x0) => {
                x0.write_word(sink);
            }
            MaxIterations(x0) => {
                x0.write_word(sink);
            }
            IterationMultiple(x0) => {
                x0.write_word(sink);
            }
            PeelCount(x0) => {
                x0.write_word(sink);
            }
            PartialCount(x0) => {
                x0.write_word(sink);
            }
            InitiationIntervalINTEL(x0) => {
                x0.write_word(sink);
            }
            MaxConcurrencyINTEL(x0) => {
                x0.write_word(sink);
            }
            DependencyArrayINTEL(x0) => {
                x0.write_word(sink);
            }
            PipelineEnableINTEL(x0) => {
                x0.write_word(sink);
            }
            LoopCoalesceINTEL(x0) => {
                x0.write_word(sink);
            }
            MaxInterleavingINTEL(x0) => {
                x0.write_word(sink);
            }
            SpeculatedIterationsINTEL(x0) => {
                x0.write_word(sink);
            }
            NoFusionINTEL(x0) => {
                x0.write_word(sink);
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, opc_idx: usize, sink: &mut Vec<u32>) {
        use MemoryAccessBits::*;
        sink[opc_idx] |= self.opcode();
        match self {
            None => (),
            Volatile => (),
            Aligned(x0) => {
                x0.write_word(sink);
            }
            Nontemporal => (),
            MakePointerAvailable(x0) => {
                x0.write_word(sink);
            }
            MakePointerVisible(x0) => {
                x0.write_word(sink);
            }
            NonPrivatePointer => (),
            AliasScopeINTELMask(x0) => {
                x0.write_word(sink);
            }
            NoAliasINTELMask(x0) => {
                x0.write_word(sink);
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use SourceLanguage::*;
        match self {
            Unknown => (),
            ESSL => (),
            GLSL => (),
            OpenCL_C => (),
            OpenCL_CPP => (),
            HLSL => (),
            CPP_for_OpenCL => (),
            SYCL => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use ExecutionModel::*;
        match self {
            Vertex => (),
            TessellationControl => (),
            TessellationEvaluation => (),
            Geometry => (),
            Fragment => (),
            GLCompute => (),
            Kernel => (),
            TaskNV => (),
            MeshNV => (),
            RayGenerationNV => (),
            IntersectionNV => (),
            AnyHitNV => (),
            ClosestHitNV => (),
            MissNV => (),
            CallableNV => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use AddressingModel::*;
        match self {
            Logical => (),
            Physical32 => (),
            Physical64 => (),
            PhysicalStorageBuffer64 => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use MemoryModel::*;
        match self {
            Simple => (),
            GLSL450 => (),
            OpenCL => (),
            Vulkan => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use ExecutionMode::*;
        match self {
            Invocations(x0) => {
                x0.write_word(sink);
            }
            SpacingEqual => (),
            SpacingFractionalEven => (),
            SpacingFractionalOdd => (),
            VertexOrderCw => (),
            VertexOrderCcw => (),
            PixelCenterInteger => (),
            OriginUpperLeft => (),
            OriginLowerLeft => (),
            EarlyFragmentTests => (),
            PointMode => (),
            Xfb => (),
            DepthReplacing => (),
            DepthGreater => (),
            DepthLess => (),
            DepthUnchanged => (),
            LocalSize(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            LocalSizeHint(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            InputPoints => (),
            InputLines => (),
            InputLinesAdjacency => (),
            Triangles => (),
            InputTrianglesAdjacency => (),
            Quads => (),
            Isolines => (),
            OutputVertices(x0) => {
                x0.write_word(sink);
            }
            OutputPoints => (),
            OutputLineStrip => (),
            OutputTriangleStrip => (),
            VecTypeHint(x0) => {
                x0.write_word(sink);
            }
            ContractionOff => (),
            Initializer => (),
            Finalizer => (),
            SubgroupSize(x0) => {
                x0.write_word(sink);
            }
            SubgroupsPerWorkgroup(x0) => {
                x0.write_word(sink);
            }
            SubgroupsPerWorkgroupId(x0) => {
                x0.write_word(sink);
            }
            LocalSizeId(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            LocalSizeHintId(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            SubgroupUniformControlFlowKHR => (),
            PostDepthCoverage => (),
            DenormPreserve(x0) => {
                x0.write_word(sink);
            }
            DenormFlushToZero(x0) => {
                x0.write_word(sink);
            }
            SignedZeroInfNanPreserve(x0) => {
                x0.write_word(sink);
            }
            RoundingModeRTE(x0) => {
                x0.write_word(sink);
            }
            RoundingModeRTZ(x0) => {
                x0.write_word(sink);
            }
            StencilRefReplacingEXT => (),
            OutputLinesNV => (),
            OutputPrimitivesNV(x0) => {
                x0.write_word(sink);
            }
            DerivativeGroupQuadsNV => (),
            DerivativeGroupLinearNV => (),
            OutputTrianglesNV => (),
            PixelInterlockOrderedEXT => (),
            PixelInterlockUnorderedEXT => (),
            SampleInterlockOrderedEXT => (),
            SampleInterlockUnorderedEXT => (),
            ShadingRateInterlockOrderedEXT => (),
            ShadingRateInterlockUnorderedEXT => (),
            SharedLocalMemorySizeINTEL(x0) => {
                x0.write_word(sink);
            }
            RoundingModeRTPINTEL(x0) => {
                x0.write_word(sink);
            }
            RoundingModeRTNINTEL(x0) => {
                x0.write_word(sink);
            }
            FloatingPointModeALTINTEL(x0) => {
                x0.write_word(sink);
            }
            FloatingPointModeIEEEINTEL(x0) => {
                x0.write_word(sink);
            }
            MaxWorkgroupSizeINTEL(x0, x1, x2) => {
                x0.write_word(sink);
                x1.write_word(sink);
                x2.write_word(sink);
            }
            MaxWorkDimINTEL(x0) => {
                x0.write_word(sink);
            }
            NoGlobalOffsetINTEL => (),
            NumSIMDWorkitemsINTEL(x0) => {
                x0.write_word(sink);
            }
            SchedulerTargetFmaxMhzINTEL(x0) => {
                x0.write_word(sink);
            }
            NamedBarrierCountINTEL(x0) => {
                x0.write_word(sink);
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use StorageClass::*;
        match self {
            UniformConstant => (),
            Input => (),
            Uniform => (),
            Output => (),
            Workgroup => (),
            CrossWorkgroup => (),
            Private => (),
            Function => (),
            Generic => (),
            PushConstant => (),
            AtomicCounter => (),
            Image => (),
            StorageBuffer => (),
            CallableDataNV => (),
            IncomingCallableDataNV => (),
            RayPayloadNV => (),
            HitAttributeNV => (),
            IncomingRayPayloadNV => (),
            ShaderRecordBufferNV => (),
            PhysicalStorageBuffer => (),
            CodeSectionINTEL => (),
            DeviceOnlyINTEL => (),
            HostOnlyINTEL => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use Dim::*;
        match self {
            _1D => (),
            _2D => (),
            _3D => (),
            Cube => (),
            Rect => (),
            Buffer => (),
            SubpassData => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use SamplerAddressingMode::*;
        match self {
            None => (),
            ClampToEdge => (),
            Clamp => (),
            Repeat => (),
            RepeatMirrored => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use SamplerFilterMode::*;
        match self {
            Nearest => (),
            Linear => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use ImageFormat::*;
        match self {
            Unknown => (),
            Rgba32f => (),
            Rgba16f => (),
            R32f => (),
            Rgba8 => (),
            Rgba8Snorm => (),
            Rg32f => (),
            Rg16f => (),
            R11fG11fB10f => (),
            R16f => (),
            Rgba16 => (),
            Rgb10A2 => (),
            Rg16 => (),
            Rg8 => (),
            R16 => (),
            R8 => (),
            Rgba16Snorm => (),
            Rg16Snorm => (),
            Rg8Snorm => (),
            R16Snorm => (),
            R8Snorm => (),
            Rgba32i => (),
            Rgba16i => (),
            Rgba8i => (),
            R32i => (),
            Rg32i => (),
            Rg16i => (),
            Rg8i => (),
            R16i => (),
            R8i => (),
            Rgba32ui => (),
            Rgba16ui => (),
            Rgba8ui => (),
            R32ui => (),
            Rgb10a2ui => (),
            Rg32ui => (),
            Rg16ui => (),
            Rg8ui => (),
            R16ui => (),
            R8ui => (),
            R64ui => (),
            R64i => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use ImageChannelOrder::*;
        match self {
            R => (),
            A => (),
            RG => (),
            RA => (),
            RGB => (),
            RGBA => (),
            BGRA => (),
            ARGB => (),
            Intensity => (),
            Luminance => (),
            Rx => (),
            RGx => (),
            RGBx => (),
            Depth => (),
            DepthStencil => (),
            sRGB => (),
            sRGBx => (),
            sRGBA => (),
            sBGRA => (),
            ABGR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use ImageChannelDataType::*;
        match self {
            SnormInt8 => (),
            SnormInt16 => (),
            UnormInt8 => (),
            UnormInt16 => (),
            UnormShort565 => (),
            UnormShort555 => (),
            UnormInt101010 => (),
            SignedInt8 => (),
            SignedInt16 => (),
            SignedInt32 => (),
            UnsignedInt8 => (),
            UnsignedInt16 => (),
            UnsignedInt32 => (),
            HalfFloat => (),
            Float => (),
            UnormInt24 => (),
            UnormInt101010_2 => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use FPRoundingMode::*;
        match self {
            RTE => (),
            RTZ => (),
            RTP => (),
            RTN => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use FPDenormMode::*;
        match self {
            Preserve => (),
            FlushToZero => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use QuantizationModes::*;
        match self {
            TRN => (),
            TRN_ZERO => (),
            RND => (),
            RND_ZERO => (),
            RND_INF => (),
            RND_MIN_INF => (),
            RND_CONV => (),
            RND_CONV_ODD => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use FPOperationMode::*;
        match self {
            IEEE => (),
            ALT => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use OverflowModes::*;
        match self {
            WRAP => (),
            SAT => (),
            SAT_ZERO => (),
            SAT_SYM => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use LinkageType::*;
        match self {
            Export => (),
            Import => (),
            LinkOnceODR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use AccessQualifier::*;
        match self {
            ReadOnly => (),
            WriteOnly => (),
            ReadWrite => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use FunctionParameterAttribute::*;
        match self {
            Zext => (),
            Sext => (),
            ByVal => (),
            Sret => (),
            NoAlias => (),
            NoCapture => (),
            NoWrite => (),
            NoReadWrite => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use Decoration::*;
        match self {
            RelaxedPrecision => (),
            SpecId(x0) => {
                x0.write_word(sink);
            }
            Block => (),
            BufferBlock => (),
            RowMajor => (),
            ColMajor => (),
            ArrayStride(x0) => {
                x0.write_word(sink);
            }
            MatrixStride(x0) => {
                x0.write_word(sink);
            }
            GLSLShared => (),
            GLSLPacked => (),
            CPacked => (),
            BuiltIn(x0) => {
                x0.write_word(sink);
            }
            NoPerspective => (),
            Flat => (),
            Patch => (),
            Centroid => (),
            Sample => (),
            Invariant => (),
            Restrict => (),
            Aliased => (),
            Volatile => (),
            Constant => (),
            Coherent => (),
            NonWritable => (),
            NonReadable => (),
            Uniform => (),
            UniformId(x0) => {
                x0.write_word(sink);
            }
            SaturatedConversion => (),
            Stream(x0) => {
                x0.write_word(sink);
            }
            Location(x0) => {
                x0.write_word(sink);
            }
            Component(x0) => {
                x0.write_word(sink);
            }
            Index(x0) => {
                x0.write_word(sink);
            }
            Binding(x0) => {
                x0.write_word(sink);
            }
            DescriptorSet(x0) => {
                x0.write_word(sink);
            }
            Offset(x0) => {
                x0.write_word(sink);
            }
            XfbBuffer(x0) => {
                x0.write_word(sink);
            }
            XfbStride(x0) => {
                x0.write_word(sink);
            }
            FuncParamAttr(x0) => {
                x0.write_word(sink);
            }
            FPRoundingMode(x0) => {
                x0.write_word(sink);
            }
            FPFastMathMode(x0) => {
                x0.write_word(sink);
            }
            LinkageAttributes(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            NoContraction => (),
            InputAttachmentIndex(x0) => {
                x0.write_word(sink);
            }
            Alignment(x0) => {
                x0.write_word(sink);
            }
            MaxByteOffset(x0) => {
                x0.write_word(sink);
            }
            AlignmentId(x0) => {
                x0.write_word(sink);
            }
            MaxByteOffsetId(x0) => {
                x0.write_word(sink);
            }
            NoSignedWrap => (),
            NoUnsignedWrap => (),
            ExplicitInterpAMD => (),
            OverrideCoverageNV => (),
            PassthroughNV => (),
            ViewportRelativeNV => (),
            SecondaryViewportRelativeNV(x0) => {
                x0.write_word(sink);
            }
            PerPrimitiveNV => (),
            PerViewNV => (),
            PerTaskNV => (),
            PerVertexKHR => (),
            NonUniform => (),
            RestrictPointer => (),
            AliasedPointer => (),
            BindlessSamplerNV => (),
            BindlessImageNV => (),
            BoundSamplerNV => (),
            BoundImageNV => (),
            SIMTCallINTEL(x0) => {
                x0.write_word(sink);
            }
            ReferencedIndirectlyINTEL => (),
            ClobberINTEL(x0) => {
                x0.write_word(sink);
            }
            SideEffectsINTEL => (),
            VectorComputeVariableINTEL => (),
            FuncParamIOKindINTEL(x0) => {
                x0.write_word(sink);
            }
            VectorComputeFunctionINTEL => (),
            StackCallINTEL => (),
            GlobalVariableOffsetINTEL(x0) => {
                x0.write_word(sink);
            }
            CounterBuffer(x0) => {
                x0.write_word(sink);
            }
            UserSemantic(x0) => {
                x0.write_word(sink);
            }
            UserTypeGOOGLE(x0) => {
                x0.write_word(sink);
            }
            FunctionRoundingModeINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            FunctionDenormModeINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            RegisterINTEL => (),
            MemoryINTEL(x0) => {
                x0.write_word(sink);
            }
            NumbanksINTEL(x0) => {
                x0.write_word(sink);
            }
            BankwidthINTEL(x0) => {
                x0.write_word(sink);
            }
            MaxPrivateCopiesINTEL(x0) => {
                x0.write_word(sink);
            }
            SinglepumpINTEL => (),
            DoublepumpINTEL => (),
            MaxReplicatesINTEL(x0) => {
                x0.write_word(sink);
            }
            SimpleDualPortINTEL => (),
            MergeINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            BankBitsINTEL(x0) => {
                x0.write_word(sink);
            }
            ForcePow2DepthINTEL(x0) => {
                x0.write_word(sink);
            }
            BurstCoalesceINTEL => (),
            CacheSizeINTEL(x0) => {
                x0.write_word(sink);
            }
            DontStaticallyCoalesceINTEL => (),
            PrefetchINTEL(x0) => {
                x0.write_word(sink);
            }
            StallEnableINTEL => (),
            FuseLoopsInFunctionINTEL => (),
            AliasScopeINTEL(x0) => {
                x0.write_word(sink);
            }
            NoAliasINTEL(x0) => {
                x0.write_word(sink);
            }
            BufferLocationINTEL(x0) => {
                x0.write_word(sink);
            }
            IOPipeStorageINTEL(x0) => {
                x0.write_word(sink);
            }
            FunctionFloatingPointModeINTEL(x0, x1) => {
                x0.write_word(sink);
                x1.write_word(sink);
            }
            SingleElementVectorINTEL => (),
            VectorComputeCallableFunctionINTEL => (),
            MediaBlockIOINTEL => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use BuiltIn::*;
        match self {
            Position => (),
            PointSize => (),
            ClipDistance => (),
            CullDistance => (),
            VertexId => (),
            InstanceId => (),
            PrimitiveId => (),
            InvocationId => (),
            Layer => (),
            ViewportIndex => (),
            TessLevelOuter => (),
            TessLevelInner => (),
            TessCoord => (),
            PatchVertices => (),
            FragCoord => (),
            PointCoord => (),
            FrontFacing => (),
            SampleId => (),
            SamplePosition => (),
            SampleMask => (),
            FragDepth => (),
            HelperInvocation => (),
            NumWorkgroups => (),
            WorkgroupSize => (),
            WorkgroupId => (),
            LocalInvocationId => (),
            GlobalInvocationId => (),
            LocalInvocationIndex => (),
            WorkDim => (),
            GlobalSize => (),
            EnqueuedWorkgroupSize => (),
            GlobalOffset => (),
            GlobalLinearId => (),
            SubgroupSize => (),
            SubgroupMaxSize => (),
            NumSubgroups => (),
            NumEnqueuedSubgroups => (),
            SubgroupId => (),
            SubgroupLocalInvocationId => (),
            VertexIndex => (),
            InstanceIndex => (),
            SubgroupEqMask => (),
            SubgroupGeMask => (),
            SubgroupGtMask => (),
            SubgroupLeMask => (),
            SubgroupLtMask => (),
            BaseVertex => (),
            BaseInstance => (),
            DrawIndex => (),
            PrimitiveShadingRateKHR => (),
            DeviceIndex => (),
            ViewIndex => (),
            ShadingRateKHR => (),
            BaryCoordNoPerspAMD => (),
            BaryCoordNoPerspCentroidAMD => (),
            BaryCoordNoPerspSampleAMD => (),
            BaryCoordSmoothAMD => (),
            BaryCoordSmoothCentroidAMD => (),
            BaryCoordSmoothSampleAMD => (),
            BaryCoordPullModelAMD => (),
            FragStencilRefEXT => (),
            ViewportMaskNV => (),
            SecondaryPositionNV => (),
            SecondaryViewportMaskNV => (),
            PositionPerViewNV => (),
            ViewportMaskPerViewNV => (),
            FullyCoveredEXT => (),
            TaskCountNV => (),
            PrimitiveCountNV => (),
            PrimitiveIndicesNV => (),
            ClipDistancePerViewNV => (),
            CullDistancePerViewNV => (),
            LayerPerViewNV => (),
            MeshViewCountNV => (),
            MeshViewIndicesNV => (),
            BaryCoordKHR => (),
            BaryCoordNoPerspKHR => (),
            FragSizeEXT => (),
            FragInvocationCountEXT => (),
            LaunchIdNV => (),
            LaunchSizeNV => (),
            WorldRayOriginNV => (),
            WorldRayDirectionNV => (),
            ObjectRayOriginNV => (),
            ObjectRayDirectionNV => (),
            RayTminNV => (),
            RayTmaxNV => (),
            InstanceCustomIndexNV => (),
            ObjectToWorldNV => (),
            WorldToObjectNV => (),
            HitTNV => (),
            HitKindNV => (),
            CurrentRayTimeNV => (),
            IncomingRayFlagsNV => (),
            RayGeometryIndexKHR => (),
            WarpsPerSMNV => (),
            SMCountNV => (),
            WarpIDNV => (),
            SMIDNV => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use Scope::*;
        match self {
            CrossDevice => (),
            Device => (),
            Workgroup => (),
            Subgroup => (),
            Invocation => (),
            QueueFamily => (),
            ShaderCallKHR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use GroupOperation::*;
        match self {
            Reduce => (),
            InclusiveScan => (),
            ExclusiveScan => (),
            ClusteredReduce => (),
            PartitionedReduceNV => (),
            PartitionedInclusiveScanNV => (),
            PartitionedExclusiveScanNV => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use KernelEnqueueFlags::*;
        match self {
            NoWait => (),
            WaitKernel => (),
            WaitWorkGroup => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use Capability::*;
        match self {
            Matrix => (),
            Shader => (),
            Geometry => (),
            Tessellation => (),
            Addresses => (),
            Linkage => (),
            Kernel => (),
            Vector16 => (),
            Float16Buffer => (),
            Float16 => (),
            Float64 => (),
            Int64 => (),
            Int64Atomics => (),
            ImageBasic => (),
            ImageReadWrite => (),
            ImageMipmap => (),
            Pipes => (),
            Groups => (),
            DeviceEnqueue => (),
            LiteralSampler => (),
            AtomicStorage => (),
            Int16 => (),
            TessellationPointSize => (),
            GeometryPointSize => (),
            ImageGatherExtended => (),
            StorageImageMultisample => (),
            UniformBufferArrayDynamicIndexing => (),
            SampledImageArrayDynamicIndexing => (),
            StorageBufferArrayDynamicIndexing => (),
            StorageImageArrayDynamicIndexing => (),
            ClipDistance => (),
            CullDistance => (),
            ImageCubeArray => (),
            SampleRateShading => (),
            ImageRect => (),
            SampledRect => (),
            GenericPointer => (),
            Int8 => (),
            InputAttachment => (),
            SparseResidency => (),
            MinLod => (),
            Sampled1D => (),
            Image1D => (),
            SampledCubeArray => (),
            SampledBuffer => (),
            ImageBuffer => (),
            ImageMSArray => (),
            StorageImageExtendedFormats => (),
            ImageQuery => (),
            DerivativeControl => (),
            InterpolationFunction => (),
            TransformFeedback => (),
            GeometryStreams => (),
            StorageImageReadWithoutFormat => (),
            StorageImageWriteWithoutFormat => (),
            MultiViewport => (),
            SubgroupDispatch => (),
            NamedBarrier => (),
            PipeStorage => (),
            GroupNonUniform => (),
            GroupNonUniformVote => (),
            GroupNonUniformArithmetic => (),
            GroupNonUniformBallot => (),
            GroupNonUniformShuffle => (),
            GroupNonUniformShuffleRelative => (),
            GroupNonUniformClustered => (),
            GroupNonUniformQuad => (),
            ShaderLayer => (),
            ShaderViewportIndex => (),
            UniformDecoration => (),
            FragmentShadingRateKHR => (),
            SubgroupBallotKHR => (),
            DrawParameters => (),
            WorkgroupMemoryExplicitLayoutKHR => (),
            WorkgroupMemoryExplicitLayout8BitAccessKHR => (),
            WorkgroupMemoryExplicitLayout16BitAccessKHR => (),
            SubgroupVoteKHR => (),
            StorageBuffer16BitAccess => (),
            UniformAndStorageBuffer16BitAccess => (),
            StoragePushConstant16 => (),
            StorageInputOutput16 => (),
            DeviceGroup => (),
            MultiView => (),
            VariablePointersStorageBuffer => (),
            VariablePointers => (),
            AtomicStorageOps => (),
            SampleMaskPostDepthCoverage => (),
            StorageBuffer8BitAccess => (),
            UniformAndStorageBuffer8BitAccess => (),
            StoragePushConstant8 => (),
            DenormPreserve => (),
            DenormFlushToZero => (),
            SignedZeroInfNanPreserve => (),
            RoundingModeRTE => (),
            RoundingModeRTZ => (),
            RayQueryProvisionalKHR => (),
            RayQueryKHR => (),
            RayTraversalPrimitiveCullingKHR => (),
            RayTracingKHR => (),
            Float16ImageAMD => (),
            ImageGatherBiasLodAMD => (),
            FragmentMaskAMD => (),
            StencilExportEXT => (),
            ImageReadWriteLodAMD => (),
            Int64ImageEXT => (),
            ShaderClockKHR => (),
            SampleMaskOverrideCoverageNV => (),
            GeometryShaderPassthroughNV => (),
            ShaderViewportIndexLayerEXT => (),
            ShaderViewportMaskNV => (),
            ShaderStereoViewNV => (),
            PerViewAttributesNV => (),
            FragmentFullyCoveredEXT => (),
            MeshShadingNV => (),
            ImageFootprintNV => (),
            FragmentBarycentricKHR => (),
            ComputeDerivativeGroupQuadsNV => (),
            FragmentDensityEXT => (),
            GroupNonUniformPartitionedNV => (),
            ShaderNonUniform => (),
            RuntimeDescriptorArray => (),
            InputAttachmentArrayDynamicIndexing => (),
            UniformTexelBufferArrayDynamicIndexing => (),
            StorageTexelBufferArrayDynamicIndexing => (),
            UniformBufferArrayNonUniformIndexing => (),
            SampledImageArrayNonUniformIndexing => (),
            StorageBufferArrayNonUniformIndexing => (),
            StorageImageArrayNonUniformIndexing => (),
            InputAttachmentArrayNonUniformIndexing => (),
            UniformTexelBufferArrayNonUniformIndexing => (),
            StorageTexelBufferArrayNonUniformIndexing => (),
            RayTracingNV => (),
            RayTracingMotionBlurNV => (),
            VulkanMemoryModel => (),
            VulkanMemoryModelDeviceScope => (),
            PhysicalStorageBufferAddresses => (),
            ComputeDerivativeGroupLinearNV => (),
            RayTracingProvisionalKHR => (),
            CooperativeMatrixNV => (),
            FragmentShaderSampleInterlockEXT => (),
            FragmentShaderShadingRateInterlockEXT => (),
            ShaderSMBuiltinsNV => (),
            FragmentShaderPixelInterlockEXT => (),
            DemoteToHelperInvocation => (),
            BindlessTextureNV => (),
            SubgroupShuffleINTEL => (),
            SubgroupBufferBlockIOINTEL => (),
            SubgroupImageBlockIOINTEL => (),
            SubgroupImageMediaBlockIOINTEL => (),
            RoundToInfinityINTEL => (),
            FloatingPointModeINTEL => (),
            IntegerFunctions2INTEL => (),
            FunctionPointersINTEL => (),
            IndirectReferencesINTEL => (),
            AsmINTEL => (),
            AtomicFloat32MinMaxEXT => (),
            AtomicFloat64MinMaxEXT => (),
            AtomicFloat16MinMaxEXT => (),
            VectorComputeINTEL => (),
            VectorAnyINTEL => (),
            ExpectAssumeKHR => (),
            SubgroupAvcMotionEstimationINTEL => (),
            SubgroupAvcMotionEstimationIntraINTEL => (),
            SubgroupAvcMotionEstimationChromaINTEL => (),
            VariableLengthArrayINTEL => (),
            FunctionFloatControlINTEL => (),
            FPGAMemoryAttributesINTEL => (),
            FPFastMathModeINTEL => (),
            ArbitraryPrecisionIntegersINTEL => (),
            ArbitraryPrecisionFloatingPointINTEL => (),
            UnstructuredLoopControlsINTEL => (),
            FPGALoopControlsINTEL => (),
            KernelAttributesINTEL => (),
            FPGAKernelAttributesINTEL => (),
            FPGAMemoryAccessesINTEL => (),
            FPGAClusterAttributesINTEL => (),
            LoopFuseINTEL => (),
            MemoryAccessAliasingINTEL => (),
            FPGABufferLocationINTEL => (),
            ArbitraryPrecisionFixedPointINTEL => (),
            USMStorageClassesINTEL => (),
            IOPipesINTEL => (),
            BlockingPipesINTEL => (),
            FPGARegINTEL => (),
            DotProductInputAll => (),
            DotProductInput4x8Bit => (),
            DotProductInput4x8BitPacked => (),
            DotProduct => (),
            BitInstructions => (),
            AtomicFloat32AddEXT => (),
            AtomicFloat64AddEXT => (),
            LongConstantCompositeINTEL => (),
            OptNoneINTEL => (),
            AtomicFloat16AddEXT => (),
            DebugInfoModuleINTEL => (),
            SplitBarrierINTEL => (),
            GroupUniformArithmeticKHR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use RayQueryIntersection::*;
        match self {
            RayQueryCandidateIntersectionKHR => (),
            RayQueryCommittedIntersectionKHR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use RayQueryCommittedIntersectionType::*;
        match self {
            RayQueryCommittedIntersectionNoneKHR => (),
            RayQueryCommittedIntersectionTriangleKHR => (),
            RayQueryCommittedIntersectionGeneratedKHR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use RayQueryCandidateIntersectionType::*;
        match self {
            RayQueryCandidateIntersectionTriangleKHR => (),
            RayQueryCandidateIntersectionAABBKHR => (),
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.opcode());
        use PackedVectorFormat::*;
        match self {
            PackedVectorFormat4x8Bit => (),
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
impl<T: Asm> Asm for Option<T> {
    fn write_word(&self, sink: &mut Vec<u32>) {
        self.as_ref().map(|t| t.write_word(sink));
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        self.0.write_word(sink);
        self.1.write_word(sink);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let t = T::read_word(chunk, idx);
        let u = U::read_word(chunk, idx);
        (t, u)
    }
}
impl<T: Asm> Asm for Box<T> {
    fn write_word(&self, sink: &mut Vec<u32>) {
        self.as_ref().write_word(sink);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        Box::new(T::read_word(chunk, idx))
    }
}
impl<T: Asm> Asm for Vec<T> {
    fn write_word(&self, sink: &mut Vec<u32>) {
        self.iter().for_each(|t| t.write_word(sink));
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(*self);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        *idx += 1;
        chunk[*idx as usize - 1]
    }
}
impl Asm for String {
    fn write_word(&self, sink: &mut Vec<u32>) {
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
    fn write_word(&self, sink: &mut Vec<u32>) {
        sink.push(self.0);
    }
    fn read_word(chunk: &[u32], idx: &mut usize) -> Self {
        let id = ID(chunk[*idx as usize]);
        *idx += 1;
        id
    }
}
