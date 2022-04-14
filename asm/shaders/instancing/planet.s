; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 102
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %14 %16 %28 %43 %62 %73 %93 %98
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outColor"
               OpName %14 "outUV"
               OpName %16 "inUV"
               OpName %26 "gl_PerVertex"
               OpMemberName %26 0 "gl_Position"
               OpMemberName %26 1 "gl_PointSize"
               OpMemberName %26 2 "gl_ClipDistance"
               OpMemberName %26 3 "gl_CullDistance"
               OpName %28 ""
               OpName %32 "UBO"
               OpMemberName %32 0 "projection"
               OpMemberName %32 1 "modelview"
               OpMemberName %32 2 "lightPos"
               OpName %34 "ubo"
               OpName %43 "inPos"
               OpName %53 "pos"
               OpName %62 "outNormal"
               OpName %73 "inNormal"
               OpName %77 "lPos"
               OpName %93 "outLightVec"
               OpName %98 "outViewVec"
               OpDecorate %9 Location 1
               OpDecorate %14 Location 2
               OpDecorate %16 Location 2
               OpMemberDecorate %26 0 BuiltIn Position
               OpMemberDecorate %26 1 BuiltIn PointSize
               OpMemberDecorate %26 2 BuiltIn ClipDistance
               OpMemberDecorate %26 3 BuiltIn CullDistance
               OpDecorate %26 Block
               OpMemberDecorate %32 0 ColMajor
               OpMemberDecorate %32 0 Offset 0
               OpMemberDecorate %32 0 MatrixStride 16
               OpMemberDecorate %32 1 ColMajor
               OpMemberDecorate %32 1 Offset 64
               OpMemberDecorate %32 1 MatrixStride 16
               OpMemberDecorate %32 2 Offset 128
               OpDecorate %32 Block
               OpDecorate %34 DescriptorSet 0
               OpDecorate %34 Binding 0
               OpDecorate %43 Location 0
               OpDecorate %62 Location 0
               OpDecorate %73 Location 1
               OpDecorate %93 Location 4
               OpDecorate %98 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpConstant %6 1
         %11 = OpConstantComposite %7 %10 %10 %10
         %12 = OpTypeVector %6 2
         %13 = OpTypePointer Output %12
         %14 = OpVariable %13 Output
         %15 = OpTypePointer Input %12
         %16 = OpVariable %15 Input
         %18 = OpConstant %6 10
         %19 = OpConstant %6 6
         %20 = OpConstantComposite %12 %18 %19
         %22 = OpTypeVector %6 4
         %23 = OpTypeInt 32 0
         %24 = OpConstant %23 1
         %25 = OpTypeArray %6 %24
         %26 = OpTypeStruct %22 %6 %25 %25
         %27 = OpTypePointer Output %26
         %28 = OpVariable %27 Output
         %29 = OpTypeInt 32 1
         %30 = OpConstant %29 0
         %31 = OpTypeMatrix %22 4
         %32 = OpTypeStruct %31 %31 %22
         %33 = OpTypePointer Uniform %32
         %34 = OpVariable %33 Uniform
         %35 = OpTypePointer Uniform %31
         %38 = OpConstant %29 1
         %42 = OpTypePointer Input %7
         %43 = OpVariable %42 Input
         %50 = OpTypePointer Output %22
         %52 = OpTypePointer Function %22
         %62 = OpVariable %8 Output
         %65 = OpTypeMatrix %7 3
         %73 = OpVariable %42 Input
         %76 = OpTypePointer Function %7
         %87 = OpConstant %29 2
         %88 = OpTypePointer Uniform %22
         %93 = OpVariable %8 Output
         %98 = OpVariable %8 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %53 = OpVariable %52 Function
         %77 = OpVariable %76 Function
               OpStore %9 %11
         %17 = OpLoad %12 %16
         %21 = OpFMul %12 %17 %20
               OpStore %14 %21
         %36 = OpAccessChain %35 %34 %30
         %37 = OpLoad %31 %36
         %39 = OpAccessChain %35 %34 %38
         %40 = OpLoad %31 %39
         %41 = OpMatrixTimesMatrix %31 %37 %40
         %44 = OpLoad %7 %43
         %45 = OpCompositeExtract %6 %44 0
         %46 = OpCompositeExtract %6 %44 1
         %47 = OpCompositeExtract %6 %44 2
         %48 = OpCompositeConstruct %22 %45 %46 %47 %10
         %49 = OpMatrixTimesVector %22 %41 %48
         %51 = OpAccessChain %50 %28 %30
               OpStore %51 %49
         %54 = OpAccessChain %35 %34 %38
         %55 = OpLoad %31 %54
         %56 = OpLoad %7 %43
         %57 = OpCompositeExtract %6 %56 0
         %58 = OpCompositeExtract %6 %56 1
         %59 = OpCompositeExtract %6 %56 2
         %60 = OpCompositeConstruct %22 %57 %58 %59 %10
         %61 = OpMatrixTimesVector %22 %55 %60
               OpStore %53 %61
         %63 = OpAccessChain %35 %34 %38
         %64 = OpLoad %31 %63
         %66 = OpCompositeExtract %22 %64 0
         %67 = OpVectorShuffle %7 %66 %66 0 1 2
         %68 = OpCompositeExtract %22 %64 1
         %69 = OpVectorShuffle %7 %68 %68 0 1 2
         %70 = OpCompositeExtract %22 %64 2
         %71 = OpVectorShuffle %7 %70 %70 0 1 2
         %72 = OpCompositeConstruct %65 %67 %69 %71
         %74 = OpLoad %7 %73
         %75 = OpMatrixTimesVector %7 %72 %74
               OpStore %62 %75
         %78 = OpAccessChain %35 %34 %38
         %79 = OpLoad %31 %78
         %80 = OpCompositeExtract %22 %79 0
         %81 = OpVectorShuffle %7 %80 %80 0 1 2
         %82 = OpCompositeExtract %22 %79 1
         %83 = OpVectorShuffle %7 %82 %82 0 1 2
         %84 = OpCompositeExtract %22 %79 2
         %85 = OpVectorShuffle %7 %84 %84 0 1 2
         %86 = OpCompositeConstruct %65 %81 %83 %85
         %89 = OpAccessChain %88 %34 %87
         %90 = OpLoad %22 %89
         %91 = OpVectorShuffle %7 %90 %90 0 1 2
         %92 = OpMatrixTimesVector %7 %86 %91
               OpStore %77 %92
         %94 = OpLoad %7 %77
         %95 = OpLoad %22 %53
         %96 = OpVectorShuffle %7 %95 %95 0 1 2
         %97 = OpFSub %7 %94 %96
               OpStore %93 %97
         %99 = OpLoad %22 %53
        %100 = OpVectorShuffle %7 %99 %99 0 1 2
        %101 = OpFNegate %7 %100
               OpStore %98 %101
               OpReturn
               OpFunctionEnd
