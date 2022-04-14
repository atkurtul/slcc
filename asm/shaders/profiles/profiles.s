; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 28
; Schema: 0
               OpCapability Shader
               OpCapability ShaderNonUniform
               OpCapability RuntimeDescriptorArray
               OpCapability SampledImageArrayNonUniformIndexing
               OpExtension "SPV_EXT_descriptor_indexing"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %9 %17 %25
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_nonuniform_qualifier"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outFragColor"
               OpName %14 "textures"
               OpName %17 "inTexIndex"
               OpName %25 "inUV"
               OpDecorate %9 Location 0
               OpDecorate %14 DescriptorSet 0
               OpDecorate %14 Binding 1
               OpDecorate %17 Flat
               OpDecorate %17 Location 1
               OpDecorate %19 NonUniform
               OpDecorate %21 NonUniform
               OpDecorate %22 NonUniform
               OpDecorate %25 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypeRuntimeArray %11
         %13 = OpTypePointer UniformConstant %12
         %14 = OpVariable %13 UniformConstant
         %15 = OpTypeInt 32 1
         %16 = OpTypePointer Input %15
         %17 = OpVariable %16 Input
         %20 = OpTypePointer UniformConstant %11
         %23 = OpTypeVector %6 2
         %24 = OpTypePointer Input %23
         %25 = OpVariable %24 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %18 = OpLoad %15 %17
         %19 = OpCopyObject %15 %18
         %21 = OpAccessChain %20 %14 %19
         %22 = OpLoad %11 %21
         %26 = OpLoad %23 %25
         %27 = OpImageSampleImplicitLod %7 %22 %26
               OpStore %9 %27
               OpReturn
               OpFunctionEnd
