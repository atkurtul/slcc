; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 25
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outFragColor %inColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outFragColor "outFragColor"
               OpName %inColor "inColor"
               OpDecorate %outFragColor Location 0
               OpDecorate %inColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
    %inColor = OpVariable %_ptr_Input_v3float Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
         %13 = OpLoad %v3float %inColor
         %17 = OpAccessChain %_ptr_Output_float %outFragColor %uint_0
         %18 = OpCompositeExtract %float %13 0
               OpStore %17 %18
         %20 = OpAccessChain %_ptr_Output_float %outFragColor %uint_1
         %21 = OpCompositeExtract %float %13 1
               OpStore %20 %21
         %23 = OpAccessChain %_ptr_Output_float %outFragColor %uint_2
         %24 = OpCompositeExtract %float %13 2
               OpStore %23 %24
               OpReturn
               OpFunctionEnd
