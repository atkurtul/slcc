; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 52
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %11 %19 %20 %33
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "normal_color"
               OpName %11 "in_normal"
               OpName %19 "o_color"
               OpName %20 "in_color"
               OpName %33 "in_delta_pos"
               OpDecorate %11 Location 1
               OpDecorate %19 Location 0
               OpDecorate %20 Location 0
               OpDecorate %33 Location 2
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
         %10 = OpTypePointer Input %7
         %11 = OpVariable %10 Input
         %13 = OpConstant %6 0.5
         %17 = OpTypeVector %6 4
         %18 = OpTypePointer Output %17
         %19 = OpVariable %18 Output
         %20 = OpVariable %10 Input
         %23 = OpConstant %6 0.300000012
         %26 = OpConstant %6 1
         %31 = OpConstant %6 0.200000003
         %32 = OpConstant %6 2
         %33 = OpVariable %10 Input
         %41 = OpTypeInt 32 0
         %42 = OpConstant %41 0
         %43 = OpTypePointer Output %6
         %46 = OpConstant %41 1
         %49 = OpConstant %41 2
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %12 = OpLoad %7 %11
         %14 = OpVectorTimesScalar %7 %12 %13
         %15 = OpCompositeConstruct %7 %13 %13 %13
         %16 = OpFAdd %7 %14 %15
               OpStore %9 %16
         %21 = OpLoad %7 %20
         %22 = OpLoad %7 %9
         %24 = OpCompositeConstruct %7 %23 %23 %23
         %25 = OpExtInst %7 %1 FMix %21 %22 %24
         %27 = OpCompositeExtract %6 %25 0
         %28 = OpCompositeExtract %6 %25 1
         %29 = OpCompositeExtract %6 %25 2
         %30 = OpCompositeConstruct %17 %27 %28 %29 %26
               OpStore %19 %30
         %34 = OpLoad %7 %33
         %35 = OpVectorTimesScalar %7 %34 %32
         %36 = OpExtInst %7 %1 Fract %35
         %37 = OpVectorTimesScalar %7 %36 %31
         %38 = OpLoad %17 %19
         %39 = OpVectorShuffle %7 %38 %38 0 1 2
         %40 = OpFSub %7 %39 %37
         %44 = OpAccessChain %43 %19 %42
         %45 = OpCompositeExtract %6 %40 0
               OpStore %44 %45
         %47 = OpAccessChain %43 %19 %46
         %48 = OpCompositeExtract %6 %40 1
               OpStore %47 %48
         %50 = OpAccessChain %43 %19 %49
         %51 = OpCompositeExtract %6 %40 2
               OpStore %50 %51
               OpReturn
               OpFunctionEnd
