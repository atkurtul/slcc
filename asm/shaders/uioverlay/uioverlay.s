; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 40
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %15 %17 %21 %31
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %11 "inUV"
               OpName %15 "outColor"
               OpName %17 "inColor"
               OpName %19 "gl_PerVertex"
               OpMemberName %19 0 "gl_Position"
               OpName %21 ""
               OpName %25 "PushConstants"
               OpMemberName %25 0 "transform"
               OpName %27 "pushConstants"
               OpName %31 "inPos"
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpDecorate %15 Location 1
               OpDecorate %17 Location 2
               OpMemberDecorate %19 0 BuiltIn Position
               OpDecorate %19 Block
               OpMemberDecorate %25 0 ColMajor
               OpMemberDecorate %25 0 Offset 0
               OpMemberDecorate %25 0 MatrixStride 16
               OpDecorate %25 Block
               OpDecorate %31 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeVector %6 4
         %14 = OpTypePointer Output %13
         %15 = OpVariable %14 Output
         %16 = OpTypePointer Input %13
         %17 = OpVariable %16 Input
         %19 = OpTypeStruct %13
         %20 = OpTypePointer Output %19
         %21 = OpVariable %20 Output
         %22 = OpTypeInt 32 1
         %23 = OpConstant %22 0
         %24 = OpTypeMatrix %13 4
         %25 = OpTypeStruct %24
         %26 = OpTypePointer PushConstant %25
         %27 = OpVariable %26 PushConstant
         %28 = OpTypePointer PushConstant %24
         %31 = OpVariable %10 Input
         %33 = OpConstant %6 0
         %34 = OpConstant %6 1
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %18 = OpLoad %13 %17
               OpStore %15 %18
         %29 = OpAccessChain %28 %27 %23
         %30 = OpLoad %24 %29
         %32 = OpLoad %7 %31
         %35 = OpCompositeExtract %6 %32 0
         %36 = OpCompositeExtract %6 %32 1
         %37 = OpCompositeConstruct %13 %35 %36 %33 %34
         %38 = OpMatrixTimesVector %13 %30 %37
         %39 = OpAccessChain %14 %21 %23
               OpStore %39 %38
               OpReturn
               OpFunctionEnd
