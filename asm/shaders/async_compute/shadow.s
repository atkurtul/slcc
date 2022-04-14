; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 39
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %21 %31
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "o_pos"
               OpName %11 "GlobalUniform"
               OpMemberName %11 0 "model"
               OpMemberName %11 1 "view_proj"
               OpName %13 "global_uniform"
               OpName %21 "position"
               OpName %29 "gl_PerVertex"
               OpMemberName %29 0 "gl_Position"
               OpMemberName %29 1 "gl_PointSize"
               OpName %31 ""
               OpMemberDecorate %11 0 ColMajor
               OpMemberDecorate %11 0 Offset 0
               OpMemberDecorate %11 0 MatrixStride 16
               OpMemberDecorate %11 1 ColMajor
               OpMemberDecorate %11 1 Offset 64
               OpMemberDecorate %11 1 MatrixStride 16
               OpDecorate %11 Block
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %21 Location 0
               OpMemberDecorate %29 0 BuiltIn Position
               OpMemberDecorate %29 1 BuiltIn PointSize
               OpDecorate %29 Block
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
         %10 = OpTypeMatrix %7 4
         %11 = OpTypeStruct %10 %10
         %12 = OpTypePointer Uniform %11
         %13 = OpVariable %12 Uniform
         %14 = OpTypeInt 32 1
         %15 = OpConstant %14 0
         %16 = OpTypePointer Uniform %10
         %19 = OpTypeVector %6 3
         %20 = OpTypePointer Input %19
         %21 = OpVariable %20 Input
         %23 = OpConstant %6 1
         %29 = OpTypeStruct %7 %6
         %30 = OpTypePointer Output %29
         %31 = OpVariable %30 Output
         %32 = OpConstant %14 1
         %37 = OpTypePointer Output %7
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %17 = OpAccessChain %16 %13 %15
         %18 = OpLoad %10 %17
         %22 = OpLoad %19 %21
         %24 = OpCompositeExtract %6 %22 0
         %25 = OpCompositeExtract %6 %22 1
         %26 = OpCompositeExtract %6 %22 2
         %27 = OpCompositeConstruct %7 %24 %25 %26 %23
         %28 = OpMatrixTimesVector %7 %18 %27
               OpStore %9 %28
         %33 = OpAccessChain %16 %13 %32
         %34 = OpLoad %10 %33
         %35 = OpLoad %7 %9
         %36 = OpMatrixTimesVector %7 %34 %35
         %38 = OpAccessChain %37 %31 %15
               OpStore %38 %36
               OpReturn
               OpFunctionEnd
