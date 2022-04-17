; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 24
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outColor %inColor %inUV
               OpExecutionMode %main OriginUpperLeft
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outColor "outColor"
               OpName %inColor "inColor"
               OpName %fontSampler "fontSampler"
               OpName %inUV "inUV"
               OpDecorate %outColor RelaxedPrecision
               OpDecorate %outColor Location 0
               OpDecorate %inColor RelaxedPrecision
               OpDecorate %inColor Location 1
               OpDecorate %12 RelaxedPrecision
               OpDecorate %fontSampler RelaxedPrecision
               OpDecorate %fontSampler DescriptorSet 0
               OpDecorate %fontSampler Binding 0
               OpDecorate %17 RelaxedPrecision
               OpDecorate %inUV RelaxedPrecision
               OpDecorate %inUV Location 0
               OpDecorate %21 RelaxedPrecision
               OpDecorate %22 RelaxedPrecision
               OpDecorate %23 RelaxedPrecision
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %outColor = OpVariable %_ptr_Output_v4float Output
%_ptr_Input_v4float = OpTypePointer Input %v4float
    %inColor = OpVariable %_ptr_Input_v4float Input
         %13 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %14 = OpTypeSampledImage %13
%_ptr_UniformConstant_14 = OpTypePointer UniformConstant %14
%fontSampler = OpVariable %_ptr_UniformConstant_14 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %12 = OpLoad %v4float %inColor
         %17 = OpLoad %14 %fontSampler
         %21 = OpLoad %v2float %inUV
         %22 = OpImageSampleImplicitLod %v4float %17 %21
         %23 = OpFMul %v4float %12 %22
               OpStore %outColor %23
               OpReturn
               OpFunctionEnd
