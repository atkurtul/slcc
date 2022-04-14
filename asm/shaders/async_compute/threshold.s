; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 79
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %9
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "gl_GlobalInvocationID"
               OpName %15 "Registers"
               OpMemberName %15 0 "resolution"
               OpMemberName %15 1 "inv_resolution"
               OpMemberName %15 2 "inv_input_resolution"
               OpName %17 "registers"
               OpName %30 "uv"
               OpName %44 "rgb"
               OpName %48 "in_tex"
               OpName %65 "out_tex"
               OpDecorate %9 BuiltIn GlobalInvocationId
               OpMemberDecorate %15 0 Offset 0
               OpMemberDecorate %15 1 Offset 8
               OpMemberDecorate %15 2 Offset 16
               OpDecorate %15 Block
               OpDecorate %48 DescriptorSet 0
               OpDecorate %48 Binding 0
               OpDecorate %65 DescriptorSet 0
               OpDecorate %65 Binding 1
               OpDecorate %65 NonReadable
               OpDecorate %78 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 0
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Input %7
          %9 = OpVariable %8 Input
         %10 = OpTypeVector %6 2
         %13 = OpTypeFloat 32
         %14 = OpTypeVector %13 2
         %15 = OpTypeStruct %10 %14 %14
         %16 = OpTypePointer PushConstant %15
         %17 = OpVariable %16 PushConstant
         %18 = OpTypeInt 32 1
         %19 = OpConstant %18 0
         %20 = OpTypePointer PushConstant %10
         %23 = OpTypeBool
         %24 = OpTypeVector %23 2
         %29 = OpTypePointer Function %14
         %34 = OpConstant %13 0.5
         %37 = OpConstant %18 1
         %38 = OpTypePointer PushConstant %14
         %42 = OpTypeVector %13 3
         %43 = OpTypePointer Function %42
         %45 = OpTypeImage %13 2D 0 0 0 1 Unknown
         %46 = OpTypeSampledImage %45
         %47 = OpTypePointer UniformConstant %46
         %48 = OpVariable %47 UniformConstant
         %51 = OpConstant %13 0
         %52 = OpTypeVector %13 4
         %55 = OpConstant %13 0.200000003
         %57 = OpConstant %13 1
         %60 = OpConstantComposite %42 %51 %51 %51
         %63 = OpTypeImage %13 2D 0 0 0 2 Rgba16f
         %64 = OpTypePointer UniformConstant %63
         %65 = OpVariable %64 UniformConstant
         %69 = OpTypeVector %18 2
         %76 = OpConstant %6 8
         %77 = OpConstant %6 1
         %78 = OpConstantComposite %7 %76 %76 %77
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %30 = OpVariable %29 Function
         %44 = OpVariable %43 Function
         %11 = OpLoad %7 %9
         %12 = OpVectorShuffle %10 %11 %11 0 1
         %21 = OpAccessChain %20 %17 %19
         %22 = OpLoad %10 %21
         %25 = OpULessThan %24 %12 %22
         %26 = OpAll %23 %25
               OpSelectionMerge %28 None
               OpBranchConditional %26 %27 %28
         %27 = OpLabel
         %31 = OpLoad %7 %9
         %32 = OpVectorShuffle %10 %31 %31 0 1
         %33 = OpConvertUToF %14 %32
         %35 = OpCompositeConstruct %14 %34 %34
         %36 = OpFAdd %14 %33 %35
         %39 = OpAccessChain %38 %17 %37
         %40 = OpLoad %14 %39
         %41 = OpFMul %14 %36 %40
               OpStore %30 %41
         %49 = OpLoad %46 %48
         %50 = OpLoad %14 %30
         %53 = OpImageSampleExplicitLod %52 %49 %50 Lod %51
         %54 = OpVectorShuffle %42 %53 %53 0 1 2
               OpStore %44 %54
         %56 = OpLoad %42 %44
         %58 = OpCompositeConstruct %42 %57 %57 %57
         %59 = OpFSub %42 %56 %58
         %61 = OpExtInst %42 %1 FMax %59 %60
         %62 = OpVectorTimesScalar %42 %61 %55
               OpStore %44 %62
         %66 = OpLoad %63 %65
         %67 = OpLoad %7 %9
         %68 = OpVectorShuffle %10 %67 %67 0 1
         %70 = OpBitcast %69 %68
         %71 = OpLoad %42 %44
         %72 = OpCompositeExtract %13 %71 0
         %73 = OpCompositeExtract %13 %71 1
         %74 = OpCompositeExtract %13 %71 2
         %75 = OpCompositeConstruct %52 %72 %73 %74 %57
               OpImageWrite %66 %70 %75
               OpBranch %28
         %28 = OpLabel
               OpReturn
               OpFunctionEnd
