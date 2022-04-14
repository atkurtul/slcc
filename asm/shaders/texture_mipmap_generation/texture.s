; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 57
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %37 %50
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "color"
               OpName %12 "textureColor"
               OpName %19 "samplers"
               OpName %23 "UBO"
               OpMemberName %23 0 "projection"
               OpMemberName %23 1 "modelview"
               OpMemberName %23 2 "lodBias"
               OpMemberName %23 3 "samplerIndex"
               OpName %25 "ubo"
               OpName %37 "inUV"
               OpName %50 "outFragColor"
               OpDecorate %12 DescriptorSet 0
               OpDecorate %12 Binding 1
               OpDecorate %19 DescriptorSet 0
               OpDecorate %19 Binding 2
               OpMemberDecorate %23 0 ColMajor
               OpMemberDecorate %23 0 Offset 0
               OpMemberDecorate %23 0 MatrixStride 16
               OpMemberDecorate %23 1 ColMajor
               OpMemberDecorate %23 1 Offset 64
               OpMemberDecorate %23 1 MatrixStride 16
               OpMemberDecorate %23 2 Offset 128
               OpMemberDecorate %23 3 Offset 132
               OpDecorate %23 Block
               OpDecorate %25 DescriptorSet 0
               OpDecorate %25 Binding 0
               OpDecorate %37 Location 0
               OpDecorate %50 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypePointer UniformConstant %10
         %12 = OpVariable %11 UniformConstant
         %14 = OpTypeSampler
         %15 = OpTypeInt 32 0
         %16 = OpConstant %15 3
         %17 = OpTypeArray %14 %16
         %18 = OpTypePointer UniformConstant %17
         %19 = OpVariable %18 UniformConstant
         %20 = OpTypeVector %6 4
         %21 = OpTypeMatrix %20 4
         %22 = OpTypeInt 32 1
         %23 = OpTypeStruct %21 %21 %6 %22
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpConstant %22 3
         %27 = OpTypePointer Uniform %22
         %30 = OpTypePointer UniformConstant %14
         %33 = OpTypeSampledImage %10
         %35 = OpTypeVector %6 2
         %36 = OpTypePointer Input %35
         %37 = OpVariable %36 Input
         %39 = OpConstant %6 2
         %40 = OpConstant %6 0.25
         %41 = OpConstantComposite %35 %39 %40
         %43 = OpConstant %22 2
         %44 = OpTypePointer Uniform %6
         %49 = OpTypePointer Output %20
         %50 = OpVariable %49 Output
         %52 = OpConstant %6 1
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %13 = OpLoad %10 %12
         %28 = OpAccessChain %27 %25 %26
         %29 = OpLoad %22 %28
         %31 = OpAccessChain %30 %19 %29
         %32 = OpLoad %14 %31
         %34 = OpSampledImage %33 %13 %32
         %38 = OpLoad %35 %37
         %42 = OpFMul %35 %38 %41
         %45 = OpAccessChain %44 %25 %43
         %46 = OpLoad %6 %45
         %47 = OpImageSampleImplicitLod %20 %34 %42 Bias %46
         %48 = OpVectorShuffle %7 %47 %47 0 1 2
               OpStore %9 %48
         %51 = OpLoad %7 %9
         %53 = OpCompositeExtract %6 %51 0
         %54 = OpCompositeExtract %6 %51 1
         %55 = OpCompositeExtract %6 %51 2
         %56 = OpCompositeConstruct %20 %53 %54 %55 %52
               OpStore %50 %56
               OpReturn
               OpFunctionEnd
