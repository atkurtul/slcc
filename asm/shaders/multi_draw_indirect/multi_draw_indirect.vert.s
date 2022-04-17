; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 53
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %o_uv %uv %_ %position %o_texture_index %texture_index %sphere_center %sphere_radius
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_uv "o_uv"
               OpName %uv "uv"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "view"
               OpMemberName %GlobalUniform 1 "proj"
               OpMemberName %GlobalUniform 2 "proj_view"
               OpMemberName %GlobalUniform 3 "model_count"
               OpName %global_uniform "global_uniform"
               OpName %position "position"
               OpName %o_texture_index "o_texture_index"
               OpName %texture_index "texture_index"
               OpName %sphere_center "sphere_center"
               OpName %sphere_radius "sphere_radius"
               OpDecorate %o_uv Location 1
               OpDecorate %uv Location 1
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 ColMajor
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpMemberDecorate %GlobalUniform 2 MatrixStride 16
               OpMemberDecorate %GlobalUniform 3 Offset 192
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 2
               OpDecorate %position Location 0
               OpDecorate %o_texture_index Location 2
               OpDecorate %texture_index Location 4
               OpDecorate %sphere_center Location 2
               OpDecorate %sphere_radius Location 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
       %o_uv = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
         %uv = OpVariable %_ptr_Input_v2float Input
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %uint
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Output_uint = OpTypePointer Output %uint
%o_texture_index = OpVariable %_ptr_Output_uint Output
%_ptr_Input_uint = OpTypePointer Input %uint
%texture_index = OpVariable %_ptr_Input_uint Input
%sphere_center = OpVariable %_ptr_Input_v3float Input
%_ptr_Input_float = OpTypePointer Input %float
%sphere_radius = OpVariable %_ptr_Input_float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
         %12 = OpLoad %v2float %uv
               OpStore %o_uv %12
         %28 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_1
         %29 = OpLoad %mat4v4float %28
         %30 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %31 = OpLoad %mat4v4float %30
         %32 = OpMatrixTimesMatrix %mat4v4float %29 %31
         %36 = OpLoad %v3float %position
         %38 = OpCompositeExtract %float %36 0
         %39 = OpCompositeExtract %float %36 1
         %40 = OpCompositeExtract %float %36 2
         %41 = OpCompositeConstruct %v4float %38 %39 %40 %float_1
         %42 = OpMatrixTimesVector %v4float %32 %41
         %44 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %44 %42
         %49 = OpLoad %uint %texture_index
               OpStore %o_texture_index %49
               OpReturn
               OpFunctionEnd
