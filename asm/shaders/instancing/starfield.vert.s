; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 48
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUVW %gl_VertexIndex %_
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUVW "outUVW"
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpDecorate %outUVW Location 0
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
     %outUVW = OpVariable %_ptr_Output_v3float Output
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
      %int_1 = OpConstant %int 1
      %int_2 = OpConstant %int 2
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
    %v2float = OpTypeVector %float 2
    %float_2 = OpConstant %float 2
    %float_1 = OpConstant %float 1
    %float_0 = OpConstant %float 0
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
         %13 = OpLoad %int %gl_VertexIndex
         %15 = OpShiftLeftLogical %int %13 %int_1
         %17 = OpBitwiseAnd %int %15 %int_2
         %18 = OpConvertSToF %float %17
         %19 = OpLoad %int %gl_VertexIndex
         %20 = OpBitwiseAnd %int %19 %int_2
         %21 = OpConvertSToF %float %20
         %22 = OpLoad %int %gl_VertexIndex
         %23 = OpBitwiseAnd %int %22 %int_2
         %24 = OpConvertSToF %float %23
         %25 = OpCompositeConstruct %v3float %18 %21 %24
               OpStore %outUVW %25
         %35 = OpLoad %v3float %outUVW
         %36 = OpVectorShuffle %v2float %35 %35 0 1
         %38 = OpVectorTimesScalar %v2float %36 %float_2
         %40 = OpCompositeConstruct %v2float %float_1 %float_1
         %41 = OpFSub %v2float %38 %40
         %43 = OpCompositeExtract %float %41 0
         %44 = OpCompositeExtract %float %41 1
         %45 = OpCompositeConstruct %v4float %43 %44 %float_0 %float_1
         %47 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %47 %45
               OpReturn
               OpFunctionEnd
