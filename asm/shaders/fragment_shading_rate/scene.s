; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 174
; Schema: 0
               OpCapability Shader
               OpCapability FragmentShadingRateKHR
               OpExtension "SPV_KHR_fragment_shading_rate"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %28 %52 %56 %60 %110 %142 %173
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_fragment_shading_rate"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "Push_Constants"
               OpMemberName %9 0 "offset"
               OpMemberName %9 1 "object_type"
               OpName %11 "push_constants"
               OpName %20 "color"
               OpName %24 "samplerEnvMap"
               OpName %28 "inUV"
               OpName %44 "ambient"
               OpName %45 "samplerSphere"
               OpName %50 "N"
               OpName %52 "inNormal"
               OpName %55 "L"
               OpName %56 "inLightVec"
               OpName %59 "V"
               OpName %60 "inViewVec"
               OpName %63 "R"
               OpName %68 "diffuse"
               OpName %75 "specular"
               OpName %95 "UBO"
               OpMemberName %95 0 "projection"
               OpMemberName %95 1 "modelview"
               OpMemberName %95 2 "skybox_modelview"
               OpMemberName %95 3 "color_shading_rates"
               OpName %97 "ubo"
               OpName %107 "v"
               OpName %108 "h"
               OpName %110 "gl_ShadingRateEXT"
               OpName %142 "outColor"
               OpName %173 "inPos"
               OpMemberDecorate %9 0 Offset 0
               OpMemberDecorate %9 1 Offset 16
               OpDecorate %9 Block
               OpDecorate %24 DescriptorSet 0
               OpDecorate %24 Binding 1
               OpDecorate %28 Location 0
               OpDecorate %45 DescriptorSet 0
               OpDecorate %45 Binding 2
               OpDecorate %52 Location 2
               OpDecorate %56 Location 4
               OpDecorate %60 Location 3
               OpMemberDecorate %95 0 ColMajor
               OpMemberDecorate %95 0 Offset 0
               OpMemberDecorate %95 0 MatrixStride 16
               OpMemberDecorate %95 1 ColMajor
               OpMemberDecorate %95 1 Offset 64
               OpMemberDecorate %95 1 MatrixStride 16
               OpMemberDecorate %95 2 ColMajor
               OpMemberDecorate %95 2 Offset 128
               OpMemberDecorate %95 2 MatrixStride 16
               OpMemberDecorate %95 3 Offset 192
               OpDecorate %95 Block
               OpDecorate %97 DescriptorSet 0
               OpDecorate %97 Binding 0
               OpDecorate %110 Flat
               OpDecorate %110 BuiltIn ShadingRateKHR
               OpDecorate %142 Location 0
               OpDecorate %173 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypeInt 32 1
          %9 = OpTypeStruct %7 %8
         %10 = OpTypePointer PushConstant %9
         %11 = OpVariable %10 PushConstant
         %12 = OpConstant %8 1
         %13 = OpTypePointer PushConstant %8
         %19 = OpTypePointer Function %7
         %21 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %22 = OpTypeSampledImage %21
         %23 = OpTypePointer UniformConstant %22
         %24 = OpVariable %23 UniformConstant
         %26 = OpTypeVector %6 2
         %27 = OpTypePointer Input %26
         %28 = OpVariable %27 Input
         %29 = OpTypeInt 32 0
         %30 = OpConstant %29 0
         %31 = OpTypePointer Input %6
         %34 = OpConstant %6 1
         %35 = OpConstant %29 1
         %42 = OpTypeVector %6 3
         %43 = OpTypePointer Function %42
         %45 = OpVariable %23 UniformConstant
         %51 = OpTypePointer Input %42
         %52 = OpVariable %51 Input
         %56 = OpVariable %51 Input
         %60 = OpVariable %51 Input
         %72 = OpConstant %6 0
         %80 = OpConstant %6 8
         %94 = OpTypeMatrix %7 4
         %95 = OpTypeStruct %94 %94 %94 %8
         %96 = OpTypePointer Uniform %95
         %97 = OpVariable %96 Uniform
         %98 = OpConstant %8 3
         %99 = OpTypePointer Uniform %8
        %102 = OpTypeBool
        %106 = OpTypePointer Function %8
        %109 = OpTypePointer Input %8
        %110 = OpVariable %109 Input
        %116 = OpConstant %8 2
        %122 = OpConstant %8 4
        %129 = OpConstant %8 8
        %141 = OpTypePointer Output %7
        %142 = OpVariable %141 Output
        %158 = OpConstant %6 0.0500000007
        %173 = OpVariable %51 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %20 = OpVariable %19 Function
         %44 = OpVariable %43 Function
         %50 = OpVariable %43 Function
         %55 = OpVariable %43 Function
         %59 = OpVariable %43 Function
         %63 = OpVariable %43 Function
         %68 = OpVariable %43 Function
         %75 = OpVariable %43 Function
        %107 = OpVariable %106 Function
        %108 = OpVariable %106 Function
         %14 = OpAccessChain %13 %11 %12
         %15 = OpLoad %8 %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %25 = OpLoad %22 %24
         %32 = OpAccessChain %31 %28 %30
         %33 = OpLoad %6 %32
         %36 = OpAccessChain %31 %28 %35
         %37 = OpLoad %6 %36
         %38 = OpFSub %6 %34 %37
         %39 = OpCompositeConstruct %26 %33 %38
         %40 = OpImageSampleImplicitLod %7 %25 %39
               OpStore %20 %40
               OpBranch %18
         %17 = OpLabel
         %46 = OpLoad %22 %45
         %47 = OpLoad %26 %28
         %48 = OpImageSampleImplicitLod %7 %46 %47
         %49 = OpVectorShuffle %42 %48 %48 0 1 2
               OpStore %44 %49
         %53 = OpLoad %42 %52
         %54 = OpExtInst %42 %1 Normalize %53
               OpStore %50 %54
         %57 = OpLoad %42 %56
         %58 = OpExtInst %42 %1 Normalize %57
               OpStore %55 %58
         %61 = OpLoad %42 %60
         %62 = OpExtInst %42 %1 Normalize %61
               OpStore %59 %62
         %64 = OpLoad %42 %55
         %65 = OpFNegate %42 %64
         %66 = OpLoad %42 %50
         %67 = OpExtInst %42 %1 Reflect %65 %66
               OpStore %63 %67
         %69 = OpLoad %42 %50
         %70 = OpLoad %42 %55
         %71 = OpDot %6 %69 %70
         %73 = OpExtInst %6 %1 FMax %71 %72
         %74 = OpCompositeConstruct %42 %73 %73 %73
               OpStore %68 %74
         %76 = OpLoad %42 %63
         %77 = OpLoad %42 %59
         %78 = OpDot %6 %76 %77
         %79 = OpExtInst %6 %1 FMax %78 %72
         %81 = OpExtInst %6 %1 Pow %79 %80
         %82 = OpCompositeConstruct %42 %81 %81 %81
               OpStore %75 %82
         %83 = OpLoad %42 %44
         %84 = OpLoad %42 %68
         %85 = OpFAdd %42 %83 %84
         %86 = OpLoad %42 %75
         %87 = OpFAdd %42 %85 %86
         %88 = OpCompositeExtract %6 %87 0
         %89 = OpCompositeExtract %6 %87 1
         %90 = OpCompositeExtract %6 %87 2
         %91 = OpCompositeConstruct %7 %88 %89 %90 %34
               OpStore %20 %91
               OpBranch %18
         %18 = OpLabel
        %100 = OpAccessChain %99 %97 %98
        %101 = OpLoad %8 %100
        %103 = OpIEqual %102 %101 %12
               OpSelectionMerge %105 None
               OpBranchConditional %103 %104 %166
        %104 = OpLabel
               OpStore %107 %12
               OpStore %108 %12
        %111 = OpLoad %8 %110
        %112 = OpBitwiseAnd %8 %111 %12
        %113 = OpIEqual %102 %112 %12
               OpSelectionMerge %115 None
               OpBranchConditional %113 %114 %115
        %114 = OpLabel
               OpStore %107 %116
               OpBranch %115
        %115 = OpLabel
        %117 = OpLoad %8 %110
        %118 = OpBitwiseAnd %8 %117 %116
        %119 = OpIEqual %102 %118 %116
               OpSelectionMerge %121 None
               OpBranchConditional %119 %120 %121
        %120 = OpLabel
               OpStore %107 %122
               OpBranch %121
        %121 = OpLabel
        %123 = OpLoad %8 %110
        %124 = OpBitwiseAnd %8 %123 %122
        %125 = OpIEqual %102 %124 %122
               OpSelectionMerge %127 None
               OpBranchConditional %125 %126 %127
        %126 = OpLabel
               OpStore %108 %116
               OpBranch %127
        %127 = OpLabel
        %128 = OpLoad %8 %110
        %130 = OpBitwiseAnd %8 %128 %129
        %131 = OpIEqual %102 %130 %129
               OpSelectionMerge %133 None
               OpBranchConditional %131 %132 %133
        %132 = OpLabel
               OpStore %108 %122
               OpBranch %133
        %133 = OpLabel
        %134 = OpLoad %8 %107
        %135 = OpIEqual %102 %134 %12
        %136 = OpLoad %8 %108
        %137 = OpIEqual %102 %136 %12
        %138 = OpLogicalAnd %102 %135 %137
               OpSelectionMerge %140 None
               OpBranchConditional %138 %139 %150
        %139 = OpLabel
        %143 = OpLoad %7 %20
        %144 = OpVectorShuffle %42 %143 %143 0 0 0
        %145 = OpVectorTimesScalar %42 %144 %34
        %146 = OpCompositeExtract %6 %145 0
        %147 = OpCompositeExtract %6 %145 1
        %148 = OpCompositeExtract %6 %145 2
        %149 = OpCompositeConstruct %7 %146 %147 %148 %34
               OpStore %142 %149
               OpBranch %140
        %150 = OpLabel
        %151 = OpLoad %7 %20
        %152 = OpVectorShuffle %42 %151 %151 0 0 0
        %153 = OpVectorTimesScalar %42 %152 %34
        %154 = OpLoad %8 %107
        %155 = OpLoad %8 %108
        %156 = OpIAdd %8 %154 %155
        %157 = OpConvertSToF %6 %156
        %159 = OpFMul %6 %157 %158
        %160 = OpCompositeConstruct %42 %159 %159 %159
        %161 = OpFSub %42 %153 %160
        %162 = OpCompositeExtract %6 %161 0
        %163 = OpCompositeExtract %6 %161 1
        %164 = OpCompositeExtract %6 %161 2
        %165 = OpCompositeConstruct %7 %162 %163 %164 %34
               OpStore %142 %165
               OpBranch %140
        %140 = OpLabel
               OpBranch %105
        %166 = OpLabel
        %167 = OpLoad %7 %20
        %168 = OpVectorShuffle %42 %167 %167 0 1 2
        %169 = OpCompositeExtract %6 %168 0
        %170 = OpCompositeExtract %6 %168 1
        %171 = OpCompositeExtract %6 %168 2
        %172 = OpCompositeConstruct %7 %169 %170 %171 %34
               OpStore %142 %172
               OpBranch %105
        %105 = OpLabel
               OpReturn
               OpFunctionEnd
