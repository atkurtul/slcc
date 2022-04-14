; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 191
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %11 %65
               OpExecutionMode %4 LocalSize 1 1 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "index"
               OpName %11 "gl_GlobalInvocationID"
               OpName %19 "UBO"
               OpMemberName %19 0 "deltaT"
               OpMemberName %19 1 "particleCount"
               OpName %21 "ubo"
               OpName %34 "position"
               OpName %35 "Particle"
               OpMemberName %35 0 "pos"
               OpMemberName %35 1 "vel"
               OpName %37 "Pos"
               OpMemberName %37 0 "particles"
               OpName %39 ""
               OpName %45 "velocity"
               OpName %49 "acceleration"
               OpName %53 "i"
               OpName %65 "gl_LocalInvocationID"
               OpName %75 "SHARED_DATA_SIZE"
               OpName %78 "sharedData"
               OpName %96 "j"
               OpName %109 "other"
               OpName %115 "len"
               OpName %121 "GRAVITY"
               OpName %132 "SOFTEN"
               OpName %134 "POWER"
               OpDecorate %11 BuiltIn GlobalInvocationId
               OpMemberDecorate %19 0 Offset 0
               OpMemberDecorate %19 1 Offset 4
               OpDecorate %19 Block
               OpDecorate %21 DescriptorSet 0
               OpDecorate %21 Binding 1
               OpMemberDecorate %35 0 Offset 0
               OpMemberDecorate %35 1 Offset 16
               OpDecorate %36 ArrayStride 32
               OpMemberDecorate %37 0 Offset 0
               OpDecorate %37 BufferBlock
               OpDecorate %39 DescriptorSet 0
               OpDecorate %39 Binding 0
               OpDecorate %65 BuiltIn LocalInvocationId
               OpDecorate %75 SpecId 1
               OpDecorate %104 SpecId 0
               OpDecorate %106 BuiltIn WorkgroupSize
               OpDecorate %121 SpecId 2
               OpDecorate %132 SpecId 4
               OpDecorate %134 SpecId 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 0
          %7 = OpTypePointer Function %6
          %9 = OpTypeVector %6 3
         %10 = OpTypePointer Input %9
         %11 = OpVariable %10 Input
         %12 = OpConstant %6 0
         %13 = OpTypePointer Input %6
         %17 = OpTypeFloat 32
         %18 = OpTypeInt 32 1
         %19 = OpTypeStruct %17 %18
         %20 = OpTypePointer Uniform %19
         %21 = OpVariable %20 Uniform
         %22 = OpConstant %18 1
         %23 = OpTypePointer Uniform %18
         %27 = OpTypeBool
         %32 = OpTypeVector %17 4
         %33 = OpTypePointer Function %32
         %35 = OpTypeStruct %32 %32
         %36 = OpTypeRuntimeArray %35
         %37 = OpTypeStruct %36
         %38 = OpTypePointer Uniform %37
         %39 = OpVariable %38 Uniform
         %40 = OpConstant %18 0
         %42 = OpTypePointer Uniform %32
         %50 = OpConstant %17 0
         %51 = OpConstantComposite %32 %50 %50 %50 %50
         %52 = OpTypePointer Function %18
         %65 = OpVariable %10 Input
         %75 = OpSpecConstant %18 1024
         %76 = OpTypeArray %32 %75
         %77 = OpTypePointer Workgroup %76
         %78 = OpVariable %77 Workgroup
         %88 = OpTypePointer Workgroup %32
         %94 = OpConstant %6 2
         %95 = OpConstant %6 264
        %104 = OpSpecConstant %6 1
        %105 = OpConstant %6 1
        %106 = OpSpecConstantComposite %9 %104 %105 %105
        %113 = OpTypeVector %17 3
        %114 = OpTypePointer Function %113
        %121 = OpSpecConstant %17 0.00200000009
        %124 = OpConstant %6 3
        %125 = OpTypePointer Function %17
        %132 = OpSpecConstant %17 0.0500000007
        %134 = OpSpecConstant %17 0.75
        %152 = OpTypePointer Uniform %17
        %155 = OpConstant %17 0.0500000007
        %171 = OpConstant %17 0.00499999989
        %182 = OpConstant %17 1
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %8 = OpVariable %7 Function
         %34 = OpVariable %33 Function
         %45 = OpVariable %33 Function
         %49 = OpVariable %33 Function
         %53 = OpVariable %52 Function
         %96 = OpVariable %52 Function
        %109 = OpVariable %33 Function
        %115 = OpVariable %114 Function
         %14 = OpAccessChain %13 %11 %12
         %15 = OpLoad %6 %14
               OpStore %8 %15
         %16 = OpLoad %6 %8
         %24 = OpAccessChain %23 %21 %22
         %25 = OpLoad %18 %24
         %26 = OpBitcast %6 %25
         %28 = OpUGreaterThanEqual %27 %16 %26
               OpSelectionMerge %30 None
               OpBranchConditional %28 %29 %30
         %29 = OpLabel
               OpReturn
         %30 = OpLabel
         %41 = OpLoad %6 %8
         %43 = OpAccessChain %42 %39 %40 %41 %40
         %44 = OpLoad %32 %43
               OpStore %34 %44
         %46 = OpLoad %6 %8
         %47 = OpAccessChain %42 %39 %40 %46 %22
         %48 = OpLoad %32 %47
               OpStore %45 %48
               OpStore %49 %51
               OpStore %53 %40
               OpBranch %54
         %54 = OpLabel
               OpLoopMerge %56 %57 None
               OpBranch %58
         %58 = OpLabel
         %59 = OpLoad %18 %53
         %60 = OpAccessChain %23 %21 %22
         %61 = OpLoad %18 %60
         %62 = OpSLessThan %27 %59 %61
               OpBranchConditional %62 %55 %56
         %55 = OpLabel
         %63 = OpLoad %18 %53
         %64 = OpBitcast %6 %63
         %66 = OpAccessChain %13 %65 %12
         %67 = OpLoad %6 %66
         %68 = OpIAdd %6 %64 %67
         %69 = OpAccessChain %23 %21 %22
         %70 = OpLoad %18 %69
         %71 = OpBitcast %6 %70
         %72 = OpULessThan %27 %68 %71
               OpSelectionMerge %74 None
               OpBranchConditional %72 %73 %90
         %73 = OpLabel
         %79 = OpAccessChain %13 %65 %12
         %80 = OpLoad %6 %79
         %81 = OpLoad %18 %53
         %82 = OpBitcast %6 %81
         %83 = OpAccessChain %13 %65 %12
         %84 = OpLoad %6 %83
         %85 = OpIAdd %6 %82 %84
         %86 = OpAccessChain %42 %39 %40 %85 %40
         %87 = OpLoad %32 %86
         %89 = OpAccessChain %88 %78 %80
               OpStore %89 %87
               OpBranch %74
         %90 = OpLabel
         %91 = OpAccessChain %13 %65 %12
         %92 = OpLoad %6 %91
         %93 = OpAccessChain %88 %78 %92
               OpStore %93 %51
               OpBranch %74
         %74 = OpLabel
               OpControlBarrier %94 %94 %95
               OpStore %96 %40
               OpBranch %97
         %97 = OpLabel
               OpLoopMerge %99 %100 None
               OpBranch %101
        %101 = OpLabel
        %102 = OpLoad %18 %96
        %103 = OpBitcast %6 %102
        %107 = OpCompositeExtract %6 %106 0
        %108 = OpULessThan %27 %103 %107
               OpBranchConditional %108 %98 %99
         %98 = OpLabel
        %110 = OpLoad %18 %96
        %111 = OpAccessChain %88 %78 %110
        %112 = OpLoad %32 %111
               OpStore %109 %112
        %116 = OpLoad %32 %109
        %117 = OpVectorShuffle %113 %116 %116 0 1 2
        %118 = OpLoad %32 %34
        %119 = OpVectorShuffle %113 %118 %118 0 1 2
        %120 = OpFSub %113 %117 %119
               OpStore %115 %120
        %122 = OpLoad %113 %115
        %123 = OpVectorTimesScalar %113 %122 %121
        %126 = OpAccessChain %125 %109 %124
        %127 = OpLoad %17 %126
        %128 = OpVectorTimesScalar %113 %123 %127
        %129 = OpLoad %113 %115
        %130 = OpLoad %113 %115
        %131 = OpDot %17 %129 %130
        %133 = OpFAdd %17 %131 %132
        %135 = OpExtInst %17 %1 Pow %133 %134
        %136 = OpCompositeConstruct %113 %135 %135 %135
        %137 = OpFDiv %113 %128 %136
        %138 = OpLoad %32 %49
        %139 = OpVectorShuffle %113 %138 %138 0 1 2
        %140 = OpFAdd %113 %139 %137
        %141 = OpAccessChain %125 %49 %12
        %142 = OpCompositeExtract %17 %140 0
               OpStore %141 %142
        %143 = OpAccessChain %125 %49 %105
        %144 = OpCompositeExtract %17 %140 1
               OpStore %143 %144
        %145 = OpAccessChain %125 %49 %94
        %146 = OpCompositeExtract %17 %140 2
               OpStore %145 %146
               OpBranch %100
        %100 = OpLabel
        %147 = OpLoad %18 %96
        %148 = OpIAdd %18 %147 %22
               OpStore %96 %148
               OpBranch %97
         %99 = OpLabel
               OpControlBarrier %94 %94 %95
               OpBranch %57
         %57 = OpLabel
        %149 = OpLoad %18 %53
        %150 = OpIAdd %18 %149 %75
               OpStore %53 %150
               OpBranch %54
         %56 = OpLabel
        %151 = OpLoad %6 %8
        %153 = OpAccessChain %152 %21 %40
        %154 = OpLoad %17 %153
        %156 = OpFMul %17 %154 %155
        %157 = OpLoad %32 %49
        %158 = OpVectorShuffle %113 %157 %157 0 1 2
        %159 = OpVectorTimesScalar %113 %158 %156
        %160 = OpAccessChain %42 %39 %40 %151 %22
        %161 = OpLoad %32 %160
        %162 = OpVectorShuffle %113 %161 %161 0 1 2
        %163 = OpFAdd %113 %162 %159
        %164 = OpAccessChain %152 %39 %40 %151 %22 %12
        %165 = OpCompositeExtract %17 %163 0
               OpStore %164 %165
        %166 = OpAccessChain %152 %39 %40 %151 %22 %105
        %167 = OpCompositeExtract %17 %163 1
               OpStore %166 %167
        %168 = OpAccessChain %152 %39 %40 %151 %22 %94
        %169 = OpCompositeExtract %17 %163 2
               OpStore %168 %169
        %170 = OpLoad %6 %8
        %172 = OpAccessChain %152 %21 %40
        %173 = OpLoad %17 %172
        %174 = OpFMul %17 %171 %173
        %175 = OpAccessChain %152 %39 %40 %170 %22 %124
        %176 = OpLoad %17 %175
        %177 = OpFAdd %17 %176 %174
        %178 = OpAccessChain %152 %39 %40 %170 %22 %124
               OpStore %178 %177
        %179 = OpLoad %6 %8
        %180 = OpAccessChain %152 %39 %40 %179 %22 %124
        %181 = OpLoad %17 %180
        %183 = OpFOrdGreaterThan %27 %181 %182
               OpSelectionMerge %185 None
               OpBranchConditional %183 %184 %185
        %184 = OpLabel
        %186 = OpLoad %6 %8
        %187 = OpAccessChain %152 %39 %40 %186 %22 %124
        %188 = OpLoad %17 %187
        %189 = OpFSub %17 %188 %182
        %190 = OpAccessChain %152 %39 %40 %186 %22 %124
               OpStore %190 %189
               OpBranch %185
        %185 = OpLabel
               OpReturn
               OpFunctionEnd
