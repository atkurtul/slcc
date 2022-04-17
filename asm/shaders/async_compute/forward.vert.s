; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 67
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %o_pos %position %o_uv %texcoord_0 %o_normal %normal %o_shadow_clip %_
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_pos "o_pos"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "model"
               OpMemberName %GlobalUniform 1 "view_proj"
               OpMemberName %GlobalUniform 2 "camera_position"
               OpName %global_uniform "global_uniform"
               OpName %position "position"
               OpName %o_uv "o_uv"
               OpName %texcoord_0 "texcoord_0"
               OpName %o_normal "o_normal"
               OpName %normal "normal"
               OpName %o_shadow_clip "o_shadow_clip"
               OpName %ShadowUniform "ShadowUniform"
               OpMemberName %ShadowUniform 0 "matrix"
               OpName %shadow_uniform "shadow_uniform"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpName %_ ""
               OpDecorate %o_pos Location 0
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
               OpDecorate %position Location 0
               OpDecorate %o_uv Location 1
               OpDecorate %texcoord_0 Location 1
               OpDecorate %o_normal Location 2
               OpDecorate %normal Location 2
               OpDecorate %o_shadow_clip Location 3
               OpMemberDecorate %ShadowUniform 0 ColMajor
               OpMemberDecorate %ShadowUniform 0 Offset 0
               OpMemberDecorate %ShadowUniform 0 MatrixStride 16
               OpDecorate %ShadowUniform Block
               OpDecorate %shadow_uniform DescriptorSet 0
               OpDecorate %shadow_uniform Binding 5
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
    %v3float = OpTypeVector %float 3
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %v3float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
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
%o_shadow_clip = OpVariable %_ptr_Output_v4float Output
%ShadowUniform = OpTypeStruct %mat4v4float
%_ptr_Uniform_ShadowUniform = OpTypePointer Uniform %ShadowUniform
%shadow_uniform = OpVariable %_ptr_Uniform_ShadowUniform Uniform
%gl_PerVertex = OpTypeStruct %v4float %float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_1 = OpConstant %int 1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %18 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %19 = OpLoad %mat4v4float %18
         %22 = OpLoad %v3float %position
         %24 = OpCompositeExtract %float %22 0
         %25 = OpCompositeExtract %float %22 1
         %26 = OpCompositeExtract %float %22 2
         %27 = OpCompositeConstruct %v4float %24 %25 %26 %float_1
         %28 = OpMatrixTimesVector %v4float %19 %27
               OpStore %o_pos %28
         %34 = OpLoad %v2float %texcoord_0
               OpStore %o_uv %34
         %37 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
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
         %54 = OpAccessChain %_ptr_Uniform_mat4v4float %shadow_uniform %int_0
         %55 = OpLoad %mat4v4float %54
         %56 = OpLoad %v4float %o_pos
         %57 = OpMatrixTimesVector %v4float %55 %56
               OpStore %o_shadow_clip %57
         %62 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_1
         %63 = OpLoad %mat4v4float %62
         %64 = OpLoad %v4float %o_pos
         %65 = OpMatrixTimesVector %v4float %63 %64
         %66 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %66 %65
               OpReturn
               OpFunctionEnd
