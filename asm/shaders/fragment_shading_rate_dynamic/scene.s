; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 144
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %21 %39 %51 %106 %108 %110 %120 %137 %141
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "Push_Constants"
               OpMemberName %9 0 "offset"
               OpMemberName %9 1 "object_type"
               OpName %11 "push_constants"
               OpName %21 "outPos"
               OpName %23 "UBO"
               OpMemberName %23 0 "projection"
               OpMemberName %23 1 "modelview"
               OpMemberName %23 2 "skybox_modelview"
               OpMemberName %23 3 "color_shading_rates"
               OpName %25 "ubo"
               OpName %39 "inPos"
               OpName %49 "gl_PerVertex"
               OpMemberName %49 0 "gl_Position"
               OpMemberName %49 1 "gl_PointSize"
               OpMemberName %49 2 "gl_ClipDistance"
               OpMemberName %49 3 "gl_CullDistance"
               OpName %51 ""
               OpName %71 "localPos"
               OpName %106 "outUV"
               OpName %108 "inUV"
               OpName %110 "outNormal"
               OpName %120 "inNormal"
               OpName %123 "lightPos"
               OpName %137 "outLightVec"
               OpName %141 "outViewVec"
               OpMemberDecorate %9 0 Offset 0
               OpMemberDecorate %9 1 Offset 16
               OpDecorate %9 Block
               OpDecorate %21 Location 1
               OpMemberDecorate %23 0 ColMajor
               OpMemberDecorate %23 0 Offset 0
               OpMemberDecorate %23 0 MatrixStride 16
               OpMemberDecorate %23 1 ColMajor
               OpMemberDecorate %23 1 Offset 64
               OpMemberDecorate %23 1 MatrixStride 16
               OpMemberDecorate %23 2 ColMajor
               OpMemberDecorate %23 2 Offset 128
               OpMemberDecorate %23 2 MatrixStride 16
               OpMemberDecorate %23 3 Offset 192
               OpDecorate %23 Block
               OpDecorate %25 DescriptorSet 0
               OpDecorate %25 Binding 0
               OpDecorate %39 Location 0
               OpMemberDecorate %49 0 BuiltIn Position
               OpMemberDecorate %49 1 BuiltIn PointSize
               OpMemberDecorate %49 2 BuiltIn ClipDistance
               OpMemberDecorate %49 3 BuiltIn CullDistance
               OpDecorate %49 Block
               OpDecorate %106 Location 0
               OpDecorate %108 Location 2
               OpDecorate %110 Location 2
               OpDecorate %120 Location 1
               OpDecorate %137 Location 4
               OpDecorate %141 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypeInt 32 1
          %9 = OpTypeStruct %7 %8
         %10 = OpTypePointer PushConstant %9
         %11 = OpVariable %10 PushConstant
         %12 = OpConstant %8 1
         %13 = OpTypePointer PushConstant %8
         %19 = OpTypeVector %6 3
         %20 = OpTypePointer Output %19
         %21 = OpVariable %20 Output
         %22 = OpTypeMatrix %7 4
         %23 = OpTypeStruct %22 %22 %22 %8
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpConstant %8 2
         %27 = OpTypePointer Uniform %22
         %30 = OpTypeMatrix %19 3
         %38 = OpTypePointer Input %19
         %39 = OpVariable %38 Input
         %46 = OpTypeInt 32 0
         %47 = OpConstant %46 1
         %48 = OpTypeArray %6 %47
         %49 = OpTypeStruct %7 %6 %48 %48
         %50 = OpTypePointer Output %49
         %51 = OpVariable %50 Output
         %52 = OpConstant %8 0
         %56 = OpConstant %6 1
         %67 = OpTypePointer Output %7
         %70 = OpTypePointer Function %19
         %73 = OpTypePointer PushConstant %7
        %104 = OpTypeVector %6 2
        %105 = OpTypePointer Output %104
        %106 = OpVariable %105 Output
        %107 = OpTypePointer Input %104
        %108 = OpVariable %107 Input
        %110 = OpVariable %20 Output
        %120 = OpVariable %38 Input
        %133 = OpConstant %6 0
        %134 = OpConstant %6 -10
        %135 = OpConstantComposite %19 %133 %134 %134
        %137 = OpVariable %20 Output
        %141 = OpVariable %20 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %71 = OpVariable %70 Function
        %123 = OpVariable %70 Function
         %14 = OpAccessChain %13 %11 %12
         %15 = OpLoad %8 %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %28 = OpAccessChain %27 %25 %26
         %29 = OpLoad %22 %28
         %31 = OpCompositeExtract %7 %29 0
         %32 = OpVectorShuffle %19 %31 %31 0 1 2
         %33 = OpCompositeExtract %7 %29 1
         %34 = OpVectorShuffle %19 %33 %33 0 1 2
         %35 = OpCompositeExtract %7 %29 2
         %36 = OpVectorShuffle %19 %35 %35 0 1 2
         %37 = OpCompositeConstruct %30 %32 %34 %36
         %40 = OpLoad %19 %39
         %41 = OpMatrixTimesVector %19 %37 %40
         %42 = OpCompositeExtract %6 %41 0
         %43 = OpCompositeExtract %6 %41 1
         %44 = OpCompositeExtract %6 %41 2
         %45 = OpCompositeConstruct %19 %42 %43 %44
               OpStore %21 %45
         %53 = OpAccessChain %27 %25 %52
         %54 = OpLoad %22 %53
         %55 = OpLoad %19 %21
         %57 = OpCompositeExtract %6 %55 0
         %58 = OpCompositeExtract %6 %55 1
         %59 = OpCompositeExtract %6 %55 2
         %60 = OpCompositeConstruct %7 %57 %58 %59 %56
         %61 = OpMatrixTimesVector %7 %54 %60
         %62 = OpCompositeExtract %6 %61 0
         %63 = OpCompositeExtract %6 %61 1
         %64 = OpCompositeExtract %6 %61 2
         %65 = OpCompositeExtract %6 %61 3
         %66 = OpCompositeConstruct %7 %62 %63 %64 %65
         %68 = OpAccessChain %67 %51 %52
               OpStore %68 %66
               OpBranch %18
         %17 = OpLabel
         %72 = OpLoad %19 %39
         %74 = OpAccessChain %73 %11 %52
         %75 = OpLoad %7 %74
         %76 = OpVectorShuffle %19 %75 %75 0 1 2
         %77 = OpFAdd %19 %72 %76
               OpStore %71 %77
         %78 = OpAccessChain %27 %25 %12
         %79 = OpLoad %22 %78
         %80 = OpLoad %19 %71
         %81 = OpCompositeExtract %6 %80 0
         %82 = OpCompositeExtract %6 %80 1
         %83 = OpCompositeExtract %6 %80 2
         %84 = OpCompositeConstruct %7 %81 %82 %83 %56
         %85 = OpMatrixTimesVector %7 %79 %84
         %86 = OpCompositeExtract %6 %85 0
         %87 = OpCompositeExtract %6 %85 1
         %88 = OpCompositeExtract %6 %85 2
         %89 = OpCompositeConstruct %19 %86 %87 %88
               OpStore %21 %89
         %90 = OpAccessChain %27 %25 %52
         %91 = OpLoad %22 %90
         %92 = OpAccessChain %27 %25 %12
         %93 = OpLoad %22 %92
         %94 = OpMatrixTimesMatrix %22 %91 %93
         %95 = OpLoad %19 %71
         %96 = OpCompositeExtract %6 %95 0
         %97 = OpCompositeExtract %6 %95 1
         %98 = OpCompositeExtract %6 %95 2
         %99 = OpCompositeConstruct %7 %96 %97 %98 %56
        %100 = OpMatrixTimesVector %7 %94 %99
        %101 = OpAccessChain %67 %51 %52
               OpStore %101 %100
               OpBranch %18
         %18 = OpLabel
        %109 = OpLoad %104 %108
               OpStore %106 %109
        %111 = OpAccessChain %27 %25 %12
        %112 = OpLoad %22 %111
        %113 = OpCompositeExtract %7 %112 0
        %114 = OpVectorShuffle %19 %113 %113 0 1 2
        %115 = OpCompositeExtract %7 %112 1
        %116 = OpVectorShuffle %19 %115 %115 0 1 2
        %117 = OpCompositeExtract %7 %112 2
        %118 = OpVectorShuffle %19 %117 %117 0 1 2
        %119 = OpCompositeConstruct %30 %114 %116 %118
        %121 = OpLoad %19 %120
        %122 = OpMatrixTimesVector %19 %119 %121
               OpStore %110 %122
        %124 = OpAccessChain %27 %25 %12
        %125 = OpLoad %22 %124
        %126 = OpCompositeExtract %7 %125 0
        %127 = OpVectorShuffle %19 %126 %126 0 1 2
        %128 = OpCompositeExtract %7 %125 1
        %129 = OpVectorShuffle %19 %128 %128 0 1 2
        %130 = OpCompositeExtract %7 %125 2
        %131 = OpVectorShuffle %19 %130 %130 0 1 2
        %132 = OpCompositeConstruct %30 %127 %129 %131
        %136 = OpMatrixTimesVector %19 %132 %135
               OpStore %123 %136
        %138 = OpLoad %19 %123
        %139 = OpLoad %19 %21
        %140 = OpFSub %19 %138 %139
               OpStore %137 %140
        %142 = OpLoad %19 %21
        %143 = OpFNegate %19 %142
               OpStore %141 %143
               OpReturn
               OpFunctionEnd
