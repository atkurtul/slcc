; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 59
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %o_pos %position %o_uv %texcoord_0 %o_normal %normal %_
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_pos "o_pos"
               OpName %MVPUniform "MVPUniform"
               OpMemberName %MVPUniform 0 "model"
               OpMemberName %MVPUniform 1 "view_proj"
               OpName %mvp_uniform "mvp_uniform"
               OpName %position "position"
               OpName %o_uv "o_uv"
               OpName %texcoord_0 "texcoord_0"
               OpName %o_normal "o_normal"
               OpName %normal "normal"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpName %_ ""
               OpDecorate %o_pos Location 0
               OpMemberDecorate %MVPUniform 0 ColMajor
               OpMemberDecorate %MVPUniform 0 Offset 0
               OpMemberDecorate %MVPUniform 0 MatrixStride 16
               OpMemberDecorate %MVPUniform 1 ColMajor
               OpMemberDecorate %MVPUniform 1 Offset 64
               OpMemberDecorate %MVPUniform 1 MatrixStride 16
               OpDecorate %MVPUniform Block
               OpDecorate %position Location 0
               OpDecorate %o_uv Location 1
               OpDecorate %texcoord_0 Location 1
               OpDecorate %o_normal Location 2
               OpDecorate %normal Location 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
      %o_pos = OpVariable %_ptr_Output_v4float Output
%mat4v4float = OpTypeMatrix %v4float 4
 %MVPUniform = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_PushConstant_MVPUniform = OpTypePointer PushConstant %MVPUniform
%mvp_uniform = OpVariable %_ptr_PushConstant_MVPUniform PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_mat4v4float = OpTypePointer PushConstant %mat4v4float
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
       %o_uv = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
 %texcoord_0 = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v3float = OpTypePointer Output %v3float
   %o_normal = OpVariable %_ptr_Output_v3float Output
%mat3v3float = OpTypeMatrix %v3float 3
     %normal = OpVariable %_ptr_Input_v3float Input
%gl_PerVertex = OpTypeStruct %v4float %float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_1 = OpConstant %int 1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %17 = OpAccessChain %_ptr_PushConstant_mat4v4float %mvp_uniform %int_0
         %18 = OpLoad %mat4v4float %17
         %22 = OpLoad %v3float %position
         %24 = OpCompositeExtract %float %22 0
         %25 = OpCompositeExtract %float %22 1
         %26 = OpCompositeExtract %float %22 2
         %27 = OpCompositeConstruct %v4float %24 %25 %26 %float_1
         %28 = OpMatrixTimesVector %v4float %18 %27
               OpStore %o_pos %28
         %34 = OpLoad %v2float %texcoord_0
               OpStore %o_uv %34
         %37 = OpAccessChain %_ptr_PushConstant_mat4v4float %mvp_uniform %int_0
         %38 = OpLoad %mat4v4float %37
         %40 = OpCompositeExtract %v4float %38 0
         %41 = OpVectorShuffle %v3float %40 %40 0 1 2
         %42 = OpCompositeExtract %v4float %38 1
         %43 = OpVectorShuffle %v3float %42 %42 0 1 2
         %44 = OpCompositeExtract %v4float %38 2
         %45 = OpVectorShuffle %v3float %44 %44 0 1 2
         %46 = OpCompositeConstruct %mat3v3float %41 %43 %45
         %48 = OpLoad %v3float %normal
         %49 = OpMatrixTimesVector %v3float %46 %48
               OpStore %o_normal %49
         %54 = OpAccessChain %_ptr_PushConstant_mat4v4float %mvp_uniform %int_1
         %55 = OpLoad %mat4v4float %54
         %56 = OpLoad %v4float %o_pos
         %57 = OpMatrixTimesVector %v4float %55 %56
         %58 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %58 %57
               OpReturn
               OpFunctionEnd
