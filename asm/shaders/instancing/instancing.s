; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 94
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %20 %30 %34 %38 %83
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %13 "samplerArray"
               OpName %17 "inUV"
               OpName %20 "inColor"
               OpName %29 "N"
               OpName %30 "inNormal"
               OpName %33 "L"
               OpName %34 "inLightVec"
               OpName %37 "V"
               OpName %38 "inViewVec"
               OpName %41 "R"
               OpName %46 "diffuse"
               OpName %54 "specular"
               OpName %83 "outFragColor"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %17 Location 2
               OpDecorate %20 Location 1
               OpDecorate %30 Location 0
               OpDecorate %34 Location 4
               OpDecorate %38 Location 3
               OpDecorate %83 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 1 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypeVector %6 3
         %16 = OpTypePointer Input %15
         %17 = OpVariable %16 Input
         %20 = OpVariable %16 Input
         %22 = OpConstant %6 1
         %28 = OpTypePointer Function %15
         %30 = OpVariable %16 Input
         %34 = OpVariable %16 Input
         %38 = OpVariable %16 Input
         %50 = OpConstant %6 0.100000001
         %58 = OpConstant %6 0
         %59 = OpTypeBool
         %68 = OpConstant %6 16
         %70 = OpConstant %6 0.75
         %71 = OpConstantComposite %15 %70 %70 %70
         %73 = OpTypeInt 32 0
         %74 = OpConstant %73 0
         %75 = OpTypePointer Function %6
         %80 = OpConstantComposite %15 %58 %58 %58
         %82 = OpTypePointer Output %7
         %83 = OpVariable %82 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %29 = OpVariable %28 Function
         %33 = OpVariable %28 Function
         %37 = OpVariable %28 Function
         %41 = OpVariable %28 Function
         %46 = OpVariable %28 Function
         %54 = OpVariable %28 Function
         %61 = OpVariable %28 Function
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %19 = OpImageSampleImplicitLod %7 %14 %18
         %21 = OpLoad %15 %20
         %23 = OpCompositeExtract %6 %21 0
         %24 = OpCompositeExtract %6 %21 1
         %25 = OpCompositeExtract %6 %21 2
         %26 = OpCompositeConstruct %7 %23 %24 %25 %22
         %27 = OpFMul %7 %19 %26
               OpStore %9 %27
         %31 = OpLoad %15 %30
         %32 = OpExtInst %15 %1 Normalize %31
               OpStore %29 %32
         %35 = OpLoad %15 %34
         %36 = OpExtInst %15 %1 Normalize %35
               OpStore %33 %36
         %39 = OpLoad %15 %38
         %40 = OpExtInst %15 %1 Normalize %39
               OpStore %37 %40
         %42 = OpLoad %15 %33
         %43 = OpFNegate %15 %42
         %44 = OpLoad %15 %29
         %45 = OpExtInst %15 %1 Reflect %43 %44
               OpStore %41 %45
         %47 = OpLoad %15 %29
         %48 = OpLoad %15 %33
         %49 = OpDot %6 %47 %48
         %51 = OpExtInst %6 %1 FMax %49 %50
         %52 = OpLoad %15 %20
         %53 = OpVectorTimesScalar %15 %52 %51
               OpStore %46 %53
         %55 = OpLoad %15 %29
         %56 = OpLoad %15 %33
         %57 = OpDot %6 %55 %56
         %60 = OpFOrdGreaterThan %59 %57 %58
               OpSelectionMerge %63 None
               OpBranchConditional %60 %62 %79
         %62 = OpLabel
         %64 = OpLoad %15 %41
         %65 = OpLoad %15 %37
         %66 = OpDot %6 %64 %65
         %67 = OpExtInst %6 %1 FMax %66 %58
         %69 = OpExtInst %6 %1 Pow %67 %68
         %72 = OpVectorTimesScalar %15 %71 %69
         %76 = OpAccessChain %75 %9 %74
         %77 = OpLoad %6 %76
         %78 = OpVectorTimesScalar %15 %72 %77
               OpStore %61 %78
               OpBranch %63
         %79 = OpLabel
               OpStore %61 %80
               OpBranch %63
         %63 = OpLabel
         %81 = OpLoad %15 %61
               OpStore %54 %81
         %84 = OpLoad %15 %46
         %85 = OpLoad %7 %9
         %86 = OpVectorShuffle %15 %85 %85 0 1 2
         %87 = OpFMul %15 %84 %86
         %88 = OpLoad %15 %54
         %89 = OpFAdd %15 %87 %88
         %90 = OpCompositeExtract %6 %89 0
         %91 = OpCompositeExtract %6 %89 1
         %92 = OpCompositeExtract %6 %89 2
         %93 = OpCompositeConstruct %7 %90 %91 %92 %22
               OpStore %83 %93
               OpReturn
               OpFunctionEnd
