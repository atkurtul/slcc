; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 78
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %20 %27 %31 %35 %66
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %13 "samplerColor"
               OpName %17 "inUV"
               OpName %20 "inLodBias"
               OpName %25 "N"
               OpName %27 "inNormal"
               OpName %30 "L"
               OpName %31 "inLightVec"
               OpName %34 "V"
               OpName %35 "inViewVec"
               OpName %38 "R"
               OpName %43 "diffuse"
               OpName %53 "specular"
               OpName %66 "outFragColor"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %17 Location 0
               OpDecorate %20 Location 1
               OpDecorate %27 Location 2
               OpDecorate %31 Location 4
               OpDecorate %35 Location 3
               OpDecorate %66 Location 0
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
         %19 = OpTypePointer Input %6
         %20 = OpVariable %19 Input
         %23 = OpTypeVector %6 3
         %24 = OpTypePointer Function %23
         %26 = OpTypePointer Input %23
         %27 = OpVariable %26 Input
         %31 = OpVariable %26 Input
         %35 = OpVariable %26 Input
         %47 = OpConstant %6 0
         %49 = OpConstant %6 1
         %50 = OpConstantComposite %23 %49 %49 %49
         %52 = OpTypePointer Function %6
         %58 = OpConstant %6 16
         %60 = OpTypeInt 32 0
         %61 = OpConstant %60 3
         %65 = OpTypePointer Output %7
         %66 = OpVariable %65 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %25 = OpVariable %24 Function
         %30 = OpVariable %24 Function
         %34 = OpVariable %24 Function
         %38 = OpVariable %24 Function
         %43 = OpVariable %24 Function
         %53 = OpVariable %52 Function
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %21 = OpLoad %6 %20
         %22 = OpImageSampleImplicitLod %7 %14 %18 Bias %21
               OpStore %9 %22
         %28 = OpLoad %23 %27
         %29 = OpExtInst %23 %1 Normalize %28
               OpStore %25 %29
         %32 = OpLoad %23 %31
         %33 = OpExtInst %23 %1 Normalize %32
               OpStore %30 %33
         %36 = OpLoad %23 %35
         %37 = OpExtInst %23 %1 Normalize %36
               OpStore %34 %37
         %39 = OpLoad %23 %30
         %40 = OpFNegate %23 %39
         %41 = OpLoad %23 %25
         %42 = OpExtInst %23 %1 Reflect %40 %41
               OpStore %38 %42
         %44 = OpLoad %23 %25
         %45 = OpLoad %23 %30
         %46 = OpDot %6 %44 %45
         %48 = OpExtInst %6 %1 FMax %46 %47
         %51 = OpVectorTimesScalar %23 %50 %48
               OpStore %43 %51
         %54 = OpLoad %23 %38
         %55 = OpLoad %23 %34
         %56 = OpDot %6 %54 %55
         %57 = OpExtInst %6 %1 FMax %56 %47
         %59 = OpExtInst %6 %1 Pow %57 %58
         %62 = OpAccessChain %52 %9 %61
         %63 = OpLoad %6 %62
         %64 = OpFMul %6 %59 %63
               OpStore %53 %64
         %67 = OpLoad %23 %43
         %68 = OpLoad %7 %9
         %69 = OpVectorShuffle %23 %68 %68 0 1 2
         %70 = OpFMul %23 %67 %69
         %71 = OpLoad %6 %53
         %72 = OpCompositeConstruct %23 %71 %71 %71
         %73 = OpFAdd %23 %70 %72
         %74 = OpCompositeExtract %6 %73 0
         %75 = OpCompositeExtract %6 %73 1
         %76 = OpCompositeExtract %6 %73 2
         %77 = OpCompositeConstruct %7 %74 %75 %76 %49
               OpStore %66 %77
               OpReturn
               OpFunctionEnd
