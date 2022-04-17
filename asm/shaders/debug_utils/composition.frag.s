; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 21
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outColor %inUV
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outColor "outColor"
               OpName %samplerColor0 "samplerColor0"
               OpName %inUV "inUV"
               OpName %samplerColor1 "samplerColor1"
               OpDecorate %outColor Location 0
               OpDecorate %samplerColor0 DescriptorSet 0
               OpDecorate %samplerColor0 Binding 0
               OpDecorate %inUV Location 0
               OpDecorate %samplerColor1 DescriptorSet 0
               OpDecorate %samplerColor1 Binding 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %outColor = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
%samplerColor0 = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%samplerColor1 = OpVariable %_ptr_UniformConstant_11 UniformConstant
       %main = OpFunction %void None %3
          %5 = OpLabel
         %14 = OpLoad %11 %samplerColor0
         %18 = OpLoad %v2float %inUV
         %19 = OpImageSampleImplicitLod %v4float %14 %18
               OpStore %outColor %19
               OpReturn
               OpFunctionEnd
