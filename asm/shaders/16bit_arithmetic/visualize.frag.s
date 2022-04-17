; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 28
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %o_color %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_color "o_color"
               OpName %tex "tex"
               OpName %in_uv "in_uv"
               OpDecorate %o_color Location 0
               OpDecorate %tex DescriptorSet 0
               OpDecorate %tex Binding 0
               OpDecorate %in_uv Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
        %tex = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
    %float_0 = OpConstant %float 0
    %v3float = OpTypeVector %float 3
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %14 = OpLoad %11 %tex
         %18 = OpLoad %v2float %in_uv
         %20 = OpImageSampleExplicitLod %v4float %14 %18 Lod %float_0
         %22 = OpVectorShuffle %v3float %20 %20 0 1 2
         %24 = OpCompositeExtract %float %22 0
         %25 = OpCompositeExtract %float %22 1
         %26 = OpCompositeExtract %float %22 2
         %27 = OpCompositeConstruct %v4float %24 %25 %26 %float_1
               OpStore %o_color %27
               OpReturn
               OpFunctionEnd
