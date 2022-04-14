; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 112
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %102 %104
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %11 "hash33(vf3;"
               OpName %10 "p3"
               OpName %15 "starField(vf3;"
               OpName %14 "pos"
               OpName %68 "color"
               OpName %71 "threshhold"
               OpName %73 "rnd"
               OpName %74 "param"
               OpName %83 "starCol"
               OpName %102 "outFragColor"
               OpName %104 "inUVW"
               OpName %105 "param"
               OpDecorate %102 Location 0
               OpDecorate %104 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
          %9 = OpTypeFunction %6 %8
         %13 = OpTypeFunction %7 %8
         %18 = OpConstant %6 443.897003
         %19 = OpConstant %6 441.423004
         %20 = OpConstant %6 437.195007
         %21 = OpConstantComposite %7 %18 %19 %20
         %27 = OpConstant %6 19.1900005
         %28 = OpConstantComposite %7 %27 %27 %27
         %34 = OpTypeInt 32 0
         %35 = OpConstant %34 0
         %36 = OpTypePointer Function %6
         %39 = OpConstant %34 1
         %43 = OpConstant %34 2
         %69 = OpConstant %6 0
         %70 = OpConstantComposite %7 %69 %69 %69
         %72 = OpConstant %6 0.99000001
         %79 = OpTypeBool
         %87 = OpConstant %6 1
         %91 = OpConstant %6 16
        %100 = OpTypeVector %6 4
        %101 = OpTypePointer Output %100
        %102 = OpVariable %101 Output
        %103 = OpTypePointer Input %7
        %104 = OpVariable %103 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %105 = OpVariable %8 Function
        %106 = OpLoad %7 %104
               OpStore %105 %106
        %107 = OpFunctionCall %7 %15 %105
        %108 = OpCompositeExtract %6 %107 0
        %109 = OpCompositeExtract %6 %107 1
        %110 = OpCompositeExtract %6 %107 2
        %111 = OpCompositeConstruct %100 %108 %109 %110 %87
               OpStore %102 %111
               OpReturn
               OpFunctionEnd
         %11 = OpFunction %6 None %9
         %10 = OpFunctionParameter %8
         %12 = OpLabel
         %17 = OpLoad %7 %10
         %22 = OpFMul %7 %17 %21
         %23 = OpExtInst %7 %1 Fract %22
               OpStore %10 %23
         %24 = OpLoad %7 %10
         %25 = OpLoad %7 %10
         %26 = OpVectorShuffle %7 %25 %25 1 0 2
         %29 = OpFAdd %7 %26 %28
         %30 = OpDot %6 %24 %29
         %31 = OpLoad %7 %10
         %32 = OpCompositeConstruct %7 %30 %30 %30
         %33 = OpFAdd %7 %31 %32
               OpStore %10 %33
         %37 = OpAccessChain %36 %10 %35
         %38 = OpLoad %6 %37
         %40 = OpAccessChain %36 %10 %39
         %41 = OpLoad %6 %40
         %42 = OpFAdd %6 %38 %41
         %44 = OpAccessChain %36 %10 %43
         %45 = OpLoad %6 %44
         %46 = OpFMul %6 %42 %45
         %47 = OpAccessChain %36 %10 %35
         %48 = OpLoad %6 %47
         %49 = OpAccessChain %36 %10 %43
         %50 = OpLoad %6 %49
         %51 = OpFAdd %6 %48 %50
         %52 = OpAccessChain %36 %10 %39
         %53 = OpLoad %6 %52
         %54 = OpFMul %6 %51 %53
         %55 = OpFAdd %6 %46 %54
         %56 = OpAccessChain %36 %10 %39
         %57 = OpLoad %6 %56
         %58 = OpAccessChain %36 %10 %43
         %59 = OpLoad %6 %58
         %60 = OpFAdd %6 %57 %59
         %61 = OpAccessChain %36 %10 %35
         %62 = OpLoad %6 %61
         %63 = OpFMul %6 %60 %62
         %64 = OpFAdd %6 %55 %63
         %65 = OpExtInst %6 %1 Fract %64
               OpReturnValue %65
               OpFunctionEnd
         %15 = OpFunction %7 None %13
         %14 = OpFunctionParameter %8
         %16 = OpLabel
         %68 = OpVariable %8 Function
         %71 = OpVariable %36 Function
         %73 = OpVariable %36 Function
         %74 = OpVariable %8 Function
         %83 = OpVariable %36 Function
               OpStore %68 %70
               OpStore %71 %72
         %75 = OpLoad %7 %14
               OpStore %74 %75
         %76 = OpFunctionCall %6 %11 %74
               OpStore %73 %76
         %77 = OpLoad %6 %73
         %78 = OpLoad %6 %71
         %80 = OpFOrdGreaterThanEqual %79 %77 %78
               OpSelectionMerge %82 None
               OpBranchConditional %80 %81 %82
         %81 = OpLabel
         %84 = OpLoad %6 %73
         %85 = OpLoad %6 %71
         %86 = OpFSub %6 %84 %85
         %88 = OpLoad %6 %71
         %89 = OpFSub %6 %87 %88
         %90 = OpFDiv %6 %86 %89
         %92 = OpExtInst %6 %1 Pow %90 %91
               OpStore %83 %92
         %93 = OpLoad %6 %83
         %94 = OpCompositeConstruct %7 %93 %93 %93
         %95 = OpLoad %7 %68
         %96 = OpFAdd %7 %95 %94
               OpStore %68 %96
               OpBranch %82
         %82 = OpLabel
         %97 = OpLoad %7 %68
               OpReturnValue %97
               OpFunctionEnd
