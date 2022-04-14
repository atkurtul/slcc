; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 47
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %13 %25 %37 %39 %46
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %11 "gl_PerVertex"
               OpMemberName %11 0 "gl_Position"
               OpMemberName %11 1 "gl_PointSize"
               OpMemberName %11 2 "gl_ClipDistance"
               OpMemberName %11 3 "gl_CullDistance"
               OpName %13 ""
               OpName %17 "UBO"
               OpMemberName %17 0 "mvp"
               OpName %19 "ubo"
               OpName %25 "inPos"
               OpName %37 "outUV"
               OpName %39 "inUV"
               OpName %46 "inNormal"
               OpMemberDecorate %11 0 BuiltIn Position
               OpMemberDecorate %11 1 BuiltIn PointSize
               OpMemberDecorate %11 2 BuiltIn ClipDistance
               OpMemberDecorate %11 3 BuiltIn CullDistance
               OpDecorate %11 Block
               OpMemberDecorate %17 0 ColMajor
               OpMemberDecorate %17 0 Offset 0
               OpMemberDecorate %17 0 MatrixStride 16
               OpDecorate %17 Block
               OpDecorate %19 DescriptorSet 0
               OpDecorate %19 Binding 0
               OpDecorate %25 Location 0
               OpDecorate %37 Location 0
               OpDecorate %39 Location 2
               OpDecorate %46 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypeInt 32 0
          %9 = OpConstant %8 1
         %10 = OpTypeArray %6 %9
         %11 = OpTypeStruct %7 %6 %10 %10
         %12 = OpTypePointer Output %11
         %13 = OpVariable %12 Output
         %14 = OpTypeInt 32 1
         %15 = OpConstant %14 0
         %16 = OpTypeMatrix %7 4
         %17 = OpTypeStruct %16
         %18 = OpTypePointer Uniform %17
         %19 = OpVariable %18 Uniform
         %20 = OpTypePointer Uniform %16
         %23 = OpTypeVector %6 3
         %24 = OpTypePointer Input %23
         %25 = OpVariable %24 Input
         %27 = OpConstant %6 1
         %33 = OpTypePointer Output %7
         %35 = OpTypeVector %6 2
         %36 = OpTypePointer Output %35
         %37 = OpVariable %36 Output
         %38 = OpTypePointer Input %35
         %39 = OpVariable %38 Input
         %41 = OpTypePointer Output %6
         %46 = OpVariable %24 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %21 = OpAccessChain %20 %19 %15
         %22 = OpLoad %16 %21
         %26 = OpLoad %23 %25
         %28 = OpCompositeExtract %6 %26 0
         %29 = OpCompositeExtract %6 %26 1
         %30 = OpCompositeExtract %6 %26 2
         %31 = OpCompositeConstruct %7 %28 %29 %30 %27
         %32 = OpMatrixTimesVector %7 %22 %31
         %34 = OpAccessChain %33 %13 %15
               OpStore %34 %32
         %40 = OpLoad %35 %39
               OpStore %37 %40
         %42 = OpAccessChain %41 %37 %9
         %43 = OpLoad %6 %42
         %44 = OpFSub %6 %27 %43
         %45 = OpAccessChain %41 %37 %9
               OpStore %45 %44
               OpReturn
               OpFunctionEnd
