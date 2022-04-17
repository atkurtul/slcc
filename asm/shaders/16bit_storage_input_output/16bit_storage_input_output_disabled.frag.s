; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 52
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %in_normal %o_color %in_color %in_delta_pos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %normal_color "normal_color"
               OpName %in_normal "in_normal"
               OpName %o_color "o_color"
               OpName %in_color "in_color"
               OpName %in_delta_pos "in_delta_pos"
               OpDecorate %in_normal Location 1
               OpDecorate %o_color Location 0
               OpDecorate %in_color Location 0
               OpDecorate %in_delta_pos Location 2
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_Input_v3float = OpTypePointer Input %v3float
  %in_normal = OpVariable %_ptr_Input_v3float Input
  %float_0_5 = OpConstant %float 0.5
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
   %in_color = OpVariable %_ptr_Input_v3float Input
%float_0_300000012 = OpConstant %float 0.300000012
    %float_1 = OpConstant %float 1
%float_0_200000003 = OpConstant %float 0.200000003
    %float_2 = OpConstant %float 2
%in_delta_pos = OpVariable %_ptr_Input_v3float Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
%normal_color = OpVariable %_ptr_Function_v3float Function
         %12 = OpLoad %v3float %in_normal
         %14 = OpVectorTimesScalar %v3float %12 %float_0_5
         %15 = OpCompositeConstruct %v3float %float_0_5 %float_0_5 %float_0_5
         %16 = OpFAdd %v3float %14 %15
               OpStore %normal_color %16
         %21 = OpLoad %v3float %in_color
         %22 = OpLoad %v3float %normal_color
         %24 = OpCompositeConstruct %v3float %float_0_300000012 %float_0_300000012 %float_0_300000012
         %25 = OpExtInst %v3float %1 FMix %21 %22 %24
         %27 = OpCompositeExtract %float %25 0
         %28 = OpCompositeExtract %float %25 1
         %29 = OpCompositeExtract %float %25 2
         %30 = OpCompositeConstruct %v4float %27 %28 %29 %float_1
               OpStore %o_color %30
         %34 = OpLoad %v3float %in_delta_pos
         %35 = OpVectorTimesScalar %v3float %34 %float_2
         %36 = OpExtInst %v3float %1 Fract %35
         %37 = OpVectorTimesScalar %v3float %36 %float_0_200000003
         %38 = OpLoad %v4float %o_color
         %39 = OpVectorShuffle %v3float %38 %38 0 1 2
         %40 = OpFSub %v3float %39 %37
         %44 = OpAccessChain %_ptr_Output_float %o_color %uint_0
         %45 = OpCompositeExtract %float %40 0
               OpStore %44 %45
         %47 = OpAccessChain %_ptr_Output_float %o_color %uint_1
         %48 = OpCompositeExtract %float %40 1
               OpStore %47 %48
         %50 = OpAccessChain %_ptr_Output_float %o_color %uint_2
         %51 = OpCompositeExtract %float %40 2
               OpStore %50 %51
               OpReturn
               OpFunctionEnd
