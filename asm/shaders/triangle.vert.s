; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 52
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %_ %gl_VertexIndex %out_color
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %triangle_positions "triangle_positions"
               OpName %triangle_colors "triangle_colors"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpName %_ ""
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %out_color "out_color"
               OpDecorate %triangle_positions RelaxedPrecision
               OpDecorate %triangle_colors RelaxedPrecision
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpDecorate %gl_PerVertex Block
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpDecorate %40 RelaxedPrecision
               OpDecorate %41 RelaxedPrecision
               OpDecorate %42 RelaxedPrecision
               OpDecorate %43 RelaxedPrecision
               OpDecorate %out_color RelaxedPrecision
               OpDecorate %out_color Location 0
               OpDecorate %51 RelaxedPrecision
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
%_arr_v2float_uint_3 = OpTypeArray %v2float %uint_3
%_ptr_Private__arr_v2float_uint_3 = OpTypePointer Private %_arr_v2float_uint_3
%triangle_positions = OpVariable %_ptr_Private__arr_v2float_uint_3 Private
  %float_0_5 = OpConstant %float 0.5
 %float_n0_5 = OpConstant %float -0.5
         %15 = OpConstantComposite %v2float %float_0_5 %float_n0_5
         %16 = OpConstantComposite %v2float %float_0_5 %float_0_5
         %17 = OpConstantComposite %v2float %float_n0_5 %float_0_5
         %18 = OpConstantComposite %_arr_v2float_uint_3 %15 %16 %17
    %v3float = OpTypeVector %float 3
%_arr_v3float_uint_3 = OpTypeArray %v3float %uint_3
%_ptr_Private__arr_v3float_uint_3 = OpTypePointer Private %_arr_v3float_uint_3
%triangle_colors = OpVariable %_ptr_Private__arr_v3float_uint_3 Private
    %float_1 = OpConstant %float 1
    %float_0 = OpConstant %float 0
         %25 = OpConstantComposite %v3float %float_1 %float_0 %float_0
         %26 = OpConstantComposite %v3float %float_0 %float_1 %float_0
         %27 = OpConstantComposite %v3float %float_0 %float_0 %float_1
         %28 = OpConstantComposite %_arr_v3float_uint_3 %25 %26 %27
    %v4float = OpTypeVector %float 4
%gl_PerVertex = OpTypeStruct %v4float %float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
%_ptr_Private_v2float = OpTypePointer Private %v2float
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %out_color = OpVariable %_ptr_Output_v3float Output
%_ptr_Private_v3float = OpTypePointer Private %v3float
       %main = OpFunction %void None %3
          %5 = OpLabel
               OpStore %triangle_positions %18
               OpStore %triangle_colors %28
         %37 = OpLoad %int %gl_VertexIndex
         %39 = OpAccessChain %_ptr_Private_v2float %triangle_positions %37
         %40 = OpLoad %v2float %39
         %41 = OpCompositeExtract %float %40 0
         %42 = OpCompositeExtract %float %40 1
         %43 = OpCompositeConstruct %v4float %41 %42 %float_0 %float_1
         %45 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %45 %43
         %48 = OpLoad %int %gl_VertexIndex
         %50 = OpAccessChain %_ptr_Private_v3float %triangle_colors %48
         %51 = OpLoad %v3float %50
               OpStore %out_color %51
               OpReturn
               OpFunctionEnd
