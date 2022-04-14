; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 46
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %19 %35 %45
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %17 "gl_PerVertex"
               OpMemberName %17 0 "gl_Position"
               OpMemberName %17 1 "gl_PointSize"
               OpMemberName %17 2 "gl_ClipDistance"
               OpMemberName %17 3 "gl_CullDistance"
               OpName %19 ""
               OpName %23 "UBO"
               OpMemberName %23 0 "projection"
               OpMemberName %23 1 "model"
               OpMemberName %23 2 "viewPos"
               OpName %25 "ubo"
               OpName %35 "inPos"
               OpName %45 "inNormal"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpMemberDecorate %17 0 BuiltIn Position
               OpMemberDecorate %17 1 BuiltIn PointSize
               OpMemberDecorate %17 2 BuiltIn ClipDistance
               OpMemberDecorate %17 3 BuiltIn CullDistance
               OpDecorate %17 Block
               OpMemberDecorate %23 0 ColMajor
               OpMemberDecorate %23 0 Offset 0
               OpMemberDecorate %23 0 MatrixStride 16
               OpMemberDecorate %23 1 ColMajor
               OpMemberDecorate %23 1 Offset 64
               OpMemberDecorate %23 1 MatrixStride 16
               OpMemberDecorate %23 2 Offset 128
               OpDecorate %23 Block
               OpDecorate %25 DescriptorSet 0
               OpDecorate %25 Binding 0
               OpDecorate %35 Location 0
               OpDecorate %45 Location 2
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeVector %6 4
         %14 = OpTypeInt 32 0
         %15 = OpConstant %14 1
         %16 = OpTypeArray %6 %15
         %17 = OpTypeStruct %13 %6 %16 %16
         %18 = OpTypePointer Output %17
         %19 = OpVariable %18 Output
         %20 = OpTypeInt 32 1
         %21 = OpConstant %20 0
         %22 = OpTypeMatrix %13 4
         %23 = OpTypeStruct %22 %22 %13
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpTypePointer Uniform %22
         %29 = OpConstant %20 1
         %33 = OpTypeVector %6 3
         %34 = OpTypePointer Input %33
         %35 = OpVariable %34 Input
         %37 = OpConstant %6 1
         %43 = OpTypePointer Output %13
         %45 = OpVariable %34 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %27 = OpAccessChain %26 %25 %21
         %28 = OpLoad %22 %27
         %30 = OpAccessChain %26 %25 %29
         %31 = OpLoad %22 %30
         %32 = OpMatrixTimesMatrix %22 %28 %31
         %36 = OpLoad %33 %35
         %38 = OpCompositeExtract %6 %36 0
         %39 = OpCompositeExtract %6 %36 1
         %40 = OpCompositeExtract %6 %36 2
         %41 = OpCompositeConstruct %13 %38 %39 %40 %37
         %42 = OpMatrixTimesVector %13 %32 %41
         %44 = OpAccessChain %43 %19 %21
               OpStore %44 %42
               OpReturn
               OpFunctionEnd
