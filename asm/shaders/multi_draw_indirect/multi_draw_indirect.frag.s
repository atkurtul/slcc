; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 51
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %o_color %in_texture_index %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_color "o_color"
               OpName %textures "textures"
               OpName %in_texture_index "in_texture_index"
               OpName %in_uv "in_uv"
               OpDecorate %o_color Location 0
               OpDecorate %textures DescriptorSet 0
               OpDecorate %textures Binding 1
               OpDecorate %in_texture_index Flat
               OpDecorate %in_texture_index Location 2
               OpDecorate %in_uv Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
       %uint = OpTypeInt 32 0
   %uint_225 = OpConstant %uint 225
%_arr_11_uint_225 = OpTypeArray %11 %uint_225
%_ptr_UniformConstant__arr_11_uint_225 = OpTypePointer UniformConstant %_arr_11_uint_225
   %textures = OpVariable %_ptr_UniformConstant__arr_11_uint_225 UniformConstant
%_ptr_Input_uint = OpTypePointer Input %uint
%in_texture_index = OpVariable %_ptr_Input_uint Input
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
  %float_1_5 = OpConstant %float 1.5
    %v3float = OpTypeVector %float 3
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
         %19 = OpLoad %uint %in_texture_index
         %20 = OpConvertUToF %float %19
         %21 = OpExtInst %float %1 Round %20
         %22 = OpConvertFToU %uint %21
         %24 = OpAccessChain %_ptr_UniformConstant_11 %textures %22
         %25 = OpLoad %11 %24
         %29 = OpLoad %v2float %in_uv
         %30 = OpImageSampleImplicitLod %v4float %25 %29
         %31 = OpCompositeExtract %float %30 0
         %32 = OpCompositeExtract %float %30 1
         %33 = OpCompositeExtract %float %30 2
         %34 = OpCompositeExtract %float %30 3
         %35 = OpCompositeConstruct %v4float %31 %32 %33 %34
               OpStore %o_color %35
         %38 = OpLoad %v4float %o_color
         %39 = OpVectorShuffle %v3float %38 %38 0 1 2
         %40 = OpVectorTimesScalar %v3float %39 %float_1_5
         %43 = OpAccessChain %_ptr_Output_float %o_color %uint_0
         %44 = OpCompositeExtract %float %40 0
               OpStore %43 %44
         %46 = OpAccessChain %_ptr_Output_float %o_color %uint_1
         %47 = OpCompositeExtract %float %40 1
               OpStore %46 %47
         %49 = OpAccessChain %_ptr_Output_float %o_color %uint_2
         %50 = OpCompositeExtract %float %40 2
               OpStore %49 %50
               OpReturn
               OpFunctionEnd
