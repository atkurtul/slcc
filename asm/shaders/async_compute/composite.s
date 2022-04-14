; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 42
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %36
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "col"
               OpName %13 "hdr_tex"
               OpName %17 "in_uv"
               OpName %23 "bloom_tex"
               OpName %36 "o_color"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 0
               OpDecorate %17 Location 0
               OpDecorate %23 DescriptorSet 0
               OpDecorate %23 Binding 1
               OpDecorate %36 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypeVector %6 2
         %16 = OpTypePointer Input %15
         %17 = OpVariable %16 Input
         %19 = OpConstant %6 0
         %20 = OpTypeVector %6 4
         %23 = OpVariable %12 UniformConstant
         %30 = OpConstant %6 1
         %35 = OpTypePointer Output %20
         %36 = OpVariable %35 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %21 = OpImageSampleExplicitLod %20 %14 %18 Lod %19
         %22 = OpVectorShuffle %7 %21 %21 0 1 2
         %24 = OpLoad %11 %23
         %25 = OpLoad %15 %17
         %26 = OpImageSampleExplicitLod %20 %24 %25 Lod %19
         %27 = OpVectorShuffle %7 %26 %26 0 1 2
         %28 = OpFAdd %7 %22 %27
               OpStore %9 %28
         %29 = OpLoad %7 %9
         %31 = OpLoad %7 %9
         %32 = OpCompositeConstruct %7 %30 %30 %30
         %33 = OpFAdd %7 %32 %31
         %34 = OpFDiv %7 %29 %33
               OpStore %9 %34
         %37 = OpLoad %7 %9
         %38 = OpCompositeExtract %6 %37 0
         %39 = OpCompositeExtract %6 %37 1
         %40 = OpCompositeExtract %6 %37 2
         %41 = OpCompositeConstruct %20 %38 %39 %40 %30
               OpStore %36 %41
               OpReturn
               OpFunctionEnd
