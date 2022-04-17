; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 46
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inGradientPos %outFragColor %gl_PointCoord
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %color "color"
               OpName %samplerGradientRamp "samplerGradientRamp"
               OpName %inGradientPos "inGradientPos"
               OpName %outFragColor "outFragColor"
               OpName %samplerColorMap "samplerColorMap"
               OpName %gl_PointCoord "gl_PointCoord"
               OpDecorate %samplerGradientRamp DescriptorSet 0
               OpDecorate %samplerGradientRamp Binding 1
               OpDecorate %inGradientPos Location 0
               OpDecorate %outFragColor Location 0
               OpDecorate %samplerColorMap DescriptorSet 0
               OpDecorate %samplerColorMap Binding 0
               OpDecorate %gl_PointCoord BuiltIn PointCoord
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
%samplerGradientRamp = OpVariable %_ptr_UniformConstant_11 UniformConstant
%_ptr_Input_float = OpTypePointer Input %float
%inGradientPos = OpVariable %_ptr_Input_float Input
    %float_0 = OpConstant %float 0
    %v2float = OpTypeVector %float 2
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
%samplerColorMap = OpVariable %_ptr_UniformConstant_11 UniformConstant
%_ptr_Input_v2float = OpTypePointer Input %v2float
%gl_PointCoord = OpVariable %_ptr_Input_v2float Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v3float Function
         %14 = OpLoad %11 %samplerGradientRamp
         %17 = OpLoad %float %inGradientPos
         %20 = OpCompositeConstruct %v2float %17 %float_0
         %22 = OpImageSampleImplicitLod %v4float %14 %20
         %23 = OpVectorShuffle %v3float %22 %22 0 1 2
               OpStore %color %23
         %27 = OpLoad %11 %samplerColorMap
         %30 = OpLoad %v2float %gl_PointCoord
         %31 = OpImageSampleImplicitLod %v4float %27 %30
         %32 = OpVectorShuffle %v3float %31 %31 0 1 2
         %33 = OpLoad %v3float %color
         %34 = OpFMul %v3float %32 %33
         %38 = OpAccessChain %_ptr_Output_float %outFragColor %uint_0
         %39 = OpCompositeExtract %float %34 0
               OpStore %38 %39
         %41 = OpAccessChain %_ptr_Output_float %outFragColor %uint_1
         %42 = OpCompositeExtract %float %34 1
               OpStore %41 %42
         %44 = OpAccessChain %_ptr_Output_float %outFragColor %uint_2
         %45 = OpCompositeExtract %float %34 2
               OpStore %44 %45
               OpReturn
               OpFunctionEnd
