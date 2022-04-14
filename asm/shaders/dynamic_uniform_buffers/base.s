; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 62
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %11 %35 %49
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outColor"
               OpName %11 "inColor"
               OpName %16 "modelView"
               OpName %17 "UboView"
               OpMemberName %17 0 "projection"
               OpMemberName %17 1 "view"
               OpName %19 "uboView"
               OpName %25 "UboInstance"
               OpMemberName %25 0 "model"
               OpName %27 "uboInstance"
               OpName %33 "worldPos"
               OpName %35 "inPos"
               OpName %47 "gl_PerVertex"
               OpMemberName %47 0 "gl_Position"
               OpName %49 ""
               OpDecorate %9 Location 0
               OpDecorate %11 Location 1
               OpMemberDecorate %17 0 ColMajor
               OpMemberDecorate %17 0 Offset 0
               OpMemberDecorate %17 0 MatrixStride 16
               OpMemberDecorate %17 1 ColMajor
               OpMemberDecorate %17 1 Offset 64
               OpMemberDecorate %17 1 MatrixStride 16
               OpDecorate %17 Block
               OpDecorate %19 DescriptorSet 0
               OpDecorate %19 Binding 0
               OpMemberDecorate %25 0 ColMajor
               OpMemberDecorate %25 0 Offset 0
               OpMemberDecorate %25 0 MatrixStride 16
               OpDecorate %25 Block
               OpDecorate %27 DescriptorSet 0
               OpDecorate %27 Binding 1
               OpDecorate %35 Location 0
               OpMemberDecorate %47 0 BuiltIn Position
               OpDecorate %47 Block
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpTypeVector %6 4
         %14 = OpTypeMatrix %13 4
         %15 = OpTypePointer Function %14
         %17 = OpTypeStruct %14 %14
         %18 = OpTypePointer Uniform %17
         %19 = OpVariable %18 Uniform
         %20 = OpTypeInt 32 1
         %21 = OpConstant %20 1
         %22 = OpTypePointer Uniform %14
         %25 = OpTypeStruct %14
         %26 = OpTypePointer Uniform %25
         %27 = OpVariable %26 Uniform
         %28 = OpConstant %20 0
         %32 = OpTypePointer Function %7
         %35 = OpVariable %10 Input
         %37 = OpConstant %6 1
         %47 = OpTypeStruct %13
         %48 = OpTypePointer Output %47
         %49 = OpVariable %48 Output
         %60 = OpTypePointer Output %13
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %16 = OpVariable %15 Function
         %33 = OpVariable %32 Function
         %12 = OpLoad %7 %11
               OpStore %9 %12
         %23 = OpAccessChain %22 %19 %21
         %24 = OpLoad %14 %23
         %29 = OpAccessChain %22 %27 %28
         %30 = OpLoad %14 %29
         %31 = OpMatrixTimesMatrix %14 %24 %30
               OpStore %16 %31
         %34 = OpLoad %14 %16
         %36 = OpLoad %7 %35
         %38 = OpCompositeExtract %6 %36 0
         %39 = OpCompositeExtract %6 %36 1
         %40 = OpCompositeExtract %6 %36 2
         %41 = OpCompositeConstruct %13 %38 %39 %40 %37
         %42 = OpMatrixTimesVector %13 %34 %41
         %43 = OpCompositeExtract %6 %42 0
         %44 = OpCompositeExtract %6 %42 1
         %45 = OpCompositeExtract %6 %42 2
         %46 = OpCompositeConstruct %7 %43 %44 %45
               OpStore %33 %46
         %50 = OpAccessChain %22 %19 %28
         %51 = OpLoad %14 %50
         %52 = OpLoad %14 %16
         %53 = OpMatrixTimesMatrix %14 %51 %52
         %54 = OpLoad %7 %35
         %55 = OpCompositeExtract %6 %54 0
         %56 = OpCompositeExtract %6 %54 1
         %57 = OpCompositeExtract %6 %54 2
         %58 = OpCompositeConstruct %13 %55 %56 %57 %37
         %59 = OpMatrixTimesVector %13 %53 %58
         %61 = OpAccessChain %60 %49 %28
               OpStore %61 %59
               OpReturn
               OpFunctionEnd
