; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 44
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
               OpName %samplerColor "samplerColor"
               OpName %inUV "inUV"
               OpName %outFragColor "outFragColor"
               OpDecorate %samplerColor DescriptorSet 0
               OpDecorate %samplerColor Binding 1
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
%samplerColor = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
%float_0_454545468 = OpConstant %float 0.454545468
         %26 = OpConstantComposite %v3float %float_0_454545468 %float_0_454545468 %float_0_454545468
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
     %uint_3 = OpConstant %uint 3
%_ptr_Function_float = OpTypePointer Function %float
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
         %14 = OpLoad %11 %samplerColor
         %18 = OpLoad %v2float %inUV
         %19 = OpImageSampleImplicitLod %v4float %14 %18
               OpStore %color %19
         %23 = OpLoad %v4float %color
         %24 = OpVectorShuffle %v3float %23 %23 0 1 2
         %27 = OpExtInst %v3float %1 Pow %24 %26
         %31 = OpAccessChain %_ptr_Output_float %outFragColor %uint_0
         %32 = OpCompositeExtract %float %27 0
               OpStore %31 %32
         %34 = OpAccessChain %_ptr_Output_float %outFragColor %uint_1
         %35 = OpCompositeExtract %float %27 1
               OpStore %34 %35
         %37 = OpAccessChain %_ptr_Output_float %outFragColor %uint_2
         %38 = OpCompositeExtract %float %27 2
               OpStore %37 %38
         %41 = OpAccessChain %_ptr_Function_float %color %uint_3
         %42 = OpLoad %float %41
         %43 = OpAccessChain %_ptr_Output_float %outFragColor %uint_3
               OpStore %43 %42
               OpReturn
               OpFunctionEnd
