; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 40
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %12 %26
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outUV"
               OpName %12 "gl_VertexIndex"
               OpName %24 "gl_PerVertex"
               OpMemberName %24 0 "gl_Position"
               OpName %26 ""
               OpDecorate %9 Location 0
               OpDecorate %12 BuiltIn VertexIndex
               OpMemberDecorate %24 0 BuiltIn Position
               OpDecorate %24 Block
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeInt 32 1
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %14 = OpConstant %10 1
         %16 = OpConstant %10 2
         %23 = OpTypeVector %6 4
         %24 = OpTypeStruct %23
         %25 = OpTypePointer Output %24
         %26 = OpVariable %25 Output
         %27 = OpConstant %10 0
         %29 = OpConstant %6 2
         %31 = OpConstant %6 1
         %34 = OpConstant %6 0
         %38 = OpTypePointer Output %23
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %13 = OpLoad %10 %12
         %15 = OpShiftLeftLogical %10 %13 %14
         %17 = OpBitwiseAnd %10 %15 %16
         %18 = OpConvertSToF %6 %17
         %19 = OpLoad %10 %12
         %20 = OpBitwiseAnd %10 %19 %16
         %21 = OpConvertSToF %6 %20
         %22 = OpCompositeConstruct %7 %18 %21
               OpStore %9 %22
         %28 = OpLoad %7 %9
         %30 = OpVectorTimesScalar %7 %28 %29
         %32 = OpCompositeConstruct %7 %31 %31
         %33 = OpFSub %7 %30 %32
         %35 = OpCompositeExtract %6 %33 0
         %36 = OpCompositeExtract %6 %33 1
         %37 = OpCompositeConstruct %23 %35 %36 %34 %31
         %39 = OpAccessChain %38 %26 %27
               OpStore %39 %37
               OpReturn
               OpFunctionEnd
