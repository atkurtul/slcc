; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 129
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %18 %44 %98 %108 %112 %122 %126
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUVW"
               OpName %11 "inPos"
               OpName %14 "type"
               OpName %18 "outPos"
               OpName %21 "UBO"
               OpMemberName %21 0 "projection"
               OpMemberName %21 1 "modelview"
               OpMemberName %21 2 "skybox_modelview"
               OpMemberName %21 3 "modelscale"
               OpName %23 "ubo"
               OpName %42 "gl_PerVertex"
               OpMemberName %42 0 "gl_Position"
               OpName %44 ""
               OpName %98 "outNormal"
               OpName %108 "inNormal"
               OpName %112 "outInvModelView"
               OpName %117 "lightPos"
               OpName %122 "outLightVec"
               OpName %126 "outViewVec"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 0
               OpDecorate %14 SpecId 0
               OpDecorate %18 Location 1
               OpMemberDecorate %21 0 ColMajor
               OpMemberDecorate %21 0 Offset 0
               OpMemberDecorate %21 0 MatrixStride 16
               OpMemberDecorate %21 1 ColMajor
               OpMemberDecorate %21 1 Offset 64
               OpMemberDecorate %21 1 MatrixStride 16
               OpMemberDecorate %21 2 ColMajor
               OpMemberDecorate %21 2 Offset 128
               OpMemberDecorate %21 2 MatrixStride 16
               OpMemberDecorate %21 3 Offset 192
               OpDecorate %21 Block
               OpDecorate %23 DescriptorSet 0
               OpDecorate %23 Binding 0
               OpMemberDecorate %42 0 BuiltIn Position
               OpDecorate %42 Block
               OpDecorate %98 Location 2
               OpDecorate %108 Location 1
               OpDecorate %112 Location 5
               OpDecorate %122 Location 4
               OpDecorate %126 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeInt 32 1
         %14 = OpSpecConstant %13 0
         %18 = OpVariable %8 Output
         %19 = OpTypeVector %6 4
         %20 = OpTypeMatrix %19 4
         %21 = OpTypeStruct %20 %20 %20 %6
         %22 = OpTypePointer Uniform %21
         %23 = OpVariable %22 Uniform
         %24 = OpConstant %13 2
         %25 = OpTypePointer Uniform %20
         %28 = OpTypeMatrix %7 3
         %42 = OpTypeStruct %19
         %43 = OpTypePointer Output %42
         %44 = OpVariable %43 Output
         %45 = OpConstant %13 0
         %49 = OpConstant %6 1
         %60 = OpTypePointer Output %19
         %63 = OpConstant %13 1
         %67 = OpConstant %13 3
         %68 = OpTypePointer Uniform %6
         %98 = OpVariable %8 Output
        %108 = OpVariable %10 Input
        %111 = OpTypePointer Output %20
        %112 = OpVariable %111 Output
        %116 = OpTypePointer Function %7
        %118 = OpConstant %6 0
        %119 = OpConstant %6 -5
        %120 = OpConstant %6 5
        %121 = OpConstantComposite %7 %118 %119 %120
        %122 = OpVariable %8 Output
        %126 = OpVariable %8 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %117 = OpVariable %116 Function
         %12 = OpLoad %7 %11
               OpStore %9 %12
               OpSelectionMerge %17 None
               OpSwitch %14 %17 0 %15 1 %16
         %15 = OpLabel
         %26 = OpAccessChain %25 %23 %24
         %27 = OpLoad %20 %26
         %29 = OpCompositeExtract %19 %27 0
         %30 = OpVectorShuffle %7 %29 %29 0 1 2
         %31 = OpCompositeExtract %19 %27 1
         %32 = OpVectorShuffle %7 %31 %31 0 1 2
         %33 = OpCompositeExtract %19 %27 2
         %34 = OpVectorShuffle %7 %33 %33 0 1 2
         %35 = OpCompositeConstruct %28 %30 %32 %34
         %36 = OpLoad %7 %11
         %37 = OpMatrixTimesVector %7 %35 %36
         %38 = OpCompositeExtract %6 %37 0
         %39 = OpCompositeExtract %6 %37 1
         %40 = OpCompositeExtract %6 %37 2
         %41 = OpCompositeConstruct %7 %38 %39 %40
               OpStore %18 %41
         %46 = OpAccessChain %25 %23 %45
         %47 = OpLoad %20 %46
         %48 = OpLoad %7 %18
         %50 = OpCompositeExtract %6 %48 0
         %51 = OpCompositeExtract %6 %48 1
         %52 = OpCompositeExtract %6 %48 2
         %53 = OpCompositeConstruct %19 %50 %51 %52 %49
         %54 = OpMatrixTimesVector %19 %47 %53
         %55 = OpCompositeExtract %6 %54 0
         %56 = OpCompositeExtract %6 %54 1
         %57 = OpCompositeExtract %6 %54 2
         %58 = OpCompositeExtract %6 %54 3
         %59 = OpCompositeConstruct %19 %55 %56 %57 %58
         %61 = OpAccessChain %60 %44 %45
               OpStore %61 %59
               OpBranch %17
         %16 = OpLabel
         %64 = OpAccessChain %25 %23 %63
         %65 = OpLoad %20 %64
         %66 = OpLoad %7 %11
         %69 = OpAccessChain %68 %23 %67
         %70 = OpLoad %6 %69
         %71 = OpVectorTimesScalar %7 %66 %70
         %72 = OpCompositeExtract %6 %71 0
         %73 = OpCompositeExtract %6 %71 1
         %74 = OpCompositeExtract %6 %71 2
         %75 = OpCompositeConstruct %19 %72 %73 %74 %49
         %76 = OpMatrixTimesVector %19 %65 %75
         %77 = OpCompositeExtract %6 %76 0
         %78 = OpCompositeExtract %6 %76 1
         %79 = OpCompositeExtract %6 %76 2
         %80 = OpCompositeConstruct %7 %77 %78 %79
               OpStore %18 %80
         %81 = OpAccessChain %25 %23 %45
         %82 = OpLoad %20 %81
         %83 = OpAccessChain %25 %23 %63
         %84 = OpLoad %20 %83
         %85 = OpMatrixTimesMatrix %20 %82 %84
         %86 = OpLoad %7 %11
         %87 = OpAccessChain %68 %23 %67
         %88 = OpLoad %6 %87
         %89 = OpVectorTimesScalar %7 %86 %88
         %90 = OpCompositeExtract %6 %89 0
         %91 = OpCompositeExtract %6 %89 1
         %92 = OpCompositeExtract %6 %89 2
         %93 = OpCompositeConstruct %19 %90 %91 %92 %49
         %94 = OpMatrixTimesVector %19 %85 %93
         %95 = OpAccessChain %60 %44 %45
               OpStore %95 %94
               OpBranch %17
         %17 = OpLabel
         %99 = OpAccessChain %25 %23 %63
        %100 = OpLoad %20 %99
        %101 = OpCompositeExtract %19 %100 0
        %102 = OpVectorShuffle %7 %101 %101 0 1 2
        %103 = OpCompositeExtract %19 %100 1
        %104 = OpVectorShuffle %7 %103 %103 0 1 2
        %105 = OpCompositeExtract %19 %100 2
        %106 = OpVectorShuffle %7 %105 %105 0 1 2
        %107 = OpCompositeConstruct %28 %102 %104 %106
        %109 = OpLoad %7 %108
        %110 = OpMatrixTimesVector %7 %107 %109
               OpStore %98 %110
        %113 = OpAccessChain %25 %23 %24
        %114 = OpLoad %20 %113
        %115 = OpExtInst %20 %1 MatrixInverse %114
               OpStore %112 %115
               OpStore %117 %121
        %123 = OpLoad %7 %117
        %124 = OpLoad %7 %18
        %125 = OpFSub %7 %123 %124
               OpStore %122 %125
        %127 = OpLoad %7 %18
        %128 = OpFNegate %7 %127
               OpStore %126 %128
               OpReturn
               OpFunctionEnd
