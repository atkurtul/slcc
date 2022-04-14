; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 48
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %12 %32
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUVW"
               OpName %12 "gl_VertexIndex"
               OpName %30 "gl_PerVertex"
               OpMemberName %30 0 "gl_Position"
               OpMemberName %30 1 "gl_PointSize"
               OpMemberName %30 2 "gl_ClipDistance"
               OpMemberName %30 3 "gl_CullDistance"
               OpName %32 ""
               OpDecorate %9 Location 0
               OpDecorate %12 BuiltIn VertexIndex
               OpMemberDecorate %30 0 BuiltIn Position
               OpMemberDecorate %30 1 BuiltIn PointSize
               OpMemberDecorate %30 2 BuiltIn ClipDistance
               OpMemberDecorate %30 3 BuiltIn CullDistance
               OpDecorate %30 Block
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeInt 32 1
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %14 = OpConstant %10 1
         %16 = OpConstant %10 2
         %26 = OpTypeVector %6 4
         %27 = OpTypeInt 32 0
         %28 = OpConstant %27 1
         %29 = OpTypeArray %6 %28
         %30 = OpTypeStruct %26 %6 %29 %29
         %31 = OpTypePointer Output %30
         %32 = OpVariable %31 Output
         %33 = OpConstant %10 0
         %34 = OpTypeVector %6 2
         %37 = OpConstant %6 2
         %39 = OpConstant %6 1
         %42 = OpConstant %6 0
         %46 = OpTypePointer Output %26
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %13 = OpLoad %10 %12
         %15 = OpShiftLeftLogical %10 %13 %14
         %17 = OpBitwiseAnd %10 %15 %16
         %18 = OpConvertSToF %6 %17
         %19 = OpLoad %10 %12
         %20 = OpBitwiseAnd %10 %19 %16
         %21 = OpConvertSToF %6 %20
         %22 = OpLoad %10 %12
         %23 = OpBitwiseAnd %10 %22 %16
         %24 = OpConvertSToF %6 %23
         %25 = OpCompositeConstruct %7 %18 %21 %24
               OpStore %9 %25
         %35 = OpLoad %7 %9
         %36 = OpVectorShuffle %34 %35 %35 0 1
         %38 = OpVectorTimesScalar %34 %36 %37
         %40 = OpCompositeConstruct %34 %39 %39
         %41 = OpFSub %34 %38 %40
         %43 = OpCompositeExtract %6 %41 0
         %44 = OpCompositeExtract %6 %41 1
         %45 = OpCompositeConstruct %26 %43 %44 %42 %39
         %47 = OpAccessChain %46 %32 %33
               OpStore %47 %45
               OpReturn
               OpFunctionEnd
