; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 37
; Schema: 0
               OpCapability Shader
               OpCapability RuntimeDescriptorArray
               OpExtension "SPV_EXT_descriptor_indexing"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %9 %34
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_nonuniform_qualifier"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "out_frag_color"
               OpName %13 "Textures"
               OpName %15 "Registers"
               OpMemberName %15 0 "table_offset"
               OpName %17 "registers"
               OpName %28 "ImmutableSampler"
               OpName %34 "in_uv"
               OpDecorate %9 Location 0
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 0
               OpMemberDecorate %15 0 Offset 4
               OpDecorate %15 Block
               OpDecorate %28 DescriptorSet 1
               OpDecorate %28 Binding 0
               OpDecorate %34 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeRuntimeArray %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %14 = OpTypeInt 32 0
         %15 = OpTypeStruct %14
         %16 = OpTypePointer PushConstant %15
         %17 = OpVariable %16 PushConstant
         %18 = OpTypeInt 32 1
         %19 = OpConstant %18 0
         %20 = OpTypePointer PushConstant %14
         %23 = OpTypePointer UniformConstant %10
         %26 = OpTypeSampler
         %27 = OpTypePointer UniformConstant %26
         %28 = OpVariable %27 UniformConstant
         %30 = OpTypeSampledImage %10
         %32 = OpTypeVector %6 2
         %33 = OpTypePointer Input %32
         %34 = OpVariable %33 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %21 = OpAccessChain %20 %17 %19
         %22 = OpLoad %14 %21
         %24 = OpAccessChain %23 %13 %22
         %25 = OpLoad %10 %24
         %29 = OpLoad %26 %28
         %31 = OpSampledImage %30 %25 %29
         %35 = OpLoad %32 %34
         %36 = OpImageSampleImplicitLod %7 %31 %35
               OpStore %9 %36
               OpReturn
               OpFunctionEnd
