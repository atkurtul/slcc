; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 262
; Schema: 0
               OpCapability Shader
               OpCapability Float16
               OpCapability StorageBuffer16BitAccess
               OpExtension "SPV_KHR_16bit_storage"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %134
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_16bit_storage"
               OpSourceExtension "GL_EXT_shader_explicit_arithmetic_types_float16"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %16 "compute_blob(vf162;vf164;f161;"
               OpName %13 "pos"
               OpName %14 "blob"
               OpName %15 "seed"
               OpName %18 "offset"
               OpName %23 "rg_offset"
               OpName %30 "bs_offset"
               OpName %41 "rg_dot"
               OpName %45 "bs_dot"
               OpName %49 "dots"
               OpName %100 "parabolas"
               OpName %119 "num_blobs"
               OpName %123 "Registers"
               OpMemberName %123 0 "num_blobs"
               OpMemberName %123 1 "seed"
               OpMemberName %123 2 "range"
               OpName %125 "registers"
               OpName %131 "x"
               OpName %134 "gl_GlobalInvocationID"
               OpName %140 "WIDTH"
               OpName %145 "y"
               OpName %150 "HEIGHT"
               OpName %154 "pos"
               OpName %160 "result"
               OpName %161 "seed"
               OpName %168 "range"
               OpName %176 "stride"
               OpName %180 "i"
               OpName %190 "blob"
               OpName %192 "SSBO"
               OpMemberName %192 0 "blob_data"
               OpName %194 ""
               OpName %200 "y"
               OpName %213 "x"
               OpName %235 "param"
               OpName %236 "param"
               OpName %238 "param"
               OpName %251 "o_results"
               OpMemberDecorate %123 0 Offset 0
               OpMemberDecorate %123 1 Offset 4
               OpMemberDecorate %123 2 Offset 8
               OpDecorate %123 Block
               OpDecorate %134 BuiltIn GlobalInvocationId
               OpDecorate %140 SpecId 0
               OpDecorate %150 SpecId 1
               OpDecorate %191 ArrayStride 8
               OpMemberDecorate %192 0 NonWritable
               OpMemberDecorate %192 0 Offset 0
               OpDecorate %192 BufferBlock
               OpDecorate %194 DescriptorSet 0
               OpDecorate %194 Binding 0
               OpDecorate %251 RelaxedPrecision
               OpDecorate %251 DescriptorSet 0
               OpDecorate %251 Binding 1
               OpDecorate %251 NonReadable
               OpDecorate %252 RelaxedPrecision
               OpDecorate %261 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 16
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
          %9 = OpTypeVector %6 4
         %10 = OpTypePointer Function %9
         %11 = OpTypePointer Function %6
         %12 = OpTypeFunction %9 %8 %10 %11
         %26 = OpConstant %6 0x1.e64p-1
         %27 = OpConstant %6 0x1p+0
         %28 = OpConstantComposite %9 %26 %27 %26 %27
         %33 = OpConstant %6 0x1.0ccp+0
         %34 = OpConstant %6 0x1.198p+0
         %65 = OpTypeInt 32 0
         %66 = OpConstant %65 3
        %101 = OpConstant %6 0x1.cccp-1
        %102 = OpConstantComposite %9 %27 %27 %27 %101
        %105 = OpConstant %6 0x0p+0
        %106 = OpConstantComposite %9 %105 %105 %105 %105
        %118 = OpTypePointer Function %65
        %120 = OpTypeFloat 32
        %121 = OpTypeInt 32 1
        %122 = OpTypeVector %121 2
        %123 = OpTypeStruct %65 %120 %122
        %124 = OpTypePointer PushConstant %123
        %125 = OpVariable %124 PushConstant
        %126 = OpConstant %121 0
        %127 = OpTypePointer PushConstant %65
        %130 = OpTypePointer Function %120
        %132 = OpTypeVector %65 3
        %133 = OpTypePointer Input %132
        %134 = OpVariable %133 Input
        %135 = OpConstant %65 0
        %136 = OpTypePointer Input %65
        %140 = OpSpecConstant %65 1
        %143 = OpConstant %120 0.5
        %146 = OpConstant %65 1
        %150 = OpSpecConstant %65 1
        %162 = OpConstant %121 1
        %163 = OpTypePointer PushConstant %120
        %167 = OpTypePointer Function %122
        %169 = OpConstant %121 2
        %170 = OpTypePointer PushConstant %122
        %178 = OpConstant %6 0x1.33p-2
        %188 = OpTypeBool
        %191 = OpTypeRuntimeArray %9
        %192 = OpTypeStruct %191
        %193 = OpTypePointer Uniform %192
        %194 = OpVariable %193 Uniform
        %196 = OpTypePointer Uniform %9
        %199 = OpTypePointer Function %121
        %249 = OpTypeImage %120 2D 0 0 0 2 Rgba16f
        %250 = OpTypePointer UniformConstant %249
        %251 = OpVariable %250 UniformConstant
        %253 = OpTypeVector %65 2
        %258 = OpTypeVector %120 4
        %260 = OpConstant %65 8
        %261 = OpConstantComposite %132 %260 %260 %146
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %119 = OpVariable %118 Function
        %131 = OpVariable %130 Function
        %145 = OpVariable %130 Function
        %154 = OpVariable %8 Function
        %160 = OpVariable %10 Function
        %161 = OpVariable %11 Function
        %168 = OpVariable %167 Function
        %176 = OpVariable %11 Function
        %180 = OpVariable %118 Function
        %190 = OpVariable %10 Function
        %200 = OpVariable %199 Function
        %213 = OpVariable %199 Function
        %235 = OpVariable %8 Function
        %236 = OpVariable %10 Function
        %238 = OpVariable %11 Function
        %128 = OpAccessChain %127 %125 %126
        %129 = OpLoad %65 %128
               OpStore %119 %129
        %137 = OpAccessChain %136 %134 %135
        %138 = OpLoad %65 %137
        %139 = OpConvertUToF %120 %138
        %141 = OpConvertUToF %120 %140
        %142 = OpFDiv %120 %139 %141
        %144 = OpFSub %120 %142 %143
               OpStore %131 %144
        %147 = OpAccessChain %136 %134 %146
        %148 = OpLoad %65 %147
        %149 = OpConvertUToF %120 %148
        %151 = OpConvertUToF %120 %150
        %152 = OpFDiv %120 %149 %151
        %153 = OpFSub %120 %152 %143
               OpStore %145 %153
        %155 = OpLoad %120 %131
        %156 = OpFConvert %6 %155
        %157 = OpLoad %120 %145
        %158 = OpFConvert %6 %157
        %159 = OpCompositeConstruct %7 %156 %158
               OpStore %154 %159
               OpStore %160 %106
        %164 = OpAccessChain %163 %125 %162
        %165 = OpLoad %120 %164
        %166 = OpFConvert %6 %165
               OpStore %161 %166
        %171 = OpAccessChain %170 %125 %169
        %172 = OpLoad %122 %171
        %173 = OpCompositeExtract %121 %172 0
        %174 = OpCompositeExtract %121 %172 1
        %175 = OpCompositeConstruct %122 %173 %174
               OpStore %168 %175
        %177 = OpLoad %6 %161
        %179 = OpFMul %6 %177 %178
               OpStore %176 %179
               OpStore %180 %135
               OpBranch %181
        %181 = OpLabel
               OpLoopMerge %183 %184 None
               OpBranch %185
        %185 = OpLabel
        %186 = OpLoad %65 %180
        %187 = OpLoad %65 %119
        %189 = OpULessThan %188 %186 %187
               OpBranchConditional %189 %182 %183
        %182 = OpLabel
        %195 = OpLoad %65 %180
        %197 = OpAccessChain %196 %194 %126 %195
        %198 = OpLoad %9 %197
               OpStore %190 %198
        %201 = OpAccessChain %199 %168 %146
        %202 = OpLoad %121 %201
        %203 = OpSNegate %121 %202
               OpStore %200 %203
               OpBranch %204
        %204 = OpLabel
               OpLoopMerge %206 %207 None
               OpBranch %208
        %208 = OpLabel
        %209 = OpLoad %121 %200
        %210 = OpAccessChain %199 %168 %146
        %211 = OpLoad %121 %210
        %212 = OpSLessThanEqual %188 %209 %211
               OpBranchConditional %212 %205 %206
        %205 = OpLabel
        %214 = OpAccessChain %199 %168 %135
        %215 = OpLoad %121 %214
        %216 = OpSNegate %121 %215
               OpStore %213 %216
               OpBranch %217
        %217 = OpLabel
               OpLoopMerge %219 %220 None
               OpBranch %221
        %221 = OpLabel
        %222 = OpLoad %121 %213
        %223 = OpAccessChain %199 %168 %135
        %224 = OpLoad %121 %223
        %225 = OpSLessThanEqual %188 %222 %224
               OpBranchConditional %225 %218 %219
        %218 = OpLabel
        %226 = OpLoad %7 %154
        %227 = OpLoad %6 %176
        %228 = OpLoad %121 %213
        %229 = OpConvertSToF %6 %228
        %230 = OpLoad %121 %200
        %231 = OpConvertSToF %6 %230
        %232 = OpCompositeConstruct %7 %229 %231
        %233 = OpVectorTimesScalar %7 %232 %227
        %234 = OpFAdd %7 %226 %233
               OpStore %235 %234
        %237 = OpLoad %9 %190
               OpStore %236 %237
        %239 = OpLoad %6 %161
               OpStore %238 %239
        %240 = OpFunctionCall %9 %16 %235 %236 %238
        %241 = OpLoad %9 %160
        %242 = OpFAdd %9 %241 %240
               OpStore %160 %242
               OpBranch %220
        %220 = OpLabel
        %243 = OpLoad %121 %213
        %244 = OpIAdd %121 %243 %162
               OpStore %213 %244
               OpBranch %217
        %219 = OpLabel
               OpBranch %207
        %207 = OpLabel
        %245 = OpLoad %121 %200
        %246 = OpIAdd %121 %245 %162
               OpStore %200 %246
               OpBranch %204
        %206 = OpLabel
               OpBranch %184
        %184 = OpLabel
        %247 = OpLoad %65 %180
        %248 = OpIAdd %65 %247 %162
               OpStore %180 %248
               OpBranch %181
        %183 = OpLabel
        %252 = OpLoad %249 %251
        %254 = OpLoad %132 %134
        %255 = OpVectorShuffle %253 %254 %254 0 1
        %256 = OpBitcast %122 %255
        %257 = OpLoad %9 %160
        %259 = OpFConvert %258 %257
               OpImageWrite %252 %256 %259
               OpReturn
               OpFunctionEnd
         %16 = OpFunction %9 None %12
         %13 = OpFunctionParameter %8
         %14 = OpFunctionParameter %10
         %15 = OpFunctionParameter %11
         %17 = OpLabel
         %18 = OpVariable %8 Function
         %23 = OpVariable %10 Function
         %30 = OpVariable %10 Function
         %41 = OpVariable %10 Function
         %45 = OpVariable %10 Function
         %49 = OpVariable %10 Function
        %100 = OpVariable %10 Function
         %19 = OpLoad %7 %13
         %20 = OpLoad %9 %14
         %21 = OpVectorShuffle %7 %20 %20 0 1
         %22 = OpFSub %7 %19 %21
               OpStore %18 %22
         %24 = OpLoad %7 %18
         %25 = OpVectorShuffle %9 %24 %24 0 0 1 1
         %29 = OpFMul %9 %25 %28
               OpStore %23 %29
         %31 = OpLoad %7 %18
         %32 = OpVectorShuffle %9 %31 %31 0 0 1 1
         %35 = OpLoad %6 %15
         %36 = OpFAdd %6 %34 %35
         %37 = OpLoad %6 %15
         %38 = OpFAdd %6 %34 %37
         %39 = OpCompositeConstruct %9 %33 %36 %33 %38
         %40 = OpFMul %9 %32 %39
               OpStore %30 %40
         %42 = OpLoad %9 %23
         %43 = OpLoad %9 %23
         %44 = OpFMul %9 %42 %43
               OpStore %41 %44
         %46 = OpLoad %9 %30
         %47 = OpLoad %9 %30
         %48 = OpFMul %9 %46 %47
               OpStore %45 %48
         %50 = OpLoad %9 %41
         %51 = OpVectorShuffle %7 %50 %50 0 1
         %52 = OpLoad %9 %41
         %53 = OpVectorShuffle %7 %52 %52 2 3
         %54 = OpFAdd %7 %51 %53
         %55 = OpLoad %9 %45
         %56 = OpVectorShuffle %7 %55 %55 0 1
         %57 = OpLoad %9 %45
         %58 = OpVectorShuffle %7 %57 %57 2 3
         %59 = OpFAdd %7 %56 %58
         %60 = OpCompositeExtract %6 %54 0
         %61 = OpCompositeExtract %6 %54 1
         %62 = OpCompositeExtract %6 %59 0
         %63 = OpCompositeExtract %6 %59 1
         %64 = OpCompositeConstruct %9 %60 %61 %62 %63
         %67 = OpAccessChain %11 %14 %66
         %68 = OpLoad %6 %67
         %69 = OpVectorTimesScalar %9 %64 %68
               OpStore %49 %69
         %70 = OpLoad %9 %49
         %71 = OpLoad %9 %49
         %72 = OpFMul %9 %70 %71
         %73 = OpLoad %9 %49
         %74 = OpFAdd %9 %72 %73
               OpStore %49 %74
         %75 = OpLoad %9 %49
         %76 = OpLoad %9 %49
         %77 = OpFMul %9 %75 %76
         %78 = OpLoad %9 %49
         %79 = OpFAdd %9 %77 %78
               OpStore %49 %79
         %80 = OpLoad %9 %49
         %81 = OpLoad %9 %49
         %82 = OpFMul %9 %80 %81
         %83 = OpLoad %9 %49
         %84 = OpFAdd %9 %82 %83
               OpStore %49 %84
         %85 = OpLoad %9 %49
         %86 = OpLoad %9 %49
         %87 = OpFMul %9 %85 %86
         %88 = OpLoad %9 %49
         %89 = OpFAdd %9 %87 %88
               OpStore %49 %89
         %90 = OpLoad %9 %49
         %91 = OpLoad %9 %49
         %92 = OpFMul %9 %90 %91
         %93 = OpLoad %9 %49
         %94 = OpFAdd %9 %92 %93
               OpStore %49 %94
         %95 = OpLoad %9 %49
         %96 = OpLoad %9 %49
         %97 = OpFMul %9 %95 %96
         %98 = OpLoad %9 %49
         %99 = OpFAdd %9 %97 %98
               OpStore %49 %99
        %103 = OpLoad %9 %49
        %104 = OpFSub %9 %102 %103
        %107 = OpExtInst %9 %1 FMax %104 %106
               OpStore %100 %107
        %108 = OpAccessChain %11 %100 %66
        %109 = OpLoad %6 %108
        %110 = OpLoad %9 %100
        %111 = OpCompositeConstruct %9 %109 %109 %109 %109
        %112 = OpFSub %9 %110 %111
               OpStore %100 %112
        %113 = OpLoad %9 %100
        %114 = OpExtInst %9 %1 FMax %113 %106
               OpStore %100 %114
        %115 = OpLoad %9 %100
               OpReturnValue %115
               OpFunctionEnd
