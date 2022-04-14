; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 86
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %22 %34 %38 %42 %75
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %13 "samplerColorMap"
               OpName %17 "inUV"
               OpName %22 "inColor"
               OpName %33 "N"
               OpName %34 "inNormal"
               OpName %37 "L"
               OpName %38 "inLightVec"
               OpName %41 "V"
               OpName %42 "inViewVec"
               OpName %45 "R"
               OpName %50 "diffuse"
               OpName %58 "specular"
               OpName %75 "outFragColor"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %17 Location 2
               OpDecorate %22 Location 1
               OpDecorate %34 Location 0
               OpDecorate %38 Location 4
               OpDecorate %42 Location 3
               OpDecorate %75 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypeVector %6 2
         %16 = OpTypePointer Input %15
         %17 = OpVariable %16 Input
         %20 = OpTypeVector %6 3
         %21 = OpTypePointer Input %20
         %22 = OpVariable %21 Input
         %24 = OpConstant %6 1
         %30 = OpConstant %6 1.5
         %32 = OpTypePointer Function %20
         %34 = OpVariable %21 Input
         %38 = OpVariable %21 Input
         %42 = OpVariable %21 Input
         %54 = OpConstant %6 0
         %63 = OpConstant %6 4
         %65 = OpConstant %6 0.5
         %66 = OpConstantComposite %20 %65 %65 %65
         %68 = OpTypeInt 32 0
         %69 = OpConstant %68 0
         %70 = OpTypePointer Function %6
         %74 = OpTypePointer Output %7
         %75 = OpVariable %74 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %33 = OpVariable %32 Function
         %37 = OpVariable %32 Function
         %41 = OpVariable %32 Function
         %45 = OpVariable %32 Function
         %50 = OpVariable %32 Function
         %58 = OpVariable %32 Function
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %19 = OpImageSampleImplicitLod %7 %14 %18
         %23 = OpLoad %20 %22
         %25 = OpCompositeExtract %6 %23 0
         %26 = OpCompositeExtract %6 %23 1
         %27 = OpCompositeExtract %6 %23 2
         %28 = OpCompositeConstruct %7 %25 %26 %27 %24
         %29 = OpFMul %7 %19 %28
         %31 = OpVectorTimesScalar %7 %29 %30
               OpStore %9 %31
         %35 = OpLoad %20 %34
         %36 = OpExtInst %20 %1 Normalize %35
               OpStore %33 %36
         %39 = OpLoad %20 %38
         %40 = OpExtInst %20 %1 Normalize %39
               OpStore %37 %40
         %43 = OpLoad %20 %42
         %44 = OpExtInst %20 %1 Normalize %43
               OpStore %41 %44
         %46 = OpLoad %20 %37
         %47 = OpFNegate %20 %46
         %48 = OpLoad %20 %33
         %49 = OpExtInst %20 %1 Reflect %47 %48
               OpStore %45 %49
         %51 = OpLoad %20 %33
         %52 = OpLoad %20 %37
         %53 = OpDot %6 %51 %52
         %55 = OpExtInst %6 %1 FMax %53 %54
         %56 = OpLoad %20 %22
         %57 = OpVectorTimesScalar %20 %56 %55
               OpStore %50 %57
         %59 = OpLoad %20 %45
         %60 = OpLoad %20 %41
         %61 = OpDot %6 %59 %60
         %62 = OpExtInst %6 %1 FMax %61 %54
         %64 = OpExtInst %6 %1 Pow %62 %63
         %67 = OpVectorTimesScalar %20 %66 %64
         %71 = OpAccessChain %70 %9 %69
         %72 = OpLoad %6 %71
         %73 = OpVectorTimesScalar %20 %67 %72
               OpStore %58 %73
         %76 = OpLoad %20 %50
         %77 = OpLoad %7 %9
         %78 = OpVectorShuffle %20 %77 %77 0 1 2
         %79 = OpFMul %20 %76 %78
         %80 = OpLoad %20 %58
         %81 = OpFAdd %20 %79 %80
         %82 = OpCompositeExtract %6 %81 0
         %83 = OpCompositeExtract %6 %81 1
         %84 = OpCompositeExtract %6 %81 2
         %85 = OpCompositeConstruct %7 %82 %83 %84 %24
               OpStore %75 %85
               OpReturn
               OpFunctionEnd
