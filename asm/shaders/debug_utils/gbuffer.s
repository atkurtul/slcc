; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 149
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %21 %38 %50 %111 %113 %115 %125 %142 %146
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "Push_Constants"
               OpMemberName %9 0 "offset"
               OpMemberName %9 1 "color"
               OpMemberName %9 2 "object_type"
               OpName %11 "push_constants"
               OpName %21 "outPos"
               OpName %23 "UBO"
               OpMemberName %23 0 "projection"
               OpMemberName %23 1 "modelview"
               OpMemberName %23 2 "skybox_modelview"
               OpMemberName %23 3 "modelscale"
               OpName %25 "ubo"
               OpName %38 "inPos"
               OpName %48 "gl_PerVertex"
               OpMemberName %48 0 "gl_Position"
               OpMemberName %48 1 "gl_PointSize"
               OpMemberName %48 2 "gl_ClipDistance"
               OpMemberName %48 3 "gl_CullDistance"
               OpName %50 ""
               OpName %70 "localPos"
               OpName %111 "outUV"
               OpName %113 "inUV"
               OpName %115 "outNormal"
               OpName %125 "inNormal"
               OpName %128 "lightPos"
               OpName %142 "outLightVec"
               OpName %146 "outViewVec"
               OpMemberDecorate %9 0 Offset 0
               OpMemberDecorate %9 1 Offset 16
               OpMemberDecorate %9 2 Offset 32
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
               OpDecorate %38 Location 0
               OpMemberDecorate %48 0 BuiltIn Position
               OpMemberDecorate %48 1 BuiltIn PointSize
               OpMemberDecorate %48 2 BuiltIn ClipDistance
               OpMemberDecorate %48 3 BuiltIn CullDistance
               OpDecorate %48 Block
               OpDecorate %111 Location 0
               OpDecorate %113 Location 2
               OpDecorate %115 Location 2
               OpDecorate %125 Location 1
               OpDecorate %142 Location 4
               OpDecorate %146 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypeInt 32 1
          %9 = OpTypeStruct %7 %7 %8
         %10 = OpTypePointer PushConstant %9
         %11 = OpVariable %10 PushConstant
         %12 = OpConstant %8 2
         %13 = OpTypePointer PushConstant %8
         %19 = OpTypeVector %6 3
         %20 = OpTypePointer Output %19
         %21 = OpVariable %20 Output
         %22 = OpTypeMatrix %7 4
         %23 = OpTypeStruct %22 %22 %22 %6
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpTypePointer Uniform %22
         %29 = OpTypeMatrix %19 3
         %37 = OpTypePointer Input %19
         %38 = OpVariable %37 Input
         %45 = OpTypeInt 32 0
         %46 = OpConstant %45 1
         %47 = OpTypeArray %6 %46
         %48 = OpTypeStruct %7 %6 %47 %47
         %49 = OpTypePointer Output %48
         %50 = OpVariable %49 Output
         %51 = OpConstant %8 0
         %55 = OpConstant %6 1
         %66 = OpTypePointer Output %7
         %69 = OpTypePointer Function %19
         %72 = OpConstant %8 3
         %73 = OpTypePointer Uniform %6
         %77 = OpTypePointer PushConstant %7
         %82 = OpConstant %8 1
        %109 = OpTypeVector %6 2
        %110 = OpTypePointer Output %109
        %111 = OpVariable %110 Output
        %112 = OpTypePointer Input %109
        %113 = OpVariable %112 Input
        %115 = OpVariable %20 Output
        %125 = OpVariable %37 Input
        %138 = OpConstant %6 0
        %139 = OpConstant %6 -10
        %140 = OpConstantComposite %19 %138 %139 %139
        %142 = OpVariable %20 Output
        %146 = OpVariable %20 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %70 = OpVariable %69 Function
        %128 = OpVariable %69 Function
         %14 = OpAccessChain %13 %11 %12
         %15 = OpLoad %8 %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %27 = OpAccessChain %26 %25 %12
         %28 = OpLoad %22 %27
         %30 = OpCompositeExtract %7 %28 0
         %31 = OpVectorShuffle %19 %30 %30 0 1 2
         %32 = OpCompositeExtract %7 %28 1
         %33 = OpVectorShuffle %19 %32 %32 0 1 2
         %34 = OpCompositeExtract %7 %28 2
         %35 = OpVectorShuffle %19 %34 %34 0 1 2
         %36 = OpCompositeConstruct %29 %31 %33 %35
         %39 = OpLoad %19 %38
         %40 = OpMatrixTimesVector %19 %36 %39
         %41 = OpCompositeExtract %6 %40 0
         %42 = OpCompositeExtract %6 %40 1
         %43 = OpCompositeExtract %6 %40 2
         %44 = OpCompositeConstruct %19 %41 %42 %43
               OpStore %21 %44
         %52 = OpAccessChain %26 %25 %51
         %53 = OpLoad %22 %52
         %54 = OpLoad %19 %21
         %56 = OpCompositeExtract %6 %54 0
         %57 = OpCompositeExtract %6 %54 1
         %58 = OpCompositeExtract %6 %54 2
         %59 = OpCompositeConstruct %7 %56 %57 %58 %55
         %60 = OpMatrixTimesVector %7 %53 %59
         %61 = OpCompositeExtract %6 %60 0
         %62 = OpCompositeExtract %6 %60 1
         %63 = OpCompositeExtract %6 %60 2
         %64 = OpCompositeExtract %6 %60 3
         %65 = OpCompositeConstruct %7 %61 %62 %63 %64
         %67 = OpAccessChain %66 %50 %51
               OpStore %67 %65
               OpBranch %18
         %17 = OpLabel
         %71 = OpLoad %19 %38
         %74 = OpAccessChain %73 %25 %72
         %75 = OpLoad %6 %74
         %76 = OpVectorTimesScalar %19 %71 %75
         %78 = OpAccessChain %77 %11 %51
         %79 = OpLoad %7 %78
         %80 = OpVectorShuffle %19 %79 %79 0 1 2
         %81 = OpFAdd %19 %76 %80
               OpStore %70 %81
         %83 = OpAccessChain %26 %25 %82
         %84 = OpLoad %22 %83
         %85 = OpLoad %19 %70
         %86 = OpCompositeExtract %6 %85 0
         %87 = OpCompositeExtract %6 %85 1
         %88 = OpCompositeExtract %6 %85 2
         %89 = OpCompositeConstruct %7 %86 %87 %88 %55
         %90 = OpMatrixTimesVector %7 %84 %89
         %91 = OpCompositeExtract %6 %90 0
         %92 = OpCompositeExtract %6 %90 1
         %93 = OpCompositeExtract %6 %90 2
         %94 = OpCompositeConstruct %19 %91 %92 %93
               OpStore %21 %94
         %95 = OpAccessChain %26 %25 %51
         %96 = OpLoad %22 %95
         %97 = OpAccessChain %26 %25 %82
         %98 = OpLoad %22 %97
         %99 = OpMatrixTimesMatrix %22 %96 %98
        %100 = OpLoad %19 %70
        %101 = OpCompositeExtract %6 %100 0
        %102 = OpCompositeExtract %6 %100 1
        %103 = OpCompositeExtract %6 %100 2
        %104 = OpCompositeConstruct %7 %101 %102 %103 %55
        %105 = OpMatrixTimesVector %7 %99 %104
        %106 = OpAccessChain %66 %50 %51
               OpStore %106 %105
               OpBranch %18
         %18 = OpLabel
        %114 = OpLoad %109 %113
               OpStore %111 %114
        %116 = OpAccessChain %26 %25 %82
        %117 = OpLoad %22 %116
        %118 = OpCompositeExtract %7 %117 0
        %119 = OpVectorShuffle %19 %118 %118 0 1 2
        %120 = OpCompositeExtract %7 %117 1
        %121 = OpVectorShuffle %19 %120 %120 0 1 2
        %122 = OpCompositeExtract %7 %117 2
        %123 = OpVectorShuffle %19 %122 %122 0 1 2
        %124 = OpCompositeConstruct %29 %119 %121 %123
        %126 = OpLoad %19 %125
        %127 = OpMatrixTimesVector %19 %124 %126
               OpStore %115 %127
        %129 = OpAccessChain %26 %25 %82
        %130 = OpLoad %22 %129
        %131 = OpCompositeExtract %7 %130 0
        %132 = OpVectorShuffle %19 %131 %131 0 1 2
        %133 = OpCompositeExtract %7 %130 1
        %134 = OpVectorShuffle %19 %133 %133 0 1 2
        %135 = OpCompositeExtract %7 %130 2
        %136 = OpVectorShuffle %19 %135 %135 0 1 2
        %137 = OpCompositeConstruct %29 %132 %134 %136
        %141 = OpMatrixTimesVector %19 %137 %140
               OpStore %128 %141
        %143 = OpLoad %19 %128
        %144 = OpLoad %19 %21
        %145 = OpFSub %19 %143 %144
               OpStore %142 %145
        %147 = OpLoad %19 %21
        %148 = OpFNegate %19 %147
               OpStore %146 %148
               OpReturn
               OpFunctionEnd
