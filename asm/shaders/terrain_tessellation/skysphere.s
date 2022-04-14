; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 30
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %21
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %13 "samplerColorMap"
               OpName %17 "inUV"
               OpName %21 "outFragColor"
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %17 Location 0
               OpDecorate %21 Location 0
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
         %20 = OpTypePointer Output %7
         %21 = OpVariable %20 Output
         %22 = OpTypeVector %6 3
         %25 = OpConstant %6 1
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %19 = OpImageSampleImplicitLod %7 %14 %18
               OpStore %9 %19
         %23 = OpLoad %7 %9
         %24 = OpVectorShuffle %22 %23 %23 0 1 2
         %26 = OpCompositeExtract %6 %24 0
         %27 = OpCompositeExtract %6 %24 1
         %28 = OpCompositeExtract %6 %24 2
         %29 = OpCompositeConstruct %7 %26 %27 %28 %25
               OpStore %21 %29
               OpReturn
               OpFunctionEnd
