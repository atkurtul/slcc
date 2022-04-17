; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 40
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %gl_VertexIndex %_
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpDecorate %outUV Location 0
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
      %int_1 = OpConstant %int 1
      %int_2 = OpConstant %int 2
    %v4float = OpTypeVector %float 4
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
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
         %22 = OpCompositeConstruct %v2float %18 %21
               OpStore %outUV %22
         %28 = OpLoad %v2float %outUV
         %30 = OpVectorTimesScalar %v2float %28 %float_2
         %32 = OpCompositeConstruct %v2float %float_1 %float_1
         %33 = OpFSub %v2float %30 %32
         %35 = OpCompositeExtract %float %33 0
         %36 = OpCompositeExtract %float %33 1
         %37 = OpCompositeConstruct %v4float %35 %36 %float_0 %float_1
         %39 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %39 %37
               OpReturn
               OpFunctionEnd
