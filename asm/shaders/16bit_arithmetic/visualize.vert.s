; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 51
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %gl_VertexIndex %_ %o_uv
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %o_uv "o_uv"
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %o_uv Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
      %int_0 = OpConstant %int 0
       %bool = OpTypeBool
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
   %float_n1 = OpConstant %float -1
    %float_0 = OpConstant %float 0
    %float_1 = OpConstant %float 1
         %26 = OpConstantComposite %v4float %float_n1 %float_n1 %float_0 %float_1
%_ptr_Output_v4float = OpTypePointer Output %v4float
      %int_1 = OpConstant %int 1
    %float_3 = OpConstant %float 3
         %36 = OpConstantComposite %v4float %float_n1 %float_3 %float_0 %float_1
         %39 = OpConstantComposite %v4float %float_3 %float_n1 %float_0 %float_1
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
       %o_uv = OpVariable %_ptr_Output_v2float Output
  %float_0_5 = OpConstant %float 0.5
       %main = OpFunction %void None %3
          %5 = OpLabel
          %9 = OpLoad %int %gl_VertexIndex
         %12 = OpIEqual %bool %9 %int_0
               OpSelectionMerge %14 None
               OpBranchConditional %12 %13 %29
         %13 = OpLabel
         %28 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %28 %26
               OpBranch %14
         %29 = OpLabel
         %30 = OpLoad %int %gl_VertexIndex
         %32 = OpIEqual %bool %30 %int_1
               OpSelectionMerge %34 None
               OpBranchConditional %32 %33 %38
         %33 = OpLabel
         %37 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %37 %36
               OpBranch %34
         %38 = OpLabel
         %40 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %40 %39
               OpBranch %34
         %34 = OpLabel
               OpBranch %14
         %14 = OpLabel
         %44 = OpAccessChain %_ptr_Output_v4float %_ %int_0
         %45 = OpLoad %v4float %44
         %46 = OpVectorShuffle %v2float %45 %45 0 1
         %48 = OpVectorTimesScalar %v2float %46 %float_0_5
         %49 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
         %50 = OpFAdd %v2float %48 %49
               OpStore %o_uv %50
               OpReturn
               OpFunctionEnd
