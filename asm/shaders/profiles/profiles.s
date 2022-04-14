; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 50
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %15 %17 %25 %40
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %15 "outTexIndex"
               OpName %17 "inTexIndex"
               OpName %23 "gl_PerVertex"
               OpMemberName %23 0 "gl_Position"
               OpMemberName %23 1 "gl_PointSize"
               OpMemberName %23 2 "gl_ClipDistance"
               OpMemberName %23 3 "gl_CullDistance"
               OpName %25 ""
               OpName %28 "UBO"
               OpMemberName %28 0 "projection"
               OpMemberName %28 1 "view"
               OpName %30 "ubo"
               OpName %40 "inPos"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpDecorate %15 Flat
               OpDecorate %15 Location 1
               OpDecorate %17 Location 2
               OpMemberDecorate %23 0 BuiltIn Position
               OpMemberDecorate %23 1 BuiltIn PointSize
               OpMemberDecorate %23 2 BuiltIn ClipDistance
               OpMemberDecorate %23 3 BuiltIn CullDistance
               OpDecorate %23 Block
               OpMemberDecorate %28 0 ColMajor
               OpMemberDecorate %28 0 Offset 0
               OpMemberDecorate %28 0 MatrixStride 16
               OpMemberDecorate %28 1 ColMajor
               OpMemberDecorate %28 1 Offset 64
               OpMemberDecorate %28 1 MatrixStride 16
               OpDecorate %28 Block
               OpDecorate %30 DescriptorSet 0
               OpDecorate %30 Binding 0
               OpDecorate %40 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeInt 32 1
         %14 = OpTypePointer Output %13
         %15 = OpVariable %14 Output
         %16 = OpTypePointer Input %13
         %17 = OpVariable %16 Input
         %19 = OpTypeVector %6 4
         %20 = OpTypeInt 32 0
         %21 = OpConstant %20 1
         %22 = OpTypeArray %6 %21
         %23 = OpTypeStruct %19 %6 %22 %22
         %24 = OpTypePointer Output %23
         %25 = OpVariable %24 Output
         %26 = OpConstant %13 0
         %27 = OpTypeMatrix %19 4
         %28 = OpTypeStruct %27 %27
         %29 = OpTypePointer Uniform %28
         %30 = OpVariable %29 Uniform
         %31 = OpTypePointer Uniform %27
         %34 = OpConstant %13 1
         %38 = OpTypeVector %6 3
         %39 = OpTypePointer Input %38
         %40 = OpVariable %39 Input
         %42 = OpConstant %6 1
         %48 = OpTypePointer Output %19
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %18 = OpLoad %13 %17
               OpStore %15 %18
         %32 = OpAccessChain %31 %30 %26
         %33 = OpLoad %27 %32
         %35 = OpAccessChain %31 %30 %34
         %36 = OpLoad %27 %35
         %37 = OpMatrixTimesMatrix %27 %33 %36
         %41 = OpLoad %38 %40
         %43 = OpCompositeExtract %6 %41 0
         %44 = OpCompositeExtract %6 %41 1
         %45 = OpCompositeExtract %6 %41 2
         %46 = OpCompositeConstruct %19 %43 %44 %45 %42
         %47 = OpMatrixTimesVector %19 %37 %46
         %49 = OpAccessChain %48 %25 %26
               OpStore %49 %47
               OpReturn
               OpFunctionEnd
