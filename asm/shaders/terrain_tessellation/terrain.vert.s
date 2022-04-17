; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 37
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %_ %inPos %outUV %inUV %outNormal %inNormal
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %inPos "inPos"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %inPos Location 0
               OpDecorate %outUV Location 1
               OpDecorate %inUV Location 2
               OpDecorate %outNormal Location 0
               OpDecorate %inNormal Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %outNormal = OpVariable %_ptr_Output_v3float Output
   %inNormal = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %19 = OpLoad %v3float %inPos
         %21 = OpCompositeExtract %float %19 0
         %22 = OpCompositeExtract %float %19 1
         %23 = OpCompositeExtract %float %19 2
         %24 = OpCompositeConstruct %v4float %21 %22 %23 %float_1
         %26 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %26 %24
         %32 = OpLoad %v2float %inUV
               OpStore %outUV %32
         %36 = OpLoad %v3float %inNormal
               OpStore %outNormal %36
               OpReturn
               OpFunctionEnd
