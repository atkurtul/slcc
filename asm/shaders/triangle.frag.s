; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 19
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %out_color %in_color
               OpExecutionMode %main OriginUpperLeft
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %out_color "out_color"
               OpName %in_color "in_color"
               OpDecorate %out_color RelaxedPrecision
               OpDecorate %out_color Location 0
               OpDecorate %in_color RelaxedPrecision
               OpDecorate %in_color Location 0
               OpDecorate %13 RelaxedPrecision
               OpDecorate %15 RelaxedPrecision
               OpDecorate %16 RelaxedPrecision
               OpDecorate %17 RelaxedPrecision
               OpDecorate %18 RelaxedPrecision
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %out_color = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %in_color = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %13 = OpLoad %v3float %in_color
         %15 = OpCompositeExtract %float %13 0
         %16 = OpCompositeExtract %float %13 1
         %17 = OpCompositeExtract %float %13 2
         %18 = OpCompositeConstruct %v4float %15 %16 %17 %float_1
               OpStore %out_color %18
               OpReturn
               OpFunctionEnd
