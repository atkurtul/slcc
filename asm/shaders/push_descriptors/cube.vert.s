; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 49
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %inUV %_ %inPos %inNormal
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpName %UBOScene "UBOScene"
               OpMemberName %UBOScene 0 "projection"
               OpMemberName %UBOScene 1 "view"
               OpName %uboCamera "uboCamera"
               OpName %UBOModel "UBOModel"
               OpMemberName %UBOModel 0 "local"
               OpName %uboModel "uboModel"
               OpName %inPos "inPos"
               OpName %inNormal "inNormal"
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %UBOScene 0 ColMajor
               OpMemberDecorate %UBOScene 0 Offset 0
               OpMemberDecorate %UBOScene 0 MatrixStride 16
               OpMemberDecorate %UBOScene 1 ColMajor
               OpMemberDecorate %UBOScene 1 Offset 64
               OpMemberDecorate %UBOScene 1 MatrixStride 16
               OpDecorate %UBOScene Block
               OpDecorate %uboCamera DescriptorSet 0
               OpDecorate %uboCamera Binding 0
               OpMemberDecorate %UBOModel 0 ColMajor
               OpMemberDecorate %UBOModel 0 Offset 0
               OpMemberDecorate %UBOModel 0 MatrixStride 16
               OpDecorate %UBOModel Block
               OpDecorate %uboModel DescriptorSet 0
               OpDecorate %uboModel Binding 1
               OpDecorate %inPos Location 0
               OpDecorate %inNormal Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %v4float = OpTypeVector %float 4
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
   %UBOScene = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_Uniform_UBOScene = OpTypePointer Uniform %UBOScene
  %uboCamera = OpVariable %_ptr_Uniform_UBOScene Uniform
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
      %int_1 = OpConstant %int 1
   %UBOModel = OpTypeStruct %mat4v4float
%_ptr_Uniform_UBOModel = OpTypePointer Uniform %UBOModel
   %uboModel = OpVariable %_ptr_Uniform_UBOModel Uniform
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %inNormal = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %12 = OpLoad %v2float %inUV
               OpStore %outUV %12
         %24 = OpAccessChain %_ptr_Uniform_mat4v4float %uboCamera %int_0
         %25 = OpLoad %mat4v4float %24
         %27 = OpAccessChain %_ptr_Uniform_mat4v4float %uboCamera %int_1
         %28 = OpLoad %mat4v4float %27
         %29 = OpMatrixTimesMatrix %mat4v4float %25 %28
         %33 = OpAccessChain %_ptr_Uniform_mat4v4float %uboModel %int_0
         %34 = OpLoad %mat4v4float %33
         %35 = OpMatrixTimesMatrix %mat4v4float %29 %34
         %39 = OpLoad %v3float %inPos
         %41 = OpCompositeExtract %float %39 0
         %42 = OpCompositeExtract %float %39 1
         %43 = OpCompositeExtract %float %39 2
         %44 = OpCompositeConstruct %v4float %41 %42 %43 %float_1
         %45 = OpMatrixTimesVector %v4float %35 %44
         %47 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %47 %45
               OpReturn
               OpFunctionEnd
