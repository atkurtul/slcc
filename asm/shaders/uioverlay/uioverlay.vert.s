; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 40
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %inUV %outColor %inColor %_ %inPos
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %outColor "outColor"
               OpName %inColor "inColor"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpName %PushConstants "PushConstants"
               OpMemberName %PushConstants 0 "transform"
               OpName %pushConstants "pushConstants"
               OpName %inPos "inPos"
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 1
               OpDecorate %outColor Location 1
               OpDecorate %inColor Location 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %PushConstants 0 ColMajor
               OpMemberDecorate %PushConstants 0 Offset 0
               OpMemberDecorate %PushConstants 0 MatrixStride 16
               OpDecorate %PushConstants Block
               OpDecorate %inPos Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %outColor = OpVariable %_ptr_Output_v4float Output
%_ptr_Input_v4float = OpTypePointer Input %v4float
    %inColor = OpVariable %_ptr_Input_v4float Input
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
%PushConstants = OpTypeStruct %mat4v4float
%_ptr_PushConstant_PushConstants = OpTypePointer PushConstant %PushConstants
%pushConstants = OpVariable %_ptr_PushConstant_PushConstants PushConstant
%_ptr_PushConstant_mat4v4float = OpTypePointer PushConstant %mat4v4float
      %inPos = OpVariable %_ptr_Input_v2float Input
    %float_0 = OpConstant %float 0
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %12 = OpLoad %v2float %inUV
               OpStore %outUV %12
         %18 = OpLoad %v4float %inColor
               OpStore %outColor %18
         %29 = OpAccessChain %_ptr_PushConstant_mat4v4float %pushConstants %int_0
         %30 = OpLoad %mat4v4float %29
         %32 = OpLoad %v2float %inPos
         %35 = OpCompositeExtract %float %32 0
         %36 = OpCompositeExtract %float %32 1
         %37 = OpCompositeConstruct %v4float %35 %36 %float_0 %float_1
         %38 = OpMatrixTimesVector %v4float %30 %37
         %39 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %39 %38
               OpReturn
               OpFunctionEnd
