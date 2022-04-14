; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 46
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %16 %25 %29
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %13 "samplerGradientRamp"
               OpName %16 "inGradientPos"
               OpName %25 "outFragColor"
               OpName %26 "samplerColorMap"
               OpName %29 "gl_PointCoord"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %16 Location 0
               OpDecorate %25 Location 0
               OpDecorate %26 DescriptorSet 0
               OpDecorate %26 Binding 0
               OpDecorate %29 BuiltIn PointCoord
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypePointer Input %6
         %16 = OpVariable %15 Input
         %18 = OpConstant %6 0
         %19 = OpTypeVector %6 2
         %21 = OpTypeVector %6 4
         %24 = OpTypePointer Output %21
         %25 = OpVariable %24 Output
         %26 = OpVariable %12 UniformConstant
         %28 = OpTypePointer Input %19
         %29 = OpVariable %28 Input
         %35 = OpTypeInt 32 0
         %36 = OpConstant %35 0
         %37 = OpTypePointer Output %6
         %40 = OpConstant %35 1
         %43 = OpConstant %35 2
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %14 = OpLoad %11 %13
         %17 = OpLoad %6 %16
         %20 = OpCompositeConstruct %19 %17 %18
         %22 = OpImageSampleImplicitLod %21 %14 %20
         %23 = OpVectorShuffle %7 %22 %22 0 1 2
               OpStore %9 %23
         %27 = OpLoad %11 %26
         %30 = OpLoad %19 %29
         %31 = OpImageSampleImplicitLod %21 %27 %30
         %32 = OpVectorShuffle %7 %31 %31 0 1 2
         %33 = OpLoad %7 %9
         %34 = OpFMul %7 %32 %33
         %38 = OpAccessChain %37 %25 %36
         %39 = OpCompositeExtract %6 %34 0
               OpStore %38 %39
         %41 = OpAccessChain %37 %25 %40
         %42 = OpCompositeExtract %6 %34 1
               OpStore %41 %42
         %44 = OpAccessChain %37 %25 %43
         %45 = OpCompositeExtract %6 %34 2
               OpStore %44 %45
               OpReturn
               OpFunctionEnd
