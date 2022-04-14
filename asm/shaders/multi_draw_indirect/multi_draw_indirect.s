; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 51
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %9 %18 %28
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "o_color"
               OpName %16 "textures"
               OpName %18 "in_texture_index"
               OpName %28 "in_uv"
               OpDecorate %9 Location 0
               OpDecorate %16 DescriptorSet 0
               OpDecorate %16 Binding 1
               OpDecorate %18 Flat
               OpDecorate %18 Location 2
               OpDecorate %28 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypeInt 32 0
         %13 = OpConstant %12 225
         %14 = OpTypeArray %11 %13
         %15 = OpTypePointer UniformConstant %14
         %16 = OpVariable %15 UniformConstant
         %17 = OpTypePointer Input %12
         %18 = OpVariable %17 Input
         %23 = OpTypePointer UniformConstant %11
         %26 = OpTypeVector %6 2
         %27 = OpTypePointer Input %26
         %28 = OpVariable %27 Input
         %36 = OpConstant %6 1.5
         %37 = OpTypeVector %6 3
         %41 = OpConstant %12 0
         %42 = OpTypePointer Output %6
         %45 = OpConstant %12 1
         %48 = OpConstant %12 2
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %19 = OpLoad %12 %18
         %20 = OpConvertUToF %6 %19
         %21 = OpExtInst %6 %1 Round %20
         %22 = OpConvertFToU %12 %21
         %24 = OpAccessChain %23 %16 %22
         %25 = OpLoad %11 %24
         %29 = OpLoad %26 %28
         %30 = OpImageSampleImplicitLod %7 %25 %29
         %31 = OpCompositeExtract %6 %30 0
         %32 = OpCompositeExtract %6 %30 1
         %33 = OpCompositeExtract %6 %30 2
         %34 = OpCompositeExtract %6 %30 3
         %35 = OpCompositeConstruct %7 %31 %32 %33 %34
               OpStore %9 %35
         %38 = OpLoad %7 %9
         %39 = OpVectorShuffle %37 %38 %38 0 1 2
         %40 = OpVectorTimesScalar %37 %39 %36
         %43 = OpAccessChain %42 %9 %41
         %44 = OpCompositeExtract %6 %40 0
               OpStore %43 %44
         %46 = OpAccessChain %42 %9 %45
         %47 = OpCompositeExtract %6 %40 1
               OpStore %46 %47
         %49 = OpAccessChain %42 %9 %48
         %50 = OpCompositeExtract %6 %40 2
               OpStore %49 %50
               OpReturn
               OpFunctionEnd
