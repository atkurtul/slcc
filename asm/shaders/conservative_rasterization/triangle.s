; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 40
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %16 %30
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outColor"
               OpName %11 "inColor"
               OpName %14 "gl_PerVertex"
               OpMemberName %14 0 "gl_Position"
               OpName %16 ""
               OpName %20 "UBO"
               OpMemberName %20 0 "projection"
               OpMemberName %20 1 "model"
               OpName %22 "ubo"
               OpName %30 "inPos"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
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
               OpDecorate %30 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
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
         %30 = OpVariable %10 Input
         %32 = OpConstant %6 1
         %38 = OpTypePointer Output %13
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %24 = OpAccessChain %23 %22 %18
         %25 = OpLoad %19 %24
         %27 = OpAccessChain %23 %22 %26
         %28 = OpLoad %19 %27
         %29 = OpMatrixTimesMatrix %19 %25 %28
         %31 = OpLoad %7 %30
         %33 = OpCompositeExtract %6 %31 0
         %34 = OpCompositeExtract %6 %31 1
         %35 = OpCompositeExtract %6 %31 2
         %36 = OpCompositeConstruct %13 %33 %34 %35 %32
         %37 = OpMatrixTimesVector %13 %29 %36
         %39 = OpAccessChain %38 %16 %18
               OpStore %39 %37
               OpReturn
               OpFunctionEnd
