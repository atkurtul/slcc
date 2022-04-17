; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 39
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %position %_
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_pos "o_pos"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "model"
               OpMemberName %GlobalUniform 1 "view_proj"
               OpName %global_uniform "global_uniform"
               OpName %position "position"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpName %_ ""
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
               OpDecorate %position Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%gl_PerVertex = OpTypeStruct %v4float %float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_1 = OpConstant %int 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
      %o_pos = OpVariable %_ptr_Function_v4float Function
         %17 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %18 = OpLoad %mat4v4float %17
         %22 = OpLoad %v3float %position
         %24 = OpCompositeExtract %float %22 0
         %25 = OpCompositeExtract %float %22 1
         %26 = OpCompositeExtract %float %22 2
         %27 = OpCompositeConstruct %v4float %24 %25 %26 %float_1
         %28 = OpMatrixTimesVector %v4float %18 %27
               OpStore %o_pos %28
         %33 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_1
         %34 = OpLoad %mat4v4float %33
         %35 = OpLoad %v4float %o_pos
         %36 = OpMatrixTimesVector %v4float %34 %35
         %38 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %38 %36
               OpReturn
               OpFunctionEnd
