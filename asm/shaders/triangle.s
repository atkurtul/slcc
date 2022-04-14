; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 52
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %32 %36 %47
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %12 "triangle_positions"
               OpName %22 "triangle_colors"
               OpName %30 "gl_PerVertex"
               OpMemberName %30 0 "gl_Position"
               OpMemberName %30 1 "gl_PointSize"
               OpName %32 ""
               OpName %36 "gl_VertexIndex"
               OpName %47 "out_color"
               OpDecorate %12 RelaxedPrecision
               OpDecorate %22 RelaxedPrecision
               OpMemberDecorate %30 0 BuiltIn Position
               OpMemberDecorate %30 1 BuiltIn PointSize
               OpDecorate %30 Block
               OpDecorate %36 BuiltIn VertexIndex
               OpDecorate %40 RelaxedPrecision
               OpDecorate %41 RelaxedPrecision
               OpDecorate %42 RelaxedPrecision
               OpDecorate %43 RelaxedPrecision
               OpDecorate %47 RelaxedPrecision
               OpDecorate %47 Location 0
               OpDecorate %51 RelaxedPrecision
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypeInt 32 0
          %9 = OpConstant %8 3
         %10 = OpTypeArray %7 %9
         %11 = OpTypePointer Private %10
         %12 = OpVariable %11 Private
         %13 = OpConstant %6 0.5
         %14 = OpConstant %6 -0.5
         %15 = OpConstantComposite %7 %13 %14
         %16 = OpConstantComposite %7 %13 %13
         %17 = OpConstantComposite %7 %14 %13
         %18 = OpConstantComposite %10 %15 %16 %17
         %19 = OpTypeVector %6 3
         %20 = OpTypeArray %19 %9
         %21 = OpTypePointer Private %20
         %22 = OpVariable %21 Private
         %23 = OpConstant %6 1
         %24 = OpConstant %6 0
         %25 = OpConstantComposite %19 %23 %24 %24
         %26 = OpConstantComposite %19 %24 %23 %24
         %27 = OpConstantComposite %19 %24 %24 %23
         %28 = OpConstantComposite %20 %25 %26 %27
         %29 = OpTypeVector %6 4
         %30 = OpTypeStruct %29 %6
         %31 = OpTypePointer Output %30
         %32 = OpVariable %31 Output
         %33 = OpTypeInt 32 1
         %34 = OpConstant %33 0
         %35 = OpTypePointer Input %33
         %36 = OpVariable %35 Input
         %38 = OpTypePointer Private %7
         %44 = OpTypePointer Output %29
         %46 = OpTypePointer Output %19
         %47 = OpVariable %46 Output
         %49 = OpTypePointer Private %19
          %4 = OpFunction %2 None %3
          %5 = OpLabel
               OpStore %12 %18
               OpStore %22 %28
         %37 = OpLoad %33 %36
         %39 = OpAccessChain %38 %12 %37
         %40 = OpLoad %7 %39
         %41 = OpCompositeExtract %6 %40 0
         %42 = OpCompositeExtract %6 %40 1
         %43 = OpCompositeConstruct %29 %41 %42 %24 %23
         %45 = OpAccessChain %44 %32 %34
               OpStore %45 %43
         %48 = OpLoad %33 %36
         %50 = OpAccessChain %49 %22 %48
         %51 = OpLoad %19 %50
               OpStore %47 %51
               OpReturn
               OpFunctionEnd
