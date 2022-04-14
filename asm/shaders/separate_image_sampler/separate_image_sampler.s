; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 28
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %22 %26
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %12 "_texture"
               OpName %16 "_sampler"
               OpName %22 "inUV"
               OpName %26 "outFragColor"
               OpDecorate %12 DescriptorSet 0
               OpDecorate %12 Binding 1
               OpDecorate %16 DescriptorSet 1
               OpDecorate %16 Binding 0
               OpDecorate %22 Location 0
               OpDecorate %26 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypePointer UniformConstant %10
         %12 = OpVariable %11 UniformConstant
         %14 = OpTypeSampler
         %15 = OpTypePointer UniformConstant %14
         %16 = OpVariable %15 UniformConstant
         %18 = OpTypeSampledImage %10
         %20 = OpTypeVector %6 2
         %21 = OpTypePointer Input %20
         %22 = OpVariable %21 Input
         %25 = OpTypePointer Output %7
         %26 = OpVariable %25 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %13 = OpLoad %10 %12
         %17 = OpLoad %14 %16
         %19 = OpSampledImage %18 %13 %17
         %23 = OpLoad %20 %22
         %24 = OpImageSampleImplicitLod %7 %19 %23
               OpStore %9 %24
         %27 = OpLoad %7 %9
               OpStore %26 %27
               OpReturn
               OpFunctionEnd
