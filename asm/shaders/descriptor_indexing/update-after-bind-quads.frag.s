; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 37
; Schema: 0
               OpCapability Shader
               OpCapability RuntimeDescriptorArray
               OpExtension "SPV_EXT_descriptor_indexing"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %out_frag_color %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_nonuniform_qualifier"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %out_frag_color "out_frag_color"
               OpName %Textures "Textures"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "table_offset"
               OpName %registers "registers"
               OpName %ImmutableSampler "ImmutableSampler"
               OpName %in_uv "in_uv"
               OpDecorate %out_frag_color Location 0
               OpDecorate %Textures DescriptorSet 0
               OpDecorate %Textures Binding 0
               OpMemberDecorate %Registers 0 Offset 4
               OpDecorate %Registers Block
               OpDecorate %ImmutableSampler DescriptorSet 1
               OpDecorate %ImmutableSampler Binding 0
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
       %uint = OpTypeInt 32 0
  %Registers = OpTypeStruct %uint
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_uint = OpTypePointer PushConstant %uint
%_ptr_UniformConstant_10 = OpTypePointer UniformConstant %10
         %26 = OpTypeSampler
%_ptr_UniformConstant_26 = OpTypePointer UniformConstant %26
%ImmutableSampler = OpVariable %_ptr_UniformConstant_26 UniformConstant
         %30 = OpTypeSampledImage %10
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %21 = OpAccessChain %_ptr_PushConstant_uint %registers %int_0
         %22 = OpLoad %uint %21
         %24 = OpAccessChain %_ptr_UniformConstant_10 %Textures %22
         %25 = OpLoad %10 %24
         %29 = OpLoad %26 %ImmutableSampler
         %31 = OpSampledImage %30 %25 %29
         %35 = OpLoad %v2float %in_uv
         %36 = OpImageSampleImplicitLod %v4float %31 %35
               OpStore %out_frag_color %36
               OpReturn
               OpFunctionEnd
