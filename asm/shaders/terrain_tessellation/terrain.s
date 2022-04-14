; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 37
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %13 %18 %29 %31 %34 %35
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
               OpName %18 "inPos"
               OpName %29 "outUV"
               OpName %31 "inUV"
               OpName %34 "outNormal"
               OpName %35 "inNormal"
               OpMemberDecorate %11 0 BuiltIn Position
               OpMemberDecorate %11 1 BuiltIn PointSize
               OpMemberDecorate %11 2 BuiltIn ClipDistance
               OpMemberDecorate %11 3 BuiltIn CullDistance
               OpDecorate %11 Block
               OpDecorate %18 Location 0
               OpDecorate %29 Location 1
               OpDecorate %31 Location 2
               OpDecorate %34 Location 0
               OpDecorate %35 Location 1
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
         %16 = OpTypeVector %6 3
         %17 = OpTypePointer Input %16
         %18 = OpVariable %17 Input
         %20 = OpConstant %6 1
         %25 = OpTypePointer Output %7
         %27 = OpTypeVector %6 2
         %28 = OpTypePointer Output %27
         %29 = OpVariable %28 Output
         %30 = OpTypePointer Input %27
         %31 = OpVariable %30 Input
         %33 = OpTypePointer Output %16
         %34 = OpVariable %33 Output
         %35 = OpVariable %17 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %19 = OpLoad %16 %18
         %21 = OpCompositeExtract %6 %19 0
         %22 = OpCompositeExtract %6 %19 1
         %23 = OpCompositeExtract %6 %19 2
         %24 = OpCompositeConstruct %7 %21 %22 %23 %20
         %26 = OpAccessChain %25 %13 %15
               OpStore %26 %24
         %32 = OpLoad %27 %31
               OpStore %29 %32
         %36 = OpLoad %16 %35
               OpStore %34 %36
               OpReturn
               OpFunctionEnd
