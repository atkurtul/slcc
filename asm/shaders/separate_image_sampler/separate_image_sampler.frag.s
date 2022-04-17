; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 28
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %outFragColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %color "color"
               OpName %_texture "_texture"
               OpName %_sampler "_sampler"
               OpName %inUV "inUV"
               OpName %outFragColor "outFragColor"
               OpDecorate %_texture DescriptorSet 0
               OpDecorate %_texture Binding 1
               OpDecorate %_sampler DescriptorSet 1
               OpDecorate %_sampler Binding 0
               OpDecorate %inUV Location 0
               OpDecorate %outFragColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
%_ptr_UniformConstant_10 = OpTypePointer UniformConstant %10
   %_texture = OpVariable %_ptr_UniformConstant_10 UniformConstant
         %14 = OpTypeSampler
%_ptr_UniformConstant_14 = OpTypePointer UniformConstant %14
   %_sampler = OpVariable %_ptr_UniformConstant_14 UniformConstant
         %18 = OpTypeSampledImage %10
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
         %13 = OpLoad %10 %_texture
         %17 = OpLoad %14 %_sampler
         %19 = OpSampledImage %18 %13 %17
         %23 = OpLoad %v2float %inUV
         %24 = OpImageSampleImplicitLod %v4float %19 %23
               OpStore %color %24
         %27 = OpLoad %v4float %color
               OpStore %outFragColor %27
               OpReturn
               OpFunctionEnd
