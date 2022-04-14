; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 61
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %27 %44 %60
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %15 "worldPos"
               OpName %19 "UBO"
               OpMemberName %19 0 "projection"
               OpMemberName %19 1 "modelview"
               OpMemberName %19 2 "lodBias"
               OpMemberName %19 3 "samplerIndex"
               OpName %21 "ubo"
               OpName %27 "inPos"
               OpName %42 "gl_PerVertex"
               OpMemberName %42 0 "gl_Position"
               OpMemberName %42 1 "gl_PointSize"
               OpMemberName %42 2 "gl_ClipDistance"
               OpMemberName %42 3 "gl_CullDistance"
               OpName %44 ""
               OpName %60 "outNormal"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpMemberDecorate %19 0 ColMajor
               OpMemberDecorate %19 0 Offset 0
               OpMemberDecorate %19 0 MatrixStride 16
               OpMemberDecorate %19 1 ColMajor
               OpMemberDecorate %19 1 Offset 64
               OpMemberDecorate %19 1 MatrixStride 16
               OpMemberDecorate %19 2 Offset 128
               OpMemberDecorate %19 3 Offset 132
               OpDecorate %19 Block
               OpDecorate %21 DescriptorSet 0
               OpDecorate %21 Binding 0
               OpDecorate %27 Location 0
               OpMemberDecorate %42 0 BuiltIn Position
               OpMemberDecorate %42 1 BuiltIn PointSize
               OpMemberDecorate %42 2 BuiltIn ClipDistance
               OpMemberDecorate %42 3 BuiltIn CullDistance
               OpDecorate %42 Block
               OpDecorate %60 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeVector %6 3
         %14 = OpTypePointer Function %13
         %16 = OpTypeVector %6 4
         %17 = OpTypeMatrix %16 4
         %18 = OpTypeInt 32 1
         %19 = OpTypeStruct %17 %17 %6 %18
         %20 = OpTypePointer Uniform %19
         %21 = OpVariable %20 Uniform
         %22 = OpConstant %18 1
         %23 = OpTypePointer Uniform %17
         %26 = OpTypePointer Input %13
         %27 = OpVariable %26 Input
         %29 = OpConstant %6 1
         %39 = OpTypeInt 32 0
         %40 = OpConstant %39 1
         %41 = OpTypeArray %6 %40
         %42 = OpTypeStruct %16 %6 %41 %41
         %43 = OpTypePointer Output %42
         %44 = OpVariable %43 Output
         %45 = OpConstant %18 0
         %57 = OpTypePointer Output %16
         %59 = OpTypePointer Output %13
         %60 = OpVariable %59 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %15 = OpVariable %14 Function
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %24 = OpAccessChain %23 %21 %22
         %25 = OpLoad %17 %24
         %28 = OpLoad %13 %27
         %30 = OpCompositeExtract %6 %28 0
         %31 = OpCompositeExtract %6 %28 1
         %32 = OpCompositeExtract %6 %28 2
         %33 = OpCompositeConstruct %16 %30 %31 %32 %29
         %34 = OpMatrixTimesVector %16 %25 %33
         %35 = OpCompositeExtract %6 %34 0
         %36 = OpCompositeExtract %6 %34 1
         %37 = OpCompositeExtract %6 %34 2
         %38 = OpCompositeConstruct %13 %35 %36 %37
               OpStore %15 %38
         %46 = OpAccessChain %23 %21 %45
         %47 = OpLoad %17 %46
         %48 = OpAccessChain %23 %21 %22
         %49 = OpLoad %17 %48
         %50 = OpMatrixTimesMatrix %17 %47 %49
         %51 = OpLoad %13 %27
         %52 = OpCompositeExtract %6 %51 0
         %53 = OpCompositeExtract %6 %51 1
         %54 = OpCompositeExtract %6 %51 2
         %55 = OpCompositeConstruct %16 %52 %53 %54 %29
         %56 = OpMatrixTimesVector %16 %50 %55
         %58 = OpAccessChain %57 %44 %45
               OpStore %58 %56
               OpReturn
               OpFunctionEnd
