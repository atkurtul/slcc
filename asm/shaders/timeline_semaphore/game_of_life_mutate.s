; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 64
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %18
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "v"
               OpName %13 "ImageInput"
               OpName %18 "gl_GlobalInvocationID"
               OpName %43 "Registers"
               OpMemberName %43 0 "counter"
               OpName %45 "registers"
               OpName %55 "ImageOutput"
               OpDecorate %13 DescriptorSet 1
               OpDecorate %13 Binding 0
               OpDecorate %18 BuiltIn GlobalInvocationId
               OpMemberDecorate %43 0 Offset 0
               OpDecorate %43 Block
               OpDecorate %55 DescriptorSet 0
               OpDecorate %55 Binding 0
               OpDecorate %55 NonReadable
               OpDecorate %63 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Function %7
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypeInt 32 0
         %16 = OpTypeVector %15 3
         %17 = OpTypePointer Input %16
         %18 = OpVariable %17 Input
         %19 = OpTypeVector %15 2
         %22 = OpTypeInt 32 1
         %23 = OpTypeVector %22 2
         %25 = OpConstant %22 0
         %28 = OpTypeVector %6 3
         %31 = OpConstant %6 0
         %32 = OpConstantComposite %28 %31 %31 %31
         %33 = OpTypeBool
         %34 = OpTypeVector %33 3
         %39 = OpConstant %15 3
         %40 = OpTypePointer Function %6
         %43 = OpTypeStruct %6
         %44 = OpTypePointer PushConstant %43
         %45 = OpVariable %44 PushConstant
         %46 = OpTypePointer PushConstant %6
         %53 = OpTypeImage %6 2D 0 0 0 2 Rgba8
         %54 = OpTypePointer UniformConstant %53
         %55 = OpVariable %54 UniformConstant
         %61 = OpConstant %15 8
         %62 = OpConstant %15 1
         %63 = OpConstantComposite %16 %61 %61 %62
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %14 = OpLoad %11 %13
         %20 = OpLoad %16 %18
         %21 = OpVectorShuffle %19 %20 %20 0 1
         %24 = OpBitcast %23 %21
         %26 = OpImage %10 %14
         %27 = OpImageFetch %7 %26 %24 Lod %25
               OpStore %9 %27
         %29 = OpLoad %7 %9
         %30 = OpVectorShuffle %28 %29 %29 0 1 2
         %35 = OpFUnordNotEqual %34 %30 %32
         %36 = OpAny %33 %35
               OpSelectionMerge %38 None
               OpBranchConditional %36 %37 %51
         %37 = OpLabel
         %41 = OpAccessChain %40 %9 %39
         %42 = OpLoad %6 %41
         %47 = OpAccessChain %46 %45 %25
         %48 = OpLoad %6 %47
         %49 = OpExtInst %6 %1 FMax %42 %48
         %50 = OpAccessChain %40 %9 %39
               OpStore %50 %49
               OpBranch %38
         %51 = OpLabel
         %52 = OpAccessChain %40 %9 %39
               OpStore %52 %31
               OpBranch %38
         %38 = OpLabel
         %56 = OpLoad %53 %55
         %57 = OpLoad %16 %18
         %58 = OpVectorShuffle %19 %57 %57 0 1
         %59 = OpBitcast %23 %58
         %60 = OpLoad %7 %9
               OpImageWrite %56 %59 %60
               OpReturn
               OpFunctionEnd
