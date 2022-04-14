; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 125
; Schema: 0
               OpCapability Shader
               OpCapability PhysicalStorageBufferAddresses
               OpExtension "SPV_KHR_physical_storage_buffer"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel PhysicalStorageBuffer64 GLSL450
               OpEntryPoint Vertex %4 "main" %10 %39 %66 %120
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "slice"
               OpName %10 "gl_InstanceIndex"
               OpName %16 "Position"
               OpMemberName %16 0 "positions"
               OpName %18 "positions"
               OpName %22 "Registers"
               OpMemberName %22 0 "view_projection"
               OpMemberName %22 1 "references"
               OpName %24 "PositionReferences"
               OpMemberName %24 0 "buffers"
               OpName %26 "registers"
               OpName %37 "pos"
               OpName %39 "gl_VertexIndex"
               OpName %64 "gl_PerVertex"
               OpMemberName %64 0 "gl_Position"
               OpMemberName %64 1 "gl_PointSize"
               OpMemberName %64 2 "gl_ClipDistance"
               OpMemberName %64 3 "gl_CullDistance"
               OpName %66 ""
               OpName %79 "index_x"
               OpName %83 "index_y"
               OpName %87 "r"
               OpName %95 "g"
               OpName %101 "checkerboard"
               OpName %120 "out_color"
               OpDecorate %10 BuiltIn InstanceIndex
               OpDecorate %15 ArrayStride 8
               OpMemberDecorate %16 0 NonWritable
               OpMemberDecorate %16 0 Offset 0
               OpDecorate %16 Block
               OpDecorate %18 RestrictPointer
               OpMemberDecorate %22 0 ColMajor
               OpMemberDecorate %22 0 Offset 0
               OpMemberDecorate %22 0 MatrixStride 16
               OpMemberDecorate %22 1 Offset 64
               OpDecorate %22 Block
               OpDecorate %23 ArrayStride 8
               OpMemberDecorate %24 0 NonWritable
               OpMemberDecorate %24 0 Offset 0
               OpDecorate %24 Block
               OpDecorate %39 BuiltIn VertexIndex
               OpMemberDecorate %64 0 BuiltIn Position
               OpMemberDecorate %64 1 BuiltIn PointSize
               OpMemberDecorate %64 2 BuiltIn ClipDistance
               OpMemberDecorate %64 3 BuiltIn CullDistance
               OpDecorate %64 Block
               OpDecorate %120 Flat
               OpDecorate %120 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 1
          %7 = OpTypePointer Function %6
          %9 = OpTypePointer Input %6
         %10 = OpVariable %9 Input
               OpTypeForwardPointer %12 PhysicalStorageBuffer
         %13 = OpTypeFloat 32
         %14 = OpTypeVector %13 2
         %15 = OpTypeRuntimeArray %14
         %16 = OpTypeStruct %15
         %12 = OpTypePointer PhysicalStorageBuffer %16
         %17 = OpTypePointer Function %12
         %19 = OpTypeVector %13 4
         %20 = OpTypeMatrix %19 4
               OpTypeForwardPointer %21 PhysicalStorageBuffer
         %22 = OpTypeStruct %20 %21
         %23 = OpTypeRuntimeArray %12
         %24 = OpTypeStruct %23
         %21 = OpTypePointer PhysicalStorageBuffer %24
         %25 = OpTypePointer PushConstant %22
         %26 = OpVariable %25 PushConstant
         %27 = OpConstant %6 1
         %28 = OpTypePointer PushConstant %21
         %31 = OpConstant %6 0
         %33 = OpTypePointer PhysicalStorageBuffer %12
         %36 = OpTypePointer Function %14
         %39 = OpVariable %9 Input
         %41 = OpTypePointer PhysicalStorageBuffer %14
         %44 = OpConstant %13 2.5
         %46 = OpConstant %13 3
         %48 = OpConstant %6 8
         %55 = OpConstant %13 3.5
         %61 = OpTypeInt 32 0
         %62 = OpConstant %61 1
         %63 = OpTypeArray %13 %62
         %64 = OpTypeStruct %19 %13 %63 %63
         %65 = OpTypePointer Output %64
         %66 = OpVariable %65 Output
         %67 = OpTypePointer PushConstant %20
         %71 = OpConstant %13 0
         %72 = OpConstant %13 1
         %77 = OpTypePointer Output %19
         %81 = OpConstant %6 16
         %86 = OpTypePointer Function %13
         %88 = OpConstant %13 0.5
         %89 = OpConstant %13 0.300000012
        %108 = OpConstant %13 0.800000012
        %110 = OpConstant %13 0.200000003
        %120 = OpVariable %77 Output
        %123 = OpConstant %13 0.150000006
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %8 = OpVariable %7 Function
         %18 = OpVariable %17 Function
         %37 = OpVariable %36 Function
         %79 = OpVariable %7 Function
         %83 = OpVariable %7 Function
         %87 = OpVariable %86 Function
         %95 = OpVariable %86 Function
        %101 = OpVariable %7 Function
         %11 = OpLoad %6 %10
               OpStore %8 %11
         %29 = OpAccessChain %28 %26 %27
         %30 = OpLoad %21 %29
         %32 = OpLoad %6 %8
         %34 = OpAccessChain %33 %30 %31 %32
         %35 = OpLoad %12 %34 Aligned 8
               OpStore %18 %35
         %38 = OpLoad %12 %18
         %40 = OpLoad %6 %39
         %42 = OpAccessChain %41 %38 %31 %40
         %43 = OpLoad %14 %42 Aligned 8
         %45 = OpVectorTimesScalar %14 %43 %44
               OpStore %37 %45
         %47 = OpLoad %6 %8
         %49 = OpSMod %6 %47 %48
         %50 = OpConvertSToF %13 %49
         %51 = OpLoad %6 %8
         %52 = OpSDiv %6 %51 %48
         %53 = OpConvertSToF %13 %52
         %54 = OpCompositeConstruct %14 %50 %53
         %56 = OpCompositeConstruct %14 %55 %55
         %57 = OpFSub %14 %54 %56
         %58 = OpVectorTimesScalar %14 %57 %46
         %59 = OpLoad %14 %37
         %60 = OpFAdd %14 %59 %58
               OpStore %37 %60
         %68 = OpAccessChain %67 %26 %31
         %69 = OpLoad %20 %68
         %70 = OpLoad %14 %37
         %73 = OpCompositeExtract %13 %70 0
         %74 = OpCompositeExtract %13 %70 1
         %75 = OpCompositeConstruct %19 %73 %74 %71 %72
         %76 = OpMatrixTimesVector %19 %69 %75
         %78 = OpAccessChain %77 %66 %31
               OpStore %78 %76
         %80 = OpLoad %6 %39
         %82 = OpSMod %6 %80 %81
               OpStore %79 %82
         %84 = OpLoad %6 %39
         %85 = OpSDiv %6 %84 %81
               OpStore %83 %85
         %90 = OpLoad %6 %79
         %91 = OpConvertSToF %13 %90
         %92 = OpExtInst %13 %1 Sin %91
         %93 = OpFMul %13 %89 %92
         %94 = OpFAdd %13 %88 %93
               OpStore %87 %94
         %96 = OpLoad %6 %83
         %97 = OpConvertSToF %13 %96
         %98 = OpExtInst %13 %1 Sin %97
         %99 = OpFMul %13 %89 %98
        %100 = OpFAdd %13 %88 %99
               OpStore %95 %100
        %102 = OpLoad %6 %79
        %103 = OpLoad %6 %83
        %104 = OpBitwiseXor %6 %102 %103
        %105 = OpBitwiseAnd %6 %104 %27
               OpStore %101 %105
        %106 = OpLoad %6 %101
        %107 = OpConvertSToF %13 %106
        %109 = OpFMul %13 %107 %108
        %111 = OpFAdd %13 %109 %110
        %112 = OpLoad %13 %87
        %113 = OpFMul %13 %112 %111
               OpStore %87 %113
        %114 = OpLoad %6 %101
        %115 = OpConvertSToF %13 %114
        %116 = OpFMul %13 %115 %108
        %117 = OpFAdd %13 %116 %110
        %118 = OpLoad %13 %95
        %119 = OpFMul %13 %118 %117
               OpStore %95 %119
        %121 = OpLoad %13 %87
        %122 = OpLoad %13 %95
        %124 = OpCompositeConstruct %19 %121 %122 %123 %72
               OpStore %120 %124
               OpReturn
               OpFunctionEnd
