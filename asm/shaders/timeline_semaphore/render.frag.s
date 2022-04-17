; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 55
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %out_color %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %convert_color_vf4_ "convert_color(vf4;"
               OpName %value "value"
               OpName %gray "gray"
               OpName %out_color "out_color"
               OpName %in_image "in_image"
               OpName %in_uv "in_uv"
               OpName %param "param"
               OpDecorate %out_color Location 0
               OpDecorate %in_image DescriptorSet 0
               OpDecorate %in_image Binding 0
               OpDecorate %in_uv Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
          %9 = OpTypeFunction %v4float %_ptr_Function_v4float
%_ptr_Function_float = OpTypePointer Function %float
    %v3float = OpTypeVector %float 3
%float_0_300000012 = OpConstant %float 0.300000012
%float_0_600000024 = OpConstant %float 0.600000024
%float_0_100000001 = OpConstant %float 0.100000001
         %21 = OpConstantComposite %v3float %float_0_300000012 %float_0_600000024 %float_0_100000001
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %out_color = OpVariable %_ptr_Output_v4float Output
         %42 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %43 = OpTypeSampledImage %42
%_ptr_UniformConstant_43 = OpTypePointer UniformConstant %43
   %in_image = OpVariable %_ptr_UniformConstant_43 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
    %float_0 = OpConstant %float 0
       %main = OpFunction %void None %3
          %5 = OpLabel
      %param = OpVariable %_ptr_Function_v4float Function
         %46 = OpLoad %43 %in_image
         %50 = OpLoad %v2float %in_uv
         %52 = OpImageSampleExplicitLod %v4float %46 %50 Lod %float_0
               OpStore %param %52
         %54 = OpFunctionCall %v4float %convert_color_vf4_ %param
               OpStore %out_color %54
               OpReturn
               OpFunctionEnd
%convert_color_vf4_ = OpFunction %v4float None %9
      %value = OpFunctionParameter %_ptr_Function_v4float
         %12 = OpLabel
       %gray = OpVariable %_ptr_Function_float Function
         %16 = OpLoad %v4float %value
         %17 = OpVectorShuffle %v3float %16 %16 0 1 2
         %22 = OpDot %float %17 %21
               OpStore %gray %22
         %23 = OpLoad %float %gray
         %24 = OpCompositeConstruct %v3float %23 %23 %23
         %25 = OpLoad %v4float %value
         %26 = OpVectorShuffle %v3float %25 %25 0 1 2
         %29 = OpAccessChain %_ptr_Function_float %value %uint_3
         %30 = OpLoad %float %29
         %31 = OpCompositeConstruct %v3float %30 %30 %30
         %32 = OpExtInst %v3float %1 FMix %24 %26 %31
         %34 = OpCompositeExtract %float %32 0
         %35 = OpCompositeExtract %float %32 1
         %36 = OpCompositeExtract %float %32 2
         %37 = OpCompositeConstruct %v4float %34 %35 %36 %float_1
               OpReturnValue %37
               OpFunctionEnd
