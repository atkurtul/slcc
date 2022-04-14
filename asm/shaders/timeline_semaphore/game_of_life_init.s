; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 100
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %13
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "index"
               OpName %13 "gl_GlobalInvocationID"
               OpName %18 "mask"
               OpName %33 "wrapped_index"
               OpName %78 "is_alive"
               OpName %85 "Image"
               OpDecorate %13 BuiltIn GlobalInvocationId
               OpDecorate %85 DescriptorSet 0
               OpDecorate %85 Binding 0
               OpDecorate %85 NonReadable
               OpDecorate %99 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 1
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
         %10 = OpTypeInt 32 0
         %11 = OpTypeVector %10 3
         %12 = OpTypePointer Input %11
         %13 = OpVariable %12 Input
         %14 = OpTypeVector %10 2
         %19 = OpConstant %6 3
         %20 = OpConstantComposite %7 %19 %19
         %21 = OpConstant %6 7
         %22 = OpConstantComposite %7 %21 %21
         %24 = OpConstant %6 16
         %27 = OpConstant %6 0
         %28 = OpConstantComposite %7 %27 %27
         %29 = OpTypeBool
         %30 = OpTypeVector %29 2
         %38 = OpConstant %6 1
         %39 = OpConstantComposite %7 %38 %27
         %46 = OpConstant %6 2
         %47 = OpConstantComposite %7 %46 %38
         %55 = OpConstantComposite %7 %27 %46
         %63 = OpConstantComposite %7 %38 %46
         %71 = OpConstantComposite %7 %46 %46
         %77 = OpTypePointer Function %29
         %79 = OpConstantTrue %29
         %81 = OpConstantFalse %29
         %82 = OpTypeFloat 32
         %83 = OpTypeImage %82 2D 0 0 0 2 Rgba8
         %84 = OpTypePointer UniformConstant %83
         %85 = OpVariable %84 UniformConstant
         %89 = OpTypeVector %82 4
         %90 = OpConstant %82 1
         %91 = OpConstant %82 0
         %92 = OpConstantComposite %89 %90 %90 %90 %91
         %93 = OpConstantComposite %89 %91 %91 %91 %91
         %94 = OpTypeVector %29 4
         %97 = OpConstant %10 8
         %98 = OpConstant %10 1
         %99 = OpConstantComposite %11 %97 %97 %98
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %18 = OpVariable %8 Function
         %33 = OpVariable %8 Function
         %78 = OpVariable %77 Function
         %15 = OpLoad %11 %13
         %16 = OpVectorShuffle %14 %15 %15 0 1
         %17 = OpBitcast %7 %16
               OpStore %9 %17
         %23 = OpLoad %7 %9
         %25 = OpCompositeConstruct %7 %24 %24
         %26 = OpBitwiseAnd %7 %23 %25
         %31 = OpINotEqual %30 %26 %28
         %32 = OpSelect %7 %31 %22 %20
               OpStore %18 %32
         %34 = OpLoad %7 %9
         %35 = OpLoad %7 %18
         %36 = OpBitwiseAnd %7 %34 %35
               OpStore %33 %36
         %37 = OpLoad %7 %33
         %40 = OpIEqual %30 %37 %39
         %41 = OpAll %29 %40
         %42 = OpLogicalNot %29 %41
               OpSelectionMerge %44 None
               OpBranchConditional %42 %43 %44
         %43 = OpLabel
         %45 = OpLoad %7 %33
         %48 = OpIEqual %30 %45 %47
         %49 = OpAll %29 %48
               OpBranch %44
         %44 = OpLabel
         %50 = OpPhi %29 %41 %5 %49 %43
         %51 = OpLogicalNot %29 %50
               OpSelectionMerge %53 None
               OpBranchConditional %51 %52 %53
         %52 = OpLabel
         %54 = OpLoad %7 %33
         %56 = OpIEqual %30 %54 %55
         %57 = OpAll %29 %56
               OpBranch %53
         %53 = OpLabel
         %58 = OpPhi %29 %50 %44 %57 %52
         %59 = OpLogicalNot %29 %58
               OpSelectionMerge %61 None
               OpBranchConditional %59 %60 %61
         %60 = OpLabel
         %62 = OpLoad %7 %33
         %64 = OpIEqual %30 %62 %63
         %65 = OpAll %29 %64
               OpBranch %61
         %61 = OpLabel
         %66 = OpPhi %29 %58 %53 %65 %60
         %67 = OpLogicalNot %29 %66
               OpSelectionMerge %69 None
               OpBranchConditional %67 %68 %69
         %68 = OpLabel
         %70 = OpLoad %7 %33
         %72 = OpIEqual %30 %70 %71
         %73 = OpAll %29 %72
               OpBranch %69
         %69 = OpLabel
         %74 = OpPhi %29 %66 %61 %73 %68
               OpSelectionMerge %76 None
               OpBranchConditional %74 %75 %80
         %75 = OpLabel
               OpStore %78 %79
               OpBranch %76
         %80 = OpLabel
               OpStore %78 %81
               OpBranch %76
         %76 = OpLabel
         %86 = OpLoad %83 %85
         %87 = OpLoad %7 %9
         %88 = OpLoad %29 %78
         %95 = OpCompositeConstruct %94 %88 %88 %88 %88
         %96 = OpSelect %89 %95 %92 %93
               OpImageWrite %86 %87 %96
               OpReturn
               OpFunctionEnd
