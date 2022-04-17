; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 61
; Schema: 0
               OpCapability Shader
               OpCapability RayQueryKHR
               OpExtension "SPV_KHR_ray_query"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %scene_pos %position %o_pos %o_normal %normal %_
               OpSource GLSL 460
               OpSourceExtension "GL_EXT_ray_query"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %scene_pos "scene_pos"
               OpName %position "position"
               OpName %o_pos "o_pos"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "view"
               OpMemberName %GlobalUniform 1 "proj"
               OpMemberName %GlobalUniform 2 "camera_position"
               OpMemberName %GlobalUniform 3 "light_position"
               OpName %global_uniform "global_uniform"
               OpName %o_normal "o_normal"
               OpName %normal "normal"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %topLevelAS "topLevelAS"
               OpDecorate %scene_pos Location 2
               OpDecorate %position Location 0
               OpDecorate %o_pos Location 0
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpMemberDecorate %GlobalUniform 3 Offset 144
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
               OpDecorate %o_normal Location 1
               OpDecorate %normal Location 1
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %topLevelAS DescriptorSet 0
               OpDecorate %topLevelAS Binding 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %scene_pos = OpVariable %_ptr_Output_v4float Output
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
      %o_pos = OpVariable %_ptr_Output_v4float Output
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %v3float %v3float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Output_v3float = OpTypePointer Output %v3float
   %o_normal = OpVariable %_ptr_Output_v3float Output
     %normal = OpVariable %_ptr_Input_v3float Input
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_1 = OpConstant %int 1
         %58 = OpTypeAccelerationStructureKHR
%_ptr_UniformConstant_58 = OpTypePointer UniformConstant %58
 %topLevelAS = OpVariable %_ptr_UniformConstant_58 UniformConstant
       %main = OpFunction %void None %3
          %5 = OpLabel
         %13 = OpLoad %v3float %position
         %15 = OpCompositeExtract %float %13 0
         %16 = OpCompositeExtract %float %13 1
         %17 = OpCompositeExtract %float %13 2
         %18 = OpCompositeConstruct %v4float %15 %16 %17 %float_1
               OpStore %scene_pos %18
         %27 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %28 = OpLoad %mat4v4float %27
         %29 = OpLoad %v3float %position
         %30 = OpCompositeExtract %float %29 0
         %31 = OpCompositeExtract %float %29 1
         %32 = OpCompositeExtract %float %29 2
         %33 = OpCompositeConstruct %v4float %30 %31 %32 %float_1
         %34 = OpMatrixTimesVector %v4float %28 %33
               OpStore %o_pos %34
         %38 = OpLoad %v3float %normal
               OpStore %o_normal %38
         %46 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_1
         %47 = OpLoad %mat4v4float %46
         %48 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %49 = OpLoad %mat4v4float %48
         %50 = OpMatrixTimesMatrix %mat4v4float %47 %49
         %51 = OpLoad %v3float %position
         %52 = OpCompositeExtract %float %51 0
         %53 = OpCompositeExtract %float %51 1
         %54 = OpCompositeExtract %float %51 2
         %55 = OpCompositeConstruct %v4float %52 %53 %54 %float_1
         %56 = OpMatrixTimesVector %v4float %50 %55
         %57 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %57 %56
               OpReturn
               OpFunctionEnd
