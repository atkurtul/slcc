; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 24
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outFragColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outFragColor "outFragColor"
               OpDecorate %outFragColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
    %float_1 = OpConstant %float 1
         %12 = OpConstantComposite %v3float %float_1 %float_1 %float_1
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
       %main = OpFunction %void None %3
          %5 = OpLabel
         %16 = OpAccessChain %_ptr_Output_float %outFragColor %uint_0
         %17 = OpCompositeExtract %float %12 0
               OpStore %16 %17
         %19 = OpAccessChain %_ptr_Output_float %outFragColor %uint_1
         %20 = OpCompositeExtract %float %12 1
               OpStore %19 %20
         %22 = OpAccessChain %_ptr_Output_float %outFragColor %uint_2
         %23 = OpCompositeExtract %float %12 2
               OpStore %22 %23
               OpReturn
               OpFunctionEnd
