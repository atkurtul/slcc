#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Opcode {
    Nop() = 0,
    Undef(TypeID, ResultID) = 1,
    SourceContinued(String) = 2,
    Source(SourceLanguage, u32, Option<ID>, Option<String>) = 3,
    SourceExtension(String) = 4,
    Name(ID, String) = 5,
    MemberName(ID, u32, String) = 6,
    String(ResultID, String) = 7,
    Line(ID, u32, u32) = 8,
    Extension(String) = 10,
    ExtInstImport(ResultID, String) = 11,
    ExtInst(TypeID, ResultID, ID, u32, Vec<ID>) = 12,
    MemoryModel(AddressingModel, MemoryModel) = 14,
    EntryPoint(ExecutionModel, ID, String, Vec<ID>) = 15,
    ExecutionMode(ID, ExecutionMode) = 16,
    Capability(Capability) = 17,
    TypeVoid(ResultID) = 19,
    TypeBool(ResultID) = 20,
    TypeInt(ResultID, u32, u32) = 21,
    TypeFloat(ResultID, u32) = 22,
    TypeVector(ResultID, ID, u32) = 23,
    TypeMatrix(ResultID, ID, u32) = 24,
    TypeImage(
        ResultID,
        ID,
        Dim,
        u32,
        u32,
        u32,
        u32,
        ImageFormat,
        Option<AccessQualifier>,
    ) = 25,
    TypeSampler(ResultID) = 26,
    TypeSampledImage(ResultID, ID) = 27,
    TypeArray(ResultID, ID, ID) = 28,
    TypeRuntimeArray(ResultID, ID) = 29,
    TypeStruct(ResultID, Vec<ID>) = 30,
    TypeOpaque(ResultID, String) = 31,
    TypePointer(ResultID, StorageClass, ID) = 32,
    TypeFunction(ResultID, ID, Vec<ID>) = 33,
    TypeEvent(ResultID) = 34,
    TypeDeviceEvent(ResultID) = 35,
    TypeReserveId(ResultID) = 36,
    TypeQueue(ResultID) = 37,
    TypePipe(ResultID, AccessQualifier) = 38,
    TypeForwardPointer(ID, StorageClass) = 39,
    ConstantTrue(TypeID, ResultID) = 41,
    ConstantFalse(TypeID, ResultID) = 42,
    Constant(TypeID, ResultID, u32) = 43,
    ConstantComposite(TypeID, ResultID, Vec<ID>) = 44,
    ConstantSampler(
        TypeID,
        ResultID,
        SamplerAddressingMode,
        u32,
        SamplerFilterMode,
    ) = 45,
    ConstantNull(TypeID, ResultID) = 46,
    SpecConstantTrue(TypeID, ResultID) = 48,
    SpecConstantFalse(TypeID, ResultID) = 49,
    SpecConstant(TypeID, ResultID, u32) = 50,
    SpecConstantComposite(TypeID, ResultID, Vec<ID>) = 51,
    SpecConstantOp(TypeID, ResultID, Box<Opcode>) = 52,
    Function(TypeID, ResultID, FunctionControl, ID) = 54,
    FunctionParameter(TypeID, ResultID) = 55,
    FunctionEnd() = 56,
    FunctionCall(TypeID, ResultID, ID, Vec<ID>) = 57,
    Variable(TypeID, ResultID, StorageClass, Option<ID>) = 59,
    ImageTexelPointer(TypeID, ResultID, ID, ID, ID) = 60,
    Load(TypeID, ResultID, ID, Option<MemoryAccess>) = 61,
    Store(ID, ID, Option<MemoryAccess>) = 62,
    CopyMemory(ID, ID, Option<MemoryAccess>, Option<MemoryAccess>) = 63,
    CopyMemorySized(ID, ID, ID, Option<MemoryAccess>, Option<MemoryAccess>) = 64,
    AccessChain(TypeID, ResultID, ID, Vec<ID>) = 65,
    InBoundsAccessChain(TypeID, ResultID, ID, Vec<ID>) = 66,
    PtrAccessChain(TypeID, ResultID, ID, ID, Vec<ID>) = 67,
    ArrayLength(TypeID, ResultID, ID, u32) = 68,
    GenericPtrMemSemantics(TypeID, ResultID, ID) = 69,
    InBoundsPtrAccessChain(TypeID, ResultID, ID, ID, Vec<ID>) = 70,
    Decorate(ID, Decoration) = 71,
    MemberDecorate(ID, u32, Decoration) = 72,
    DecorationGroup(ResultID) = 73,
    GroupDecorate(ID, Vec<ID>) = 74,
    GroupMemberDecorate(ID, Vec<(ID, u32)>) = 75,
    VectorExtractDynamic(TypeID, ResultID, ID, ID) = 77,
    VectorInsertDynamic(TypeID, ResultID, ID, ID, ID) = 78,
    VectorShuffle(TypeID, ResultID, ID, ID, Vec<u32>) = 79,
    CompositeConstruct(TypeID, ResultID, Vec<ID>) = 80,
    CompositeExtract(TypeID, ResultID, ID, Vec<u32>) = 81,
    CompositeInsert(TypeID, ResultID, ID, ID, Vec<u32>) = 82,
    CopyObject(TypeID, ResultID, ID) = 83,
    Transpose(TypeID, ResultID, ID) = 84,
    SampledImage(TypeID, ResultID, ID, ID) = 86,
    ImageSampleImplicitLod(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 87,
    ImageSampleExplicitLod(TypeID, ResultID, ID, ID, ImageOperands) = 88,
    ImageSampleDrefImplicitLod(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 89,
    ImageSampleDrefExplicitLod(TypeID, ResultID, ID, ID, ID, ImageOperands) = 90,
    ImageSampleProjImplicitLod(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 91,
    ImageSampleProjExplicitLod(TypeID, ResultID, ID, ID, ImageOperands) = 92,
    ImageSampleProjDrefImplicitLod(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 93,
    ImageSampleProjDrefExplicitLod(TypeID, ResultID, ID, ID, ID, ImageOperands) = 94,
    ImageFetch(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 95,
    ImageGather(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 96,
    ImageDrefGather(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 97,
    ImageRead(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 98,
    ImageWrite(ID, ID, ID, Option<ImageOperands>) = 99,
    Image(TypeID, ResultID, ID) = 100,
    ImageQueryFormat(TypeID, ResultID, ID) = 101,
    ImageQueryOrder(TypeID, ResultID, ID) = 102,
    ImageQuerySizeLod(TypeID, ResultID, ID, ID) = 103,
    ImageQuerySize(TypeID, ResultID, ID) = 104,
    ImageQueryLod(TypeID, ResultID, ID, ID) = 105,
    ImageQueryLevels(TypeID, ResultID, ID) = 106,
    ImageQuerySamples(TypeID, ResultID, ID) = 107,
    ConvertFToU(TypeID, ResultID, ID) = 109,
    ConvertFToS(TypeID, ResultID, ID) = 110,
    ConvertSToF(TypeID, ResultID, ID) = 111,
    ConvertUToF(TypeID, ResultID, ID) = 112,
    UConvert(TypeID, ResultID, ID) = 113,
    SConvert(TypeID, ResultID, ID) = 114,
    FConvert(TypeID, ResultID, ID) = 115,
    QuantizeToF16(TypeID, ResultID, ID) = 116,
    ConvertPtrToU(TypeID, ResultID, ID) = 117,
    SatConvertSToU(TypeID, ResultID, ID) = 118,
    SatConvertUToS(TypeID, ResultID, ID) = 119,
    ConvertUToPtr(TypeID, ResultID, ID) = 120,
    PtrCastToGeneric(TypeID, ResultID, ID) = 121,
    GenericCastToPtr(TypeID, ResultID, ID) = 122,
    GenericCastToPtrExplicit(TypeID, ResultID, ID, StorageClass) = 123,
    Bitcast(TypeID, ResultID, ID) = 124,
    SNegate(TypeID, ResultID, ID) = 126,
    FNegate(TypeID, ResultID, ID) = 127,
    IAdd(TypeID, ResultID, ID, ID) = 128,
    FAdd(TypeID, ResultID, ID, ID) = 129,
    ISub(TypeID, ResultID, ID, ID) = 130,
    FSub(TypeID, ResultID, ID, ID) = 131,
    IMul(TypeID, ResultID, ID, ID) = 132,
    FMul(TypeID, ResultID, ID, ID) = 133,
    UDiv(TypeID, ResultID, ID, ID) = 134,
    SDiv(TypeID, ResultID, ID, ID) = 135,
    FDiv(TypeID, ResultID, ID, ID) = 136,
    UMod(TypeID, ResultID, ID, ID) = 137,
    SRem(TypeID, ResultID, ID, ID) = 138,
    SMod(TypeID, ResultID, ID, ID) = 139,
    FRem(TypeID, ResultID, ID, ID) = 140,
    FMod(TypeID, ResultID, ID, ID) = 141,
    VectorTimesScalar(TypeID, ResultID, ID, ID) = 142,
    MatrixTimesScalar(TypeID, ResultID, ID, ID) = 143,
    VectorTimesMatrix(TypeID, ResultID, ID, ID) = 144,
    MatrixTimesVector(TypeID, ResultID, ID, ID) = 145,
    MatrixTimesMatrix(TypeID, ResultID, ID, ID) = 146,
    OuterProduct(TypeID, ResultID, ID, ID) = 147,
    Dot(TypeID, ResultID, ID, ID) = 148,
    IAddCarry(TypeID, ResultID, ID, ID) = 149,
    ISubBorrow(TypeID, ResultID, ID, ID) = 150,
    UMulExtended(TypeID, ResultID, ID, ID) = 151,
    SMulExtended(TypeID, ResultID, ID, ID) = 152,
    Any(TypeID, ResultID, ID) = 154,
    All(TypeID, ResultID, ID) = 155,
    IsNan(TypeID, ResultID, ID) = 156,
    IsInf(TypeID, ResultID, ID) = 157,
    IsFinite(TypeID, ResultID, ID) = 158,
    IsNormal(TypeID, ResultID, ID) = 159,
    SignBitSet(TypeID, ResultID, ID) = 160,
    LessOrGreater(TypeID, ResultID, ID, ID) = 161,
    Ordered(TypeID, ResultID, ID, ID) = 162,
    Unordered(TypeID, ResultID, ID, ID) = 163,
    LogicalEqual(TypeID, ResultID, ID, ID) = 164,
    LogicalNotEqual(TypeID, ResultID, ID, ID) = 165,
    LogicalOr(TypeID, ResultID, ID, ID) = 166,
    LogicalAnd(TypeID, ResultID, ID, ID) = 167,
    LogicalNot(TypeID, ResultID, ID) = 168,
    Select(TypeID, ResultID, ID, ID, ID) = 169,
    IEqual(TypeID, ResultID, ID, ID) = 170,
    INotEqual(TypeID, ResultID, ID, ID) = 171,
    UGreaterThan(TypeID, ResultID, ID, ID) = 172,
    SGreaterThan(TypeID, ResultID, ID, ID) = 173,
    UGreaterThanEqual(TypeID, ResultID, ID, ID) = 174,
    SGreaterThanEqual(TypeID, ResultID, ID, ID) = 175,
    ULessThan(TypeID, ResultID, ID, ID) = 176,
    SLessThan(TypeID, ResultID, ID, ID) = 177,
    ULessThanEqual(TypeID, ResultID, ID, ID) = 178,
    SLessThanEqual(TypeID, ResultID, ID, ID) = 179,
    FOrdEqual(TypeID, ResultID, ID, ID) = 180,
    FUnordEqual(TypeID, ResultID, ID, ID) = 181,
    FOrdNotEqual(TypeID, ResultID, ID, ID) = 182,
    FUnordNotEqual(TypeID, ResultID, ID, ID) = 183,
    FOrdLessThan(TypeID, ResultID, ID, ID) = 184,
    FUnordLessThan(TypeID, ResultID, ID, ID) = 185,
    FOrdGreaterThan(TypeID, ResultID, ID, ID) = 186,
    FUnordGreaterThan(TypeID, ResultID, ID, ID) = 187,
    FOrdLessThanEqual(TypeID, ResultID, ID, ID) = 188,
    FUnordLessThanEqual(TypeID, ResultID, ID, ID) = 189,
    FOrdGreaterThanEqual(TypeID, ResultID, ID, ID) = 190,
    FUnordGreaterThanEqual(TypeID, ResultID, ID, ID) = 191,
    ShiftRightLogical(TypeID, ResultID, ID, ID) = 194,
    ShiftRightArithmetic(TypeID, ResultID, ID, ID) = 195,
    ShiftLeftLogical(TypeID, ResultID, ID, ID) = 196,
    BitwiseOr(TypeID, ResultID, ID, ID) = 197,
    BitwiseXor(TypeID, ResultID, ID, ID) = 198,
    BitwiseAnd(TypeID, ResultID, ID, ID) = 199,
    Not(TypeID, ResultID, ID) = 200,
    BitFieldInsert(TypeID, ResultID, ID, ID, ID, ID) = 201,
    BitFieldSExtract(TypeID, ResultID, ID, ID, ID) = 202,
    BitFieldUExtract(TypeID, ResultID, ID, ID, ID) = 203,
    BitReverse(TypeID, ResultID, ID) = 204,
    BitCount(TypeID, ResultID, ID) = 205,
    DPdx(TypeID, ResultID, ID) = 207,
    DPdy(TypeID, ResultID, ID) = 208,
    Fwidth(TypeID, ResultID, ID) = 209,
    DPdxFine(TypeID, ResultID, ID) = 210,
    DPdyFine(TypeID, ResultID, ID) = 211,
    FwidthFine(TypeID, ResultID, ID) = 212,
    DPdxCoarse(TypeID, ResultID, ID) = 213,
    DPdyCoarse(TypeID, ResultID, ID) = 214,
    FwidthCoarse(TypeID, ResultID, ID) = 215,
    EmitVertex() = 218,
    EndPrimitive() = 219,
    EmitStreamVertex(ID) = 220,
    EndStreamPrimitive(ID) = 221,
    ControlBarrier(ScopeID, ScopeID, MemorySemanticsID) = 224,
    MemoryBarrier(ScopeID, MemorySemanticsID) = 225,
    AtomicLoad(TypeID, ResultID, ID, ScopeID, MemorySemanticsID) = 227,
    AtomicStore(ID, ScopeID, MemorySemanticsID, ID) = 228,
    AtomicExchange(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 229,
    AtomicCompareExchange(
        TypeID,
        ResultID,
        ID,
        ScopeID,
        MemorySemanticsID,
        MemorySemanticsID,
        ID,
        ID,
    ) = 230,
    AtomicCompareExchangeWeak(
        TypeID,
        ResultID,
        ID,
        ScopeID,
        MemorySemanticsID,
        MemorySemanticsID,
        ID,
        ID,
    ) = 231,
    AtomicIIncrement(TypeID, ResultID, ID, ScopeID, MemorySemanticsID) = 232,
    AtomicIDecrement(TypeID, ResultID, ID, ScopeID, MemorySemanticsID) = 233,
    AtomicIAdd(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 234,
    AtomicISub(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 235,
    AtomicSMin(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 236,
    AtomicUMin(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 237,
    AtomicSMax(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 238,
    AtomicUMax(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 239,
    AtomicAnd(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 240,
    AtomicOr(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 241,
    AtomicXor(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 242,
    Phi(TypeID, ResultID, Vec<(ID, ID)>) = 245,
    LoopMerge(ID, ID, LoopControl) = 246,
    SelectionMerge(ID, SelectionControl) = 247,
    Label(ResultID) = 248,
    Branch(ID) = 249,
    BranchConditional(ID, ID, ID, Vec<u32>) = 250,
    Switch(ID, ID, Vec<(u32, ID)>) = 251,
    Kill() = 252,
    Return() = 253,
    ReturnValue(ID) = 254,
    Unreachable() = 255,
    LifetimeStart(ID, u32) = 256,
    LifetimeStop(ID, u32) = 257,
    GroupAsyncCopy(TypeID, ResultID, ScopeID, ID, ID, ID, ID, ID) = 259,
    GroupWaitEvents(ScopeID, ID, ID) = 260,
    GroupAll(TypeID, ResultID, ScopeID, ID) = 261,
    GroupAny(TypeID, ResultID, ScopeID, ID) = 262,
    GroupBroadcast(TypeID, ResultID, ScopeID, ID, ID) = 263,
    GroupIAdd(TypeID, ResultID, ScopeID, GroupOperation, ID) = 264,
    GroupFAdd(TypeID, ResultID, ScopeID, GroupOperation, ID) = 265,
    GroupFMin(TypeID, ResultID, ScopeID, GroupOperation, ID) = 266,
    GroupUMin(TypeID, ResultID, ScopeID, GroupOperation, ID) = 267,
    GroupSMin(TypeID, ResultID, ScopeID, GroupOperation, ID) = 268,
    GroupFMax(TypeID, ResultID, ScopeID, GroupOperation, ID) = 269,
    GroupUMax(TypeID, ResultID, ScopeID, GroupOperation, ID) = 270,
    GroupSMax(TypeID, ResultID, ScopeID, GroupOperation, ID) = 271,
    ReadPipe(TypeID, ResultID, ID, ID, ID, ID) = 274,
    WritePipe(TypeID, ResultID, ID, ID, ID, ID) = 275,
    ReservedReadPipe(TypeID, ResultID, ID, ID, ID, ID, ID, ID) = 276,
    ReservedWritePipe(TypeID, ResultID, ID, ID, ID, ID, ID, ID) = 277,
    ReserveReadPipePackets(TypeID, ResultID, ID, ID, ID, ID) = 278,
    ReserveWritePipePackets(TypeID, ResultID, ID, ID, ID, ID) = 279,
    CommitReadPipe(ID, ID, ID, ID) = 280,
    CommitWritePipe(ID, ID, ID, ID) = 281,
    IsValidReserveId(TypeID, ResultID, ID) = 282,
    GetNumPipePackets(TypeID, ResultID, ID, ID, ID) = 283,
    GetMaxPipePackets(TypeID, ResultID, ID, ID, ID) = 284,
    GroupReserveReadPipePackets(TypeID, ResultID, ScopeID, ID, ID, ID, ID) = 285,
    GroupReserveWritePipePackets(TypeID, ResultID, ScopeID, ID, ID, ID, ID) = 286,
    GroupCommitReadPipe(ScopeID, ID, ID, ID, ID) = 287,
    GroupCommitWritePipe(ScopeID, ID, ID, ID, ID) = 288,
    EnqueueMarker(TypeID, ResultID, ID, ID, ID, ID) = 291,
    EnqueueKernel(
        TypeID,
        ResultID,
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
    GetKernelNDrangeSubGroupCount(TypeID, ResultID, ID, ID, ID, ID, ID) = 293,
    GetKernelNDrangeMaxSubGroupSize(TypeID, ResultID, ID, ID, ID, ID, ID) = 294,
    GetKernelWorkGroupSize(TypeID, ResultID, ID, ID, ID, ID) = 295,
    GetKernelPreferredWorkGroupSizeMultiple(TypeID, ResultID, ID, ID, ID, ID) = 296,
    RetainEvent(ID) = 297,
    ReleaseEvent(ID) = 298,
    CreateUserEvent(TypeID, ResultID) = 299,
    IsValidEvent(TypeID, ResultID, ID) = 300,
    SetUserEventStatus(ID, ID) = 301,
    CaptureEventProfilingInfo(ID, ID, ID) = 302,
    GetDefaultQueue(TypeID, ResultID) = 303,
    BuildNDRange(TypeID, ResultID, ID, ID, ID) = 304,
    ImageSparseSampleImplicitLod(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 305,
    ImageSparseSampleExplicitLod(TypeID, ResultID, ID, ID, ImageOperands) = 306,
    ImageSparseSampleDrefImplicitLod(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 307,
    ImageSparseSampleDrefExplicitLod(TypeID, ResultID, ID, ID, ID, ImageOperands) = 308,
    ImageSparseSampleProjImplicitLod(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 309,
    ImageSparseSampleProjExplicitLod(TypeID, ResultID, ID, ID, ImageOperands) = 310,
    ImageSparseSampleProjDrefImplicitLod(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 311,
    ImageSparseSampleProjDrefExplicitLod(TypeID, ResultID, ID, ID, ID, ImageOperands) = 312,
    ImageSparseFetch(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 313,
    ImageSparseGather(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 314,
    ImageSparseDrefGather(TypeID, ResultID, ID, ID, ID, Option<ImageOperands>) = 315,
    ImageSparseTexelsResident(TypeID, ResultID, ID) = 316,
    NoLine() = 317,
    AtomicFlagTestAndSet(TypeID, ResultID, ID, ScopeID, MemorySemanticsID) = 318,
    AtomicFlagClear(ID, ScopeID, MemorySemanticsID) = 319,
    ImageSparseRead(TypeID, ResultID, ID, ID, Option<ImageOperands>) = 320,
    SizeOf(TypeID, ResultID, ID) = 321,
    TypePipeStorage(ResultID) = 322,
    ConstantPipeStorage(TypeID, ResultID, u32, u32, u32) = 323,
    CreatePipeFromPipeStorage(TypeID, ResultID, ID) = 324,
    GetKernelLocalSizeForSubgroupCount(TypeID, ResultID, ID, ID, ID, ID, ID) = 325,
    GetKernelMaxNumSubgroups(TypeID, ResultID, ID, ID, ID, ID) = 326,
    TypeNamedBarrier(ResultID) = 327,
    NamedBarrierInitialize(TypeID, ResultID, ID) = 328,
    MemoryNamedBarrier(ID, ScopeID, MemorySemanticsID) = 329,
    ModuleProcessed(String) = 330,
    ExecutionModeId(ID, ExecutionMode) = 331,
    DecorateId(ID, Decoration) = 332,
    GroupNonUniformElect(TypeID, ResultID, ScopeID) = 333,
    GroupNonUniformAll(TypeID, ResultID, ScopeID, ID) = 334,
    GroupNonUniformAny(TypeID, ResultID, ScopeID, ID) = 335,
    GroupNonUniformAllEqual(TypeID, ResultID, ScopeID, ID) = 336,
    GroupNonUniformBroadcast(TypeID, ResultID, ScopeID, ID, ID) = 337,
    GroupNonUniformBroadcastFirst(TypeID, ResultID, ScopeID, ID) = 338,
    GroupNonUniformBallot(TypeID, ResultID, ScopeID, ID) = 339,
    GroupNonUniformInverseBallot(TypeID, ResultID, ScopeID, ID) = 340,
    GroupNonUniformBallotBitExtract(TypeID, ResultID, ScopeID, ID, ID) = 341,
    GroupNonUniformBallotBitCount(TypeID, ResultID, ScopeID, GroupOperation, ID) = 342,
    GroupNonUniformBallotFindLSB(TypeID, ResultID, ScopeID, ID) = 343,
    GroupNonUniformBallotFindMSB(TypeID, ResultID, ScopeID, ID) = 344,
    GroupNonUniformShuffle(TypeID, ResultID, ScopeID, ID, ID) = 345,
    GroupNonUniformShuffleXor(TypeID, ResultID, ScopeID, ID, ID) = 346,
    GroupNonUniformShuffleUp(TypeID, ResultID, ScopeID, ID, ID) = 347,
    GroupNonUniformShuffleDown(TypeID, ResultID, ScopeID, ID, ID) = 348,
    GroupNonUniformIAdd(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 349,
    GroupNonUniformFAdd(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 350,
    GroupNonUniformIMul(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 351,
    GroupNonUniformFMul(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 352,
    GroupNonUniformSMin(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 353,
    GroupNonUniformUMin(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 354,
    GroupNonUniformFMin(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 355,
    GroupNonUniformSMax(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 356,
    GroupNonUniformUMax(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 357,
    GroupNonUniformFMax(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 358,
    GroupNonUniformBitwiseAnd(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 359,
    GroupNonUniformBitwiseOr(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 360,
    GroupNonUniformBitwiseXor(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 361,
    GroupNonUniformLogicalAnd(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 362,
    GroupNonUniformLogicalOr(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 363,
    GroupNonUniformLogicalXor(TypeID, ResultID, ScopeID, GroupOperation, ID, Option<ID>) = 364,
    GroupNonUniformQuadBroadcast(TypeID, ResultID, ScopeID, ID, ID) = 365,
    GroupNonUniformQuadSwap(TypeID, ResultID, ScopeID, ID, ID) = 366,
    CopyLogical(TypeID, ResultID, ID) = 400,
    PtrEqual(TypeID, ResultID, ID, ID) = 401,
    PtrNotEqual(TypeID, ResultID, ID, ID) = 402,
    PtrDiff(TypeID, ResultID, ID, ID) = 403,
    TerminateInvocation() = 4416,
    SubgroupBallotKHR(TypeID, ResultID, ID) = 4421,
    SubgroupFirstInvocationKHR(TypeID, ResultID, ID) = 4422,
    SubgroupAllKHR(TypeID, ResultID, ID) = 4428,
    SubgroupAnyKHR(TypeID, ResultID, ID) = 4429,
    SubgroupAllEqualKHR(TypeID, ResultID, ID) = 4430,
    SubgroupReadInvocationKHR(TypeID, ResultID, ID, ID) = 4432,
    TraceRayKHR(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 4445,
    ExecuteCallableKHR(ID, ID) = 4446,
    ConvertUToAccelerationStructureKHR(TypeID, ResultID, ID) = 4447,
    IgnoreIntersectionKHR() = 4448,
    TerminateRayKHR() = 4449,
    SDot(TypeID, ResultID, ID, ID, Option<PackedVectorFormat>) = 4450,
    UDot(TypeID, ResultID, ID, ID, Option<PackedVectorFormat>) = 4451,
    SUDot(TypeID, ResultID, ID, ID, Option<PackedVectorFormat>) = 4452,
    SDotAccSat(TypeID, ResultID, ID, ID, ID, Option<PackedVectorFormat>) = 4453,
    UDotAccSat(TypeID, ResultID, ID, ID, ID, Option<PackedVectorFormat>) = 4454,
    SUDotAccSat(TypeID, ResultID, ID, ID, ID, Option<PackedVectorFormat>) = 4455,
    TypeRayQueryKHR(ResultID) = 4472,
    RayQueryInitializeKHR(ID, ID, ID, ID, ID, ID, ID, ID) = 4473,
    RayQueryTerminateKHR(ID) = 4474,
    RayQueryGenerateIntersectionKHR(ID, ID) = 4475,
    RayQueryConfirmIntersectionKHR(ID) = 4476,
    RayQueryProceedKHR(TypeID, ResultID, ID) = 4477,
    RayQueryGetIntersectionTypeKHR(TypeID, ResultID, ID, ID) = 4479,
    GroupIAddNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5000,
    GroupFAddNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5001,
    GroupFMinNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5002,
    GroupUMinNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5003,
    GroupSMinNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5004,
    GroupFMaxNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5005,
    GroupUMaxNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5006,
    GroupSMaxNonUniformAMD(TypeID, ResultID, ScopeID, GroupOperation, ID) = 5007,
    FragmentMaskFetchAMD(TypeID, ResultID, ID, ID) = 5011,
    FragmentFetchAMD(TypeID, ResultID, ID, ID, ID) = 5012,
    ReadClockKHR(TypeID, ResultID, ScopeID) = 5056,
    ImageSampleFootprintNV(TypeID, ResultID, ID, ID, ID, ID, Option<ImageOperands>) = 5283,
    GroupNonUniformPartitionNV(TypeID, ResultID, ID) = 5296,
    WritePackedPrimitiveIndices4x8NV(ID, ID) = 5299,
    ReportIntersectionNV(TypeID, ResultID, ID, ID) = 5334,
    IgnoreIntersectionNV() = 5335,
    TerminateRayNV() = 5336,
    TraceNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5337,
    TraceMotionNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5338,
    TraceRayMotionNV(ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID, ID) = 5339,
    TypeAccelerationStructureNV(ResultID) = 5341,
    ExecuteCallableNV(ID, ID) = 5344,
    TypeCooperativeMatrixNV(ResultID, ID, ScopeID, ID, ID) = 5358,
    CooperativeMatrixLoadNV(TypeID, ResultID, ID, ID, ID, Option<MemoryAccess>) = 5359,
    CooperativeMatrixStoreNV(ID, ID, ID, ID, Option<MemoryAccess>) = 5360,
    CooperativeMatrixMulAddNV(TypeID, ResultID, ID, ID, ID) = 5361,
    CooperativeMatrixLengthNV(TypeID, ResultID, ID) = 5362,
    BeginInvocationInterlockEXT() = 5364,
    EndInvocationInterlockEXT() = 5365,
    DemoteToHelperInvocation() = 5380,
    IsHelperInvocationEXT(TypeID, ResultID) = 5381,
    ConvertUToImageNV(TypeID, ResultID, ID) = 5391,
    ConvertUToSamplerNV(TypeID, ResultID, ID) = 5392,
    ConvertImageToUNV(TypeID, ResultID, ID) = 5393,
    ConvertSamplerToUNV(TypeID, ResultID, ID) = 5394,
    ConvertUToSampledImageNV(TypeID, ResultID, ID) = 5395,
    ConvertSampledImageToUNV(TypeID, ResultID, ID) = 5396,
    SamplerImageAddressingModeNV(u32) = 5397,
    SubgroupShuffleINTEL(TypeID, ResultID, ID, ID) = 5571,
    SubgroupShuffleDownINTEL(TypeID, ResultID, ID, ID, ID) = 5572,
    SubgroupShuffleUpINTEL(TypeID, ResultID, ID, ID, ID) = 5573,
    SubgroupShuffleXorINTEL(TypeID, ResultID, ID, ID) = 5574,
    SubgroupBlockReadINTEL(TypeID, ResultID, ID) = 5575,
    SubgroupBlockWriteINTEL(ID, ID) = 5576,
    SubgroupImageBlockReadINTEL(TypeID, ResultID, ID, ID) = 5577,
    SubgroupImageBlockWriteINTEL(ID, ID, ID) = 5578,
    SubgroupImageMediaBlockReadINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5580,
    SubgroupImageMediaBlockWriteINTEL(ID, ID, ID, ID, ID) = 5581,
    UCountLeadingZerosINTEL(TypeID, ResultID, ID) = 5585,
    UCountTrailingZerosINTEL(TypeID, ResultID, ID) = 5586,
    AbsISubINTEL(TypeID, ResultID, ID, ID) = 5587,
    AbsUSubINTEL(TypeID, ResultID, ID, ID) = 5588,
    IAddSatINTEL(TypeID, ResultID, ID, ID) = 5589,
    UAddSatINTEL(TypeID, ResultID, ID, ID) = 5590,
    IAverageINTEL(TypeID, ResultID, ID, ID) = 5591,
    UAverageINTEL(TypeID, ResultID, ID, ID) = 5592,
    IAverageRoundedINTEL(TypeID, ResultID, ID, ID) = 5593,
    UAverageRoundedINTEL(TypeID, ResultID, ID, ID) = 5594,
    ISubSatINTEL(TypeID, ResultID, ID, ID) = 5595,
    USubSatINTEL(TypeID, ResultID, ID, ID) = 5596,
    IMul32x16INTEL(TypeID, ResultID, ID, ID) = 5597,
    UMul32x16INTEL(TypeID, ResultID, ID, ID) = 5598,
    ConstantFunctionPointerINTEL(TypeID, ResultID, ID) = 5600,
    FunctionPointerCallINTEL(TypeID, ResultID, Vec<ID>) = 5601,
    AsmTargetINTEL(TypeID, ResultID, String) = 5609,
    AsmINTEL(TypeID, ResultID, ID, ID, String, String) = 5610,
    AsmCallINTEL(TypeID, ResultID, ID, Vec<ID>) = 5611,
    AtomicFMinEXT(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 5614,
    AtomicFMaxEXT(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 5615,
    AssumeTrueKHR(ID) = 5630,
    ExpectKHR(TypeID, ResultID, ID, ID) = 5631,
    DecorateString(ID, Decoration) = 5632,
    MemberDecorateString(ID, u32, Decoration) = 5633,
    VmeImageINTEL(TypeID, ResultID, ID, ID) = 5699,
    TypeVmeImageINTEL(ResultID, ID) = 5700,
    TypeAvcImePayloadINTEL(ResultID) = 5701,
    TypeAvcRefPayloadINTEL(ResultID) = 5702,
    TypeAvcSicPayloadINTEL(ResultID) = 5703,
    TypeAvcMcePayloadINTEL(ResultID) = 5704,
    TypeAvcMceResultINTEL(ResultID) = 5705,
    TypeAvcImeResultINTEL(ResultID) = 5706,
    TypeAvcImeResultSingleReferenceStreamoutINTEL(ResultID) = 5707,
    TypeAvcImeResultDualReferenceStreamoutINTEL(ResultID) = 5708,
    TypeAvcImeSingleReferenceStreaminINTEL(ResultID) = 5709,
    TypeAvcImeDualReferenceStreaminINTEL(ResultID) = 5710,
    TypeAvcRefResultINTEL(ResultID) = 5711,
    TypeAvcSicResultINTEL(ResultID) = 5712,
    SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(TypeID, ResultID, ID, ID) = 5713,
    SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(TypeID, ResultID, ID, ID) = 5714,
    SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(TypeID, ResultID, ID, ID) = 5715,
    SubgroupAvcMceSetInterShapePenaltyINTEL(TypeID, ResultID, ID, ID) = 5716,
    SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(TypeID, ResultID, ID, ID) = 5717,
    SubgroupAvcMceSetInterDirectionPenaltyINTEL(TypeID, ResultID, ID, ID) = 5718,
    SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(TypeID, ResultID, ID, ID) = 5719,
    SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(TypeID, ResultID, ID, ID) = 5720,
    SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(TypeID, ResultID) = 5721,
    SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(TypeID, ResultID) = 5722,
    SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(TypeID, ResultID) = 5723,
    SubgroupAvcMceSetMotionVectorCostFunctionINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5724,
    SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(TypeID, ResultID, ID, ID) = 5725,
    SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(TypeID, ResultID) = 5726,
    SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(TypeID, ResultID) = 5727,
    SubgroupAvcMceSetAcOnlyHaarINTEL(TypeID, ResultID, ID) = 5728,
    SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(TypeID, ResultID, ID, ID) = 5729,
    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(TypeID, ResultID, ID, ID) = 5730,
    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(TypeID, ResultID, ID, ID, ID) =
        5731,
    SubgroupAvcMceConvertToImePayloadINTEL(TypeID, ResultID, ID) = 5732,
    SubgroupAvcMceConvertToImeResultINTEL(TypeID, ResultID, ID) = 5733,
    SubgroupAvcMceConvertToRefPayloadINTEL(TypeID, ResultID, ID) = 5734,
    SubgroupAvcMceConvertToRefResultINTEL(TypeID, ResultID, ID) = 5735,
    SubgroupAvcMceConvertToSicPayloadINTEL(TypeID, ResultID, ID) = 5736,
    SubgroupAvcMceConvertToSicResultINTEL(TypeID, ResultID, ID) = 5737,
    SubgroupAvcMceGetMotionVectorsINTEL(TypeID, ResultID, ID) = 5738,
    SubgroupAvcMceGetInterDistortionsINTEL(TypeID, ResultID, ID) = 5739,
    SubgroupAvcMceGetBestInterDistortionsINTEL(TypeID, ResultID, ID) = 5740,
    SubgroupAvcMceGetInterMajorShapeINTEL(TypeID, ResultID, ID) = 5741,
    SubgroupAvcMceGetInterMinorShapeINTEL(TypeID, ResultID, ID) = 5742,
    SubgroupAvcMceGetInterDirectionsINTEL(TypeID, ResultID, ID) = 5743,
    SubgroupAvcMceGetInterMotionVectorCountINTEL(TypeID, ResultID, ID) = 5744,
    SubgroupAvcMceGetInterReferenceIdsINTEL(TypeID, ResultID, ID) = 5745,
    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(TypeID, ResultID, ID, ID, ID) =
        5746,
    SubgroupAvcImeInitializeINTEL(TypeID, ResultID, ID, ID, ID) = 5747,
    SubgroupAvcImeSetSingleReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5748,
    SubgroupAvcImeSetDualReferenceINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5749,
    SubgroupAvcImeRefWindowSizeINTEL(TypeID, ResultID, ID, ID) = 5750,
    SubgroupAvcImeAdjustRefOffsetINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5751,
    SubgroupAvcImeConvertToMcePayloadINTEL(TypeID, ResultID, ID) = 5752,
    SubgroupAvcImeSetMaxMotionVectorCountINTEL(TypeID, ResultID, ID, ID) = 5753,
    SubgroupAvcImeSetUnidirectionalMixDisableINTEL(TypeID, ResultID, ID) = 5754,
    SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(TypeID, ResultID, ID, ID) = 5755,
    SubgroupAvcImeSetWeightedSadINTEL(TypeID, ResultID, ID, ID) = 5756,
    SubgroupAvcImeEvaluateWithSingleReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5757,
    SubgroupAvcImeEvaluateWithDualReferenceINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5758,
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5759,
    SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(TypeID, ResultID, ID, ID, ID, ID, ID) =
        5760,
    SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(TypeID, ResultID, ID, ID, ID) = 5761,
    SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5762,
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(TypeID, ResultID, ID, ID, ID, ID) =
        5763,
    SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(TypeID, ResultID, ID, ID, ID, ID, ID) =
        5764,
    SubgroupAvcImeConvertToMceResultINTEL(TypeID, ResultID, ID) = 5765,
    SubgroupAvcImeGetSingleReferenceStreaminINTEL(TypeID, ResultID, ID) = 5766,
    SubgroupAvcImeGetDualReferenceStreaminINTEL(TypeID, ResultID, ID) = 5767,
    SubgroupAvcImeStripSingleReferenceStreamoutINTEL(TypeID, ResultID, ID) = 5768,
    SubgroupAvcImeStripDualReferenceStreamoutINTEL(TypeID, ResultID, ID) = 5769,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(TypeID, ResultID, ID, ID) =
        5770,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(TypeID, ResultID, ID, ID) =
        5771,
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(TypeID, ResultID, ID, ID) =
        5772,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
        TypeID,
        ResultID,
        ID,
        ID,
        ID,
    ) = 5773,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(TypeID, ResultID, ID, ID, ID) =
        5774,
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
        TypeID,
        ResultID,
        ID,
        ID,
        ID,
    ) = 5775,
    SubgroupAvcImeGetBorderReachedINTEL(TypeID, ResultID, ID, ID) = 5776,
    SubgroupAvcImeGetTruncatedSearchIndicationINTEL(TypeID, ResultID, ID) = 5777,
    SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(TypeID, ResultID, ID) = 5778,
    SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(TypeID, ResultID, ID) = 5779,
    SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(TypeID, ResultID, ID) = 5780,
    SubgroupAvcFmeInitializeINTEL(TypeID, ResultID, ID, ID, ID, ID, ID, ID, ID) = 5781,
    SubgroupAvcBmeInitializeINTEL(TypeID, ResultID, ID, ID, ID, ID, ID, ID, ID, ID) = 5782,
    SubgroupAvcRefConvertToMcePayloadINTEL(TypeID, ResultID, ID) = 5783,
    SubgroupAvcRefSetBidirectionalMixDisableINTEL(TypeID, ResultID, ID) = 5784,
    SubgroupAvcRefSetBilinearFilterEnableINTEL(TypeID, ResultID, ID) = 5785,
    SubgroupAvcRefEvaluateWithSingleReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5786,
    SubgroupAvcRefEvaluateWithDualReferenceINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5787,
    SubgroupAvcRefEvaluateWithMultiReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5788,
    SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(TypeID, ResultID, ID, ID, ID, ID) =
        5789,
    SubgroupAvcRefConvertToMceResultINTEL(TypeID, ResultID, ID) = 5790,
    SubgroupAvcSicInitializeINTEL(TypeID, ResultID, ID) = 5791,
    SubgroupAvcSicConfigureSkcINTEL(TypeID, ResultID, ID, ID, ID, ID, ID, ID) = 5792,
    SubgroupAvcSicConfigureIpeLumaINTEL(TypeID, ResultID, ID, ID, ID, ID, ID, ID, ID, ID) = 5793,
    SubgroupAvcSicConfigureIpeLumaChromaINTEL(
        TypeID,
        ResultID,
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
    SubgroupAvcSicGetMotionVectorMaskINTEL(TypeID, ResultID, ID, ID) = 5795,
    SubgroupAvcSicConvertToMcePayloadINTEL(TypeID, ResultID, ID) = 5796,
    SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(TypeID, ResultID, ID, ID) = 5797,
    SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5798,
    SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(TypeID, ResultID, ID, ID) = 5799,
    SubgroupAvcSicSetBilinearFilterEnableINTEL(TypeID, ResultID, ID) = 5800,
    SubgroupAvcSicSetSkcForwardTransformEnableINTEL(TypeID, ResultID, ID, ID) = 5801,
    SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(TypeID, ResultID, ID, ID) = 5802,
    SubgroupAvcSicEvaluateIpeINTEL(TypeID, ResultID, ID, ID) = 5803,
    SubgroupAvcSicEvaluateWithSingleReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5804,
    SubgroupAvcSicEvaluateWithDualReferenceINTEL(TypeID, ResultID, ID, ID, ID, ID) = 5805,
    SubgroupAvcSicEvaluateWithMultiReferenceINTEL(TypeID, ResultID, ID, ID, ID) = 5806,
    SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(TypeID, ResultID, ID, ID, ID, ID) =
        5807,
    SubgroupAvcSicConvertToMceResultINTEL(TypeID, ResultID, ID) = 5808,
    SubgroupAvcSicGetIpeLumaShapeINTEL(TypeID, ResultID, ID) = 5809,
    SubgroupAvcSicGetBestIpeLumaDistortionINTEL(TypeID, ResultID, ID) = 5810,
    SubgroupAvcSicGetBestIpeChromaDistortionINTEL(TypeID, ResultID, ID) = 5811,
    SubgroupAvcSicGetPackedIpeLumaModesINTEL(TypeID, ResultID, ID) = 5812,
    SubgroupAvcSicGetIpeChromaModeINTEL(TypeID, ResultID, ID) = 5813,
    SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(TypeID, ResultID, ID) = 5814,
    SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(TypeID, ResultID, ID) = 5815,
    SubgroupAvcSicGetInterRawSadsINTEL(TypeID, ResultID, ID) = 5816,
    VariableLengthArrayINTEL(TypeID, ResultID, ID) = 5818,
    SaveMemoryINTEL(TypeID, ResultID) = 5819,
    RestoreMemoryINTEL(ID) = 5820,
    ArbitraryFloatSinCosPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32, u32) = 5840,
    ArbitraryFloatCastINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5841,
    ArbitraryFloatCastFromIntINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5842,
    ArbitraryFloatCastToIntINTEL(TypeID, ResultID, ID, u32, u32, u32, u32) = 5843,
    ArbitraryFloatAddINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5846,
    ArbitraryFloatSubINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5847,
    ArbitraryFloatMulINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5848,
    ArbitraryFloatDivINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5849,
    ArbitraryFloatGTINTEL(TypeID, ResultID, ID, u32, ID, u32) = 5850,
    ArbitraryFloatGEINTEL(TypeID, ResultID, ID, u32, ID, u32) = 5851,
    ArbitraryFloatLTINTEL(TypeID, ResultID, ID, u32, ID, u32) = 5852,
    ArbitraryFloatLEINTEL(TypeID, ResultID, ID, u32, ID, u32) = 5853,
    ArbitraryFloatEQINTEL(TypeID, ResultID, ID, u32, ID, u32) = 5854,
    ArbitraryFloatRecipINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5855,
    ArbitraryFloatRSqrtINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5856,
    ArbitraryFloatCbrtINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5857,
    ArbitraryFloatHypotINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5858,
    ArbitraryFloatSqrtINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5859,
    ArbitraryFloatLogINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5860,
    ArbitraryFloatLog2INTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5861,
    ArbitraryFloatLog10INTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5862,
    ArbitraryFloatLog1pINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5863,
    ArbitraryFloatExpINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5864,
    ArbitraryFloatExp2INTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5865,
    ArbitraryFloatExp10INTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5866,
    ArbitraryFloatExpm1INTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5867,
    ArbitraryFloatSinINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5868,
    ArbitraryFloatCosINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5869,
    ArbitraryFloatSinCosINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5870,
    ArbitraryFloatSinPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5871,
    ArbitraryFloatCosPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5872,
    ArbitraryFloatASinINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5873,
    ArbitraryFloatASinPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5874,
    ArbitraryFloatACosINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5875,
    ArbitraryFloatACosPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5876,
    ArbitraryFloatATanINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5877,
    ArbitraryFloatATanPiINTEL(TypeID, ResultID, ID, u32, u32, u32, u32, u32) = 5878,
    ArbitraryFloatATan2INTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5879,
    ArbitraryFloatPowINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5880,
    ArbitraryFloatPowRINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32, u32) = 5881,
    ArbitraryFloatPowNINTEL(TypeID, ResultID, ID, u32, ID, u32, u32, u32, u32) = 5882,
    LoopControlINTEL(Vec<u32>) = 5887,
    AliasDomainDeclINTEL(ResultID, Option<ID>) = 5911,
    AliasScopeDeclINTEL(ResultID, ID, Option<ID>) = 5912,
    AliasScopeListDeclINTEL(ResultID, Vec<ID>) = 5913,
    FixedSqrtINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5923,
    FixedRecipINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5924,
    FixedRsqrtINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5925,
    FixedSinINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5926,
    FixedCosINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5927,
    FixedSinCosINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5928,
    FixedSinPiINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5929,
    FixedCosPiINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5930,
    FixedSinCosPiINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5931,
    FixedLogINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5932,
    FixedExpINTEL(TypeID, ResultID, ID, ID, u32, u32, u32, u32, u32) = 5933,
    PtrCastToCrossWorkgroupINTEL(TypeID, ResultID, ID) = 5934,
    CrossWorkgroupCastToPtrINTEL(TypeID, ResultID, ID) = 5938,
    ReadPipeBlockingINTEL(TypeID, ResultID, ID, ID) = 5946,
    WritePipeBlockingINTEL(TypeID, ResultID, ID, ID) = 5947,
    FPGARegINTEL(TypeID, ResultID, ID, ID) = 5949,
    RayQueryGetRayTMinKHR(TypeID, ResultID, ID) = 6016,
    RayQueryGetRayFlagsKHR(TypeID, ResultID, ID) = 6017,
    RayQueryGetIntersectionTKHR(TypeID, ResultID, ID, ID) = 6018,
    RayQueryGetIntersectionInstanceCustomIndexKHR(TypeID, ResultID, ID, ID) = 6019,
    RayQueryGetIntersectionInstanceIdKHR(TypeID, ResultID, ID, ID) = 6020,
    RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(TypeID, ResultID, ID, ID) =
        6021,
    RayQueryGetIntersectionGeometryIndexKHR(TypeID, ResultID, ID, ID) = 6022,
    RayQueryGetIntersectionPrimitiveIndexKHR(TypeID, ResultID, ID, ID) = 6023,
    RayQueryGetIntersectionBarycentricsKHR(TypeID, ResultID, ID, ID) = 6024,
    RayQueryGetIntersectionFrontFaceKHR(TypeID, ResultID, ID, ID) = 6025,
    RayQueryGetIntersectionCandidateAABBOpaqueKHR(TypeID, ResultID, ID) = 6026,
    RayQueryGetIntersectionObjectRayDirectionKHR(TypeID, ResultID, ID, ID) = 6027,
    RayQueryGetIntersectionObjectRayOriginKHR(TypeID, ResultID, ID, ID) = 6028,
    RayQueryGetWorldRayDirectionKHR(TypeID, ResultID, ID) = 6029,
    RayQueryGetWorldRayOriginKHR(TypeID, ResultID, ID) = 6030,
    RayQueryGetIntersectionObjectToWorldKHR(TypeID, ResultID, ID, ID) = 6031,
    RayQueryGetIntersectionWorldToObjectKHR(TypeID, ResultID, ID, ID) = 6032,
    AtomicFAddEXT(TypeID, ResultID, ID, ScopeID, MemorySemanticsID, ID) = 6035,
    TypeBufferSurfaceINTEL(ResultID, AccessQualifier) = 6086,
    TypeStructContinuedINTEL(Vec<ID>) = 6090,
    ConstantCompositeContinuedINTEL(Vec<ID>) = 6091,
    SpecConstantCompositeContinuedINTEL(Vec<ID>) = 6092,
    ControlBarrierArriveINTEL(ScopeID, ScopeID, MemorySemanticsID) = 6142,
    ControlBarrierWaitINTEL(ScopeID, ScopeID, MemorySemanticsID) = 6143,
    GroupIMulKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6401,
    GroupFMulKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6402,
    GroupBitwiseAndKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6403,
    GroupBitwiseOrKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6404,
    GroupBitwiseXorKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6405,
    GroupLogicalAndKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6406,
    GroupLogicalOrKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6407,
    GroupLogicalXorKHR(TypeID, ResultID, ScopeID, GroupOperation, ID) = 6408,
}
impl Opcode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
    fn read_as_spec_op<Env: Environ>(
        ty: TypeID,
        id: ResultID,
        chunk: &[u32],
        env: &mut Env,
        idx: &mut usize,
    ) -> Self {
        use Opcode::*;
        let i = *idx as usize;
        let mask = u16::MAX as u32;
        let opc = chunk[i] & mask;
        *idx += 1;
        match opc {
            65 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                AccessChain(ty, id, x0, x1)
            }
            66 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                InBoundsAccessChain(ty, id, x0, x1)
            }
            67 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                PtrAccessChain(ty, id, x0, x1, x2)
            }
            70 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                InBoundsPtrAccessChain(ty, id, x0, x1, x2)
            }
            79 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                VectorShuffle(ty, id, x0, x1, x2)
            }
            81 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                CompositeExtract(ty, id, x0, x1)
            }
            82 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                CompositeInsert(ty, id, x0, x1, x2)
            }
            109 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertFToU(ty, id, x0)
            }
            110 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertFToS(ty, id, x0)
            }
            111 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertSToF(ty, id, x0)
            }
            112 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertUToF(ty, id, x0)
            }
            113 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UConvert(ty, id, x0)
            }
            114 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SConvert(ty, id, x0)
            }
            115 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FConvert(ty, id, x0)
            }
            116 => {
                let x0 = Writer::read_word(chunk, env, idx);
                QuantizeToF16(ty, id, x0)
            }
            117 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertPtrToU(ty, id, x0)
            }
            120 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConvertUToPtr(ty, id, x0)
            }
            121 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PtrCastToGeneric(ty, id, x0)
            }
            122 => {
                let x0 = Writer::read_word(chunk, env, idx);
                GenericCastToPtr(ty, id, x0)
            }
            124 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Bitcast(ty, id, x0)
            }
            126 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SNegate(ty, id, x0)
            }
            127 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FNegate(ty, id, x0)
            }
            128 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                IAdd(ty, id, x0, x1)
            }
            129 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FAdd(ty, id, x0, x1)
            }
            130 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ISub(ty, id, x0, x1)
            }
            131 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FSub(ty, id, x0, x1)
            }
            132 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                IMul(ty, id, x0, x1)
            }
            133 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FMul(ty, id, x0, x1)
            }
            134 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UDiv(ty, id, x0, x1)
            }
            135 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SDiv(ty, id, x0, x1)
            }
            136 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FDiv(ty, id, x0, x1)
            }
            137 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UMod(ty, id, x0, x1)
            }
            138 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SRem(ty, id, x0, x1)
            }
            139 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SMod(ty, id, x0, x1)
            }
            140 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FRem(ty, id, x0, x1)
            }
            141 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FMod(ty, id, x0, x1)
            }
            164 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                LogicalEqual(ty, id, x0, x1)
            }
            165 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                LogicalNotEqual(ty, id, x0, x1)
            }
            166 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                LogicalOr(ty, id, x0, x1)
            }
            167 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                LogicalAnd(ty, id, x0, x1)
            }
            168 => {
                let x0 = Writer::read_word(chunk, env, idx);
                LogicalNot(ty, id, x0)
            }
            169 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                Select(ty, id, x0, x1, x2)
            }
            170 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                IEqual(ty, id, x0, x1)
            }
            171 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                INotEqual(ty, id, x0, x1)
            }
            172 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UGreaterThan(ty, id, x0, x1)
            }
            173 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SGreaterThan(ty, id, x0, x1)
            }
            174 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                UGreaterThanEqual(ty, id, x0, x1)
            }
            175 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SGreaterThanEqual(ty, id, x0, x1)
            }
            176 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ULessThan(ty, id, x0, x1)
            }
            177 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SLessThan(ty, id, x0, x1)
            }
            178 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ULessThanEqual(ty, id, x0, x1)
            }
            179 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                SLessThanEqual(ty, id, x0, x1)
            }
            194 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ShiftRightLogical(ty, id, x0, x1)
            }
            195 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ShiftRightArithmetic(ty, id, x0, x1)
            }
            196 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                ShiftLeftLogical(ty, id, x0, x1)
            }
            197 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                BitwiseOr(ty, id, x0, x1)
            }
            198 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                BitwiseXor(ty, id, x0, x1)
            }
            199 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                BitwiseAnd(ty, id, x0, x1)
            }
            200 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Not(ty, id, x0)
            }
            wtf => panic!("{}", wtf),
        }
    }
    pub fn read_word<Env: Environ + std::fmt::Debug>(
        chunk: &[u32],
        env: &mut Env,
        idx: &mut usize,
    ) -> Self {
        use Opcode::*;
        let mask = u16::MAX as usize;
        let len = (chunk[*idx] >> 16) as usize & mask;
        let opc = chunk[*idx] as usize & mask;
        let chunk = &chunk[..*idx + len];
        let mark = *idx;
        *idx += 1;
        let re = match opc {
            0 => env.insert_op(Nop()),
            1 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Undef(x0, x1))
            }
            2 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(SourceContinued(x0))
            }
            3 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(Source(x0, x1, x2, x3))
            }
            4 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(SourceExtension(x0))
            }
            5 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(Name(x0, x1))
            }
            6 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemberName(x0, x1, x2))
            }
            7 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(String(x0, x1))
            }
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(Line(x0, x1, x2))
            }
            10 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(Extension(x0))
            }
            11 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(ExtInstImport(x0, x1))
            }
            12 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ExtInst(x0, x1, x2, x3, x4))
            }
            14 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemoryModel(x0, x1))
            }
            15 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(EntryPoint(x0, x1, x2, x3))
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(ExecutionMode(x0, x1))
            }
            17 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(Capability(x0))
            }
            19 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeVoid(x0))
            }
            20 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeBool(x0))
            }
            21 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeInt(x0, x1, x2))
            }
            22 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeFloat(x0, x1))
            }
            23 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeVector(x0, x1, x2))
            }
            24 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeMatrix(x0, x1, x2))
            }
            25 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            26 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeSampler(x0))
            }
            27 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeSampledImage(x0, x1))
            }
            28 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeArray(x0, x1, x2))
            }
            29 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeRuntimeArray(x0, x1))
            }
            30 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeStruct(x0, x1))
            }
            31 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeOpaque(x0, x1))
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypePointer(x0, x1, x2))
            }
            33 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeFunction(x0, x1, x2))
            }
            34 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeEvent(x0))
            }
            35 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeDeviceEvent(x0))
            }
            36 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeReserveId(x0))
            }
            37 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeQueue(x0))
            }
            38 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypePipe(x0, x1))
            }
            39 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(TypeForwardPointer(x0, x1))
            }
            41 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantTrue(x0, x1))
            }
            42 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantFalse(x0, x1))
            }
            43 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Constant(x0, x1, x2))
            }
            44 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantComposite(x0, x1, x2))
            }
            45 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantSampler(x0, x1, x2, x3, x4))
            }
            46 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantNull(x0, x1))
            }
            48 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SpecConstantTrue(x0, x1))
            }
            49 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SpecConstantFalse(x0, x1))
            }
            50 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SpecConstant(x0, x1, x2))
            }
            51 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SpecConstantComposite(x0, x1, x2))
            }
            54 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Function(x0, x1, x2, x3))
            }
            55 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FunctionParameter(x0, x1))
            }
            56 => env.insert_op(FunctionEnd()),
            57 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FunctionCall(x0, x1, x2, x3))
            }
            59 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Variable(x0, x1, x2, x3))
            }
            60 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageTexelPointer(x0, x1, x2, x3, x4))
            }
            61 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Load(x0, x1, x2, x3))
            }
            62 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(Store(x0, x1, x2))
            }
            63 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(CopyMemory(x0, x1, x2, x3))
            }
            64 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_op(CopyMemorySized(x0, x1, x2, x3, x4))
            }
            65 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AccessChain(x0, x1, x2, x3))
            }
            66 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(InBoundsAccessChain(x0, x1, x2, x3))
            }
            67 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrAccessChain(x0, x1, x2, x3, x4))
            }
            68 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArrayLength(x0, x1, x2, x3))
            }
            69 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GenericPtrMemSemantics(x0, x1, x2))
            }
            70 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(InBoundsPtrAccessChain(x0, x1, x2, x3, x4))
            }
            71 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(Decorate(x0, x1))
            }
            72 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemberDecorate(x0, x1, x2))
            }
            73 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(DecorationGroup(x0))
            }
            74 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(GroupDecorate(x0, x1))
            }
            75 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(GroupMemberDecorate(x0, x1))
            }
            77 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VectorExtractDynamic(x0, x1, x2, x3))
            }
            78 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VectorInsertDynamic(x0, x1, x2, x3, x4))
            }
            79 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VectorShuffle(x0, x1, x2, x3, x4))
            }
            80 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CompositeConstruct(x0, x1, x2))
            }
            81 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CompositeExtract(x0, x1, x2, x3))
            }
            82 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CompositeInsert(x0, x1, x2, x3, x4))
            }
            83 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CopyObject(x0, x1, x2))
            }
            84 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Transpose(x0, x1, x2))
            }
            86 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SampledImage(x0, x1, x2, x3))
            }
            87 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleImplicitLod(x0, x1, x2, x3, x4))
            }
            88 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleExplicitLod(x0, x1, x2, x3, x4))
            }
            89 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5))
            }
            90 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5))
            }
            91 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleProjImplicitLod(x0, x1, x2, x3, x4))
            }
            92 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleProjExplicitLod(x0, x1, x2, x3, x4))
            }
            93 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5))
            }
            94 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5))
            }
            95 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageFetch(x0, x1, x2, x3, x4))
            }
            96 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageGather(x0, x1, x2, x3, x4, x5))
            }
            97 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageDrefGather(x0, x1, x2, x3, x4, x5))
            }
            98 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageRead(x0, x1, x2, x3, x4))
            }
            99 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(ImageWrite(x0, x1, x2, x3))
            }
            100 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Image(x0, x1, x2))
            }
            101 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQueryFormat(x0, x1, x2))
            }
            102 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQueryOrder(x0, x1, x2))
            }
            103 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQuerySizeLod(x0, x1, x2, x3))
            }
            104 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQuerySize(x0, x1, x2))
            }
            105 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQueryLod(x0, x1, x2, x3))
            }
            106 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQueryLevels(x0, x1, x2))
            }
            107 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageQuerySamples(x0, x1, x2))
            }
            109 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertFToU(x0, x1, x2))
            }
            110 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertFToS(x0, x1, x2))
            }
            111 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertSToF(x0, x1, x2))
            }
            112 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToF(x0, x1, x2))
            }
            113 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UConvert(x0, x1, x2))
            }
            114 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SConvert(x0, x1, x2))
            }
            115 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FConvert(x0, x1, x2))
            }
            116 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(QuantizeToF16(x0, x1, x2))
            }
            117 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertPtrToU(x0, x1, x2))
            }
            118 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SatConvertSToU(x0, x1, x2))
            }
            119 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SatConvertUToS(x0, x1, x2))
            }
            120 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToPtr(x0, x1, x2))
            }
            121 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrCastToGeneric(x0, x1, x2))
            }
            122 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GenericCastToPtr(x0, x1, x2))
            }
            123 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GenericCastToPtrExplicit(x0, x1, x2, x3))
            }
            124 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Bitcast(x0, x1, x2))
            }
            126 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SNegate(x0, x1, x2))
            }
            127 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FNegate(x0, x1, x2))
            }
            128 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IAdd(x0, x1, x2, x3))
            }
            129 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FAdd(x0, x1, x2, x3))
            }
            130 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ISub(x0, x1, x2, x3))
            }
            131 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FSub(x0, x1, x2, x3))
            }
            132 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IMul(x0, x1, x2, x3))
            }
            133 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FMul(x0, x1, x2, x3))
            }
            134 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UDiv(x0, x1, x2, x3))
            }
            135 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SDiv(x0, x1, x2, x3))
            }
            136 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FDiv(x0, x1, x2, x3))
            }
            137 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UMod(x0, x1, x2, x3))
            }
            138 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SRem(x0, x1, x2, x3))
            }
            139 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SMod(x0, x1, x2, x3))
            }
            140 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FRem(x0, x1, x2, x3))
            }
            141 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FMod(x0, x1, x2, x3))
            }
            142 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VectorTimesScalar(x0, x1, x2, x3))
            }
            143 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(MatrixTimesScalar(x0, x1, x2, x3))
            }
            144 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VectorTimesMatrix(x0, x1, x2, x3))
            }
            145 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(MatrixTimesVector(x0, x1, x2, x3))
            }
            146 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(MatrixTimesMatrix(x0, x1, x2, x3))
            }
            147 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(OuterProduct(x0, x1, x2, x3))
            }
            148 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Dot(x0, x1, x2, x3))
            }
            149 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IAddCarry(x0, x1, x2, x3))
            }
            150 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ISubBorrow(x0, x1, x2, x3))
            }
            151 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UMulExtended(x0, x1, x2, x3))
            }
            152 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SMulExtended(x0, x1, x2, x3))
            }
            154 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Any(x0, x1, x2))
            }
            155 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(All(x0, x1, x2))
            }
            156 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsNan(x0, x1, x2))
            }
            157 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsInf(x0, x1, x2))
            }
            158 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsFinite(x0, x1, x2))
            }
            159 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsNormal(x0, x1, x2))
            }
            160 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SignBitSet(x0, x1, x2))
            }
            161 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LessOrGreater(x0, x1, x2, x3))
            }
            162 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Ordered(x0, x1, x2, x3))
            }
            163 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Unordered(x0, x1, x2, x3))
            }
            164 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LogicalEqual(x0, x1, x2, x3))
            }
            165 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LogicalNotEqual(x0, x1, x2, x3))
            }
            166 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LogicalOr(x0, x1, x2, x3))
            }
            167 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LogicalAnd(x0, x1, x2, x3))
            }
            168 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(LogicalNot(x0, x1, x2))
            }
            169 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Select(x0, x1, x2, x3, x4))
            }
            170 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IEqual(x0, x1, x2, x3))
            }
            171 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(INotEqual(x0, x1, x2, x3))
            }
            172 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UGreaterThan(x0, x1, x2, x3))
            }
            173 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SGreaterThan(x0, x1, x2, x3))
            }
            174 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UGreaterThanEqual(x0, x1, x2, x3))
            }
            175 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SGreaterThanEqual(x0, x1, x2, x3))
            }
            176 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ULessThan(x0, x1, x2, x3))
            }
            177 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SLessThan(x0, x1, x2, x3))
            }
            178 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ULessThanEqual(x0, x1, x2, x3))
            }
            179 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SLessThanEqual(x0, x1, x2, x3))
            }
            180 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdEqual(x0, x1, x2, x3))
            }
            181 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordEqual(x0, x1, x2, x3))
            }
            182 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdNotEqual(x0, x1, x2, x3))
            }
            183 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordNotEqual(x0, x1, x2, x3))
            }
            184 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdLessThan(x0, x1, x2, x3))
            }
            185 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordLessThan(x0, x1, x2, x3))
            }
            186 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdGreaterThan(x0, x1, x2, x3))
            }
            187 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordGreaterThan(x0, x1, x2, x3))
            }
            188 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdLessThanEqual(x0, x1, x2, x3))
            }
            189 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordLessThanEqual(x0, x1, x2, x3))
            }
            190 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FOrdGreaterThanEqual(x0, x1, x2, x3))
            }
            191 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FUnordGreaterThanEqual(x0, x1, x2, x3))
            }
            194 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ShiftRightLogical(x0, x1, x2, x3))
            }
            195 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ShiftRightArithmetic(x0, x1, x2, x3))
            }
            196 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ShiftLeftLogical(x0, x1, x2, x3))
            }
            197 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitwiseOr(x0, x1, x2, x3))
            }
            198 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitwiseXor(x0, x1, x2, x3))
            }
            199 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitwiseAnd(x0, x1, x2, x3))
            }
            200 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Not(x0, x1, x2))
            }
            201 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitFieldInsert(x0, x1, x2, x3, x4, x5))
            }
            202 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitFieldSExtract(x0, x1, x2, x3, x4))
            }
            203 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitFieldUExtract(x0, x1, x2, x3, x4))
            }
            204 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitReverse(x0, x1, x2))
            }
            205 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BitCount(x0, x1, x2))
            }
            207 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdx(x0, x1, x2))
            }
            208 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdy(x0, x1, x2))
            }
            209 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Fwidth(x0, x1, x2))
            }
            210 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdxFine(x0, x1, x2))
            }
            211 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdyFine(x0, x1, x2))
            }
            212 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FwidthFine(x0, x1, x2))
            }
            213 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdxCoarse(x0, x1, x2))
            }
            214 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(DPdyCoarse(x0, x1, x2))
            }
            215 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FwidthCoarse(x0, x1, x2))
            }
            218 => env.insert_op(EmitVertex()),
            219 => env.insert_op(EndPrimitive()),
            220 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(EmitStreamVertex(x0))
            }
            221 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(EndStreamPrimitive(x0))
            }
            224 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(ControlBarrier(x0, x1, x2))
            }
            225 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemoryBarrier(x0, x1))
            }
            227 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicLoad(x0, x1, x2, x3, x4))
            }
            228 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(AtomicStore(x0, x1, x2, x3))
            }
            229 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicExchange(x0, x1, x2, x3, x4, x5))
            }
            230 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            231 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            232 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicIIncrement(x0, x1, x2, x3, x4))
            }
            233 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicIDecrement(x0, x1, x2, x3, x4))
            }
            234 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicIAdd(x0, x1, x2, x3, x4, x5))
            }
            235 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicISub(x0, x1, x2, x3, x4, x5))
            }
            236 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicSMin(x0, x1, x2, x3, x4, x5))
            }
            237 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicUMin(x0, x1, x2, x3, x4, x5))
            }
            238 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicSMax(x0, x1, x2, x3, x4, x5))
            }
            239 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicUMax(x0, x1, x2, x3, x4, x5))
            }
            240 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicAnd(x0, x1, x2, x3, x4, x5))
            }
            241 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicOr(x0, x1, x2, x3, x4, x5))
            }
            242 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicXor(x0, x1, x2, x3, x4, x5))
            }
            245 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(Phi(x0, x1, x2))
            }
            246 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(LoopMerge(x0, x1, x2))
            }
            247 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(SelectionMerge(x0, x1))
            }
            248 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(Label(x0))
            }
            249 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(Branch(x0))
            }
            250 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(BranchConditional(x0, x1, x2, x3))
            }
            251 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(Switch(x0, x1, x2))
            }
            252 => env.insert_op(Kill()),
            253 => env.insert_op(Return()),
            254 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(ReturnValue(x0))
            }
            255 => env.insert_op(Unreachable()),
            256 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(LifetimeStart(x0, x1))
            }
            257 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(LifetimeStop(x0, x1))
            }
            259 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            260 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(GroupWaitEvents(x0, x1, x2))
            }
            261 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupAll(x0, x1, x2, x3))
            }
            262 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupAny(x0, x1, x2, x3))
            }
            263 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupBroadcast(x0, x1, x2, x3, x4))
            }
            264 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupIAdd(x0, x1, x2, x3, x4))
            }
            265 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFAdd(x0, x1, x2, x3, x4))
            }
            266 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFMin(x0, x1, x2, x3, x4))
            }
            267 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupUMin(x0, x1, x2, x3, x4))
            }
            268 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupSMin(x0, x1, x2, x3, x4))
            }
            269 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFMax(x0, x1, x2, x3, x4))
            }
            270 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupUMax(x0, x1, x2, x3, x4))
            }
            271 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupSMax(x0, x1, x2, x3, x4))
            }
            274 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReadPipe(x0, x1, x2, x3, x4, x5))
            }
            275 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(WritePipe(x0, x1, x2, x3, x4, x5))
            }
            276 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            277 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            278 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReserveReadPipePackets(x0, x1, x2, x3, x4, x5))
            }
            279 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReserveWritePipePackets(x0, x1, x2, x3, x4, x5))
            }
            280 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(CommitReadPipe(x0, x1, x2, x3))
            }
            281 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_op(CommitWritePipe(x0, x1, x2, x3))
            }
            282 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsValidReserveId(x0, x1, x2))
            }
            283 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetNumPipePackets(x0, x1, x2, x3, x4))
            }
            284 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetMaxPipePackets(x0, x1, x2, x3, x4))
            }
            285 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6))
            }
            286 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6))
            }
            287 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_op(GroupCommitReadPipe(x0, x1, x2, x3, x4))
            }
            288 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_op(GroupCommitWritePipe(x0, x1, x2, x3, x4))
            }
            291 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(EnqueueMarker(x0, x1, x2, x3, x4, x5))
            }
            292 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                let x11 = Writer::read_word(chunk, env, idx);
                let x12 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(EnqueueKernel(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
                ))
            }
            293 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6))
            }
            294 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6))
            }
            295 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5))
            }
            296 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelPreferredWorkGroupSizeMultiple(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            297 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(RetainEvent(x0))
            }
            298 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(ReleaseEvent(x0))
            }
            299 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CreateUserEvent(x0, x1))
            }
            300 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsValidEvent(x0, x1, x2))
            }
            301 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(SetUserEventStatus(x0, x1))
            }
            302 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(CaptureEventProfilingInfo(x0, x1, x2))
            }
            303 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetDefaultQueue(x0, x1))
            }
            304 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(BuildNDRange(x0, x1, x2, x3, x4))
            }
            305 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleImplicitLod(x0, x1, x2, x3, x4))
            }
            306 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleExplicitLod(x0, x1, x2, x3, x4))
            }
            307 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5))
            }
            308 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5))
            }
            309 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4))
            }
            310 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4))
            }
            311 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5))
            }
            312 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5))
            }
            313 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseFetch(x0, x1, x2, x3, x4))
            }
            314 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseGather(x0, x1, x2, x3, x4, x5))
            }
            315 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseDrefGather(x0, x1, x2, x3, x4, x5))
            }
            316 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseTexelsResident(x0, x1, x2))
            }
            317 => env.insert_op(NoLine()),
            318 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicFlagTestAndSet(x0, x1, x2, x3, x4))
            }
            319 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(AtomicFlagClear(x0, x1, x2))
            }
            320 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSparseRead(x0, x1, x2, x3, x4))
            }
            321 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SizeOf(x0, x1, x2))
            }
            322 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypePipeStorage(x0))
            }
            323 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantPipeStorage(x0, x1, x2, x3, x4))
            }
            324 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CreatePipeFromPipeStorage(x0, x1, x2))
            }
            325 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelLocalSizeForSubgroupCount(
                    x0, x1, x2, x3, x4, x5, x6,
                ))
            }
            326 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5))
            }
            327 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeNamedBarrier(x0))
            }
            328 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(NamedBarrierInitialize(x0, x1, x2))
            }
            329 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemoryNamedBarrier(x0, x1, x2))
            }
            330 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(ModuleProcessed(x0))
            }
            331 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(ExecutionModeId(x0, x1))
            }
            332 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(DecorateId(x0, x1))
            }
            333 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformElect(x0, x1, x2))
            }
            334 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformAll(x0, x1, x2, x3))
            }
            335 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformAny(x0, x1, x2, x3))
            }
            336 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformAllEqual(x0, x1, x2, x3))
            }
            337 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBroadcast(x0, x1, x2, x3, x4))
            }
            338 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBroadcastFirst(x0, x1, x2, x3))
            }
            339 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBallot(x0, x1, x2, x3))
            }
            340 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformInverseBallot(x0, x1, x2, x3))
            }
            341 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4))
            }
            342 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBallotBitCount(x0, x1, x2, x3, x4))
            }
            343 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBallotFindLSB(x0, x1, x2, x3))
            }
            344 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBallotFindMSB(x0, x1, x2, x3))
            }
            345 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformShuffle(x0, x1, x2, x3, x4))
            }
            346 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformShuffleXor(x0, x1, x2, x3, x4))
            }
            347 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformShuffleUp(x0, x1, x2, x3, x4))
            }
            348 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformShuffleDown(x0, x1, x2, x3, x4))
            }
            349 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformIAdd(x0, x1, x2, x3, x4, x5))
            }
            350 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformFAdd(x0, x1, x2, x3, x4, x5))
            }
            351 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformIMul(x0, x1, x2, x3, x4, x5))
            }
            352 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformFMul(x0, x1, x2, x3, x4, x5))
            }
            353 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformSMin(x0, x1, x2, x3, x4, x5))
            }
            354 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformUMin(x0, x1, x2, x3, x4, x5))
            }
            355 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformFMin(x0, x1, x2, x3, x4, x5))
            }
            356 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformSMax(x0, x1, x2, x3, x4, x5))
            }
            357 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformUMax(x0, x1, x2, x3, x4, x5))
            }
            358 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformFMax(x0, x1, x2, x3, x4, x5))
            }
            359 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5))
            }
            360 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5))
            }
            361 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5))
            }
            362 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5))
            }
            363 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5))
            }
            364 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5))
            }
            365 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4))
            }
            366 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformQuadSwap(x0, x1, x2, x3, x4))
            }
            400 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CopyLogical(x0, x1, x2))
            }
            401 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrEqual(x0, x1, x2, x3))
            }
            402 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrNotEqual(x0, x1, x2, x3))
            }
            403 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrDiff(x0, x1, x2, x3))
            }
            4416 => env.insert_op(TerminateInvocation()),
            4421 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupBallotKHR(x0, x1, x2))
            }
            4422 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupFirstInvocationKHR(x0, x1, x2))
            }
            4428 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAllKHR(x0, x1, x2))
            }
            4429 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAnyKHR(x0, x1, x2))
            }
            4430 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAllEqualKHR(x0, x1, x2))
            }
            4432 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupReadInvocationKHR(x0, x1, x2, x3))
            }
            4445 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                env.insert_op(TraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10))
            }
            4446 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(ExecuteCallableKHR(x0, x1))
            }
            4447 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToAccelerationStructureKHR(x0, x1, x2))
            }
            4448 => env.insert_op(IgnoreIntersectionKHR()),
            4449 => env.insert_op(TerminateRayKHR()),
            4450 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SDot(x0, x1, x2, x3, x4))
            }
            4451 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UDot(x0, x1, x2, x3, x4))
            }
            4452 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SUDot(x0, x1, x2, x3, x4))
            }
            4453 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SDotAccSat(x0, x1, x2, x3, x4, x5))
            }
            4454 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UDotAccSat(x0, x1, x2, x3, x4, x5))
            }
            4455 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SUDotAccSat(x0, x1, x2, x3, x4, x5))
            }
            4472 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeRayQueryKHR(x0))
            }
            4473 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_op(RayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            4474 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(RayQueryTerminateKHR(x0))
            }
            4475 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(RayQueryGenerateIntersectionKHR(x0, x1))
            }
            4476 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(RayQueryConfirmIntersectionKHR(x0))
            }
            4477 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryProceedKHR(x0, x1, x2))
            }
            4479 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionTypeKHR(x0, x1, x2, x3))
            }
            5000 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupIAddNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5001 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFAddNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5002 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFMinNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5003 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupUMinNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5004 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupSMinNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5005 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFMaxNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5006 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupUMaxNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5007 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupSMaxNonUniformAMD(x0, x1, x2, x3, x4))
            }
            5011 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FragmentMaskFetchAMD(x0, x1, x2, x3))
            }
            5012 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FragmentFetchAMD(x0, x1, x2, x3, x4))
            }
            5056 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReadClockKHR(x0, x1, x2))
            }
            5283 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6))
            }
            5296 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupNonUniformPartitionNV(x0, x1, x2))
            }
            5299 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(WritePackedPrimitiveIndices4x8NV(x0, x1))
            }
            5334 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReportIntersectionNV(x0, x1, x2, x3))
            }
            5335 => env.insert_op(IgnoreIntersectionNV()),
            5336 => env.insert_op(TerminateRayNV()),
            5337 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                env.insert_op(TraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10))
            }
            5338 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                let x11 = Writer::read_word(chunk, env, idx);
                env.insert_op(TraceMotionNV(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11,
                ))
            }
            5339 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                let x11 = Writer::read_word(chunk, env, idx);
                env.insert_op(TraceRayMotionNV(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11,
                ))
            }
            5341 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAccelerationStructureNV(x0))
            }
            5344 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(ExecuteCallableNV(x0, x1))
            }
            5358 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeCooperativeMatrixNV(x0, x1, x2, x3, x4))
            }
            5359 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5))
            }
            5360 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_op(CooperativeMatrixStoreNV(x0, x1, x2, x3, x4))
            }
            5361 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CooperativeMatrixMulAddNV(x0, x1, x2, x3, x4))
            }
            5362 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CooperativeMatrixLengthNV(x0, x1, x2))
            }
            5364 => env.insert_op(BeginInvocationInterlockEXT()),
            5365 => env.insert_op(EndInvocationInterlockEXT()),
            5380 => env.insert_op(DemoteToHelperInvocation()),
            5381 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IsHelperInvocationEXT(x0, x1))
            }
            5391 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToImageNV(x0, x1, x2))
            }
            5392 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToSamplerNV(x0, x1, x2))
            }
            5393 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertImageToUNV(x0, x1, x2))
            }
            5394 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertSamplerToUNV(x0, x1, x2))
            }
            5395 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertUToSampledImageNV(x0, x1, x2))
            }
            5396 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConvertSampledImageToUNV(x0, x1, x2))
            }
            5397 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(SamplerImageAddressingModeNV(x0))
            }
            5571 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupShuffleINTEL(x0, x1, x2, x3))
            }
            5572 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupShuffleDownINTEL(x0, x1, x2, x3, x4))
            }
            5573 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupShuffleUpINTEL(x0, x1, x2, x3, x4))
            }
            5574 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupShuffleXorINTEL(x0, x1, x2, x3))
            }
            5575 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupBlockReadINTEL(x0, x1, x2))
            }
            5576 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(SubgroupBlockWriteINTEL(x0, x1))
            }
            5577 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupImageBlockReadINTEL(x0, x1, x2, x3))
            }
            5578 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(SubgroupImageBlockWriteINTEL(x0, x1, x2))
            }
            5580 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5))
            }
            5581 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_op(SubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4))
            }
            5585 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UCountLeadingZerosINTEL(x0, x1, x2))
            }
            5586 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UCountTrailingZerosINTEL(x0, x1, x2))
            }
            5587 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AbsISubINTEL(x0, x1, x2, x3))
            }
            5588 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AbsUSubINTEL(x0, x1, x2, x3))
            }
            5589 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IAddSatINTEL(x0, x1, x2, x3))
            }
            5590 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UAddSatINTEL(x0, x1, x2, x3))
            }
            5591 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IAverageINTEL(x0, x1, x2, x3))
            }
            5592 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UAverageINTEL(x0, x1, x2, x3))
            }
            5593 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IAverageRoundedINTEL(x0, x1, x2, x3))
            }
            5594 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UAverageRoundedINTEL(x0, x1, x2, x3))
            }
            5595 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ISubSatINTEL(x0, x1, x2, x3))
            }
            5596 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(USubSatINTEL(x0, x1, x2, x3))
            }
            5597 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(IMul32x16INTEL(x0, x1, x2, x3))
            }
            5598 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(UMul32x16INTEL(x0, x1, x2, x3))
            }
            5600 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ConstantFunctionPointerINTEL(x0, x1, x2))
            }
            5601 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FunctionPointerCallINTEL(x0, x1, x2))
            }
            5609 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AsmTargetINTEL(x0, x1, x2))
            }
            5610 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AsmINTEL(x0, x1, x2, x3, x4, x5))
            }
            5611 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AsmCallINTEL(x0, x1, x2, x3))
            }
            5614 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicFMinEXT(x0, x1, x2, x3, x4, x5))
            }
            5615 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicFMaxEXT(x0, x1, x2, x3, x4, x5))
            }
            5630 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(AssumeTrueKHR(x0))
            }
            5631 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ExpectKHR(x0, x1, x2, x3))
            }
            5632 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_op(DecorateString(x0, x1))
            }
            5633 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(MemberDecorateString(x0, x1, x2))
            }
            5699 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VmeImageINTEL(x0, x1, x2, x3))
            }
            5700 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeVmeImageINTEL(x0, x1))
            }
            5701 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImePayloadINTEL(x0))
            }
            5702 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcRefPayloadINTEL(x0))
            }
            5703 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcSicPayloadINTEL(x0))
            }
            5704 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcMcePayloadINTEL(x0))
            }
            5705 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcMceResultINTEL(x0))
            }
            5706 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImeResultINTEL(x0))
            }
            5707 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImeResultSingleReferenceStreamoutINTEL(x0))
            }
            5708 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImeResultDualReferenceStreamoutINTEL(x0))
            }
            5709 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImeSingleReferenceStreaminINTEL(x0))
            }
            5710 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcImeDualReferenceStreaminINTEL(x0))
            }
            5711 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcRefResultINTEL(x0))
            }
            5712 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeAvcSicResultINTEL(x0))
            }
            5713 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5714 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5715 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5716 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3))
            }
            5717 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5718 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3))
            }
            5719 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5720 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5721 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1))
            }
            5722 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1))
            }
            5723 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1))
            }
            5724 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5725 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5726 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1))
            }
            5727 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(
                    x0, x1,
                ))
            }
            5728 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2))
            }
            5729 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5730 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3),
                )
            }
            5731 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
                        x0, x1, x2, x3, x4,
                    ),
                )
            }
            5732 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2))
            }
            5733 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2))
            }
            5734 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2))
            }
            5735 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2))
            }
            5736 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2))
            }
            5737 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2))
            }
            5738 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2))
            }
            5739 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2))
            }
            5740 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2))
            }
            5741 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2))
            }
            5742 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2))
            }
            5743 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2))
            }
            5744 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2))
            }
            5745 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2))
            }
            5746 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
                        x0, x1, x2, x3, x4,
                    ),
                )
            }
            5747 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4))
            }
            5748 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4))
            }
            5749 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5))
            }
            5750 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3))
            }
            5751 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5))
            }
            5752 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2))
            }
            5753 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3))
            }
            5754 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2))
            }
            5755 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5756 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3))
            }
            5757 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithSingleReferenceINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5758 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithDualReferenceINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5759 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5760 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
                    x0, x1, x2, x3, x4, x5, x6,
                ))
            }
            5761 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5762 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5763 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5764 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                    x0, x1, x2, x3, x4, x5, x6,
                ))
            }
            5765 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2))
            }
            5766 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2))
            }
            5767 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2))
            }
            5768 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2))
            }
            5769 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2))
            }
            5770 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                        x0, x1, x2, x3,
                    ),
                )
            }
            5771 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                        x0, x1, x2, x3,
                    ),
                )
            }
            5772 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                        x0, x1, x2, x3,
                    ),
                )
            }
            5773 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                        x0, x1, x2, x3, x4,
                    ),
                )
            }
            5774 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                        x0, x1, x2, x3, x4,
                    ),
                )
            }
            5775 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                        x0, x1, x2, x3, x4,
                    ),
                )
            }
            5776 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3))
            }
            5777 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2))
            }
            5778 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(
                    x0, x1, x2,
                ))
            }
            5779 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(
                    x0, x1, x2,
                ))
            }
            5780 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(
                    x0, x1, x2,
                ))
            }
            5781 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcFmeInitializeINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8,
                ))
            }
            5782 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcBmeInitializeINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5783 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2))
            }
            5784 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2))
            }
            5785 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2))
            }
            5786 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefEvaluateWithSingleReferenceINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5787 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefEvaluateWithDualReferenceINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5788 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefEvaluateWithMultiReferenceINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5789 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5790 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2))
            }
            5791 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicInitializeINTEL(x0, x1, x2))
            }
            5792 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicConfigureSkcINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7,
                ))
            }
            5793 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicConfigureIpeLumaINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5794 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                let x10 = Writer::read_word(chunk, env, idx);
                let x11 = Writer::read_word(chunk, env, idx);
                let x12 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicConfigureIpeLumaChromaINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12,
                ))
            }
            5795 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3))
            }
            5796 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2))
            }
            5797 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3))
            }
            5798 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5799 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5800 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2))
            }
            5801 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetSkcForwardTransformEnableINTEL(
                    x0, x1, x2, x3,
                ))
            }
            5802 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3))
            }
            5803 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3))
            }
            5804 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicEvaluateWithSingleReferenceINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5805 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicEvaluateWithDualReferenceINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5806 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicEvaluateWithMultiReferenceINTEL(
                    x0, x1, x2, x3, x4,
                ))
            }
            5807 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
                    x0, x1, x2, x3, x4, x5,
                ))
            }
            5808 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2))
            }
            5809 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2))
            }
            5810 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2))
            }
            5811 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2))
            }
            5812 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2))
            }
            5813 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2))
            }
            5814 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(
                    x0, x1, x2,
                ))
            }
            5815 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2))
            }
            5816 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2))
            }
            5818 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(VariableLengthArrayINTEL(x0, x1, x2))
            }
            5819 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(SaveMemoryINTEL(x0, x1))
            }
            5820 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(RestoreMemoryINTEL(x0))
            }
            5840 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSinCosPiINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8,
                ))
            }
            5841 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5842 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCastFromIntINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7,
                ))
            }
            5843 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6))
            }
            5846 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatAddINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5847 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSubINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5848 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatMulINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5849 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatDivINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5850 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5))
            }
            5851 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5))
            }
            5852 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5))
            }
            5853 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5))
            }
            5854 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5))
            }
            5855 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5856 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5857 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5858 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatHypotINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5859 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5860 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5861 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5862 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5863 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5864 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5865 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5866 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5867 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5868 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5869 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5870 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5871 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5872 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5873 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5874 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5875 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5876 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5877 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5878 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7))
            }
            5879 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatATan2INTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5880 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatPowINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5881 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                let x9 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatPowRINTEL(
                    x0, x1, x2, x3, x4, x5, x6, x7, x8, x9,
                ))
            }
            5882 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5887 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(LoopControlINTEL(x0))
            }
            5911 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(AliasDomainDeclINTEL(x0, x1))
            }
            5912 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(AliasScopeDeclINTEL(x0, x1, x2))
            }
            5913 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(AliasScopeListDeclINTEL(x0, x1))
            }
            5923 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5924 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5925 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5926 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5927 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5928 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5929 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5930 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5931 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5932 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5933 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                let x6 = Writer::read_word(chunk, env, idx);
                let x7 = Writer::read_word(chunk, env, idx);
                let x8 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8))
            }
            5934 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(PtrCastToCrossWorkgroupINTEL(x0, x1, x2))
            }
            5938 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(CrossWorkgroupCastToPtrINTEL(x0, x1, x2))
            }
            5946 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(ReadPipeBlockingINTEL(x0, x1, x2, x3))
            }
            5947 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(WritePipeBlockingINTEL(x0, x1, x2, x3))
            }
            5949 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(FPGARegINTEL(x0, x1, x2, x3))
            }
            6016 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetRayTMinKHR(x0, x1, x2))
            }
            6017 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetRayFlagsKHR(x0, x1, x2))
            }
            6018 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionTKHR(x0, x1, x2, x3))
            }
            6019 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionInstanceCustomIndexKHR(
                    x0, x1, x2, x3,
                ))
            }
            6020 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3))
            }
            6021 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(
                    RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(
                        x0, x1, x2, x3,
                    ),
                )
            }
            6022 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3))
            }
            6023 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3))
            }
            6024 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3))
            }
            6025 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3))
            }
            6026 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2))
            }
            6027 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3))
            }
            6028 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3))
            }
            6029 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetWorldRayDirectionKHR(x0, x1, x2))
            }
            6030 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetWorldRayOriginKHR(x0, x1, x2))
            }
            6031 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3))
            }
            6032 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(RayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3))
            }
            6035 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                let x5 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(AtomicFAddEXT(x0, x1, x2, x3, x4, x5))
            }
            6086 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                env.insert_id(x0);
                env.insert_op(TypeBufferSurfaceINTEL(x0, x1))
            }
            6090 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(TypeStructContinuedINTEL(x0))
            }
            6091 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(ConstantCompositeContinuedINTEL(x0))
            }
            6092 => {
                let x0 = Writer::read_word(chunk, env, idx);
                env.insert_op(SpecConstantCompositeContinuedINTEL(x0))
            }
            6142 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(ControlBarrierArriveINTEL(x0, x1, x2))
            }
            6143 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                env.insert_op(ControlBarrierWaitINTEL(x0, x1, x2))
            }
            6401 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupIMulKHR(x0, x1, x2, x3, x4))
            }
            6402 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupFMulKHR(x0, x1, x2, x3, x4))
            }
            6403 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupBitwiseAndKHR(x0, x1, x2, x3, x4))
            }
            6404 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupBitwiseOrKHR(x0, x1, x2, x3, x4))
            }
            6405 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupBitwiseXorKHR(x0, x1, x2, x3, x4))
            }
            6406 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupLogicalAndKHR(x0, x1, x2, x3, x4))
            }
            6407 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupLogicalOrKHR(x0, x1, x2, x3, x4))
            }
            6408 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                let x3 = Writer::read_word(chunk, env, idx);
                let x4 = Writer::read_word(chunk, env, idx);
                env.insert_id(x1);
                env.insert_op(GroupLogicalXorKHR(x0, x1, x2, x3, x4))
            }
            52 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Opcode::read_as_spec_op(x0, x1, chunk, env, idx);
                env.insert_op(SpecConstantOp(x0, x1, Box::new(x2)))
            }
            wtf => panic!("{}", wtf),
        };
        assert_eq!(*idx - mark, len, "{:?}", env);
        re
    }
    pub fn write_as_spec_op<Env: Environ>(&self, env: &Env, sink: &mut Vec<u32>) {
        use Opcode::*;
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            AccessChain(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            InBoundsAccessChain(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            PtrAccessChain(_, _, x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            InBoundsPtrAccessChain(_, _, x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            VectorShuffle(_, _, x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            CompositeExtract(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            CompositeInsert(_, _, x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertFToU(_, _, x0) => {
                x0.write_word(env, sink);
            }
            ConvertFToS(_, _, x0) => {
                x0.write_word(env, sink);
            }
            ConvertSToF(_, _, x0) => {
                x0.write_word(env, sink);
            }
            ConvertUToF(_, _, x0) => {
                x0.write_word(env, sink);
            }
            UConvert(_, _, x0) => {
                x0.write_word(env, sink);
            }
            SConvert(_, _, x0) => {
                x0.write_word(env, sink);
            }
            FConvert(_, _, x0) => {
                x0.write_word(env, sink);
            }
            QuantizeToF16(_, _, x0) => {
                x0.write_word(env, sink);
            }
            ConvertPtrToU(_, _, x0) => {
                x0.write_word(env, sink);
            }
            ConvertUToPtr(_, _, x0) => {
                x0.write_word(env, sink);
            }
            PtrCastToGeneric(_, _, x0) => {
                x0.write_word(env, sink);
            }
            GenericCastToPtr(_, _, x0) => {
                x0.write_word(env, sink);
            }
            Bitcast(_, _, x0) => {
                x0.write_word(env, sink);
            }
            SNegate(_, _, x0) => {
                x0.write_word(env, sink);
            }
            FNegate(_, _, x0) => {
                x0.write_word(env, sink);
            }
            IAdd(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FAdd(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ISub(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FSub(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            IMul(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FMul(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UDiv(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SDiv(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FDiv(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UMod(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SRem(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SMod(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FRem(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FMod(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LogicalEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LogicalNotEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LogicalOr(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LogicalAnd(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LogicalNot(_, _, x0) => {
                x0.write_word(env, sink);
            }
            Select(_, _, x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            INotEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UGreaterThan(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SGreaterThan(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            UGreaterThanEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SGreaterThanEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ULessThan(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SLessThan(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ULessThanEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SLessThanEqual(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ShiftRightLogical(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ShiftRightArithmetic(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ShiftLeftLogical(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            BitwiseOr(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            BitwiseXor(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            BitwiseAnd(_, _, x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Not(_, _, x0) => {
                x0.write_word(env, sink);
            }
            _ => panic!(),
        }
    }
    pub fn write_word<Env: Environ>(&self, env: &Env, sink: &mut Vec<u32>) {
        use Opcode::*;
        let mark = sink.len();
        sink.push(self.opcode());
        match self {
            Nop() => {}
            Undef(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SourceContinued(x0) => {
                x0.write_word(env, sink);
            }
            Source(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SourceExtension(x0) => {
                x0.write_word(env, sink);
            }
            Name(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            MemberName(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            String(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Line(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Extension(x0) => {
                x0.write_word(env, sink);
            }
            ExtInstImport(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ExtInst(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            MemoryModel(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            EntryPoint(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ExecutionMode(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Capability(x0) => {
                x0.write_word(env, sink);
            }
            TypeVoid(x0) => {
                x0.write_word(env, sink);
            }
            TypeBool(x0) => {
                x0.write_word(env, sink);
            }
            TypeInt(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeFloat(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeVector(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeMatrix(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeImage(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            TypeSampler(x0) => {
                x0.write_word(env, sink);
            }
            TypeSampledImage(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeArray(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeRuntimeArray(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeStruct(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeOpaque(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypePointer(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeFunction(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypeEvent(x0) => {
                x0.write_word(env, sink);
            }
            TypeDeviceEvent(x0) => {
                x0.write_word(env, sink);
            }
            TypeReserveId(x0) => {
                x0.write_word(env, sink);
            }
            TypeQueue(x0) => {
                x0.write_word(env, sink);
            }
            TypePipe(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeForwardPointer(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ConstantTrue(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ConstantFalse(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Constant(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConstantComposite(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConstantSampler(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ConstantNull(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SpecConstantTrue(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SpecConstantFalse(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SpecConstant(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SpecConstantComposite(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Function(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FunctionParameter(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FunctionEnd() => {}
            FunctionCall(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Variable(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ImageTexelPointer(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            Load(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Store(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            CopyMemory(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            CopyMemorySized(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            AccessChain(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            InBoundsAccessChain(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            PtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ArrayLength(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GenericPtrMemSemantics(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            InBoundsPtrAccessChain(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            Decorate(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            MemberDecorate(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DecorationGroup(x0) => {
                x0.write_word(env, sink);
            }
            GroupDecorate(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            GroupMemberDecorate(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            VectorExtractDynamic(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            VectorInsertDynamic(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            VectorShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CompositeConstruct(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            CompositeExtract(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            CompositeInsert(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CopyObject(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Transpose(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SampledImage(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ImageSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageRead(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageWrite(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Image(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageQueryFormat(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageQueryOrder(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageQuerySizeLod(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ImageQuerySize(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageQueryLod(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ImageQueryLevels(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageQuerySamples(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertFToU(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertFToS(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertSToF(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertUToF(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            UConvert(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SConvert(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FConvert(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            QuantizeToF16(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertPtrToU(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SatConvertSToU(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SatConvertUToS(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertUToPtr(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            PtrCastToGeneric(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GenericCastToPtr(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GenericCastToPtrExplicit(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Bitcast(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SNegate(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FNegate(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IAdd(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FAdd(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ISub(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FSub(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IMul(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FMul(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UDiv(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SDiv(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FDiv(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UMod(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SRem(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SMod(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FRem(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FMod(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            VectorTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            MatrixTimesScalar(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            VectorTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            MatrixTimesVector(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            MatrixTimesMatrix(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            OuterProduct(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Dot(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IAddCarry(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ISubBorrow(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UMulExtended(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SMulExtended(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Any(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            All(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IsNan(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IsInf(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IsFinite(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IsNormal(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SignBitSet(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            LessOrGreater(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Ordered(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Unordered(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            LogicalEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            LogicalNotEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            LogicalOr(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            LogicalAnd(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            LogicalNot(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Select(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            IEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            INotEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ULessThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SLessThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ULessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdNotEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordNotEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdLessThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordLessThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordGreaterThan(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordLessThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FOrdGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FUnordGreaterThanEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ShiftRightLogical(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ShiftRightArithmetic(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ShiftLeftLogical(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            BitwiseOr(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            BitwiseXor(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            BitwiseAnd(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Not(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            BitFieldInsert(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            BitFieldSExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            BitFieldUExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            BitReverse(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            BitCount(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdx(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdy(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Fwidth(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdxFine(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdyFine(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FwidthFine(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdxCoarse(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DPdyCoarse(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FwidthCoarse(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            EmitVertex() => {}
            EndPrimitive() => {}
            EmitStreamVertex(x0) => {
                x0.write_word(env, sink);
            }
            EndStreamPrimitive(x0) => {
                x0.write_word(env, sink);
            }
            ControlBarrier(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            MemoryBarrier(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            AtomicLoad(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            AtomicStore(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            AtomicExchange(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicCompareExchange(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            AtomicCompareExchangeWeak(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            AtomicIIncrement(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            AtomicIDecrement(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            AtomicIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicISub(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            Phi(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            LoopMerge(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SelectionMerge(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            Label(x0) => {
                x0.write_word(env, sink);
            }
            Branch(x0) => {
                x0.write_word(env, sink);
            }
            BranchConditional(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            Switch(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            Kill() => {}
            Return() => {}
            ReturnValue(x0) => {
                x0.write_word(env, sink);
            }
            Unreachable() => {}
            LifetimeStart(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            LifetimeStop(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            GroupAsyncCopy(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            GroupWaitEvents(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GroupAll(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupAny(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupIAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFAdd(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFMin(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupUMin(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupSMin(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFMax(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupUMax(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupSMax(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ReadPipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            WritePipe(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ReservedReadPipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ReservedWritePipe(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ReserveReadPipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ReserveWritePipePackets(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            CommitReadPipe(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            CommitWritePipe(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IsValidReserveId(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GetNumPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GetMaxPipePackets(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupReserveReadPipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GroupReserveWritePipePackets(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GroupCommitReadPipe(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupCommitWritePipe(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            EnqueueMarker(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            EnqueueKernel(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
                x11.write_word(env, sink);
                x12.write_word(env, sink);
            }
            GetKernelNDrangeSubGroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GetKernelNDrangeMaxSubGroupSize(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GetKernelWorkGroupSize(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GetKernelPreferredWorkGroupSizeMultiple(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            RetainEvent(x0) => {
                x0.write_word(env, sink);
            }
            ReleaseEvent(x0) => {
                x0.write_word(env, sink);
            }
            CreateUserEvent(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            IsValidEvent(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SetUserEventStatus(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            CaptureEventProfilingInfo(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GetDefaultQueue(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            BuildNDRange(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseSampleImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseSampleExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseSampleDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseSampleDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseSampleProjImplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseSampleProjExplicitLod(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseSampleProjDrefImplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseSampleProjDrefExplicitLod(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseFetch(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ImageSparseGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseDrefGather(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ImageSparseTexelsResident(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            NoLine() => {}
            AtomicFlagTestAndSet(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            AtomicFlagClear(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageSparseRead(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SizeOf(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            TypePipeStorage(x0) => {
                x0.write_word(env, sink);
            }
            ConstantPipeStorage(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CreatePipeFromPipeStorage(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GetKernelLocalSizeForSubgroupCount(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GetKernelMaxNumSubgroups(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            TypeNamedBarrier(x0) => {
                x0.write_word(env, sink);
            }
            NamedBarrierInitialize(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            MemoryNamedBarrier(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ModuleProcessed(x0) => {
                x0.write_word(env, sink);
            }
            ExecutionModeId(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            DecorateId(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            GroupNonUniformElect(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GroupNonUniformAll(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformAny(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformAllEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformBroadcastFirst(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformBallot(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformInverseBallot(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformBallotBitExtract(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformBallotBitCount(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformBallotFindLSB(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformBallotFindMSB(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupNonUniformShuffle(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformShuffleXor(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformShuffleUp(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformShuffleDown(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformIAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformFAdd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformIMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformFMul(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformSMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformUMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformFMin(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformSMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformUMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformFMax(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformBitwiseAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformBitwiseOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformBitwiseXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformLogicalAnd(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformLogicalOr(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformLogicalXor(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            GroupNonUniformQuadBroadcast(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupNonUniformQuadSwap(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CopyLogical(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            PtrEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            PtrNotEqual(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            PtrDiff(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            TerminateInvocation() => {}
            SubgroupBallotKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupFirstInvocationKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAllKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAnyKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAllEqualKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupReadInvocationKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            TraceRayKHR(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
            }
            ExecuteCallableKHR(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ConvertUToAccelerationStructureKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            IgnoreIntersectionKHR() => {}
            TerminateRayKHR() => {}
            SDot(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            UDot(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SUDot(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            UDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SUDotAccSat(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            TypeRayQueryKHR(x0) => {
                x0.write_word(env, sink);
            }
            RayQueryInitializeKHR(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            RayQueryTerminateKHR(x0) => {
                x0.write_word(env, sink);
            }
            RayQueryGenerateIntersectionKHR(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            RayQueryConfirmIntersectionKHR(x0) => {
                x0.write_word(env, sink);
            }
            RayQueryProceedKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetIntersectionTypeKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            GroupIAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFAddNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupUMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupSMinNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupUMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupSMaxNonUniformAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            FragmentMaskFetchAMD(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FragmentFetchAMD(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            ReadClockKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ImageSampleFootprintNV(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            GroupNonUniformPartitionNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            WritePackedPrimitiveIndices4x8NV(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ReportIntersectionNV(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IgnoreIntersectionNV() => {}
            TerminateRayNV() => {}
            TraceNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
            }
            TraceMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
                x11.write_word(env, sink);
            }
            TraceRayMotionNV(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
                x11.write_word(env, sink);
            }
            TypeAccelerationStructureNV(x0) => {
                x0.write_word(env, sink);
            }
            ExecuteCallableNV(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeCooperativeMatrixNV(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CooperativeMatrixLoadNV(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            CooperativeMatrixStoreNV(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CooperativeMatrixMulAddNV(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            CooperativeMatrixLengthNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            BeginInvocationInterlockEXT() => {}
            EndInvocationInterlockEXT() => {}
            DemoteToHelperInvocation() => {}
            IsHelperInvocationEXT(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ConvertUToImageNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertUToSamplerNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertImageToUNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertSamplerToUNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertUToSampledImageNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ConvertSampledImageToUNV(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SamplerImageAddressingModeNV(x0) => {
                x0.write_word(env, sink);
            }
            SubgroupShuffleINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupShuffleDownINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupShuffleUpINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupShuffleXorINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupBlockReadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupBlockWriteINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupImageBlockReadINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupImageBlockWriteINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupImageMediaBlockReadINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupImageMediaBlockWriteINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            UCountLeadingZerosINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            UCountTrailingZerosINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            AbsISubINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            AbsUSubINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UAddSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UAverageINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UAverageRoundedINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ISubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            USubSatINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            IMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            UMul32x16INTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            ConstantFunctionPointerINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            FunctionPointerCallINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            AsmTargetINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            AsmINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AsmCallINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            AtomicFMinEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AtomicFMaxEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            AssumeTrueKHR(x0) => {
                x0.write_word(env, sink);
            }
            ExpectKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            DecorateString(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            MemberDecorateString(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            VmeImageINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            TypeVmeImageINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeAvcImePayloadINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcRefPayloadINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcSicPayloadINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcMcePayloadINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcMceResultINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcImeResultINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcImeResultSingleReferenceStreamoutINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcImeResultDualReferenceStreamoutINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcImeSingleReferenceStreaminINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcImeDualReferenceStreaminINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcRefResultINTEL(x0) => {
                x0.write_word(env, sink);
            }
            TypeAvcSicResultINTEL(x0) => {
                x0.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceSetInterShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceSetInterDirectionPenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupAvcMceSetMotionVectorCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            SubgroupAvcMceSetAcOnlyHaarINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcMceConvertToImePayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceConvertToImeResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceConvertToRefPayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceConvertToRefResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceConvertToSicPayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceConvertToSicResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetMotionVectorsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetBestInterDistortionsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterMajorShapeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterMinorShapeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterDirectionsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterMotionVectorCountINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterReferenceIdsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeInitializeINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeSetSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeSetDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeRefWindowSizeINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeAdjustRefOffsetINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeSetMaxMotionVectorCountINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeSetUnidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeSetWeightedSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            SubgroupAvcImeConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetSingleReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetDualReferenceStreaminINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeStripSingleReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeStripDualReferenceStreamoutINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
            ) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                x0,
                x1,
                x2,
                x3,
                x4,
            ) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcImeGetBorderReachedINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcImeGetTruncatedSearchIndicationINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcFmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            SubgroupAvcBmeInitializeINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            SubgroupAvcRefConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcRefSetBidirectionalMixDisableINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcRefSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcRefEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcRefEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcRefEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcRefConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicInitializeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicConfigureSkcINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            SubgroupAvcSicConfigureIpeLumaINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            SubgroupAvcSicConfigureIpeLumaChromaINTEL(
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
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
                x10.write_word(env, sink);
                x11.write_word(env, sink);
                x12.write_word(env, sink);
            }
            SubgroupAvcSicGetMotionVectorMaskINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicConvertToMcePayloadINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicSetBilinearFilterEnableINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicSetSkcForwardTransformEnableINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicEvaluateIpeINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            SubgroupAvcSicEvaluateWithSingleReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcSicEvaluateWithDualReferenceINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcSicEvaluateWithMultiReferenceINTEL(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            SubgroupAvcSicConvertToMceResultINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetIpeLumaShapeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetBestIpeLumaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetBestIpeChromaDistortionINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetPackedIpeLumaModesINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetIpeChromaModeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SubgroupAvcSicGetInterRawSadsINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            VariableLengthArrayINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            SaveMemoryINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            RestoreMemoryINTEL(x0) => {
                x0.write_word(env, sink);
            }
            ArbitraryFloatSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            ArbitraryFloatCastINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatCastFromIntINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatCastToIntINTEL(x0, x1, x2, x3, x4, x5, x6) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
            }
            ArbitraryFloatAddINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatSubINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatMulINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatDivINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatGTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ArbitraryFloatGEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ArbitraryFloatLTINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ArbitraryFloatLEINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ArbitraryFloatEQINTEL(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            ArbitraryFloatRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatRSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatCbrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatHypotINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatLog2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatLog10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatLog1pINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatExp2INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatExp10INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatExpm1INTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatASinINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatASinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatACosINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatACosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatATanINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatATanPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
            }
            ArbitraryFloatATan2INTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatPowINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatPowRINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
                x9.write_word(env, sink);
            }
            ArbitraryFloatPowNINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            LoopControlINTEL(x0) => {
                x0.write_word(env, sink);
            }
            AliasDomainDeclINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            AliasScopeDeclINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            AliasScopeListDeclINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FixedSqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedRecipINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedRsqrtINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedSinINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedSinCosINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedSinPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedSinCosPiINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedLogINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            FixedExpINTEL(x0, x1, x2, x3, x4, x5, x6, x7, x8) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
                x6.write_word(env, sink);
                x7.write_word(env, sink);
                x8.write_word(env, sink);
            }
            PtrCastToCrossWorkgroupINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            CrossWorkgroupCastToPtrINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ReadPipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            WritePipeBlockingINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            FPGARegINTEL(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetRayTMinKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetRayFlagsKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetIntersectionTKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionInstanceCustomIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionInstanceIdKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionGeometryIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionPrimitiveIndexKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionBarycentricsKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionFrontFaceKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionCandidateAABBOpaqueKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetIntersectionObjectRayDirectionKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionObjectRayOriginKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetWorldRayDirectionKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetWorldRayOriginKHR(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            RayQueryGetIntersectionObjectToWorldKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            RayQueryGetIntersectionWorldToObjectKHR(x0, x1, x2, x3) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
            }
            AtomicFAddEXT(x0, x1, x2, x3, x4, x5) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
                x5.write_word(env, sink);
            }
            TypeBufferSurfaceINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            TypeStructContinuedINTEL(x0) => {
                x0.write_word(env, sink);
            }
            ConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(env, sink);
            }
            SpecConstantCompositeContinuedINTEL(x0) => {
                x0.write_word(env, sink);
            }
            ControlBarrierArriveINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            ControlBarrierWaitINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            GroupIMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupFMulKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupBitwiseAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupBitwiseOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupBitwiseXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupLogicalAndKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupLogicalOrKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            GroupLogicalXorKHR(x0, x1, x2, x3, x4) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
                x3.write_word(env, sink);
                x4.write_word(env, sink);
            }
            SpecConstantOp(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_as_spec_op(env, sink);
            }
        }
        sink[mark] |= ((sink.len() - mark) as u32) << 16;
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageOperands {
    None() = 0,
    Bias(ID) = 1,
    Lod(ID) = 2,
    Grad(ID, ID) = 4,
    ConstOffset(ID) = 8,
    Offset(ID) = 16,
    ConstOffsets(ID) = 32,
    Sample(ID) = 64,
    MinLod(ID) = 128,
    MakeTexelAvailable(ScopeID) = 256,
    MakeTexelVisible(ScopeID) = 512,
    NonPrivateTexel() = 1024,
    VolatileTexel() = 2048,
    SignExtend() = 4096,
    ZeroExtend() = 8192,
    Nontemporal() = 16384,
    Offsets(ID) = 65536,
}
impl ImageOperands {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ImageOperands {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ImageOperands::*;
        sink.push(self.opcode());
        match self {
            Bias(x0) => {
                x0.write_word(env, sink);
            }
            Lod(x0) => {
                x0.write_word(env, sink);
            }
            Grad(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            ConstOffset(x0) => {
                x0.write_word(env, sink);
            }
            Offset(x0) => {
                x0.write_word(env, sink);
            }
            ConstOffsets(x0) => {
                x0.write_word(env, sink);
            }
            Sample(x0) => {
                x0.write_word(env, sink);
            }
            MinLod(x0) => {
                x0.write_word(env, sink);
            }
            MakeTexelAvailable(x0) => {
                x0.write_word(env, sink);
            }
            MakeTexelVisible(x0) => {
                x0.write_word(env, sink);
            }
            Offsets(x0) => {
                x0.write_word(env, sink);
            }
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ImageOperands::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Bias(x0)
            }
            2 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Lod(x0)
            }
            4 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                Grad(x0, x1)
            }
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConstOffset(x0)
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Offset(x0)
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ConstOffsets(x0)
            }
            64 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Sample(x0)
            }
            128 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MinLod(x0)
            }
            256 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MakeTexelAvailable(x0)
            }
            512 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MakeTexelVisible(x0)
            }
            1024 => NonPrivateTexel(),
            2048 => VolatileTexel(),
            4096 => SignExtend(),
            8192 => ZeroExtend(),
            16384 => Nontemporal(),
            65536 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Offsets(x0)
            }
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FPFastMathMode {
    None() = 0,
    NotNaN() = 1,
    NotInf() = 2,
    NSZ() = 4,
    AllowRecip() = 8,
    Fast() = 16,
    AllowContractFastINTEL() = 65536,
    AllowReassocINTEL() = 131072,
}
impl FPFastMathMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FPFastMathMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FPFastMathMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FPFastMathMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => NotNaN(),
            2 => NotInf(),
            4 => NSZ(),
            8 => AllowRecip(),
            16 => Fast(),
            65536 => AllowContractFastINTEL(),
            131072 => AllowReassocINTEL(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SelectionControl {
    None() = 0,
    Flatten() = 1,
    DontFlatten() = 2,
}
impl SelectionControl {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for SelectionControl {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use SelectionControl::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use SelectionControl::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => Flatten(),
            2 => DontFlatten(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LoopControl {
    None() = 0,
    Unroll() = 1,
    DontUnroll() = 2,
    DependencyInfinite() = 4,
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
impl LoopControl {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for LoopControl {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use LoopControl::*;
        sink.push(self.opcode());
        match self {
            DependencyLength(x0) => {
                x0.write_word(env, sink);
            }
            MinIterations(x0) => {
                x0.write_word(env, sink);
            }
            MaxIterations(x0) => {
                x0.write_word(env, sink);
            }
            IterationMultiple(x0) => {
                x0.write_word(env, sink);
            }
            PeelCount(x0) => {
                x0.write_word(env, sink);
            }
            PartialCount(x0) => {
                x0.write_word(env, sink);
            }
            InitiationIntervalINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MaxConcurrencyINTEL(x0) => {
                x0.write_word(env, sink);
            }
            DependencyArrayINTEL(x0) => {
                x0.write_word(env, sink);
            }
            PipelineEnableINTEL(x0) => {
                x0.write_word(env, sink);
            }
            LoopCoalesceINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MaxInterleavingINTEL(x0) => {
                x0.write_word(env, sink);
            }
            SpeculatedIterationsINTEL(x0) => {
                x0.write_word(env, sink);
            }
            NoFusionINTEL(x0) => {
                x0.write_word(env, sink);
            }
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use LoopControl::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => Unroll(),
            2 => DontUnroll(),
            4 => DependencyInfinite(),
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                DependencyLength(x0)
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MinIterations(x0)
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxIterations(x0)
            }
            64 => {
                let x0 = Writer::read_word(chunk, env, idx);
                IterationMultiple(x0)
            }
            128 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PeelCount(x0)
            }
            256 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PartialCount(x0)
            }
            65536 => {
                let x0 = Writer::read_word(chunk, env, idx);
                InitiationIntervalINTEL(x0)
            }
            131072 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxConcurrencyINTEL(x0)
            }
            262144 => {
                let x0 = Writer::read_word(chunk, env, idx);
                DependencyArrayINTEL(x0)
            }
            524288 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PipelineEnableINTEL(x0)
            }
            1048576 => {
                let x0 = Writer::read_word(chunk, env, idx);
                LoopCoalesceINTEL(x0)
            }
            2097152 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxInterleavingINTEL(x0)
            }
            4194304 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SpeculatedIterationsINTEL(x0)
            }
            8388608 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NoFusionINTEL(x0)
            }
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionControl {
    None() = 0,
    Inline() = 1,
    DontInline() = 2,
    Pure() = 4,
    Const() = 8,
    OptNoneINTEL() = 65536,
}
impl FunctionControl {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FunctionControl {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FunctionControl::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FunctionControl::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => Inline(),
            2 => DontInline(),
            4 => Pure(),
            8 => Const(),
            65536 => OptNoneINTEL(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemorySemantics {
    Relaxed() = 0,
    Acquire() = 2,
    Release() = 4,
    AcquireRelease() = 8,
    SequentiallyConsistent() = 16,
    UniformMemory() = 64,
    SubgroupMemory() = 128,
    WorkgroupMemory() = 256,
    CrossWorkgroupMemory() = 512,
    AtomicCounterMemory() = 1024,
    ImageMemory() = 2048,
    OutputMemory() = 4096,
    MakeAvailable() = 8192,
    MakeVisible() = 16384,
    Volatile() = 32768,
}
impl MemorySemantics {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for MemorySemantics {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use MemorySemantics::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use MemorySemantics::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Relaxed(),
            2 => Acquire(),
            4 => Release(),
            8 => AcquireRelease(),
            16 => SequentiallyConsistent(),
            64 => UniformMemory(),
            128 => SubgroupMemory(),
            256 => WorkgroupMemory(),
            512 => CrossWorkgroupMemory(),
            1024 => AtomicCounterMemory(),
            2048 => ImageMemory(),
            4096 => OutputMemory(),
            8192 => MakeAvailable(),
            16384 => MakeVisible(),
            32768 => Volatile(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryAccess {
    None() = 0,
    Volatile() = 1,
    Aligned(u32) = 2,
    Nontemporal() = 4,
    MakePointerAvailable(ScopeID) = 8,
    MakePointerVisible(ScopeID) = 16,
    NonPrivatePointer() = 32,
    AliasScopeINTELMask(ID) = 65536,
    NoAliasINTELMask(ID) = 131072,
}
impl MemoryAccess {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for MemoryAccess {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use MemoryAccess::*;
        sink.push(self.opcode());
        match self {
            Aligned(x0) => {
                x0.write_word(env, sink);
            }
            MakePointerAvailable(x0) => {
                x0.write_word(env, sink);
            }
            MakePointerVisible(x0) => {
                x0.write_word(env, sink);
            }
            AliasScopeINTELMask(x0) => {
                x0.write_word(env, sink);
            }
            NoAliasINTELMask(x0) => {
                x0.write_word(env, sink);
            }
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use MemoryAccess::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => Volatile(),
            2 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Aligned(x0)
            }
            4 => Nontemporal(),
            8 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MakePointerAvailable(x0)
            }
            16 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MakePointerVisible(x0)
            }
            32 => NonPrivatePointer(),
            65536 => {
                let x0 = Writer::read_word(chunk, env, idx);
                AliasScopeINTELMask(x0)
            }
            131072 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NoAliasINTELMask(x0)
            }
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KernelProfilingInfo {
    None() = 0,
    CmdExecTime() = 1,
}
impl KernelProfilingInfo {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for KernelProfilingInfo {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use KernelProfilingInfo::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use KernelProfilingInfo::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => CmdExecTime(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RayFlags {
    NoneKHR() = 0,
    OpaqueKHR() = 1,
    NoOpaqueKHR() = 2,
    TerminateOnFirstHitKHR() = 4,
    SkipClosestHitShaderKHR() = 8,
    CullBackFacingTrianglesKHR() = 16,
    CullFrontFacingTrianglesKHR() = 32,
    CullOpaqueKHR() = 64,
    CullNoOpaqueKHR() = 128,
    SkipTrianglesKHR() = 256,
    SkipAABBsKHR() = 512,
}
impl RayFlags {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for RayFlags {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use RayFlags::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use RayFlags::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => NoneKHR(),
            1 => OpaqueKHR(),
            2 => NoOpaqueKHR(),
            4 => TerminateOnFirstHitKHR(),
            8 => SkipClosestHitShaderKHR(),
            16 => CullBackFacingTrianglesKHR(),
            32 => CullFrontFacingTrianglesKHR(),
            64 => CullOpaqueKHR(),
            128 => CullNoOpaqueKHR(),
            256 => SkipTrianglesKHR(),
            512 => SkipAABBsKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FragmentShadingRate {
    Vertical2Pixels() = 1,
    Vertical4Pixels() = 2,
    Horizontal2Pixels() = 4,
    Horizontal4Pixels() = 8,
}
impl FragmentShadingRate {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FragmentShadingRate {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FragmentShadingRate::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FragmentShadingRate::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            1 => Vertical2Pixels(),
            2 => Vertical4Pixels(),
            4 => Horizontal2Pixels(),
            8 => Horizontal4Pixels(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SourceLanguage {
    Unknown() = 0,
    ESSL() = 1,
    GLSL() = 2,
    OpenCL_C() = 3,
    OpenCL_CPP() = 4,
    HLSL() = 5,
    CPP_for_OpenCL() = 6,
    SYCL() = 7,
}
impl SourceLanguage {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for SourceLanguage {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use SourceLanguage::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use SourceLanguage::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Unknown(),
            1 => ESSL(),
            2 => GLSL(),
            3 => OpenCL_C(),
            4 => OpenCL_CPP(),
            5 => HLSL(),
            6 => CPP_for_OpenCL(),
            7 => SYCL(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExecutionModel {
    Vertex() = 0,
    TessellationControl() = 1,
    TessellationEvaluation() = 2,
    Geometry() = 3,
    Fragment() = 4,
    GLCompute() = 5,
    Kernel() = 6,
    TaskNV() = 5267,
    MeshNV() = 5268,
    RayGenerationNV() = 5313,
    IntersectionNV() = 5314,
    AnyHitNV() = 5315,
    ClosestHitNV() = 5316,
    MissNV() = 5317,
    CallableNV() = 5318,
}
impl ExecutionModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ExecutionModel {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ExecutionModel::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ExecutionModel::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Vertex(),
            1 => TessellationControl(),
            2 => TessellationEvaluation(),
            3 => Geometry(),
            4 => Fragment(),
            5 => GLCompute(),
            6 => Kernel(),
            5267 => TaskNV(),
            5268 => MeshNV(),
            5313 => RayGenerationNV(),
            5314 => IntersectionNV(),
            5315 => AnyHitNV(),
            5316 => ClosestHitNV(),
            5317 => MissNV(),
            5318 => CallableNV(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AddressingModel {
    Logical() = 0,
    Physical32() = 1,
    Physical64() = 2,
    PhysicalStorageBuffer64() = 5348,
}
impl AddressingModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for AddressingModel {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use AddressingModel::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use AddressingModel::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Logical(),
            1 => Physical32(),
            2 => Physical64(),
            5348 => PhysicalStorageBuffer64(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryModel {
    Simple() = 0,
    GLSL450() = 1,
    OpenCL() = 2,
    Vulkan() = 3,
}
impl MemoryModel {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for MemoryModel {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use MemoryModel::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use MemoryModel::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Simple(),
            1 => GLSL450(),
            2 => OpenCL(),
            3 => Vulkan(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExecutionMode {
    Invocations(u32) = 0,
    SpacingEqual() = 1,
    SpacingFractionalEven() = 2,
    SpacingFractionalOdd() = 3,
    VertexOrderCw() = 4,
    VertexOrderCcw() = 5,
    PixelCenterInteger() = 6,
    OriginUpperLeft() = 7,
    OriginLowerLeft() = 8,
    EarlyFragmentTests() = 9,
    PointMode() = 10,
    Xfb() = 11,
    DepthReplacing() = 12,
    DepthGreater() = 14,
    DepthLess() = 15,
    DepthUnchanged() = 16,
    LocalSize(u32, u32, u32) = 17,
    LocalSizeHint(u32, u32, u32) = 18,
    InputPoints() = 19,
    InputLines() = 20,
    InputLinesAdjacency() = 21,
    Triangles() = 22,
    InputTrianglesAdjacency() = 23,
    Quads() = 24,
    Isolines() = 25,
    OutputVertices(u32) = 26,
    OutputPoints() = 27,
    OutputLineStrip() = 28,
    OutputTriangleStrip() = 29,
    VecTypeHint(u32) = 30,
    ContractionOff() = 31,
    Initializer() = 33,
    Finalizer() = 34,
    SubgroupSize(u32) = 35,
    SubgroupsPerWorkgroup(u32) = 36,
    SubgroupsPerWorkgroupId(ID) = 37,
    LocalSizeId(ID, ID, ID) = 38,
    LocalSizeHintId(ID, ID, ID) = 39,
    SubgroupUniformControlFlowKHR() = 4421,
    PostDepthCoverage() = 4446,
    DenormPreserve(u32) = 4459,
    DenormFlushToZero(u32) = 4460,
    SignedZeroInfNanPreserve(u32) = 4461,
    RoundingModeRTE(u32) = 4462,
    RoundingModeRTZ(u32) = 4463,
    StencilRefReplacingEXT() = 5027,
    OutputLinesNV() = 5269,
    OutputPrimitivesNV(u32) = 5270,
    DerivativeGroupQuadsNV() = 5289,
    DerivativeGroupLinearNV() = 5290,
    OutputTrianglesNV() = 5298,
    PixelInterlockOrderedEXT() = 5366,
    PixelInterlockUnorderedEXT() = 5367,
    SampleInterlockOrderedEXT() = 5368,
    SampleInterlockUnorderedEXT() = 5369,
    ShadingRateInterlockOrderedEXT() = 5370,
    ShadingRateInterlockUnorderedEXT() = 5371,
    SharedLocalMemorySizeINTEL(u32) = 5618,
    RoundingModeRTPINTEL(u32) = 5620,
    RoundingModeRTNINTEL(u32) = 5621,
    FloatingPointModeALTINTEL(u32) = 5622,
    FloatingPointModeIEEEINTEL(u32) = 5623,
    MaxWorkgroupSizeINTEL(u32, u32, u32) = 5893,
    MaxWorkDimINTEL(u32) = 5894,
    NoGlobalOffsetINTEL() = 5895,
    NumSIMDWorkitemsINTEL(u32) = 5896,
    SchedulerTargetFmaxMhzINTEL(u32) = 5903,
    NamedBarrierCountINTEL(u32) = 6417,
}
impl ExecutionMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ExecutionMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ExecutionMode::*;
        sink.push(self.opcode());
        match self {
            Invocations(x0) => {
                x0.write_word(env, sink);
            }
            LocalSize(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            LocalSizeHint(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            OutputVertices(x0) => {
                x0.write_word(env, sink);
            }
            VecTypeHint(x0) => {
                x0.write_word(env, sink);
            }
            SubgroupSize(x0) => {
                x0.write_word(env, sink);
            }
            SubgroupsPerWorkgroup(x0) => {
                x0.write_word(env, sink);
            }
            SubgroupsPerWorkgroupId(x0) => {
                x0.write_word(env, sink);
            }
            LocalSizeId(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            LocalSizeHintId(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            DenormPreserve(x0) => {
                x0.write_word(env, sink);
            }
            DenormFlushToZero(x0) => {
                x0.write_word(env, sink);
            }
            SignedZeroInfNanPreserve(x0) => {
                x0.write_word(env, sink);
            }
            RoundingModeRTE(x0) => {
                x0.write_word(env, sink);
            }
            RoundingModeRTZ(x0) => {
                x0.write_word(env, sink);
            }
            OutputPrimitivesNV(x0) => {
                x0.write_word(env, sink);
            }
            SharedLocalMemorySizeINTEL(x0) => {
                x0.write_word(env, sink);
            }
            RoundingModeRTPINTEL(x0) => {
                x0.write_word(env, sink);
            }
            RoundingModeRTNINTEL(x0) => {
                x0.write_word(env, sink);
            }
            FloatingPointModeALTINTEL(x0) => {
                x0.write_word(env, sink);
            }
            FloatingPointModeIEEEINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MaxWorkgroupSizeINTEL(x0, x1, x2) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
                x2.write_word(env, sink);
            }
            MaxWorkDimINTEL(x0) => {
                x0.write_word(env, sink);
            }
            NumSIMDWorkitemsINTEL(x0) => {
                x0.write_word(env, sink);
            }
            SchedulerTargetFmaxMhzINTEL(x0) => {
                x0.write_word(env, sink);
            }
            NamedBarrierCountINTEL(x0) => {
                x0.write_word(env, sink);
            }
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ExecutionMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Invocations(x0)
            }
            1 => SpacingEqual(),
            2 => SpacingFractionalEven(),
            3 => SpacingFractionalOdd(),
            4 => VertexOrderCw(),
            5 => VertexOrderCcw(),
            6 => PixelCenterInteger(),
            7 => OriginUpperLeft(),
            8 => OriginLowerLeft(),
            9 => EarlyFragmentTests(),
            10 => PointMode(),
            11 => Xfb(),
            12 => DepthReplacing(),
            14 => DepthGreater(),
            15 => DepthLess(),
            16 => DepthUnchanged(),
            17 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                LocalSize(x0, x1, x2)
            }
            18 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                LocalSizeHint(x0, x1, x2)
            }
            19 => InputPoints(),
            20 => InputLines(),
            21 => InputLinesAdjacency(),
            22 => Triangles(),
            23 => InputTrianglesAdjacency(),
            24 => Quads(),
            25 => Isolines(),
            26 => {
                let x0 = Writer::read_word(chunk, env, idx);
                OutputVertices(x0)
            }
            27 => OutputPoints(),
            28 => OutputLineStrip(),
            29 => OutputTriangleStrip(),
            30 => {
                let x0 = Writer::read_word(chunk, env, idx);
                VecTypeHint(x0)
            }
            31 => ContractionOff(),
            33 => Initializer(),
            34 => Finalizer(),
            35 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SubgroupSize(x0)
            }
            36 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SubgroupsPerWorkgroup(x0)
            }
            37 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SubgroupsPerWorkgroupId(x0)
            }
            38 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                LocalSizeId(x0, x1, x2)
            }
            39 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                LocalSizeHintId(x0, x1, x2)
            }
            4421 => SubgroupUniformControlFlowKHR(),
            4446 => PostDepthCoverage(),
            4459 => {
                let x0 = Writer::read_word(chunk, env, idx);
                DenormPreserve(x0)
            }
            4460 => {
                let x0 = Writer::read_word(chunk, env, idx);
                DenormFlushToZero(x0)
            }
            4461 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SignedZeroInfNanPreserve(x0)
            }
            4462 => {
                let x0 = Writer::read_word(chunk, env, idx);
                RoundingModeRTE(x0)
            }
            4463 => {
                let x0 = Writer::read_word(chunk, env, idx);
                RoundingModeRTZ(x0)
            }
            5027 => StencilRefReplacingEXT(),
            5269 => OutputLinesNV(),
            5270 => {
                let x0 = Writer::read_word(chunk, env, idx);
                OutputPrimitivesNV(x0)
            }
            5289 => DerivativeGroupQuadsNV(),
            5290 => DerivativeGroupLinearNV(),
            5298 => OutputTrianglesNV(),
            5366 => PixelInterlockOrderedEXT(),
            5367 => PixelInterlockUnorderedEXT(),
            5368 => SampleInterlockOrderedEXT(),
            5369 => SampleInterlockUnorderedEXT(),
            5370 => ShadingRateInterlockOrderedEXT(),
            5371 => ShadingRateInterlockUnorderedEXT(),
            5618 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SharedLocalMemorySizeINTEL(x0)
            }
            5620 => {
                let x0 = Writer::read_word(chunk, env, idx);
                RoundingModeRTPINTEL(x0)
            }
            5621 => {
                let x0 = Writer::read_word(chunk, env, idx);
                RoundingModeRTNINTEL(x0)
            }
            5622 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FloatingPointModeALTINTEL(x0)
            }
            5623 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FloatingPointModeIEEEINTEL(x0)
            }
            5893 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                let x2 = Writer::read_word(chunk, env, idx);
                MaxWorkgroupSizeINTEL(x0, x1, x2)
            }
            5894 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxWorkDimINTEL(x0)
            }
            5895 => NoGlobalOffsetINTEL(),
            5896 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NumSIMDWorkitemsINTEL(x0)
            }
            5903 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SchedulerTargetFmaxMhzINTEL(x0)
            }
            6417 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NamedBarrierCountINTEL(x0)
            }
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum StorageClass {
    UniformConstant() = 0,
    Input() = 1,
    Uniform() = 2,
    Output() = 3,
    Workgroup() = 4,
    CrossWorkgroup() = 5,
    Private() = 6,
    Function() = 7,
    Generic() = 8,
    PushConstant() = 9,
    AtomicCounter() = 10,
    Image() = 11,
    StorageBuffer() = 12,
    CallableDataNV() = 5328,
    IncomingCallableDataNV() = 5329,
    RayPayloadNV() = 5338,
    HitAttributeNV() = 5339,
    IncomingRayPayloadNV() = 5342,
    ShaderRecordBufferNV() = 5343,
    PhysicalStorageBuffer() = 5349,
    CodeSectionINTEL() = 5605,
    DeviceOnlyINTEL() = 5936,
    HostOnlyINTEL() = 5937,
}
impl StorageClass {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for StorageClass {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use StorageClass::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use StorageClass::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => UniformConstant(),
            1 => Input(),
            2 => Uniform(),
            3 => Output(),
            4 => Workgroup(),
            5 => CrossWorkgroup(),
            6 => Private(),
            7 => Function(),
            8 => Generic(),
            9 => PushConstant(),
            10 => AtomicCounter(),
            11 => Image(),
            12 => StorageBuffer(),
            5328 => CallableDataNV(),
            5329 => IncomingCallableDataNV(),
            5338 => RayPayloadNV(),
            5339 => HitAttributeNV(),
            5342 => IncomingRayPayloadNV(),
            5343 => ShaderRecordBufferNV(),
            5349 => PhysicalStorageBuffer(),
            5605 => CodeSectionINTEL(),
            5936 => DeviceOnlyINTEL(),
            5937 => HostOnlyINTEL(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Dim {
    _1D() = 0,
    _2D() = 1,
    _3D() = 2,
    Cube() = 3,
    Rect() = 4,
    Buffer() = 5,
    SubpassData() = 6,
}
impl Dim {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for Dim {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use Dim::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use Dim::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => _1D(),
            1 => _2D(),
            2 => _3D(),
            3 => Cube(),
            4 => Rect(),
            5 => Buffer(),
            6 => SubpassData(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SamplerAddressingMode {
    None() = 0,
    ClampToEdge() = 1,
    Clamp() = 2,
    Repeat() = 3,
    RepeatMirrored() = 4,
}
impl SamplerAddressingMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for SamplerAddressingMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use SamplerAddressingMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use SamplerAddressingMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => None(),
            1 => ClampToEdge(),
            2 => Clamp(),
            3 => Repeat(),
            4 => RepeatMirrored(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SamplerFilterMode {
    Nearest() = 0,
    Linear() = 1,
}
impl SamplerFilterMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for SamplerFilterMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use SamplerFilterMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use SamplerFilterMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Nearest(),
            1 => Linear(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageFormat {
    Unknown() = 0,
    Rgba32f() = 1,
    Rgba16f() = 2,
    R32f() = 3,
    Rgba8() = 4,
    Rgba8Snorm() = 5,
    Rg32f() = 6,
    Rg16f() = 7,
    R11fG11fB10f() = 8,
    R16f() = 9,
    Rgba16() = 10,
    Rgb10A2() = 11,
    Rg16() = 12,
    Rg8() = 13,
    R16() = 14,
    R8() = 15,
    Rgba16Snorm() = 16,
    Rg16Snorm() = 17,
    Rg8Snorm() = 18,
    R16Snorm() = 19,
    R8Snorm() = 20,
    Rgba32i() = 21,
    Rgba16i() = 22,
    Rgba8i() = 23,
    R32i() = 24,
    Rg32i() = 25,
    Rg16i() = 26,
    Rg8i() = 27,
    R16i() = 28,
    R8i() = 29,
    Rgba32ui() = 30,
    Rgba16ui() = 31,
    Rgba8ui() = 32,
    R32ui() = 33,
    Rgb10a2ui() = 34,
    Rg32ui() = 35,
    Rg16ui() = 36,
    Rg8ui() = 37,
    R16ui() = 38,
    R8ui() = 39,
    R64ui() = 40,
    R64i() = 41,
}
impl ImageFormat {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ImageFormat {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ImageFormat::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ImageFormat::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Unknown(),
            1 => Rgba32f(),
            2 => Rgba16f(),
            3 => R32f(),
            4 => Rgba8(),
            5 => Rgba8Snorm(),
            6 => Rg32f(),
            7 => Rg16f(),
            8 => R11fG11fB10f(),
            9 => R16f(),
            10 => Rgba16(),
            11 => Rgb10A2(),
            12 => Rg16(),
            13 => Rg8(),
            14 => R16(),
            15 => R8(),
            16 => Rgba16Snorm(),
            17 => Rg16Snorm(),
            18 => Rg8Snorm(),
            19 => R16Snorm(),
            20 => R8Snorm(),
            21 => Rgba32i(),
            22 => Rgba16i(),
            23 => Rgba8i(),
            24 => R32i(),
            25 => Rg32i(),
            26 => Rg16i(),
            27 => Rg8i(),
            28 => R16i(),
            29 => R8i(),
            30 => Rgba32ui(),
            31 => Rgba16ui(),
            32 => Rgba8ui(),
            33 => R32ui(),
            34 => Rgb10a2ui(),
            35 => Rg32ui(),
            36 => Rg16ui(),
            37 => Rg8ui(),
            38 => R16ui(),
            39 => R8ui(),
            40 => R64ui(),
            41 => R64i(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageChannelOrder {
    R() = 0,
    A() = 1,
    RG() = 2,
    RA() = 3,
    RGB() = 4,
    RGBA() = 5,
    BGRA() = 6,
    ARGB() = 7,
    Intensity() = 8,
    Luminance() = 9,
    Rx() = 10,
    RGx() = 11,
    RGBx() = 12,
    Depth() = 13,
    DepthStencil() = 14,
    sRGB() = 15,
    sRGBx() = 16,
    sRGBA() = 17,
    sBGRA() = 18,
    ABGR() = 19,
}
impl ImageChannelOrder {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ImageChannelOrder {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ImageChannelOrder::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ImageChannelOrder::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => R(),
            1 => A(),
            2 => RG(),
            3 => RA(),
            4 => RGB(),
            5 => RGBA(),
            6 => BGRA(),
            7 => ARGB(),
            8 => Intensity(),
            9 => Luminance(),
            10 => Rx(),
            11 => RGx(),
            12 => RGBx(),
            13 => Depth(),
            14 => DepthStencil(),
            15 => sRGB(),
            16 => sRGBx(),
            17 => sRGBA(),
            18 => sBGRA(),
            19 => ABGR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageChannelDataType {
    SnormInt8() = 0,
    SnormInt16() = 1,
    UnormInt8() = 2,
    UnormInt16() = 3,
    UnormShort565() = 4,
    UnormShort555() = 5,
    UnormInt101010() = 6,
    SignedInt8() = 7,
    SignedInt16() = 8,
    SignedInt32() = 9,
    UnsignedInt8() = 10,
    UnsignedInt16() = 11,
    UnsignedInt32() = 12,
    HalfFloat() = 13,
    Float() = 14,
    UnormInt24() = 15,
    UnormInt101010_2() = 16,
}
impl ImageChannelDataType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for ImageChannelDataType {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use ImageChannelDataType::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use ImageChannelDataType::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => SnormInt8(),
            1 => SnormInt16(),
            2 => UnormInt8(),
            3 => UnormInt16(),
            4 => UnormShort565(),
            5 => UnormShort555(),
            6 => UnormInt101010(),
            7 => SignedInt8(),
            8 => SignedInt16(),
            9 => SignedInt32(),
            10 => UnsignedInt8(),
            11 => UnsignedInt16(),
            12 => UnsignedInt32(),
            13 => HalfFloat(),
            14 => Float(),
            15 => UnormInt24(),
            16 => UnormInt101010_2(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FPRoundingMode {
    RTE() = 0,
    RTZ() = 1,
    RTP() = 2,
    RTN() = 3,
}
impl FPRoundingMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FPRoundingMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FPRoundingMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FPRoundingMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => RTE(),
            1 => RTZ(),
            2 => RTP(),
            3 => RTN(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FPDenormMode {
    Preserve() = 0,
    FlushToZero() = 1,
}
impl FPDenormMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FPDenormMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FPDenormMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FPDenormMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Preserve(),
            1 => FlushToZero(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QuantizationModes {
    TRN() = 0,
    TRN_ZERO() = 1,
    RND() = 2,
    RND_ZERO() = 3,
    RND_INF() = 4,
    RND_MIN_INF() = 5,
    RND_CONV() = 6,
    RND_CONV_ODD() = 7,
}
impl QuantizationModes {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for QuantizationModes {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use QuantizationModes::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use QuantizationModes::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => TRN(),
            1 => TRN_ZERO(),
            2 => RND(),
            3 => RND_ZERO(),
            4 => RND_INF(),
            5 => RND_MIN_INF(),
            6 => RND_CONV(),
            7 => RND_CONV_ODD(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FPOperationMode {
    IEEE() = 0,
    ALT() = 1,
}
impl FPOperationMode {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FPOperationMode {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FPOperationMode::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FPOperationMode::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => IEEE(),
            1 => ALT(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OverflowModes {
    WRAP() = 0,
    SAT() = 1,
    SAT_ZERO() = 2,
    SAT_SYM() = 3,
}
impl OverflowModes {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for OverflowModes {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use OverflowModes::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use OverflowModes::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => WRAP(),
            1 => SAT(),
            2 => SAT_ZERO(),
            3 => SAT_SYM(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LinkageType {
    Export() = 0,
    Import() = 1,
    LinkOnceODR() = 2,
}
impl LinkageType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for LinkageType {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use LinkageType::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use LinkageType::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Export(),
            1 => Import(),
            2 => LinkOnceODR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccessQualifier {
    ReadOnly() = 0,
    WriteOnly() = 1,
    ReadWrite() = 2,
}
impl AccessQualifier {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for AccessQualifier {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use AccessQualifier::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use AccessQualifier::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => ReadOnly(),
            1 => WriteOnly(),
            2 => ReadWrite(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionParameterAttribute {
    Zext() = 0,
    Sext() = 1,
    ByVal() = 2,
    Sret() = 3,
    NoAlias() = 4,
    NoCapture() = 5,
    NoWrite() = 6,
    NoReadWrite() = 7,
}
impl FunctionParameterAttribute {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for FunctionParameterAttribute {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use FunctionParameterAttribute::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use FunctionParameterAttribute::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Zext(),
            1 => Sext(),
            2 => ByVal(),
            3 => Sret(),
            4 => NoAlias(),
            5 => NoCapture(),
            6 => NoWrite(),
            7 => NoReadWrite(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Decoration {
    RelaxedPrecision() = 0,
    SpecId(u32) = 1,
    Block() = 2,
    BufferBlock() = 3,
    RowMajor() = 4,
    ColMajor() = 5,
    ArrayStride(u32) = 6,
    MatrixStride(u32) = 7,
    GLSLShared() = 8,
    GLSLPacked() = 9,
    CPacked() = 10,
    BuiltIn(BuiltIn) = 11,
    NoPerspective() = 13,
    Flat() = 14,
    Patch() = 15,
    Centroid() = 16,
    Sample() = 17,
    Invariant() = 18,
    Restrict() = 19,
    Aliased() = 20,
    Volatile() = 21,
    Constant() = 22,
    Coherent() = 23,
    NonWritable() = 24,
    NonReadable() = 25,
    Uniform() = 26,
    UniformId(ScopeID) = 27,
    SaturatedConversion() = 28,
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
    NoContraction() = 42,
    InputAttachmentIndex(u32) = 43,
    Alignment(u32) = 44,
    MaxByteOffset(u32) = 45,
    AlignmentId(ID) = 46,
    MaxByteOffsetId(ID) = 47,
    NoSignedWrap() = 4469,
    NoUnsignedWrap() = 4470,
    ExplicitInterpAMD() = 4999,
    OverrideCoverageNV() = 5248,
    PassthroughNV() = 5250,
    ViewportRelativeNV() = 5252,
    SecondaryViewportRelativeNV(u32) = 5256,
    PerPrimitiveNV() = 5271,
    PerViewNV() = 5272,
    PerTaskNV() = 5273,
    PerVertexKHR() = 5285,
    NonUniform() = 5300,
    RestrictPointer() = 5355,
    AliasedPointer() = 5356,
    BindlessSamplerNV() = 5398,
    BindlessImageNV() = 5399,
    BoundSamplerNV() = 5400,
    BoundImageNV() = 5401,
    SIMTCallINTEL(u32) = 5599,
    ReferencedIndirectlyINTEL() = 5602,
    ClobberINTEL(String) = 5607,
    SideEffectsINTEL() = 5608,
    VectorComputeVariableINTEL() = 5624,
    FuncParamIOKindINTEL(u32) = 5625,
    VectorComputeFunctionINTEL() = 5626,
    StackCallINTEL() = 5627,
    GlobalVariableOffsetINTEL(u32) = 5628,
    CounterBuffer(ID) = 5634,
    UserSemantic(String) = 5635,
    UserTypeGOOGLE(String) = 5636,
    FunctionRoundingModeINTEL(u32, FPRoundingMode) = 5822,
    FunctionDenormModeINTEL(u32, FPDenormMode) = 5823,
    RegisterINTEL() = 5825,
    MemoryINTEL(String) = 5826,
    NumbanksINTEL(u32) = 5827,
    BankwidthINTEL(u32) = 5828,
    MaxPrivateCopiesINTEL(u32) = 5829,
    SinglepumpINTEL() = 5830,
    DoublepumpINTEL() = 5831,
    MaxReplicatesINTEL(u32) = 5832,
    SimpleDualPortINTEL() = 5833,
    MergeINTEL(String, String) = 5834,
    BankBitsINTEL(u32) = 5835,
    ForcePow2DepthINTEL(u32) = 5836,
    BurstCoalesceINTEL() = 5899,
    CacheSizeINTEL(u32) = 5900,
    DontStaticallyCoalesceINTEL() = 5901,
    PrefetchINTEL(u32) = 5902,
    StallEnableINTEL() = 5905,
    FuseLoopsInFunctionINTEL() = 5907,
    AliasScopeINTEL(ID) = 5914,
    NoAliasINTEL(ID) = 5915,
    BufferLocationINTEL(u32) = 5921,
    IOPipeStorageINTEL(u32) = 5944,
    FunctionFloatingPointModeINTEL(u32, FPOperationMode) = 6080,
    SingleElementVectorINTEL() = 6085,
    VectorComputeCallableFunctionINTEL() = 6087,
    MediaBlockIOINTEL() = 6140,
}
impl Decoration {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for Decoration {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use Decoration::*;
        sink.push(self.opcode());
        match self {
            SpecId(x0) => {
                x0.write_word(env, sink);
            }
            ArrayStride(x0) => {
                x0.write_word(env, sink);
            }
            MatrixStride(x0) => {
                x0.write_word(env, sink);
            }
            BuiltIn(x0) => {
                x0.write_word(env, sink);
            }
            UniformId(x0) => {
                x0.write_word(env, sink);
            }
            Stream(x0) => {
                x0.write_word(env, sink);
            }
            Location(x0) => {
                x0.write_word(env, sink);
            }
            Component(x0) => {
                x0.write_word(env, sink);
            }
            Index(x0) => {
                x0.write_word(env, sink);
            }
            Binding(x0) => {
                x0.write_word(env, sink);
            }
            DescriptorSet(x0) => {
                x0.write_word(env, sink);
            }
            Offset(x0) => {
                x0.write_word(env, sink);
            }
            XfbBuffer(x0) => {
                x0.write_word(env, sink);
            }
            XfbStride(x0) => {
                x0.write_word(env, sink);
            }
            FuncParamAttr(x0) => {
                x0.write_word(env, sink);
            }
            FPRoundingMode(x0) => {
                x0.write_word(env, sink);
            }
            FPFastMathMode(x0) => {
                x0.write_word(env, sink);
            }
            LinkageAttributes(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            InputAttachmentIndex(x0) => {
                x0.write_word(env, sink);
            }
            Alignment(x0) => {
                x0.write_word(env, sink);
            }
            MaxByteOffset(x0) => {
                x0.write_word(env, sink);
            }
            AlignmentId(x0) => {
                x0.write_word(env, sink);
            }
            MaxByteOffsetId(x0) => {
                x0.write_word(env, sink);
            }
            SecondaryViewportRelativeNV(x0) => {
                x0.write_word(env, sink);
            }
            SIMTCallINTEL(x0) => {
                x0.write_word(env, sink);
            }
            ClobberINTEL(x0) => {
                x0.write_word(env, sink);
            }
            FuncParamIOKindINTEL(x0) => {
                x0.write_word(env, sink);
            }
            GlobalVariableOffsetINTEL(x0) => {
                x0.write_word(env, sink);
            }
            CounterBuffer(x0) => {
                x0.write_word(env, sink);
            }
            UserSemantic(x0) => {
                x0.write_word(env, sink);
            }
            UserTypeGOOGLE(x0) => {
                x0.write_word(env, sink);
            }
            FunctionRoundingModeINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            FunctionDenormModeINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            MemoryINTEL(x0) => {
                x0.write_word(env, sink);
            }
            NumbanksINTEL(x0) => {
                x0.write_word(env, sink);
            }
            BankwidthINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MaxPrivateCopiesINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MaxReplicatesINTEL(x0) => {
                x0.write_word(env, sink);
            }
            MergeINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            BankBitsINTEL(x0) => {
                x0.write_word(env, sink);
            }
            ForcePow2DepthINTEL(x0) => {
                x0.write_word(env, sink);
            }
            CacheSizeINTEL(x0) => {
                x0.write_word(env, sink);
            }
            PrefetchINTEL(x0) => {
                x0.write_word(env, sink);
            }
            AliasScopeINTEL(x0) => {
                x0.write_word(env, sink);
            }
            NoAliasINTEL(x0) => {
                x0.write_word(env, sink);
            }
            BufferLocationINTEL(x0) => {
                x0.write_word(env, sink);
            }
            IOPipeStorageINTEL(x0) => {
                x0.write_word(env, sink);
            }
            FunctionFloatingPointModeINTEL(x0, x1) => {
                x0.write_word(env, sink);
                x1.write_word(env, sink);
            }
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use Decoration::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => RelaxedPrecision(),
            1 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SpecId(x0)
            }
            2 => Block(),
            3 => BufferBlock(),
            4 => RowMajor(),
            5 => ColMajor(),
            6 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ArrayStride(x0)
            }
            7 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MatrixStride(x0)
            }
            8 => GLSLShared(),
            9 => GLSLPacked(),
            10 => CPacked(),
            11 => {
                let x0 = Writer::read_word(chunk, env, idx);
                BuiltIn(x0)
            }
            13 => NoPerspective(),
            14 => Flat(),
            15 => Patch(),
            16 => Centroid(),
            17 => Sample(),
            18 => Invariant(),
            19 => Restrict(),
            20 => Aliased(),
            21 => Volatile(),
            22 => Constant(),
            23 => Coherent(),
            24 => NonWritable(),
            25 => NonReadable(),
            26 => Uniform(),
            27 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UniformId(x0)
            }
            28 => SaturatedConversion(),
            29 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Stream(x0)
            }
            30 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Location(x0)
            }
            31 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Component(x0)
            }
            32 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Index(x0)
            }
            33 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Binding(x0)
            }
            34 => {
                let x0 = Writer::read_word(chunk, env, idx);
                DescriptorSet(x0)
            }
            35 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Offset(x0)
            }
            36 => {
                let x0 = Writer::read_word(chunk, env, idx);
                XfbBuffer(x0)
            }
            37 => {
                let x0 = Writer::read_word(chunk, env, idx);
                XfbStride(x0)
            }
            38 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FuncParamAttr(x0)
            }
            39 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FPRoundingMode(x0)
            }
            40 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FPFastMathMode(x0)
            }
            41 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                LinkageAttributes(x0, x1)
            }
            42 => NoContraction(),
            43 => {
                let x0 = Writer::read_word(chunk, env, idx);
                InputAttachmentIndex(x0)
            }
            44 => {
                let x0 = Writer::read_word(chunk, env, idx);
                Alignment(x0)
            }
            45 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxByteOffset(x0)
            }
            46 => {
                let x0 = Writer::read_word(chunk, env, idx);
                AlignmentId(x0)
            }
            47 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxByteOffsetId(x0)
            }
            4469 => NoSignedWrap(),
            4470 => NoUnsignedWrap(),
            4999 => ExplicitInterpAMD(),
            5248 => OverrideCoverageNV(),
            5250 => PassthroughNV(),
            5252 => ViewportRelativeNV(),
            5256 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SecondaryViewportRelativeNV(x0)
            }
            5271 => PerPrimitiveNV(),
            5272 => PerViewNV(),
            5273 => PerTaskNV(),
            5285 => PerVertexKHR(),
            5300 => NonUniform(),
            5355 => RestrictPointer(),
            5356 => AliasedPointer(),
            5398 => BindlessSamplerNV(),
            5399 => BindlessImageNV(),
            5400 => BoundSamplerNV(),
            5401 => BoundImageNV(),
            5599 => {
                let x0 = Writer::read_word(chunk, env, idx);
                SIMTCallINTEL(x0)
            }
            5602 => ReferencedIndirectlyINTEL(),
            5607 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ClobberINTEL(x0)
            }
            5608 => SideEffectsINTEL(),
            5624 => VectorComputeVariableINTEL(),
            5625 => {
                let x0 = Writer::read_word(chunk, env, idx);
                FuncParamIOKindINTEL(x0)
            }
            5626 => VectorComputeFunctionINTEL(),
            5627 => StackCallINTEL(),
            5628 => {
                let x0 = Writer::read_word(chunk, env, idx);
                GlobalVariableOffsetINTEL(x0)
            }
            5634 => {
                let x0 = Writer::read_word(chunk, env, idx);
                CounterBuffer(x0)
            }
            5635 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UserSemantic(x0)
            }
            5636 => {
                let x0 = Writer::read_word(chunk, env, idx);
                UserTypeGOOGLE(x0)
            }
            5822 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FunctionRoundingModeINTEL(x0, x1)
            }
            5823 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FunctionDenormModeINTEL(x0, x1)
            }
            5825 => RegisterINTEL(),
            5826 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MemoryINTEL(x0)
            }
            5827 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NumbanksINTEL(x0)
            }
            5828 => {
                let x0 = Writer::read_word(chunk, env, idx);
                BankwidthINTEL(x0)
            }
            5829 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxPrivateCopiesINTEL(x0)
            }
            5830 => SinglepumpINTEL(),
            5831 => DoublepumpINTEL(),
            5832 => {
                let x0 = Writer::read_word(chunk, env, idx);
                MaxReplicatesINTEL(x0)
            }
            5833 => SimpleDualPortINTEL(),
            5834 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                MergeINTEL(x0, x1)
            }
            5835 => {
                let x0 = Writer::read_word(chunk, env, idx);
                BankBitsINTEL(x0)
            }
            5836 => {
                let x0 = Writer::read_word(chunk, env, idx);
                ForcePow2DepthINTEL(x0)
            }
            5899 => BurstCoalesceINTEL(),
            5900 => {
                let x0 = Writer::read_word(chunk, env, idx);
                CacheSizeINTEL(x0)
            }
            5901 => DontStaticallyCoalesceINTEL(),
            5902 => {
                let x0 = Writer::read_word(chunk, env, idx);
                PrefetchINTEL(x0)
            }
            5905 => StallEnableINTEL(),
            5907 => FuseLoopsInFunctionINTEL(),
            5914 => {
                let x0 = Writer::read_word(chunk, env, idx);
                AliasScopeINTEL(x0)
            }
            5915 => {
                let x0 = Writer::read_word(chunk, env, idx);
                NoAliasINTEL(x0)
            }
            5921 => {
                let x0 = Writer::read_word(chunk, env, idx);
                BufferLocationINTEL(x0)
            }
            5944 => {
                let x0 = Writer::read_word(chunk, env, idx);
                IOPipeStorageINTEL(x0)
            }
            6080 => {
                let x0 = Writer::read_word(chunk, env, idx);
                let x1 = Writer::read_word(chunk, env, idx);
                FunctionFloatingPointModeINTEL(x0, x1)
            }
            6085 => SingleElementVectorINTEL(),
            6087 => VectorComputeCallableFunctionINTEL(),
            6140 => MediaBlockIOINTEL(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BuiltIn {
    Position() = 0,
    PointSize() = 1,
    ClipDistance() = 3,
    CullDistance() = 4,
    VertexId() = 5,
    InstanceId() = 6,
    PrimitiveId() = 7,
    InvocationId() = 8,
    Layer() = 9,
    ViewportIndex() = 10,
    TessLevelOuter() = 11,
    TessLevelInner() = 12,
    TessCoord() = 13,
    PatchVertices() = 14,
    FragCoord() = 15,
    PointCoord() = 16,
    FrontFacing() = 17,
    SampleId() = 18,
    SamplePosition() = 19,
    SampleMask() = 20,
    FragDepth() = 22,
    HelperInvocation() = 23,
    NumWorkgroups() = 24,
    WorkgroupSize() = 25,
    WorkgroupId() = 26,
    LocalInvocationId() = 27,
    GlobalInvocationId() = 28,
    LocalInvocationIndex() = 29,
    WorkDim() = 30,
    GlobalSize() = 31,
    EnqueuedWorkgroupSize() = 32,
    GlobalOffset() = 33,
    GlobalLinearId() = 34,
    SubgroupSize() = 36,
    SubgroupMaxSize() = 37,
    NumSubgroups() = 38,
    NumEnqueuedSubgroups() = 39,
    SubgroupId() = 40,
    SubgroupLocalInvocationId() = 41,
    VertexIndex() = 42,
    InstanceIndex() = 43,
    SubgroupEqMask() = 4416,
    SubgroupGeMask() = 4417,
    SubgroupGtMask() = 4418,
    SubgroupLeMask() = 4419,
    SubgroupLtMask() = 4420,
    BaseVertex() = 4424,
    BaseInstance() = 4425,
    DrawIndex() = 4426,
    PrimitiveShadingRateKHR() = 4432,
    DeviceIndex() = 4438,
    ViewIndex() = 4440,
    ShadingRateKHR() = 4444,
    BaryCoordNoPerspAMD() = 4992,
    BaryCoordNoPerspCentroidAMD() = 4993,
    BaryCoordNoPerspSampleAMD() = 4994,
    BaryCoordSmoothAMD() = 4995,
    BaryCoordSmoothCentroidAMD() = 4996,
    BaryCoordSmoothSampleAMD() = 4997,
    BaryCoordPullModelAMD() = 4998,
    FragStencilRefEXT() = 5014,
    ViewportMaskNV() = 5253,
    SecondaryPositionNV() = 5257,
    SecondaryViewportMaskNV() = 5258,
    PositionPerViewNV() = 5261,
    ViewportMaskPerViewNV() = 5262,
    FullyCoveredEXT() = 5264,
    TaskCountNV() = 5274,
    PrimitiveCountNV() = 5275,
    PrimitiveIndicesNV() = 5276,
    ClipDistancePerViewNV() = 5277,
    CullDistancePerViewNV() = 5278,
    LayerPerViewNV() = 5279,
    MeshViewCountNV() = 5280,
    MeshViewIndicesNV() = 5281,
    BaryCoordKHR() = 5286,
    BaryCoordNoPerspKHR() = 5287,
    FragSizeEXT() = 5292,
    FragInvocationCountEXT() = 5293,
    LaunchIdNV() = 5319,
    LaunchSizeNV() = 5320,
    WorldRayOriginNV() = 5321,
    WorldRayDirectionNV() = 5322,
    ObjectRayOriginNV() = 5323,
    ObjectRayDirectionNV() = 5324,
    RayTminNV() = 5325,
    RayTmaxNV() = 5326,
    InstanceCustomIndexNV() = 5327,
    ObjectToWorldNV() = 5330,
    WorldToObjectNV() = 5331,
    HitTNV() = 5332,
    HitKindNV() = 5333,
    CurrentRayTimeNV() = 5334,
    IncomingRayFlagsNV() = 5351,
    RayGeometryIndexKHR() = 5352,
    WarpsPerSMNV() = 5374,
    SMCountNV() = 5375,
    WarpIDNV() = 5376,
    SMIDNV() = 5377,
}
impl BuiltIn {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for BuiltIn {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use BuiltIn::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use BuiltIn::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Position(),
            1 => PointSize(),
            3 => ClipDistance(),
            4 => CullDistance(),
            5 => VertexId(),
            6 => InstanceId(),
            7 => PrimitiveId(),
            8 => InvocationId(),
            9 => Layer(),
            10 => ViewportIndex(),
            11 => TessLevelOuter(),
            12 => TessLevelInner(),
            13 => TessCoord(),
            14 => PatchVertices(),
            15 => FragCoord(),
            16 => PointCoord(),
            17 => FrontFacing(),
            18 => SampleId(),
            19 => SamplePosition(),
            20 => SampleMask(),
            22 => FragDepth(),
            23 => HelperInvocation(),
            24 => NumWorkgroups(),
            25 => WorkgroupSize(),
            26 => WorkgroupId(),
            27 => LocalInvocationId(),
            28 => GlobalInvocationId(),
            29 => LocalInvocationIndex(),
            30 => WorkDim(),
            31 => GlobalSize(),
            32 => EnqueuedWorkgroupSize(),
            33 => GlobalOffset(),
            34 => GlobalLinearId(),
            36 => SubgroupSize(),
            37 => SubgroupMaxSize(),
            38 => NumSubgroups(),
            39 => NumEnqueuedSubgroups(),
            40 => SubgroupId(),
            41 => SubgroupLocalInvocationId(),
            42 => VertexIndex(),
            43 => InstanceIndex(),
            4416 => SubgroupEqMask(),
            4417 => SubgroupGeMask(),
            4418 => SubgroupGtMask(),
            4419 => SubgroupLeMask(),
            4420 => SubgroupLtMask(),
            4424 => BaseVertex(),
            4425 => BaseInstance(),
            4426 => DrawIndex(),
            4432 => PrimitiveShadingRateKHR(),
            4438 => DeviceIndex(),
            4440 => ViewIndex(),
            4444 => ShadingRateKHR(),
            4992 => BaryCoordNoPerspAMD(),
            4993 => BaryCoordNoPerspCentroidAMD(),
            4994 => BaryCoordNoPerspSampleAMD(),
            4995 => BaryCoordSmoothAMD(),
            4996 => BaryCoordSmoothCentroidAMD(),
            4997 => BaryCoordSmoothSampleAMD(),
            4998 => BaryCoordPullModelAMD(),
            5014 => FragStencilRefEXT(),
            5253 => ViewportMaskNV(),
            5257 => SecondaryPositionNV(),
            5258 => SecondaryViewportMaskNV(),
            5261 => PositionPerViewNV(),
            5262 => ViewportMaskPerViewNV(),
            5264 => FullyCoveredEXT(),
            5274 => TaskCountNV(),
            5275 => PrimitiveCountNV(),
            5276 => PrimitiveIndicesNV(),
            5277 => ClipDistancePerViewNV(),
            5278 => CullDistancePerViewNV(),
            5279 => LayerPerViewNV(),
            5280 => MeshViewCountNV(),
            5281 => MeshViewIndicesNV(),
            5286 => BaryCoordKHR(),
            5287 => BaryCoordNoPerspKHR(),
            5292 => FragSizeEXT(),
            5293 => FragInvocationCountEXT(),
            5319 => LaunchIdNV(),
            5320 => LaunchSizeNV(),
            5321 => WorldRayOriginNV(),
            5322 => WorldRayDirectionNV(),
            5323 => ObjectRayOriginNV(),
            5324 => ObjectRayDirectionNV(),
            5325 => RayTminNV(),
            5326 => RayTmaxNV(),
            5327 => InstanceCustomIndexNV(),
            5330 => ObjectToWorldNV(),
            5331 => WorldToObjectNV(),
            5332 => HitTNV(),
            5333 => HitKindNV(),
            5334 => CurrentRayTimeNV(),
            5351 => IncomingRayFlagsNV(),
            5352 => RayGeometryIndexKHR(),
            5374 => WarpsPerSMNV(),
            5375 => SMCountNV(),
            5376 => WarpIDNV(),
            5377 => SMIDNV(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scope {
    CrossDevice() = 0,
    Device() = 1,
    Workgroup() = 2,
    Subgroup() = 3,
    Invocation() = 4,
    QueueFamily() = 5,
    ShaderCallKHR() = 6,
}
impl Scope {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for Scope {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use Scope::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use Scope::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => CrossDevice(),
            1 => Device(),
            2 => Workgroup(),
            3 => Subgroup(),
            4 => Invocation(),
            5 => QueueFamily(),
            6 => ShaderCallKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GroupOperation {
    Reduce() = 0,
    InclusiveScan() = 1,
    ExclusiveScan() = 2,
    ClusteredReduce() = 3,
    PartitionedReduceNV() = 6,
    PartitionedInclusiveScanNV() = 7,
    PartitionedExclusiveScanNV() = 8,
}
impl GroupOperation {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for GroupOperation {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use GroupOperation::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use GroupOperation::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Reduce(),
            1 => InclusiveScan(),
            2 => ExclusiveScan(),
            3 => ClusteredReduce(),
            6 => PartitionedReduceNV(),
            7 => PartitionedInclusiveScanNV(),
            8 => PartitionedExclusiveScanNV(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum KernelEnqueueFlags {
    NoWait() = 0,
    WaitKernel() = 1,
    WaitWorkGroup() = 2,
}
impl KernelEnqueueFlags {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for KernelEnqueueFlags {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use KernelEnqueueFlags::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use KernelEnqueueFlags::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => NoWait(),
            1 => WaitKernel(),
            2 => WaitWorkGroup(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Capability {
    Matrix() = 0,
    Shader() = 1,
    Geometry() = 2,
    Tessellation() = 3,
    Addresses() = 4,
    Linkage() = 5,
    Kernel() = 6,
    Vector16() = 7,
    Float16Buffer() = 8,
    Float16() = 9,
    Float64() = 10,
    Int64() = 11,
    Int64Atomics() = 12,
    ImageBasic() = 13,
    ImageReadWrite() = 14,
    ImageMipmap() = 15,
    Pipes() = 17,
    Groups() = 18,
    DeviceEnqueue() = 19,
    LiteralSampler() = 20,
    AtomicStorage() = 21,
    Int16() = 22,
    TessellationPointSize() = 23,
    GeometryPointSize() = 24,
    ImageGatherExtended() = 25,
    StorageImageMultisample() = 27,
    UniformBufferArrayDynamicIndexing() = 28,
    SampledImageArrayDynamicIndexing() = 29,
    StorageBufferArrayDynamicIndexing() = 30,
    StorageImageArrayDynamicIndexing() = 31,
    ClipDistance() = 32,
    CullDistance() = 33,
    ImageCubeArray() = 34,
    SampleRateShading() = 35,
    ImageRect() = 36,
    SampledRect() = 37,
    GenericPointer() = 38,
    Int8() = 39,
    InputAttachment() = 40,
    SparseResidency() = 41,
    MinLod() = 42,
    Sampled1D() = 43,
    Image1D() = 44,
    SampledCubeArray() = 45,
    SampledBuffer() = 46,
    ImageBuffer() = 47,
    ImageMSArray() = 48,
    StorageImageExtendedFormats() = 49,
    ImageQuery() = 50,
    DerivativeControl() = 51,
    InterpolationFunction() = 52,
    TransformFeedback() = 53,
    GeometryStreams() = 54,
    StorageImageReadWithoutFormat() = 55,
    StorageImageWriteWithoutFormat() = 56,
    MultiViewport() = 57,
    SubgroupDispatch() = 58,
    NamedBarrier() = 59,
    PipeStorage() = 60,
    GroupNonUniform() = 61,
    GroupNonUniformVote() = 62,
    GroupNonUniformArithmetic() = 63,
    GroupNonUniformBallot() = 64,
    GroupNonUniformShuffle() = 65,
    GroupNonUniformShuffleRelative() = 66,
    GroupNonUniformClustered() = 67,
    GroupNonUniformQuad() = 68,
    ShaderLayer() = 69,
    ShaderViewportIndex() = 70,
    UniformDecoration() = 71,
    FragmentShadingRateKHR() = 4422,
    SubgroupBallotKHR() = 4423,
    DrawParameters() = 4427,
    WorkgroupMemoryExplicitLayoutKHR() = 4428,
    WorkgroupMemoryExplicitLayout8BitAccessKHR() = 4429,
    WorkgroupMemoryExplicitLayout16BitAccessKHR() = 4430,
    SubgroupVoteKHR() = 4431,
    StorageBuffer16BitAccess() = 4433,
    UniformAndStorageBuffer16BitAccess() = 4434,
    StoragePushConstant16() = 4435,
    StorageInputOutput16() = 4436,
    DeviceGroup() = 4437,
    MultiView() = 4439,
    VariablePointersStorageBuffer() = 4441,
    VariablePointers() = 4442,
    AtomicStorageOps() = 4445,
    SampleMaskPostDepthCoverage() = 4447,
    StorageBuffer8BitAccess() = 4448,
    UniformAndStorageBuffer8BitAccess() = 4449,
    StoragePushConstant8() = 4450,
    DenormPreserve() = 4464,
    DenormFlushToZero() = 4465,
    SignedZeroInfNanPreserve() = 4466,
    RoundingModeRTE() = 4467,
    RoundingModeRTZ() = 4468,
    RayQueryProvisionalKHR() = 4471,
    RayQueryKHR() = 4472,
    RayTraversalPrimitiveCullingKHR() = 4478,
    RayTracingKHR() = 4479,
    Float16ImageAMD() = 5008,
    ImageGatherBiasLodAMD() = 5009,
    FragmentMaskAMD() = 5010,
    StencilExportEXT() = 5013,
    ImageReadWriteLodAMD() = 5015,
    Int64ImageEXT() = 5016,
    ShaderClockKHR() = 5055,
    SampleMaskOverrideCoverageNV() = 5249,
    GeometryShaderPassthroughNV() = 5251,
    ShaderViewportIndexLayerEXT() = 5254,
    ShaderViewportMaskNV() = 5255,
    ShaderStereoViewNV() = 5259,
    PerViewAttributesNV() = 5260,
    FragmentFullyCoveredEXT() = 5265,
    MeshShadingNV() = 5266,
    ImageFootprintNV() = 5282,
    FragmentBarycentricKHR() = 5284,
    ComputeDerivativeGroupQuadsNV() = 5288,
    FragmentDensityEXT() = 5291,
    GroupNonUniformPartitionedNV() = 5297,
    ShaderNonUniform() = 5301,
    RuntimeDescriptorArray() = 5302,
    InputAttachmentArrayDynamicIndexing() = 5303,
    UniformTexelBufferArrayDynamicIndexing() = 5304,
    StorageTexelBufferArrayDynamicIndexing() = 5305,
    UniformBufferArrayNonUniformIndexing() = 5306,
    SampledImageArrayNonUniformIndexing() = 5307,
    StorageBufferArrayNonUniformIndexing() = 5308,
    StorageImageArrayNonUniformIndexing() = 5309,
    InputAttachmentArrayNonUniformIndexing() = 5310,
    UniformTexelBufferArrayNonUniformIndexing() = 5311,
    StorageTexelBufferArrayNonUniformIndexing() = 5312,
    RayTracingNV() = 5340,
    RayTracingMotionBlurNV() = 5341,
    VulkanMemoryModel() = 5345,
    VulkanMemoryModelDeviceScope() = 5346,
    PhysicalStorageBufferAddresses() = 5347,
    ComputeDerivativeGroupLinearNV() = 5350,
    RayTracingProvisionalKHR() = 5353,
    CooperativeMatrixNV() = 5357,
    FragmentShaderSampleInterlockEXT() = 5363,
    FragmentShaderShadingRateInterlockEXT() = 5372,
    ShaderSMBuiltinsNV() = 5373,
    FragmentShaderPixelInterlockEXT() = 5378,
    DemoteToHelperInvocation() = 5379,
    BindlessTextureNV() = 5390,
    SubgroupShuffleINTEL() = 5568,
    SubgroupBufferBlockIOINTEL() = 5569,
    SubgroupImageBlockIOINTEL() = 5570,
    SubgroupImageMediaBlockIOINTEL() = 5579,
    RoundToInfinityINTEL() = 5582,
    FloatingPointModeINTEL() = 5583,
    IntegerFunctions2INTEL() = 5584,
    FunctionPointersINTEL() = 5603,
    IndirectReferencesINTEL() = 5604,
    AsmINTEL() = 5606,
    AtomicFloat32MinMaxEXT() = 5612,
    AtomicFloat64MinMaxEXT() = 5613,
    AtomicFloat16MinMaxEXT() = 5616,
    VectorComputeINTEL() = 5617,
    VectorAnyINTEL() = 5619,
    ExpectAssumeKHR() = 5629,
    SubgroupAvcMotionEstimationINTEL() = 5696,
    SubgroupAvcMotionEstimationIntraINTEL() = 5697,
    SubgroupAvcMotionEstimationChromaINTEL() = 5698,
    VariableLengthArrayINTEL() = 5817,
    FunctionFloatControlINTEL() = 5821,
    FPGAMemoryAttributesINTEL() = 5824,
    FPFastMathModeINTEL() = 5837,
    ArbitraryPrecisionIntegersINTEL() = 5844,
    ArbitraryPrecisionFloatingPointINTEL() = 5845,
    UnstructuredLoopControlsINTEL() = 5886,
    FPGALoopControlsINTEL() = 5888,
    KernelAttributesINTEL() = 5892,
    FPGAKernelAttributesINTEL() = 5897,
    FPGAMemoryAccessesINTEL() = 5898,
    FPGAClusterAttributesINTEL() = 5904,
    LoopFuseINTEL() = 5906,
    MemoryAccessAliasingINTEL() = 5910,
    FPGABufferLocationINTEL() = 5920,
    ArbitraryPrecisionFixedPointINTEL() = 5922,
    USMStorageClassesINTEL() = 5935,
    IOPipesINTEL() = 5943,
    BlockingPipesINTEL() = 5945,
    FPGARegINTEL() = 5948,
    DotProductInputAll() = 6016,
    DotProductInput4x8Bit() = 6017,
    DotProductInput4x8BitPacked() = 6018,
    DotProduct() = 6019,
    BitInstructions() = 6025,
    AtomicFloat32AddEXT() = 6033,
    AtomicFloat64AddEXT() = 6034,
    LongConstantCompositeINTEL() = 6089,
    OptNoneINTEL() = 6094,
    AtomicFloat16AddEXT() = 6095,
    DebugInfoModuleINTEL() = 6114,
    SplitBarrierINTEL() = 6141,
    GroupUniformArithmeticKHR() = 6400,
}
impl Capability {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for Capability {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use Capability::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use Capability::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => Matrix(),
            1 => Shader(),
            2 => Geometry(),
            3 => Tessellation(),
            4 => Addresses(),
            5 => Linkage(),
            6 => Kernel(),
            7 => Vector16(),
            8 => Float16Buffer(),
            9 => Float16(),
            10 => Float64(),
            11 => Int64(),
            12 => Int64Atomics(),
            13 => ImageBasic(),
            14 => ImageReadWrite(),
            15 => ImageMipmap(),
            17 => Pipes(),
            18 => Groups(),
            19 => DeviceEnqueue(),
            20 => LiteralSampler(),
            21 => AtomicStorage(),
            22 => Int16(),
            23 => TessellationPointSize(),
            24 => GeometryPointSize(),
            25 => ImageGatherExtended(),
            27 => StorageImageMultisample(),
            28 => UniformBufferArrayDynamicIndexing(),
            29 => SampledImageArrayDynamicIndexing(),
            30 => StorageBufferArrayDynamicIndexing(),
            31 => StorageImageArrayDynamicIndexing(),
            32 => ClipDistance(),
            33 => CullDistance(),
            34 => ImageCubeArray(),
            35 => SampleRateShading(),
            36 => ImageRect(),
            37 => SampledRect(),
            38 => GenericPointer(),
            39 => Int8(),
            40 => InputAttachment(),
            41 => SparseResidency(),
            42 => MinLod(),
            43 => Sampled1D(),
            44 => Image1D(),
            45 => SampledCubeArray(),
            46 => SampledBuffer(),
            47 => ImageBuffer(),
            48 => ImageMSArray(),
            49 => StorageImageExtendedFormats(),
            50 => ImageQuery(),
            51 => DerivativeControl(),
            52 => InterpolationFunction(),
            53 => TransformFeedback(),
            54 => GeometryStreams(),
            55 => StorageImageReadWithoutFormat(),
            56 => StorageImageWriteWithoutFormat(),
            57 => MultiViewport(),
            58 => SubgroupDispatch(),
            59 => NamedBarrier(),
            60 => PipeStorage(),
            61 => GroupNonUniform(),
            62 => GroupNonUniformVote(),
            63 => GroupNonUniformArithmetic(),
            64 => GroupNonUniformBallot(),
            65 => GroupNonUniformShuffle(),
            66 => GroupNonUniformShuffleRelative(),
            67 => GroupNonUniformClustered(),
            68 => GroupNonUniformQuad(),
            69 => ShaderLayer(),
            70 => ShaderViewportIndex(),
            71 => UniformDecoration(),
            4422 => FragmentShadingRateKHR(),
            4423 => SubgroupBallotKHR(),
            4427 => DrawParameters(),
            4428 => WorkgroupMemoryExplicitLayoutKHR(),
            4429 => WorkgroupMemoryExplicitLayout8BitAccessKHR(),
            4430 => WorkgroupMemoryExplicitLayout16BitAccessKHR(),
            4431 => SubgroupVoteKHR(),
            4433 => StorageBuffer16BitAccess(),
            4434 => UniformAndStorageBuffer16BitAccess(),
            4435 => StoragePushConstant16(),
            4436 => StorageInputOutput16(),
            4437 => DeviceGroup(),
            4439 => MultiView(),
            4441 => VariablePointersStorageBuffer(),
            4442 => VariablePointers(),
            4445 => AtomicStorageOps(),
            4447 => SampleMaskPostDepthCoverage(),
            4448 => StorageBuffer8BitAccess(),
            4449 => UniformAndStorageBuffer8BitAccess(),
            4450 => StoragePushConstant8(),
            4464 => DenormPreserve(),
            4465 => DenormFlushToZero(),
            4466 => SignedZeroInfNanPreserve(),
            4467 => RoundingModeRTE(),
            4468 => RoundingModeRTZ(),
            4471 => RayQueryProvisionalKHR(),
            4472 => RayQueryKHR(),
            4478 => RayTraversalPrimitiveCullingKHR(),
            4479 => RayTracingKHR(),
            5008 => Float16ImageAMD(),
            5009 => ImageGatherBiasLodAMD(),
            5010 => FragmentMaskAMD(),
            5013 => StencilExportEXT(),
            5015 => ImageReadWriteLodAMD(),
            5016 => Int64ImageEXT(),
            5055 => ShaderClockKHR(),
            5249 => SampleMaskOverrideCoverageNV(),
            5251 => GeometryShaderPassthroughNV(),
            5254 => ShaderViewportIndexLayerEXT(),
            5255 => ShaderViewportMaskNV(),
            5259 => ShaderStereoViewNV(),
            5260 => PerViewAttributesNV(),
            5265 => FragmentFullyCoveredEXT(),
            5266 => MeshShadingNV(),
            5282 => ImageFootprintNV(),
            5284 => FragmentBarycentricKHR(),
            5288 => ComputeDerivativeGroupQuadsNV(),
            5291 => FragmentDensityEXT(),
            5297 => GroupNonUniformPartitionedNV(),
            5301 => ShaderNonUniform(),
            5302 => RuntimeDescriptorArray(),
            5303 => InputAttachmentArrayDynamicIndexing(),
            5304 => UniformTexelBufferArrayDynamicIndexing(),
            5305 => StorageTexelBufferArrayDynamicIndexing(),
            5306 => UniformBufferArrayNonUniformIndexing(),
            5307 => SampledImageArrayNonUniformIndexing(),
            5308 => StorageBufferArrayNonUniformIndexing(),
            5309 => StorageImageArrayNonUniformIndexing(),
            5310 => InputAttachmentArrayNonUniformIndexing(),
            5311 => UniformTexelBufferArrayNonUniformIndexing(),
            5312 => StorageTexelBufferArrayNonUniformIndexing(),
            5340 => RayTracingNV(),
            5341 => RayTracingMotionBlurNV(),
            5345 => VulkanMemoryModel(),
            5346 => VulkanMemoryModelDeviceScope(),
            5347 => PhysicalStorageBufferAddresses(),
            5350 => ComputeDerivativeGroupLinearNV(),
            5353 => RayTracingProvisionalKHR(),
            5357 => CooperativeMatrixNV(),
            5363 => FragmentShaderSampleInterlockEXT(),
            5372 => FragmentShaderShadingRateInterlockEXT(),
            5373 => ShaderSMBuiltinsNV(),
            5378 => FragmentShaderPixelInterlockEXT(),
            5379 => DemoteToHelperInvocation(),
            5390 => BindlessTextureNV(),
            5568 => SubgroupShuffleINTEL(),
            5569 => SubgroupBufferBlockIOINTEL(),
            5570 => SubgroupImageBlockIOINTEL(),
            5579 => SubgroupImageMediaBlockIOINTEL(),
            5582 => RoundToInfinityINTEL(),
            5583 => FloatingPointModeINTEL(),
            5584 => IntegerFunctions2INTEL(),
            5603 => FunctionPointersINTEL(),
            5604 => IndirectReferencesINTEL(),
            5606 => AsmINTEL(),
            5612 => AtomicFloat32MinMaxEXT(),
            5613 => AtomicFloat64MinMaxEXT(),
            5616 => AtomicFloat16MinMaxEXT(),
            5617 => VectorComputeINTEL(),
            5619 => VectorAnyINTEL(),
            5629 => ExpectAssumeKHR(),
            5696 => SubgroupAvcMotionEstimationINTEL(),
            5697 => SubgroupAvcMotionEstimationIntraINTEL(),
            5698 => SubgroupAvcMotionEstimationChromaINTEL(),
            5817 => VariableLengthArrayINTEL(),
            5821 => FunctionFloatControlINTEL(),
            5824 => FPGAMemoryAttributesINTEL(),
            5837 => FPFastMathModeINTEL(),
            5844 => ArbitraryPrecisionIntegersINTEL(),
            5845 => ArbitraryPrecisionFloatingPointINTEL(),
            5886 => UnstructuredLoopControlsINTEL(),
            5888 => FPGALoopControlsINTEL(),
            5892 => KernelAttributesINTEL(),
            5897 => FPGAKernelAttributesINTEL(),
            5898 => FPGAMemoryAccessesINTEL(),
            5904 => FPGAClusterAttributesINTEL(),
            5906 => LoopFuseINTEL(),
            5910 => MemoryAccessAliasingINTEL(),
            5920 => FPGABufferLocationINTEL(),
            5922 => ArbitraryPrecisionFixedPointINTEL(),
            5935 => USMStorageClassesINTEL(),
            5943 => IOPipesINTEL(),
            5945 => BlockingPipesINTEL(),
            5948 => FPGARegINTEL(),
            6016 => DotProductInputAll(),
            6017 => DotProductInput4x8Bit(),
            6018 => DotProductInput4x8BitPacked(),
            6019 => DotProduct(),
            6025 => BitInstructions(),
            6033 => AtomicFloat32AddEXT(),
            6034 => AtomicFloat64AddEXT(),
            6089 => LongConstantCompositeINTEL(),
            6094 => OptNoneINTEL(),
            6095 => AtomicFloat16AddEXT(),
            6114 => DebugInfoModuleINTEL(),
            6141 => SplitBarrierINTEL(),
            6400 => GroupUniformArithmeticKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RayQueryIntersection {
    RayQueryCandidateIntersectionKHR() = 0,
    RayQueryCommittedIntersectionKHR() = 1,
}
impl RayQueryIntersection {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for RayQueryIntersection {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use RayQueryIntersection::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use RayQueryIntersection::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCandidateIntersectionKHR(),
            1 => RayQueryCommittedIntersectionKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RayQueryCommittedIntersectionType {
    RayQueryCommittedIntersectionNoneKHR() = 0,
    RayQueryCommittedIntersectionTriangleKHR() = 1,
    RayQueryCommittedIntersectionGeneratedKHR() = 2,
}
impl RayQueryCommittedIntersectionType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for RayQueryCommittedIntersectionType {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use RayQueryCommittedIntersectionType::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use RayQueryCommittedIntersectionType::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCommittedIntersectionNoneKHR(),
            1 => RayQueryCommittedIntersectionTriangleKHR(),
            2 => RayQueryCommittedIntersectionGeneratedKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RayQueryCandidateIntersectionType {
    RayQueryCandidateIntersectionTriangleKHR() = 0,
    RayQueryCandidateIntersectionAABBKHR() = 1,
}
impl RayQueryCandidateIntersectionType {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for RayQueryCandidateIntersectionType {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use RayQueryCandidateIntersectionType::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use RayQueryCandidateIntersectionType::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => RayQueryCandidateIntersectionTriangleKHR(),
            1 => RayQueryCandidateIntersectionAABBKHR(),
            _ => panic!(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PackedVectorFormat {
    PackedVectorFormat4x8Bit() = 0,
}
impl PackedVectorFormat {
    pub fn opcode(&self) -> u32 {
        unsafe { std::mem::transmute_copy(self) }
    }
}
impl<Env: Environ> Writer<Env> for PackedVectorFormat {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        use PackedVectorFormat::*;
        sink.push(self.opcode());
        match self {
            _ => (),
        }
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        use PackedVectorFormat::*;
        *idx += 1;
        match chunk[*idx as usize - 1] {
            0 => PackedVectorFormat4x8Bit(),
            _ => panic!(),
        }
    }
}
pub trait Environ {
    fn insert_id(&mut self, i: ResultID);
    fn insert_op(&mut self, opc: Opcode) -> Opcode;
}
pub trait Writer<Env: Environ> {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {}
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self;
}
impl<Env: Environ, T: Writer<Env>> Writer<Env> for Option<T> {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        self.as_ref().map(|t| t.write_word(env, sink));
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        if *idx < chunk.len() {
            Some(T::read_word(chunk, env, idx))
        } else {
            None
        }
    }
}
impl<Env: Environ, T: Writer<Env>, U: Writer<Env>> Writer<Env> for (T, U) {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        self.0.write_word(env, sink);
        self.1.write_word(env, sink);
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        let t = T::read_word(chunk, env, idx);
        let u = U::read_word(chunk, env, idx);
        (t, u)
    }
}
impl<Env: Environ, T: Writer<Env>> Writer<Env> for Vec<T> {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        self.iter().for_each(|t| t.write_word(env, sink));
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        let mut re = vec![];
        while *idx < chunk.len() {
            re.push(T::read_word(chunk, env, idx));
        }
        re
    }
}
impl<Env: Environ> Writer<Env> for u32 {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        sink.push(*self);
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        *idx += 1;
        chunk[*idx as usize - 1]
    }
}
impl<Env: Environ> Writer<Env> for String {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
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
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
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
impl<Env: Environ, T: From<ID> + Into<ID> + Copy> Writer<Env> for T {
    fn write_word(&self, env: &Env, sink: &mut Vec<u32>) {
        let word: ID = (*self).into();
        sink.push(word.0);
    }
    fn read_word(chunk: &[u32], env: &mut Env, idx: &mut usize) -> Self {
        let id = ID(chunk[*idx as usize]);
        *idx += 1;
        Self::from(id)
    }
}
impl From<ID> for ResultID {
    fn from(id: ID) -> Self {
        Self(id)
    }
}
impl From<ID> for TypeID {
    fn from(id: ID) -> Self {
        Self(id)
    }
}
impl From<ID> for ScopeID {
    fn from(id: ID) -> Self {
        Self(id)
    }
}
impl From<ID> for MemorySemanticsID {
    fn from(id: ID) -> Self {
        Self(id)
    }
}
impl Into<ID> for ResultID {
    fn into(self) -> ID {
        self.0
    }
}
impl Into<ID> for TypeID {
    fn into(self) -> ID {
        self.0
    }
}
impl Into<ID> for ScopeID {
    fn into(self) -> ID {
        self.0
    }
}
impl Into<ID> for MemorySemanticsID {
    fn into(self) -> ID {
        self.0
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct ID(pub u32);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct ScopeID(pub ID);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct MemorySemanticsID(pub ID);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct TypeID(pub ID);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct ResultID(pub ID);
