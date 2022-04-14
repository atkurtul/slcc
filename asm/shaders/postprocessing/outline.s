; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 142
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %42 %76 %125
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %12 "linearizeDepth(f1;f1;f1;"
               OpName %9 "depth"
               OpName %10 "near"
               OpName %11 "far"
               OpName %19 "getDepth(vi2;"
               OpName %18 "offset"
               OpName %34 "depth"
               OpName %38 "depth_sampler"
               OpName %42 "gl_FragCoord"
               OpName %55 "PostprocessingUniform"
               OpMemberName %55 0 "near_far"
               OpName %57 "postprocessing_uniform"
               OpName %58 "param"
               OpName %60 "param"
               OpName %64 "param"
               OpName %72 "color"
               OpName %73 "color_sampler"
               OpName %76 "in_uv"
               OpName %79 "depth"
               OpName %81 "param"
               OpName %85 "outline_color"
               OpName %89 "thickness"
               OpName %91 "outline"
               OpName %96 "param"
               OpName %102 "param"
               OpName %110 "param"
               OpName %119 "param"
               OpName %125 "o_color"
               OpDecorate %38 DescriptorSet 0
               OpDecorate %38 Binding 1
               OpDecorate %42 BuiltIn FragCoord
               OpMemberDecorate %55 0 Offset 0
               OpDecorate %55 Block
               OpDecorate %57 DescriptorSet 0
               OpDecorate %57 Binding 0
               OpDecorate %73 DescriptorSet 0
               OpDecorate %73 Binding 2
               OpDecorate %76 Location 0
               OpDecorate %125 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypePointer Function %6
          %8 = OpTypeFunction %6 %7 %7 %7
         %14 = OpTypeInt 32 1
         %15 = OpTypeVector %14 2
         %16 = OpTypePointer Function %15
         %17 = OpTypeFunction %6 %16
         %35 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %36 = OpTypeSampledImage %35
         %37 = OpTypePointer UniformConstant %36
         %38 = OpVariable %37 UniformConstant
         %40 = OpTypeVector %6 4
         %41 = OpTypePointer Input %40
         %42 = OpVariable %41 Input
         %43 = OpTypeVector %6 2
         %49 = OpConstant %14 0
         %52 = OpTypeInt 32 0
         %53 = OpConstant %52 0
         %55 = OpTypeStruct %43
         %56 = OpTypePointer Uniform %55
         %57 = OpVariable %56 Uniform
         %61 = OpTypePointer Uniform %6
         %65 = OpConstant %52 1
         %71 = OpTypePointer Function %40
         %73 = OpVariable %37 UniformConstant
         %75 = OpTypePointer Input %43
         %76 = OpVariable %75 Input
         %80 = OpConstantComposite %15 %49 %49
         %83 = OpTypeVector %6 3
         %84 = OpTypePointer Function %83
         %86 = OpConstant %6 0
         %87 = OpConstantComposite %83 %86 %86 %86
         %88 = OpTypePointer Function %14
         %90 = OpConstant %14 2
        %124 = OpTypePointer Output %40
        %125 = OpVariable %124 Output
        %130 = OpConstant %6 1
        %134 = OpTypePointer Output %6
        %139 = OpConstant %52 2
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %72 = OpVariable %71 Function
         %79 = OpVariable %7 Function
         %81 = OpVariable %16 Function
         %85 = OpVariable %84 Function
         %89 = OpVariable %88 Function
         %91 = OpVariable %7 Function
         %96 = OpVariable %16 Function
        %102 = OpVariable %16 Function
        %110 = OpVariable %16 Function
        %119 = OpVariable %16 Function
         %74 = OpLoad %36 %73
         %77 = OpLoad %43 %76
         %78 = OpImageSampleImplicitLod %40 %74 %77
               OpStore %72 %78
               OpStore %81 %80
         %82 = OpFunctionCall %6 %19 %81
               OpStore %79 %82
               OpStore %85 %87
               OpStore %89 %90
         %92 = OpLoad %6 %79
         %93 = OpLoad %14 %89
         %94 = OpSNegate %14 %93
         %95 = OpCompositeConstruct %15 %94 %49
               OpStore %96 %95
         %97 = OpFunctionCall %6 %19 %96
         %98 = OpFSub %6 %92 %97
               OpStore %91 %98
         %99 = OpLoad %6 %79
        %100 = OpLoad %14 %89
        %101 = OpCompositeConstruct %15 %49 %100
               OpStore %102 %101
        %103 = OpFunctionCall %6 %19 %102
        %104 = OpFSub %6 %99 %103
        %105 = OpLoad %6 %91
        %106 = OpFAdd %6 %105 %104
               OpStore %91 %106
        %107 = OpLoad %6 %79
        %108 = OpLoad %14 %89
        %109 = OpCompositeConstruct %15 %108 %49
               OpStore %110 %109
        %111 = OpFunctionCall %6 %19 %110
        %112 = OpFSub %6 %107 %111
        %113 = OpLoad %6 %91
        %114 = OpFAdd %6 %113 %112
               OpStore %91 %114
        %115 = OpLoad %6 %79
        %116 = OpLoad %14 %89
        %117 = OpSNegate %14 %116
        %118 = OpCompositeConstruct %15 %49 %117
               OpStore %119 %118
        %120 = OpFunctionCall %6 %19 %119
        %121 = OpFSub %6 %115 %120
        %122 = OpLoad %6 %91
        %123 = OpFAdd %6 %122 %121
               OpStore %91 %123
        %126 = OpLoad %40 %72
        %127 = OpVectorShuffle %83 %126 %126 0 1 2
        %128 = OpLoad %83 %85
        %129 = OpLoad %6 %91
        %131 = OpExtInst %6 %1 FClamp %129 %86 %130
        %132 = OpCompositeConstruct %83 %131 %131 %131
        %133 = OpExtInst %83 %1 FMix %127 %128 %132
        %135 = OpAccessChain %134 %125 %53
        %136 = OpCompositeExtract %6 %133 0
               OpStore %135 %136
        %137 = OpAccessChain %134 %125 %65
        %138 = OpCompositeExtract %6 %133 1
               OpStore %137 %138
        %140 = OpAccessChain %134 %125 %139
        %141 = OpCompositeExtract %6 %133 2
               OpStore %140 %141
               OpReturn
               OpFunctionEnd
         %12 = OpFunction %6 None %8
          %9 = OpFunctionParameter %7
         %10 = OpFunctionParameter %7
         %11 = OpFunctionParameter %7
         %13 = OpLabel
         %21 = OpLoad %6 %10
         %22 = OpLoad %6 %11
         %23 = OpFMul %6 %21 %22
         %24 = OpLoad %6 %11
         %25 = OpLoad %6 %9
         %26 = OpLoad %6 %10
         %27 = OpLoad %6 %11
         %28 = OpFSub %6 %26 %27
         %29 = OpFMul %6 %25 %28
         %30 = OpFAdd %6 %24 %29
         %31 = OpFDiv %6 %23 %30
               OpReturnValue %31
               OpFunctionEnd
         %19 = OpFunction %6 None %17
         %18 = OpFunctionParameter %16
         %20 = OpLabel
         %34 = OpVariable %7 Function
         %58 = OpVariable %7 Function
         %60 = OpVariable %7 Function
         %64 = OpVariable %7 Function
         %39 = OpLoad %36 %38
         %44 = OpLoad %40 %42
         %45 = OpVectorShuffle %43 %44 %44 0 1
         %46 = OpConvertFToS %15 %45
         %47 = OpLoad %15 %18
         %48 = OpIAdd %15 %46 %47
         %50 = OpImage %35 %39
         %51 = OpImageFetch %40 %50 %48 Lod %49
         %54 = OpCompositeExtract %6 %51 0
               OpStore %34 %54
         %59 = OpLoad %6 %34
               OpStore %58 %59
         %62 = OpAccessChain %61 %57 %49 %53
         %63 = OpLoad %6 %62
               OpStore %60 %63
         %66 = OpAccessChain %61 %57 %49 %65
         %67 = OpLoad %6 %66
               OpStore %64 %67
         %68 = OpFunctionCall %6 %12 %58 %60 %64
               OpReturnValue %68
               OpFunctionEnd
