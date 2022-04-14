; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 62
; Schema: 0
               OpCapability Shader
               OpCapability StorageInputOutput16
               OpExtension "SPV_KHR_16bit_storage"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %13 %23 %37 %59
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_explicit_arithmetic_types_float16"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "normal_color"
               OpName %13 "in_normal"
               OpName %22 "color"
               OpName %23 "in_color"
               OpName %37 "in_delta_pos"
               OpName %59 "o_color"
               OpDecorate %13 Location 1
               OpDecorate %23 Location 0
               OpDecorate %37 Location 2
               OpDecorate %59 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
         %10 = OpTypeFloat 16
         %11 = OpTypeVector %10 3
         %12 = OpTypePointer Input %11
         %13 = OpVariable %12 Input
         %16 = OpConstant %6 0.5
         %20 = OpTypeVector %6 4
         %21 = OpTypePointer Function %20
         %23 = OpVariable %12 Input
         %27 = OpConstant %6 0.300000012
         %30 = OpConstant %6 1
         %35 = OpConstant %6 0.200000003
         %36 = OpConstant %6 2
         %37 = OpVariable %12 Input
         %46 = OpTypeInt 32 0
         %47 = OpConstant %46 0
         %48 = OpTypePointer Function %6
         %51 = OpConstant %46 1
         %54 = OpConstant %46 2
         %57 = OpTypeVector %10 4
         %58 = OpTypePointer Output %57
         %59 = OpVariable %58 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %22 = OpVariable %21 Function
         %14 = OpLoad %11 %13
         %15 = OpFConvert %7 %14
         %17 = OpVectorTimesScalar %7 %15 %16
         %18 = OpCompositeConstruct %7 %16 %16 %16
         %19 = OpFAdd %7 %17 %18
               OpStore %9 %19
         %24 = OpLoad %11 %23
         %25 = OpFConvert %7 %24
         %26 = OpLoad %7 %9
         %28 = OpCompositeConstruct %7 %27 %27 %27
         %29 = OpExtInst %7 %1 FMix %25 %26 %28
         %31 = OpCompositeExtract %6 %29 0
         %32 = OpCompositeExtract %6 %29 1
         %33 = OpCompositeExtract %6 %29 2
         %34 = OpCompositeConstruct %20 %31 %32 %33 %30
               OpStore %22 %34
         %38 = OpLoad %11 %37
         %39 = OpFConvert %7 %38
         %40 = OpVectorTimesScalar %7 %39 %36
         %41 = OpExtInst %7 %1 Fract %40
         %42 = OpVectorTimesScalar %7 %41 %35
         %43 = OpLoad %20 %22
         %44 = OpVectorShuffle %7 %43 %43 0 1 2
         %45 = OpFSub %7 %44 %42
         %49 = OpAccessChain %48 %22 %47
         %50 = OpCompositeExtract %6 %45 0
               OpStore %49 %50
         %52 = OpAccessChain %48 %22 %51
         %53 = OpCompositeExtract %6 %45 1
               OpStore %52 %53
         %55 = OpAccessChain %48 %22 %54
         %56 = OpCompositeExtract %6 %45 2
               OpStore %55 %56
         %60 = OpLoad %20 %22
         %61 = OpFConvert %57 %60
               OpStore %59 %61
               OpReturn
               OpFunctionEnd
