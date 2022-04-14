; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 251
; Schema: 0
               OpCapability Shader
               OpCapability ImageQuery
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
               OpName %21 "uv"
               OpName %30 "ImageInput"
               OpName %38 "neighbors"
               OpName %41 "self"
               OpName %48 "is_alive"
               OpName %57 "total"
               OpName %60 "tmp"
               OpName %78 "tmp"
               OpName %94 "tmp"
               OpName %110 "tmp"
               OpName %126 "tmp"
               OpName %142 "tmp"
               OpName %158 "tmp"
               OpName %174 "tmp"
               OpName %221 "fresh_color"
               OpName %241 "ImageOutput"
               OpDecorate %13 BuiltIn GlobalInvocationId
               OpDecorate %30 DescriptorSet 1
               OpDecorate %30 Binding 0
               OpDecorate %241 DescriptorSet 0
               OpDecorate %241 Binding 0
               OpDecorate %241 NonReadable
               OpDecorate %250 BuiltIn WorkgroupSize
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
         %18 = OpTypeFloat 32
         %19 = OpTypeVector %18 2
         %20 = OpTypePointer Function %19
         %24 = OpConstant %18 0.5
         %27 = OpTypeImage %18 2D 0 0 0 1 Unknown
         %28 = OpTypeSampledImage %27
         %29 = OpTypePointer UniformConstant %28
         %30 = OpVariable %29 UniformConstant
         %32 = OpConstant %6 0
         %37 = OpTypePointer Function %6
         %39 = OpTypeVector %18 4
         %40 = OpTypePointer Function %39
         %44 = OpConstant %18 0
         %46 = OpTypeBool
         %47 = OpTypePointer Function %46
         %49 = OpTypeVector %18 3
         %52 = OpConstantComposite %49 %44 %44 %44
         %53 = OpTypeVector %46 3
         %56 = OpTypePointer Function %49
         %63 = OpConstant %6 -1
         %64 = OpConstantComposite %7 %63 %63
         %73 = OpConstant %6 1
         %81 = OpConstantComposite %7 %32 %63
         %97 = OpConstantComposite %7 %73 %63
        %113 = OpConstantComposite %7 %63 %32
        %129 = OpConstantComposite %7 %73 %32
        %145 = OpConstantComposite %7 %63 %73
        %161 = OpConstantComposite %7 %32 %73
        %177 = OpConstantComposite %7 %73 %73
        %195 = OpConstant %10 2
        %202 = OpConstant %10 3
        %222 = OpConstant %10 0
        %223 = OpTypePointer Function %18
        %226 = OpConstant %10 1
        %229 = OpConstant %18 1
        %239 = OpTypeImage %18 2D 0 0 0 2 Rgba8
        %240 = OpTypePointer UniformConstant %239
        %241 = OpVariable %240 UniformConstant
        %249 = OpConstant %10 8
        %250 = OpConstantComposite %11 %249 %249 %226
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %21 = OpVariable %20 Function
         %38 = OpVariable %37 Function
         %41 = OpVariable %40 Function
         %48 = OpVariable %47 Function
         %57 = OpVariable %56 Function
         %60 = OpVariable %56 Function
         %78 = OpVariable %56 Function
         %94 = OpVariable %56 Function
        %110 = OpVariable %56 Function
        %126 = OpVariable %56 Function
        %142 = OpVariable %56 Function
        %158 = OpVariable %56 Function
        %174 = OpVariable %56 Function
        %221 = OpVariable %56 Function
         %15 = OpLoad %11 %13
         %16 = OpVectorShuffle %14 %15 %15 0 1
         %17 = OpBitcast %7 %16
               OpStore %9 %17
         %22 = OpLoad %7 %9
         %23 = OpConvertSToF %19 %22
         %25 = OpCompositeConstruct %19 %24 %24
         %26 = OpFAdd %19 %23 %25
         %31 = OpLoad %28 %30
         %33 = OpImage %27 %31
         %34 = OpImageQuerySizeLod %7 %33 %32
         %35 = OpConvertSToF %19 %34
         %36 = OpFDiv %19 %26 %35
               OpStore %21 %36
               OpStore %38 %32
         %42 = OpLoad %28 %30
         %43 = OpLoad %19 %21
         %45 = OpImageSampleExplicitLod %39 %42 %43 Lod %44
               OpStore %41 %45
         %50 = OpLoad %39 %41
         %51 = OpVectorShuffle %49 %50 %50 0 1 2
         %54 = OpFUnordNotEqual %53 %51 %52
         %55 = OpAny %46 %54
               OpStore %48 %55
         %58 = OpLoad %39 %41
         %59 = OpVectorShuffle %49 %58 %58 0 1 2
               OpStore %57 %59
         %61 = OpLoad %28 %30
         %62 = OpLoad %19 %21
         %65 = OpImageSampleExplicitLod %39 %61 %62 Lod|ConstOffset %44 %64
         %66 = OpVectorShuffle %49 %65 %65 0 1 2
               OpStore %60 %66
         %67 = OpLoad %49 %60
         %68 = OpFUnordNotEqual %53 %67 %52
         %69 = OpAny %46 %68
               OpSelectionMerge %71 None
               OpBranchConditional %69 %70 %71
         %70 = OpLabel
         %72 = OpLoad %6 %38
         %74 = OpIAdd %6 %72 %73
               OpStore %38 %74
         %75 = OpLoad %49 %60
         %76 = OpLoad %49 %57
         %77 = OpFAdd %49 %76 %75
               OpStore %57 %77
               OpBranch %71
         %71 = OpLabel
         %79 = OpLoad %28 %30
         %80 = OpLoad %19 %21
         %82 = OpImageSampleExplicitLod %39 %79 %80 Lod|ConstOffset %44 %81
         %83 = OpVectorShuffle %49 %82 %82 0 1 2
               OpStore %78 %83
         %84 = OpLoad %49 %78
         %85 = OpFUnordNotEqual %53 %84 %52
         %86 = OpAny %46 %85
               OpSelectionMerge %88 None
               OpBranchConditional %86 %87 %88
         %87 = OpLabel
         %89 = OpLoad %6 %38
         %90 = OpIAdd %6 %89 %73
               OpStore %38 %90
         %91 = OpLoad %49 %78
         %92 = OpLoad %49 %57
         %93 = OpFAdd %49 %92 %91
               OpStore %57 %93
               OpBranch %88
         %88 = OpLabel
         %95 = OpLoad %28 %30
         %96 = OpLoad %19 %21
         %98 = OpImageSampleExplicitLod %39 %95 %96 Lod|ConstOffset %44 %97
         %99 = OpVectorShuffle %49 %98 %98 0 1 2
               OpStore %94 %99
        %100 = OpLoad %49 %94
        %101 = OpFUnordNotEqual %53 %100 %52
        %102 = OpAny %46 %101
               OpSelectionMerge %104 None
               OpBranchConditional %102 %103 %104
        %103 = OpLabel
        %105 = OpLoad %6 %38
        %106 = OpIAdd %6 %105 %73
               OpStore %38 %106
        %107 = OpLoad %49 %94
        %108 = OpLoad %49 %57
        %109 = OpFAdd %49 %108 %107
               OpStore %57 %109
               OpBranch %104
        %104 = OpLabel
        %111 = OpLoad %28 %30
        %112 = OpLoad %19 %21
        %114 = OpImageSampleExplicitLod %39 %111 %112 Lod|ConstOffset %44 %113
        %115 = OpVectorShuffle %49 %114 %114 0 1 2
               OpStore %110 %115
        %116 = OpLoad %49 %110
        %117 = OpFUnordNotEqual %53 %116 %52
        %118 = OpAny %46 %117
               OpSelectionMerge %120 None
               OpBranchConditional %118 %119 %120
        %119 = OpLabel
        %121 = OpLoad %6 %38
        %122 = OpIAdd %6 %121 %73
               OpStore %38 %122
        %123 = OpLoad %49 %110
        %124 = OpLoad %49 %57
        %125 = OpFAdd %49 %124 %123
               OpStore %57 %125
               OpBranch %120
        %120 = OpLabel
        %127 = OpLoad %28 %30
        %128 = OpLoad %19 %21
        %130 = OpImageSampleExplicitLod %39 %127 %128 Lod|ConstOffset %44 %129
        %131 = OpVectorShuffle %49 %130 %130 0 1 2
               OpStore %126 %131
        %132 = OpLoad %49 %126
        %133 = OpFUnordNotEqual %53 %132 %52
        %134 = OpAny %46 %133
               OpSelectionMerge %136 None
               OpBranchConditional %134 %135 %136
        %135 = OpLabel
        %137 = OpLoad %6 %38
        %138 = OpIAdd %6 %137 %73
               OpStore %38 %138
        %139 = OpLoad %49 %126
        %140 = OpLoad %49 %57
        %141 = OpFAdd %49 %140 %139
               OpStore %57 %141
               OpBranch %136
        %136 = OpLabel
        %143 = OpLoad %28 %30
        %144 = OpLoad %19 %21
        %146 = OpImageSampleExplicitLod %39 %143 %144 Lod|ConstOffset %44 %145
        %147 = OpVectorShuffle %49 %146 %146 0 1 2
               OpStore %142 %147
        %148 = OpLoad %49 %142
        %149 = OpFUnordNotEqual %53 %148 %52
        %150 = OpAny %46 %149
               OpSelectionMerge %152 None
               OpBranchConditional %150 %151 %152
        %151 = OpLabel
        %153 = OpLoad %6 %38
        %154 = OpIAdd %6 %153 %73
               OpStore %38 %154
        %155 = OpLoad %49 %142
        %156 = OpLoad %49 %57
        %157 = OpFAdd %49 %156 %155
               OpStore %57 %157
               OpBranch %152
        %152 = OpLabel
        %159 = OpLoad %28 %30
        %160 = OpLoad %19 %21
        %162 = OpImageSampleExplicitLod %39 %159 %160 Lod|ConstOffset %44 %161
        %163 = OpVectorShuffle %49 %162 %162 0 1 2
               OpStore %158 %163
        %164 = OpLoad %49 %158
        %165 = OpFUnordNotEqual %53 %164 %52
        %166 = OpAny %46 %165
               OpSelectionMerge %168 None
               OpBranchConditional %166 %167 %168
        %167 = OpLabel
        %169 = OpLoad %6 %38
        %170 = OpIAdd %6 %169 %73
               OpStore %38 %170
        %171 = OpLoad %49 %158
        %172 = OpLoad %49 %57
        %173 = OpFAdd %49 %172 %171
               OpStore %57 %173
               OpBranch %168
        %168 = OpLabel
        %175 = OpLoad %28 %30
        %176 = OpLoad %19 %21
        %178 = OpImageSampleExplicitLod %39 %175 %176 Lod|ConstOffset %44 %177
        %179 = OpVectorShuffle %49 %178 %178 0 1 2
               OpStore %174 %179
        %180 = OpLoad %49 %174
        %181 = OpFUnordNotEqual %53 %180 %52
        %182 = OpAny %46 %181
               OpSelectionMerge %184 None
               OpBranchConditional %182 %183 %184
        %183 = OpLabel
        %185 = OpLoad %6 %38
        %186 = OpIAdd %6 %185 %73
               OpStore %38 %186
        %187 = OpLoad %49 %174
        %188 = OpLoad %49 %57
        %189 = OpFAdd %49 %188 %187
               OpStore %57 %189
               OpBranch %184
        %184 = OpLabel
        %190 = OpLoad %46 %48
               OpSelectionMerge %192 None
               OpBranchConditional %190 %191 %214
        %191 = OpLabel
        %193 = OpLoad %6 %38
        %194 = OpBitcast %10 %193
        %196 = OpIEqual %46 %194 %195
        %197 = OpLogicalNot %46 %196
               OpSelectionMerge %199 None
               OpBranchConditional %197 %198 %199
        %198 = OpLabel
        %200 = OpLoad %6 %38
        %201 = OpBitcast %10 %200
        %203 = OpIEqual %46 %201 %202
               OpBranch %199
        %199 = OpLabel
        %204 = OpPhi %46 %196 %191 %203 %198
               OpStore %48 %204
        %205 = OpLoad %46 %48
               OpSelectionMerge %207 None
               OpBranchConditional %205 %206 %213
        %206 = OpLabel
        %208 = OpLoad %6 %38
        %209 = OpConvertSToF %18 %208
        %210 = OpLoad %49 %57
        %211 = OpCompositeConstruct %49 %209 %209 %209
        %212 = OpFDiv %49 %210 %211
               OpStore %57 %212
               OpBranch %207
        %213 = OpLabel
               OpStore %57 %52
               OpBranch %207
        %207 = OpLabel
               OpBranch %192
        %214 = OpLabel
        %215 = OpLoad %6 %38
        %216 = OpBitcast %10 %215
        %217 = OpIEqual %46 %216 %202
               OpStore %48 %217
        %218 = OpLoad %46 %48
               OpSelectionMerge %220 None
               OpBranchConditional %218 %219 %238
        %219 = OpLabel
        %224 = OpAccessChain %223 %21 %222
        %225 = OpLoad %18 %224
        %227 = OpAccessChain %223 %21 %226
        %228 = OpLoad %18 %227
        %230 = OpAccessChain %223 %21 %222
        %231 = OpLoad %18 %230
        %232 = OpFSub %18 %229 %231
        %233 = OpAccessChain %223 %21 %226
        %234 = OpLoad %18 %233
        %235 = OpFSub %18 %232 %234
        %236 = OpCompositeConstruct %49 %225 %228 %235
               OpStore %221 %236
        %237 = OpLoad %49 %221
               OpStore %57 %237
               OpBranch %220
        %238 = OpLabel
               OpStore %57 %52
               OpBranch %220
        %220 = OpLabel
               OpBranch %192
        %192 = OpLabel
        %242 = OpLoad %239 %241
        %243 = OpLoad %7 %9
        %244 = OpLoad %49 %57
        %245 = OpCompositeExtract %18 %244 0
        %246 = OpCompositeExtract %18 %244 1
        %247 = OpCompositeExtract %18 %244 2
        %248 = OpCompositeConstruct %39 %245 %246 %247 %44
               OpImageWrite %242 %243 %248
               OpReturn
               OpFunctionEnd
