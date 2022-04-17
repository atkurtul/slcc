; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 62
; Schema: 0
               OpCapability Shader
               OpCapability StorageInputOutput16
               OpExtension "SPV_KHR_16bit_storage"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %in_normal %in_color %in_delta_pos %o_color
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_explicit_arithmetic_types_float16"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %normal_color "normal_color"
               OpName %in_normal "in_normal"
               OpName %color "color"
               OpName %in_color "in_color"
               OpName %in_delta_pos "in_delta_pos"
               OpName %o_color "o_color"
               OpDecorate %in_normal Location 1
               OpDecorate %in_color Location 0
               OpDecorate %in_delta_pos Location 2
               OpDecorate %o_color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
       %half = OpTypeFloat 16
     %v3half = OpTypeVector %half 3
%_ptr_Input_v3half = OpTypePointer Input %v3half
  %in_normal = OpVariable %_ptr_Input_v3half Input
  %float_0_5 = OpConstant %float 0.5
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
   %in_color = OpVariable %_ptr_Input_v3half Input
%float_0_300000012 = OpConstant %float 0.300000012
    %float_1 = OpConstant %float 1
%float_0_200000003 = OpConstant %float 0.200000003
    %float_2 = OpConstant %float 2
%in_delta_pos = OpVariable %_ptr_Input_v3half Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Function_float = OpTypePointer Function %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
     %v4half = OpTypeVector %half 4
%_ptr_Output_v4half = OpTypePointer Output %v4half
    %o_color = OpVariable %_ptr_Output_v4half Output
       %main = OpFunction %void None %3
          %5 = OpLabel
%normal_color = OpVariable %_ptr_Function_v3float Function
      %color = OpVariable %_ptr_Function_v4float Function
         %14 = OpLoad %v3half %in_normal
         %15 = OpFConvert %v3float %14
         %17 = OpVectorTimesScalar %v3float %15 %float_0_5
         %18 = OpCompositeConstruct %v3float %float_0_5 %float_0_5 %float_0_5
         %19 = OpFAdd %v3float %17 %18
               OpStore %normal_color %19
         %24 = OpLoad %v3half %in_color
         %25 = OpFConvert %v3float %24
         %26 = OpLoad %v3float %normal_color
         %28 = OpCompositeConstruct %v3float %float_0_300000012 %float_0_300000012 %float_0_300000012
         %29 = OpExtInst %v3float %1 FMix %25 %26 %28
         %31 = OpCompositeExtract %float %29 0
         %32 = OpCompositeExtract %float %29 1
         %33 = OpCompositeExtract %float %29 2
         %34 = OpCompositeConstruct %v4float %31 %32 %33 %float_1
               OpStore %color %34
         %38 = OpLoad %v3half %in_delta_pos
         %39 = OpFConvert %v3float %38
         %40 = OpVectorTimesScalar %v3float %39 %float_2
         %41 = OpExtInst %v3float %1 Fract %40
         %42 = OpVectorTimesScalar %v3float %41 %float_0_200000003
         %43 = OpLoad %v4float %color
         %44 = OpVectorShuffle %v3float %43 %43 0 1 2
         %45 = OpFSub %v3float %44 %42
         %49 = OpAccessChain %_ptr_Function_float %color %uint_0
         %50 = OpCompositeExtract %float %45 0
               OpStore %49 %50
         %52 = OpAccessChain %_ptr_Function_float %color %uint_1
         %53 = OpCompositeExtract %float %45 1
               OpStore %52 %53
         %55 = OpAccessChain %_ptr_Function_float %color %uint_2
         %56 = OpCompositeExtract %float %45 2
               OpStore %55 %56
         %60 = OpLoad %v4float %color
         %61 = OpFConvert %v4half %60
               OpStore %o_color %61
               OpReturn
               OpFunctionEnd
