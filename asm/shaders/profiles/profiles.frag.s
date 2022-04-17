; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 28
; Schema: 0
               OpCapability Shader
               OpCapability ShaderNonUniform
               OpCapability RuntimeDescriptorArray
               OpCapability SampledImageArrayNonUniformIndexing
               OpExtension "SPV_EXT_descriptor_indexing"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outFragColor %inTexIndex %inUV
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_nonuniform_qualifier"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outFragColor "outFragColor"
               OpName %textures "textures"
               OpName %inTexIndex "inTexIndex"
               OpName %inUV "inUV"
               OpDecorate %outFragColor Location 0
               OpDecorate %textures DescriptorSet 0
               OpDecorate %textures Binding 1
               OpDecorate %inTexIndex Flat
               OpDecorate %inTexIndex Location 1
               OpDecorate %19 NonUniform
               OpDecorate %21 NonUniform
               OpDecorate %22 NonUniform
               OpDecorate %inUV Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_runtimearr_11 = OpTypeRuntimeArray %11
%_ptr_UniformConstant__runtimearr_11 = OpTypePointer UniformConstant %_runtimearr_11
   %textures = OpVariable %_ptr_UniformConstant__runtimearr_11 UniformConstant
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
 %inTexIndex = OpVariable %_ptr_Input_int Input
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %18 = OpLoad %int %inTexIndex
         %19 = OpCopyObject %int %18
         %21 = OpAccessChain %_ptr_UniformConstant_11 %textures %19
         %22 = OpLoad %11 %21
         %26 = OpLoad %v2float %inUV
         %27 = OpImageSampleImplicitLod %v4float %22 %26
               OpStore %outFragColor %27
               OpReturn
               OpFunctionEnd
