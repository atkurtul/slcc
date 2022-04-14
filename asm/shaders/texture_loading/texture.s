; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 118
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %14 %33 %47 %73 %86 %104 %109
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %14 "outLodBias"
               OpName %17 "UBO"
               OpMemberName %17 0 "projection"
               OpMemberName %17 1 "model"
               OpMemberName %17 2 "viewPos"
               OpMemberName %17 3 "lodBias"
               OpName %19 "ubo"
               OpName %27 "worldPos"
               OpName %33 "inPos"
               OpName %45 "gl_PerVertex"
               OpMemberName %45 0 "gl_Position"
               OpName %47 ""
               OpName %63 "pos"
               OpName %73 "outNormal"
               OpName %86 "inNormal"
               OpName %89 "lightPos"
               OpName %92 "lPos"
               OpName %104 "outLightVec"
               OpName %109 "outViewVec"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpDecorate %14 Location 1
               OpMemberDecorate %17 0 ColMajor
               OpMemberDecorate %17 0 Offset 0
               OpMemberDecorate %17 0 MatrixStride 16
               OpMemberDecorate %17 1 ColMajor
               OpMemberDecorate %17 1 Offset 64
               OpMemberDecorate %17 1 MatrixStride 16
               OpMemberDecorate %17 2 Offset 128
               OpMemberDecorate %17 3 Offset 144
               OpDecorate %17 Block
               OpDecorate %19 DescriptorSet 0
               OpDecorate %19 Binding 0
               OpDecorate %33 Location 0
               OpMemberDecorate %45 0 BuiltIn Position
               OpDecorate %45 Block
               OpDecorate %73 Location 2
               OpDecorate %86 Location 2
               OpDecorate %104 Location 4
               OpDecorate %109 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypePointer Output %6
         %14 = OpVariable %13 Output
         %15 = OpTypeVector %6 4
         %16 = OpTypeMatrix %15 4
         %17 = OpTypeStruct %16 %16 %15 %6
         %18 = OpTypePointer Uniform %17
         %19 = OpVariable %18 Uniform
         %20 = OpTypeInt 32 1
         %21 = OpConstant %20 3
         %22 = OpTypePointer Uniform %6
         %25 = OpTypeVector %6 3
         %26 = OpTypePointer Function %25
         %28 = OpConstant %20 1
         %29 = OpTypePointer Uniform %16
         %32 = OpTypePointer Input %25
         %33 = OpVariable %32 Input
         %35 = OpConstant %6 1
         %45 = OpTypeStruct %15
         %46 = OpTypePointer Output %45
         %47 = OpVariable %46 Output
         %48 = OpConstant %20 0
         %60 = OpTypePointer Output %15
         %62 = OpTypePointer Function %15
         %72 = OpTypePointer Output %25
         %73 = OpVariable %72 Output
         %78 = OpTypeMatrix %25 3
         %86 = OpVariable %32 Input
         %90 = OpConstant %6 0
         %91 = OpConstantComposite %25 %90 %90 %90
        %104 = OpVariable %72 Output
        %109 = OpVariable %72 Output
        %110 = OpConstant %20 2
        %111 = OpTypePointer Uniform %15
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %27 = OpVariable %26 Function
         %63 = OpVariable %62 Function
         %89 = OpVariable %26 Function
         %92 = OpVariable %26 Function
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %23 = OpAccessChain %22 %19 %21
         %24 = OpLoad %6 %23
               OpStore %14 %24
         %30 = OpAccessChain %29 %19 %28
         %31 = OpLoad %16 %30
         %34 = OpLoad %25 %33
         %36 = OpCompositeExtract %6 %34 0
         %37 = OpCompositeExtract %6 %34 1
         %38 = OpCompositeExtract %6 %34 2
         %39 = OpCompositeConstruct %15 %36 %37 %38 %35
         %40 = OpMatrixTimesVector %15 %31 %39
         %41 = OpCompositeExtract %6 %40 0
         %42 = OpCompositeExtract %6 %40 1
         %43 = OpCompositeExtract %6 %40 2
         %44 = OpCompositeConstruct %25 %41 %42 %43
               OpStore %27 %44
         %49 = OpAccessChain %29 %19 %48
         %50 = OpLoad %16 %49
         %51 = OpAccessChain %29 %19 %28
         %52 = OpLoad %16 %51
         %53 = OpMatrixTimesMatrix %16 %50 %52
         %54 = OpLoad %25 %33
         %55 = OpCompositeExtract %6 %54 0
         %56 = OpCompositeExtract %6 %54 1
         %57 = OpCompositeExtract %6 %54 2
         %58 = OpCompositeConstruct %15 %55 %56 %57 %35
         %59 = OpMatrixTimesVector %15 %53 %58
         %61 = OpAccessChain %60 %47 %48
               OpStore %61 %59
         %64 = OpAccessChain %29 %19 %28
         %65 = OpLoad %16 %64
         %66 = OpLoad %25 %33
         %67 = OpCompositeExtract %6 %66 0
         %68 = OpCompositeExtract %6 %66 1
         %69 = OpCompositeExtract %6 %66 2
         %70 = OpCompositeConstruct %15 %67 %68 %69 %35
         %71 = OpMatrixTimesVector %15 %65 %70
               OpStore %63 %71
         %74 = OpAccessChain %29 %19 %28
         %75 = OpLoad %16 %74
         %76 = OpTranspose %16 %75
         %77 = OpExtInst %16 %1 MatrixInverse %76
         %79 = OpCompositeExtract %15 %77 0
         %80 = OpVectorShuffle %25 %79 %79 0 1 2
         %81 = OpCompositeExtract %15 %77 1
         %82 = OpVectorShuffle %25 %81 %81 0 1 2
         %83 = OpCompositeExtract %15 %77 2
         %84 = OpVectorShuffle %25 %83 %83 0 1 2
         %85 = OpCompositeConstruct %78 %80 %82 %84
         %87 = OpLoad %25 %86
         %88 = OpMatrixTimesVector %25 %85 %87
               OpStore %73 %88
               OpStore %89 %91
         %93 = OpAccessChain %29 %19 %28
         %94 = OpLoad %16 %93
         %95 = OpCompositeExtract %15 %94 0
         %96 = OpVectorShuffle %25 %95 %95 0 1 2
         %97 = OpCompositeExtract %15 %94 1
         %98 = OpVectorShuffle %25 %97 %97 0 1 2
         %99 = OpCompositeExtract %15 %94 2
        %100 = OpVectorShuffle %25 %99 %99 0 1 2
        %101 = OpCompositeConstruct %78 %96 %98 %100
        %102 = OpLoad %25 %89
        %103 = OpMatrixTimesVector %25 %101 %102
               OpStore %92 %103
        %105 = OpLoad %25 %92
        %106 = OpLoad %15 %63
        %107 = OpVectorShuffle %25 %106 %106 0 1 2
        %108 = OpFSub %25 %105 %107
               OpStore %104 %108
        %112 = OpAccessChain %111 %19 %110
        %113 = OpLoad %15 %112
        %114 = OpVectorShuffle %25 %113 %113 0 1 2
        %115 = OpLoad %15 %63
        %116 = OpVectorShuffle %25 %115 %115 0 1 2
        %117 = OpFSub %25 %114 %116
               OpStore %109 %117
               OpReturn
               OpFunctionEnd
