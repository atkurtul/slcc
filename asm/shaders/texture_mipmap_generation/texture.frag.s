; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 57
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
               OpName %textureColor "textureColor"
               OpName %samplers "samplers"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lodBias"
               OpMemberName %UBO 3 "samplerIndex"
               OpName %ubo "ubo"
               OpName %inUV "inUV"
               OpName %outFragColor "outFragColor"
               OpDecorate %textureColor DescriptorSet 0
               OpDecorate %textureColor Binding 1
               OpDecorate %samplers DescriptorSet 0
               OpDecorate %samplers Binding 2
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 132
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inUV Location 0
               OpDecorate %outFragColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
%_ptr_UniformConstant_10 = OpTypePointer UniformConstant %10
%textureColor = OpVariable %_ptr_UniformConstant_10 UniformConstant
         %14 = OpTypeSampler
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
%_arr_14_uint_3 = OpTypeArray %14 %uint_3
%_ptr_UniformConstant__arr_14_uint_3 = OpTypePointer UniformConstant %_arr_14_uint_3
   %samplers = OpVariable %_ptr_UniformConstant__arr_14_uint_3 UniformConstant
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
        %int = OpTypeInt 32 1
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_3 = OpConstant %int 3
%_ptr_Uniform_int = OpTypePointer Uniform %int
%_ptr_UniformConstant_14 = OpTypePointer UniformConstant %14
         %33 = OpTypeSampledImage %10
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %float_2 = OpConstant %float 2
 %float_0_25 = OpConstant %float 0.25
         %41 = OpConstantComposite %v2float %float_2 %float_0_25
      %int_2 = OpConstant %int 2
%_ptr_Uniform_float = OpTypePointer Uniform %float
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v3float Function
         %13 = OpLoad %10 %textureColor
         %28 = OpAccessChain %_ptr_Uniform_int %ubo %int_3
         %29 = OpLoad %int %28
         %31 = OpAccessChain %_ptr_UniformConstant_14 %samplers %29
         %32 = OpLoad %14 %31
         %34 = OpSampledImage %33 %13 %32
         %38 = OpLoad %v2float %inUV
         %42 = OpFMul %v2float %38 %41
         %45 = OpAccessChain %_ptr_Uniform_float %ubo %int_2
         %46 = OpLoad %float %45
         %47 = OpImageSampleImplicitLod %v4float %34 %42 Bias %46
         %48 = OpVectorShuffle %v3float %47 %47 0 1 2
               OpStore %color %48
         %51 = OpLoad %v3float %color
         %53 = OpCompositeExtract %float %51 0
         %54 = OpCompositeExtract %float %51 1
         %55 = OpCompositeExtract %float %51 2
         %56 = OpCompositeConstruct %v4float %53 %54 %55 %float_1
               OpStore %outFragColor %56
               OpReturn
               OpFunctionEnd
