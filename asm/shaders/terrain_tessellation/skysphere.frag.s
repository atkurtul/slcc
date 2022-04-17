; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 30
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
               OpName %samplerColorMap "samplerColorMap"
               OpName %inUV "inUV"
               OpName %outFragColor "outFragColor"
               OpDecorate %samplerColorMap DescriptorSet 0
               OpDecorate %samplerColorMap Binding 1
               OpDecorate %inUV Location 0
               OpDecorate %outFragColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
%samplerColorMap = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
         %14 = OpLoad %11 %samplerColorMap
         %18 = OpLoad %v2float %inUV
         %19 = OpImageSampleImplicitLod %v4float %14 %18
               OpStore %color %19
         %23 = OpLoad %v4float %color
         %24 = OpVectorShuffle %v3float %23 %23 0 1 2
         %26 = OpCompositeExtract %float %24 0
         %27 = OpCompositeExtract %float %24 1
         %28 = OpCompositeExtract %float %24 2
         %29 = OpCompositeConstruct %v4float %26 %27 %28 %float_1
               OpStore %outFragColor %29
               OpReturn
               OpFunctionEnd
