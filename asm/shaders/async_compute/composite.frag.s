; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 42
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %in_uv %o_color
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %col "col"
               OpName %hdr_tex "hdr_tex"
               OpName %in_uv "in_uv"
               OpName %bloom_tex "bloom_tex"
               OpName %o_color "o_color"
               OpDecorate %hdr_tex DescriptorSet 0
               OpDecorate %hdr_tex Binding 0
               OpDecorate %in_uv Location 0
               OpDecorate %bloom_tex DescriptorSet 0
               OpDecorate %bloom_tex Binding 1
               OpDecorate %o_color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
    %hdr_tex = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
    %float_0 = OpConstant %float 0
    %v4float = OpTypeVector %float 4
  %bloom_tex = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
        %col = OpVariable %_ptr_Function_v3float Function
         %14 = OpLoad %11 %hdr_tex
         %18 = OpLoad %v2float %in_uv
         %21 = OpImageSampleExplicitLod %v4float %14 %18 Lod %float_0
         %22 = OpVectorShuffle %v3float %21 %21 0 1 2
         %24 = OpLoad %11 %bloom_tex
         %25 = OpLoad %v2float %in_uv
         %26 = OpImageSampleExplicitLod %v4float %24 %25 Lod %float_0
         %27 = OpVectorShuffle %v3float %26 %26 0 1 2
         %28 = OpFAdd %v3float %22 %27
               OpStore %col %28
         %29 = OpLoad %v3float %col
         %31 = OpLoad %v3float %col
         %32 = OpCompositeConstruct %v3float %float_1 %float_1 %float_1
         %33 = OpFAdd %v3float %32 %31
         %34 = OpFDiv %v3float %29 %33
               OpStore %col %34
         %37 = OpLoad %v3float %col
         %38 = OpCompositeExtract %float %37 0
         %39 = OpCompositeExtract %float %37 1
         %40 = OpCompositeExtract %float %37 2
         %41 = OpCompositeConstruct %v4float %38 %39 %40 %float_1
               OpStore %o_color %41
               OpReturn
               OpFunctionEnd
