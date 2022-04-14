; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 85
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %12 %60 %81 %82
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
               OpName %58 "gl_PerVertex"
               OpMemberName %58 0 "gl_Position"
               OpMemberName %58 1 "gl_PointSize"
               OpName %60 ""
               OpName %81 "outGradientPos"
               OpName %82 "inVel"
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
               OpMemberDecorate %58 0 BuiltIn Position
               OpMemberDecorate %58 1 BuiltIn PointSize
               OpDecorate %58 Block
               OpDecorate %81 Location 0
               OpDecorate %82 Location 1
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
         %58 = OpTypeStruct %10 %6
         %59 = OpTypePointer Output %58
         %60 = OpVariable %59 Output
         %61 = OpConstant %26 2
         %62 = OpTypePointer Uniform %6
         %71 = OpConstant %6 128
         %73 = OpTypePointer Output %6
         %79 = OpTypePointer Output %10
         %81 = OpVariable %73 Output
         %82 = OpVariable %11 Input
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
         %63 = OpAccessChain %62 %25 %61 %31
         %64 = OpLoad %6 %63
         %65 = OpAccessChain %7 %43 %31
         %66 = OpLoad %6 %65
         %67 = OpFMul %6 %64 %66
         %68 = OpAccessChain %7 %43 %14
         %69 = OpLoad %6 %68
         %70 = OpFDiv %6 %67 %69
         %72 = OpExtInst %6 %1 FClamp %70 %40 %71
         %74 = OpAccessChain %73 %60 %27
               OpStore %74 %72
         %75 = OpAccessChain %28 %25 %44
         %76 = OpLoad %21 %75
         %77 = OpLoad %10 %20
         %78 = OpMatrixTimesVector %10 %76 %77
         %80 = OpAccessChain %79 %60 %44
               OpStore %80 %78
         %83 = OpAccessChain %15 %82 %14
         %84 = OpLoad %6 %83
               OpStore %81 %84
               OpReturn
               OpFunctionEnd
