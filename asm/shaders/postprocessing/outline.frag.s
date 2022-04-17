; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 142
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %gl_FragCoord %in_uv %o_color
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %linearizeDepth_f1_f1_f1_ "linearizeDepth(f1;f1;f1;"
               OpName %depth "depth"
               OpName %near "near"
               OpName %far "far"
               OpName %getDepth_vi2_ "getDepth(vi2;"
               OpName %offset "offset"
               OpName %depth_0 "depth"
               OpName %depth_sampler "depth_sampler"
               OpName %gl_FragCoord "gl_FragCoord"
               OpName %PostprocessingUniform "PostprocessingUniform"
               OpMemberName %PostprocessingUniform 0 "near_far"
               OpName %postprocessing_uniform "postprocessing_uniform"
               OpName %param "param"
               OpName %param_0 "param"
               OpName %param_1 "param"
               OpName %color "color"
               OpName %color_sampler "color_sampler"
               OpName %in_uv "in_uv"
               OpName %depth_1 "depth"
               OpName %param_2 "param"
               OpName %outline_color "outline_color"
               OpName %thickness "thickness"
               OpName %outline "outline"
               OpName %param_3 "param"
               OpName %param_4 "param"
               OpName %param_5 "param"
               OpName %param_6 "param"
               OpName %o_color "o_color"
               OpDecorate %depth_sampler DescriptorSet 0
               OpDecorate %depth_sampler Binding 1
               OpDecorate %gl_FragCoord BuiltIn FragCoord
               OpMemberDecorate %PostprocessingUniform 0 Offset 0
               OpDecorate %PostprocessingUniform Block
               OpDecorate %postprocessing_uniform DescriptorSet 0
               OpDecorate %postprocessing_uniform Binding 0
               OpDecorate %color_sampler DescriptorSet 0
               OpDecorate %color_sampler Binding 2
               OpDecorate %in_uv Location 0
               OpDecorate %o_color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
%_ptr_Function_float = OpTypePointer Function %float
          %8 = OpTypeFunction %float %_ptr_Function_float %_ptr_Function_float %_ptr_Function_float
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
         %17 = OpTypeFunction %float %_ptr_Function_v2int
         %35 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %36 = OpTypeSampledImage %35
%_ptr_UniformConstant_36 = OpTypePointer UniformConstant %36
%depth_sampler = OpVariable %_ptr_UniformConstant_36 UniformConstant
    %v4float = OpTypeVector %float 4
%_ptr_Input_v4float = OpTypePointer Input %v4float
%gl_FragCoord = OpVariable %_ptr_Input_v4float Input
    %v2float = OpTypeVector %float 2
      %int_0 = OpConstant %int 0
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%PostprocessingUniform = OpTypeStruct %v2float
%_ptr_Uniform_PostprocessingUniform = OpTypePointer Uniform %PostprocessingUniform
%postprocessing_uniform = OpVariable %_ptr_Uniform_PostprocessingUniform Uniform
%_ptr_Uniform_float = OpTypePointer Uniform %float
     %uint_1 = OpConstant %uint 1
%_ptr_Function_v4float = OpTypePointer Function %v4float
%color_sampler = OpVariable %_ptr_UniformConstant_36 UniformConstant
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
         %80 = OpConstantComposite %v2int %int_0 %int_0
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %float_0 = OpConstant %float 0
         %87 = OpConstantComposite %v3float %float_0 %float_0 %float_0
%_ptr_Function_int = OpTypePointer Function %int
      %int_2 = OpConstant %int 2
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
    %float_1 = OpConstant %float 1
%_ptr_Output_float = OpTypePointer Output %float
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
    %depth_1 = OpVariable %_ptr_Function_float Function
    %param_2 = OpVariable %_ptr_Function_v2int Function
%outline_color = OpVariable %_ptr_Function_v3float Function
  %thickness = OpVariable %_ptr_Function_int Function
    %outline = OpVariable %_ptr_Function_float Function
    %param_3 = OpVariable %_ptr_Function_v2int Function
    %param_4 = OpVariable %_ptr_Function_v2int Function
    %param_5 = OpVariable %_ptr_Function_v2int Function
    %param_6 = OpVariable %_ptr_Function_v2int Function
         %74 = OpLoad %36 %color_sampler
         %77 = OpLoad %v2float %in_uv
         %78 = OpImageSampleImplicitLod %v4float %74 %77
               OpStore %color %78
               OpStore %param_2 %80
         %82 = OpFunctionCall %float %getDepth_vi2_ %param_2
               OpStore %depth_1 %82
               OpStore %outline_color %87
               OpStore %thickness %int_2
         %92 = OpLoad %float %depth_1
         %93 = OpLoad %int %thickness
         %94 = OpSNegate %int %93
         %95 = OpCompositeConstruct %v2int %94 %int_0
               OpStore %param_3 %95
         %97 = OpFunctionCall %float %getDepth_vi2_ %param_3
         %98 = OpFSub %float %92 %97
               OpStore %outline %98
         %99 = OpLoad %float %depth_1
        %100 = OpLoad %int %thickness
        %101 = OpCompositeConstruct %v2int %int_0 %100
               OpStore %param_4 %101
        %103 = OpFunctionCall %float %getDepth_vi2_ %param_4
        %104 = OpFSub %float %99 %103
        %105 = OpLoad %float %outline
        %106 = OpFAdd %float %105 %104
               OpStore %outline %106
        %107 = OpLoad %float %depth_1
        %108 = OpLoad %int %thickness
        %109 = OpCompositeConstruct %v2int %108 %int_0
               OpStore %param_5 %109
        %111 = OpFunctionCall %float %getDepth_vi2_ %param_5
        %112 = OpFSub %float %107 %111
        %113 = OpLoad %float %outline
        %114 = OpFAdd %float %113 %112
               OpStore %outline %114
        %115 = OpLoad %float %depth_1
        %116 = OpLoad %int %thickness
        %117 = OpSNegate %int %116
        %118 = OpCompositeConstruct %v2int %int_0 %117
               OpStore %param_6 %118
        %120 = OpFunctionCall %float %getDepth_vi2_ %param_6
        %121 = OpFSub %float %115 %120
        %122 = OpLoad %float %outline
        %123 = OpFAdd %float %122 %121
               OpStore %outline %123
        %126 = OpLoad %v4float %color
        %127 = OpVectorShuffle %v3float %126 %126 0 1 2
        %128 = OpLoad %v3float %outline_color
        %129 = OpLoad %float %outline
        %131 = OpExtInst %float %1 FClamp %129 %float_0 %float_1
        %132 = OpCompositeConstruct %v3float %131 %131 %131
        %133 = OpExtInst %v3float %1 FMix %127 %128 %132
        %135 = OpAccessChain %_ptr_Output_float %o_color %uint_0
        %136 = OpCompositeExtract %float %133 0
               OpStore %135 %136
        %137 = OpAccessChain %_ptr_Output_float %o_color %uint_1
        %138 = OpCompositeExtract %float %133 1
               OpStore %137 %138
        %140 = OpAccessChain %_ptr_Output_float %o_color %uint_2
        %141 = OpCompositeExtract %float %133 2
               OpStore %140 %141
               OpReturn
               OpFunctionEnd
%linearizeDepth_f1_f1_f1_ = OpFunction %float None %8
      %depth = OpFunctionParameter %_ptr_Function_float
       %near = OpFunctionParameter %_ptr_Function_float
        %far = OpFunctionParameter %_ptr_Function_float
         %13 = OpLabel
         %21 = OpLoad %float %near
         %22 = OpLoad %float %far
         %23 = OpFMul %float %21 %22
         %24 = OpLoad %float %far
         %25 = OpLoad %float %depth
         %26 = OpLoad %float %near
         %27 = OpLoad %float %far
         %28 = OpFSub %float %26 %27
         %29 = OpFMul %float %25 %28
         %30 = OpFAdd %float %24 %29
         %31 = OpFDiv %float %23 %30
               OpReturnValue %31
               OpFunctionEnd
%getDepth_vi2_ = OpFunction %float None %17
     %offset = OpFunctionParameter %_ptr_Function_v2int
         %20 = OpLabel
    %depth_0 = OpVariable %_ptr_Function_float Function
      %param = OpVariable %_ptr_Function_float Function
    %param_0 = OpVariable %_ptr_Function_float Function
    %param_1 = OpVariable %_ptr_Function_float Function
         %39 = OpLoad %36 %depth_sampler
         %44 = OpLoad %v4float %gl_FragCoord
         %45 = OpVectorShuffle %v2float %44 %44 0 1
         %46 = OpConvertFToS %v2int %45
         %47 = OpLoad %v2int %offset
         %48 = OpIAdd %v2int %46 %47
         %50 = OpImage %35 %39
         %51 = OpImageFetch %v4float %50 %48 Lod %int_0
         %54 = OpCompositeExtract %float %51 0
               OpStore %depth_0 %54
         %59 = OpLoad %float %depth_0
               OpStore %param %59
         %62 = OpAccessChain %_ptr_Uniform_float %postprocessing_uniform %int_0 %uint_0
         %63 = OpLoad %float %62
               OpStore %param_0 %63
         %66 = OpAccessChain %_ptr_Uniform_float %postprocessing_uniform %int_0 %uint_1
         %67 = OpLoad %float %66
               OpStore %param_1 %67
         %68 = OpFunctionCall %float %linearizeDepth_f1_f1_f1_ %param %param_0 %param_1
               OpReturnValue %68
               OpFunctionEnd
