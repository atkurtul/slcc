; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 33
; Schema: 0
               OpCapability Shader
               OpCapability ShaderNonUniform
               OpCapability RuntimeDescriptorArray
               OpExtension "SPV_EXT_descriptor_indexing"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %out_frag_color %in_texture_index %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_nonuniform_qualifier"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %out_frag_color "out_frag_color"
               OpName %Textures "Textures"
               OpName %in_texture_index "in_texture_index"
               OpName %ImmutableSampler "ImmutableSampler"
               OpName %in_uv "in_uv"
               OpDecorate %out_frag_color Location 0
               OpDecorate %Textures DescriptorSet 0
               OpDecorate %Textures Binding 0
               OpDecorate %in_texture_index Flat
               OpDecorate %in_texture_index Location 1
               OpDecorate %ImmutableSampler DescriptorSet 1
               OpDecorate %ImmutableSampler Binding 0
               OpDecorate %27 NonUniform
               OpDecorate %in_uv Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%out_frag_color = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
%_runtimearr_10 = OpTypeRuntimeArray %10
%_ptr_UniformConstant__runtimearr_10 = OpTypePointer UniformConstant %_runtimearr_10
   %Textures = OpVariable %_ptr_UniformConstant__runtimearr_10 UniformConstant
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%in_texture_index = OpVariable %_ptr_Input_int Input
%_ptr_UniformConstant_10 = OpTypePointer UniformConstant %10
         %21 = OpTypeSampler
%_ptr_UniformConstant_21 = OpTypePointer UniformConstant %21
%ImmutableSampler = OpVariable %_ptr_UniformConstant_21 UniformConstant
         %25 = OpTypeSampledImage %10
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %17 = OpLoad %int %in_texture_index
         %19 = OpAccessChain %_ptr_UniformConstant_10 %Textures %17
         %20 = OpLoad %10 %19
         %24 = OpLoad %21 %ImmutableSampler
         %26 = OpSampledImage %25 %20 %24
         %27 = OpCopyObject %25 %26
         %31 = OpLoad %v2float %in_uv
         %32 = OpImageSampleImplicitLod %v4float %27 %31
               OpStore %out_frag_color %32
               OpReturn
               OpFunctionEnd
