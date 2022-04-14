; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 49
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %16 %38 %48
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %14 "gl_PerVertex"
               OpMemberName %14 0 "gl_Position"
               OpName %16 ""
               OpName %20 "UBOScene"
               OpMemberName %20 0 "projection"
               OpMemberName %20 1 "view"
               OpName %22 "uboCamera"
               OpName %30 "UBOModel"
               OpMemberName %30 0 "local"
               OpName %32 "uboModel"
               OpName %38 "inPos"
               OpName %48 "inNormal"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 2
               OpMemberDecorate %14 0 BuiltIn Position
               OpDecorate %14 Block
               OpMemberDecorate %20 0 ColMajor
               OpMemberDecorate %20 0 Offset 0
               OpMemberDecorate %20 0 MatrixStride 16
               OpMemberDecorate %20 1 ColMajor
               OpMemberDecorate %20 1 Offset 64
               OpMemberDecorate %20 1 MatrixStride 16
               OpDecorate %20 Block
               OpDecorate %22 DescriptorSet 0
               OpDecorate %22 Binding 0
               OpMemberDecorate %30 0 ColMajor
               OpMemberDecorate %30 0 Offset 0
               OpMemberDecorate %30 0 MatrixStride 16
               OpDecorate %30 Block
               OpDecorate %32 DescriptorSet 0
               OpDecorate %32 Binding 1
               OpDecorate %38 Location 0
               OpDecorate %48 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeVector %6 4
         %14 = OpTypeStruct %13
         %15 = OpTypePointer Output %14
         %16 = OpVariable %15 Output
         %17 = OpTypeInt 32 1
         %18 = OpConstant %17 0
         %19 = OpTypeMatrix %13 4
         %20 = OpTypeStruct %19 %19
         %21 = OpTypePointer Uniform %20
         %22 = OpVariable %21 Uniform
         %23 = OpTypePointer Uniform %19
         %26 = OpConstant %17 1
         %30 = OpTypeStruct %19
         %31 = OpTypePointer Uniform %30
         %32 = OpVariable %31 Uniform
         %36 = OpTypeVector %6 3
         %37 = OpTypePointer Input %36
         %38 = OpVariable %37 Input
         %40 = OpConstant %6 1
         %46 = OpTypePointer Output %13
         %48 = OpVariable %37 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %24 = OpAccessChain %23 %22 %18
         %25 = OpLoad %19 %24
         %27 = OpAccessChain %23 %22 %26
         %28 = OpLoad %19 %27
         %29 = OpMatrixTimesMatrix %19 %25 %28
         %33 = OpAccessChain %23 %32 %18
         %34 = OpLoad %19 %33
         %35 = OpMatrixTimesMatrix %19 %29 %34
         %39 = OpLoad %36 %38
         %41 = OpCompositeExtract %6 %39 0
         %42 = OpCompositeExtract %6 %39 1
         %43 = OpCompositeExtract %6 %39 2
         %44 = OpCompositeConstruct %13 %41 %42 %43 %40
         %45 = OpMatrixTimesVector %13 %35 %44
         %47 = OpAccessChain %46 %16 %18
               OpStore %47 %45
               OpReturn
               OpFunctionEnd
