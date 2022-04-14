; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 86
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %12 %61 %82 %83
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "spriteSize"
               OpName %12 "inPos"
               OpName %20 "eyePos"
               OpName %23 "UBO"
               OpMemberName %23 0 "projection"
               OpMemberName %23 1 "modelview"
               OpMemberName %23 2 "screendim"
               OpName %25 "ubo"
               OpName %43 "projectedCorner"
               OpName %59 "gl_PerVertex"
               OpMemberName %59 0 "gl_Position"
               OpMemberName %59 1 "gl_PointSize"
               OpMemberName %59 2 "gl_ClipDistance"
               OpMemberName %59 3 "gl_CullDistance"
               OpName %61 ""
               OpName %82 "outGradientPos"
               OpName %83 "inVel"
               OpDecorate %12 Location 0
               OpMemberDecorate %23 0 ColMajor
               OpMemberDecorate %23 0 Offset 0
               OpMemberDecorate %23 0 MatrixStride 16
               OpMemberDecorate %23 1 ColMajor
               OpMemberDecorate %23 1 Offset 64
               OpMemberDecorate %23 1 MatrixStride 16
               OpMemberDecorate %23 2 Offset 128
               OpDecorate %23 Block
               OpDecorate %25 DescriptorSet 0
               OpDecorate %25 Binding 2
               OpMemberDecorate %59 0 BuiltIn Position
               OpMemberDecorate %59 1 BuiltIn PointSize
               OpMemberDecorate %59 2 BuiltIn ClipDistance
               OpMemberDecorate %59 3 BuiltIn CullDistance
               OpDecorate %59 Block
               OpDecorate %82 Location 0
               OpDecorate %83 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypePointer Function %6
          %9 = OpConstant %6 0.00499999989
         %10 = OpTypeVector %6 4
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %13 = OpTypeInt 32 0
         %14 = OpConstant %13 3
         %15 = OpTypePointer Input %6
         %19 = OpTypePointer Function %10
         %21 = OpTypeMatrix %10 4
         %22 = OpTypeVector %6 2
         %23 = OpTypeStruct %21 %21 %22
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpTypeInt 32 1
         %27 = OpConstant %26 1
         %28 = OpTypePointer Uniform %21
         %31 = OpConstant %13 0
         %34 = OpConstant %13 1
         %37 = OpConstant %13 2
         %40 = OpConstant %6 1
         %44 = OpConstant %26 0
         %47 = OpConstant %6 0.5
         %58 = OpTypeArray %6 %34
         %59 = OpTypeStruct %10 %6 %58 %58
         %60 = OpTypePointer Output %59
         %61 = OpVariable %60 Output
         %62 = OpConstant %26 2
         %63 = OpTypePointer Uniform %6
         %72 = OpConstant %6 128
         %74 = OpTypePointer Output %6
         %80 = OpTypePointer Output %10
         %82 = OpVariable %74 Output
         %83 = OpVariable %11 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %8 = OpVariable %7 Function
         %20 = OpVariable %19 Function
         %43 = OpVariable %19 Function
         %16 = OpAccessChain %15 %12 %14
         %17 = OpLoad %6 %16
         %18 = OpFMul %6 %9 %17
               OpStore %8 %18
         %29 = OpAccessChain %28 %25 %27
         %30 = OpLoad %21 %29
         %32 = OpAccessChain %15 %12 %31
         %33 = OpLoad %6 %32
         %35 = OpAccessChain %15 %12 %34
         %36 = OpLoad %6 %35
         %38 = OpAccessChain %15 %12 %37
         %39 = OpLoad %6 %38
         %41 = OpCompositeConstruct %10 %33 %36 %39 %40
         %42 = OpMatrixTimesVector %10 %30 %41
               OpStore %20 %42
         %45 = OpAccessChain %28 %25 %44
         %46 = OpLoad %21 %45
         %48 = OpLoad %6 %8
         %49 = OpFMul %6 %47 %48
         %50 = OpLoad %6 %8
         %51 = OpFMul %6 %47 %50
         %52 = OpAccessChain %7 %20 %37
         %53 = OpLoad %6 %52
         %54 = OpAccessChain %7 %20 %14
         %55 = OpLoad %6 %54
         %56 = OpCompositeConstruct %10 %49 %51 %53 %55
         %57 = OpMatrixTimesVector %10 %46 %56
               OpStore %43 %57
         %64 = OpAccessChain %63 %25 %62 %31
         %65 = OpLoad %6 %64
         %66 = OpAccessChain %7 %43 %31
         %67 = OpLoad %6 %66
         %68 = OpFMul %6 %65 %67
         %69 = OpAccessChain %7 %43 %14
         %70 = OpLoad %6 %69
         %71 = OpFDiv %6 %68 %70
         %73 = OpExtInst %6 %1 FClamp %71 %40 %72
         %75 = OpAccessChain %74 %61 %27
               OpStore %75 %73
         %76 = OpAccessChain %28 %25 %44
         %77 = OpLoad %21 %76
         %78 = OpLoad %10 %20
         %79 = OpMatrixTimesVector %10 %77 %78
         %81 = OpAccessChain %80 %61 %44
               OpStore %81 %79
         %84 = OpAccessChain %15 %83 %14
         %85 = OpLoad %6 %84
               OpStore %82 %85
               OpReturn
               OpFunctionEnd
