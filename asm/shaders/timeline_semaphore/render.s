; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 55
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %41 %49
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %11 "convert_color(vf4;"
               OpName %10 "value"
               OpName %14 "gray"
               OpName %41 "out_color"
               OpName %45 "in_image"
               OpName %49 "in_uv"
               OpName %53 "param"
               OpDecorate %41 Location 0
               OpDecorate %45 DescriptorSet 0
               OpDecorate %45 Binding 0
               OpDecorate %49 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
          %9 = OpTypeFunction %7 %8
         %13 = OpTypePointer Function %6
         %15 = OpTypeVector %6 3
         %18 = OpConstant %6 0.300000012
         %19 = OpConstant %6 0.600000024
         %20 = OpConstant %6 0.100000001
         %21 = OpConstantComposite %15 %18 %19 %20
         %27 = OpTypeInt 32 0
         %28 = OpConstant %27 3
         %33 = OpConstant %6 1
         %40 = OpTypePointer Output %7
         %41 = OpVariable %40 Output
         %42 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %43 = OpTypeSampledImage %42
         %44 = OpTypePointer UniformConstant %43
         %45 = OpVariable %44 UniformConstant
         %47 = OpTypeVector %6 2
         %48 = OpTypePointer Input %47
         %49 = OpVariable %48 Input
         %51 = OpConstant %6 0
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %53 = OpVariable %8 Function
         %46 = OpLoad %43 %45
         %50 = OpLoad %47 %49
         %52 = OpImageSampleExplicitLod %7 %46 %50 Lod %51
               OpStore %53 %52
         %54 = OpFunctionCall %7 %11 %53
               OpStore %41 %54
               OpReturn
               OpFunctionEnd
         %11 = OpFunction %7 None %9
         %10 = OpFunctionParameter %8
         %12 = OpLabel
         %14 = OpVariable %13 Function
         %16 = OpLoad %7 %10
         %17 = OpVectorShuffle %15 %16 %16 0 1 2
         %22 = OpDot %6 %17 %21
               OpStore %14 %22
         %23 = OpLoad %6 %14
         %24 = OpCompositeConstruct %15 %23 %23 %23
         %25 = OpLoad %7 %10
         %26 = OpVectorShuffle %15 %25 %25 0 1 2
         %29 = OpAccessChain %13 %10 %28
         %30 = OpLoad %6 %29
         %31 = OpCompositeConstruct %15 %30 %30 %30
         %32 = OpExtInst %15 %1 FMix %24 %26 %31
         %34 = OpCompositeExtract %6 %32 0
         %35 = OpCompositeExtract %6 %32 1
         %36 = OpCompositeExtract %6 %32 2
         %37 = OpCompositeConstruct %7 %34 %35 %36 %33
               OpReturnValue %37
               OpFunctionEnd
