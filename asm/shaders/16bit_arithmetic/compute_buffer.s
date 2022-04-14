; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 256
; Schema: 0
               OpCapability Shader
               OpCapability StorageBuffer16BitAccess
               OpExtension "SPV_KHR_16bit_storage"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %130
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_16bit_storage"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %16 "compute_blob(vf2;vf4;f1;"
               OpName %13 "pos"
               OpName %14 "blob"
               OpName %15 "seed"
               OpName %18 "offset"
               OpName %23 "s_offset"
               OpName %29 "r_offset"
               OpName %33 "g_offset"
               OpName %37 "b_offset"
               OpName %41 "r_dot"
               OpName %45 "g_dot"
               OpName %49 "b_dot"
               OpName %53 "s_dot"
               OpName %57 "dots"
               OpName %98 "parabolas"
               OpName %117 "num_blobs"
               OpName %120 "Registers"
               OpMemberName %120 0 "num_blobs"
               OpMemberName %120 1 "seed"
               OpMemberName %120 2 "range"
               OpName %122 "registers"
               OpName %127 "x"
               OpName %130 "gl_GlobalInvocationID"
               OpName %136 "WIDTH"
               OpName %141 "y"
               OpName %146 "HEIGHT"
               OpName %150 "pos"
               OpName %154 "result"
               OpName %155 "seed"
               OpName %161 "range"
               OpName %169 "stride"
               OpName %173 "i"
               OpName %183 "blob"
               OpName %187 "SSBO"
               OpMemberName %187 0 "blob_data"
               OpName %189 ""
               OpName %196 "y"
               OpName %209 "x"
               OpName %231 "param"
               OpName %232 "param"
               OpName %234 "param"
               OpName %247 "o_results"
               OpMemberDecorate %120 0 Offset 0
               OpMemberDecorate %120 1 Offset 4
               OpMemberDecorate %120 2 Offset 8
               OpDecorate %120 Block
               OpDecorate %130 BuiltIn GlobalInvocationId
               OpDecorate %136 SpecId 0
               OpDecorate %146 SpecId 1
               OpDecorate %186 ArrayStride 8
               OpMemberDecorate %187 0 NonWritable
               OpMemberDecorate %187 0 Offset 0
               OpDecorate %187 BufferBlock
               OpDecorate %189 DescriptorSet 0
               OpDecorate %189 Binding 0
               OpDecorate %247 RelaxedPrecision
               OpDecorate %247 DescriptorSet 0
               OpDecorate %247 Binding 1
               OpDecorate %247 NonReadable
               OpDecorate %248 RelaxedPrecision
               OpDecorate %255 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
          %9 = OpTypeVector %6 4
         %10 = OpTypePointer Function %9
         %11 = OpTypePointer Function %6
         %12 = OpTypeFunction %9 %8 %10 %11
         %25 = OpConstant %6 1.10000002
         %31 = OpConstant %6 0.949999988
         %35 = OpConstant %6 1
         %39 = OpConstant %6 1.04999995
         %63 = OpTypeInt 32 0
         %64 = OpConstant %63 3
         %99 = OpConstant %6 0.899999976
        %100 = OpConstantComposite %9 %35 %35 %35 %99
        %103 = OpConstant %6 0
        %104 = OpConstantComposite %9 %103 %103 %103 %103
        %116 = OpTypePointer Function %63
        %118 = OpTypeInt 32 1
        %119 = OpTypeVector %118 2
        %120 = OpTypeStruct %63 %6 %119
        %121 = OpTypePointer PushConstant %120
        %122 = OpVariable %121 PushConstant
        %123 = OpConstant %118 0
        %124 = OpTypePointer PushConstant %63
        %128 = OpTypeVector %63 3
        %129 = OpTypePointer Input %128
        %130 = OpVariable %129 Input
        %131 = OpConstant %63 0
        %132 = OpTypePointer Input %63
        %136 = OpSpecConstant %63 1
        %139 = OpConstant %6 0.5
        %142 = OpConstant %63 1
        %146 = OpSpecConstant %63 1
        %156 = OpConstant %118 1
        %157 = OpTypePointer PushConstant %6
        %160 = OpTypePointer Function %119
        %162 = OpConstant %118 2
        %163 = OpTypePointer PushConstant %119
        %171 = OpConstant %6 0.300000012
        %181 = OpTypeBool
        %184 = OpTypeFloat 16
        %185 = OpTypeVector %184 4
        %186 = OpTypeRuntimeArray %185
        %187 = OpTypeStruct %186
        %188 = OpTypePointer Uniform %187
        %189 = OpVariable %188 Uniform
        %191 = OpTypePointer Uniform %185
        %195 = OpTypePointer Function %118
        %245 = OpTypeImage %6 2D 0 0 0 2 Rgba16f
        %246 = OpTypePointer UniformConstant %245
        %247 = OpVariable %246 UniformConstant
        %249 = OpTypeVector %63 2
        %254 = OpConstant %63 8
        %255 = OpConstantComposite %128 %254 %254 %142
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %117 = OpVariable %116 Function
        %127 = OpVariable %11 Function
        %141 = OpVariable %11 Function
        %150 = OpVariable %8 Function
        %154 = OpVariable %10 Function
        %155 = OpVariable %11 Function
        %161 = OpVariable %160 Function
        %169 = OpVariable %11 Function
        %173 = OpVariable %116 Function
        %183 = OpVariable %10 Function
        %196 = OpVariable %195 Function
        %209 = OpVariable %195 Function
        %231 = OpVariable %8 Function
        %232 = OpVariable %10 Function
        %234 = OpVariable %11 Function
        %125 = OpAccessChain %124 %122 %123
        %126 = OpLoad %63 %125
               OpStore %117 %126
        %133 = OpAccessChain %132 %130 %131
        %134 = OpLoad %63 %133
        %135 = OpConvertUToF %6 %134
        %137 = OpConvertUToF %6 %136
        %138 = OpFDiv %6 %135 %137
        %140 = OpFSub %6 %138 %139
               OpStore %127 %140
        %143 = OpAccessChain %132 %130 %142
        %144 = OpLoad %63 %143
        %145 = OpConvertUToF %6 %144
        %147 = OpConvertUToF %6 %146
        %148 = OpFDiv %6 %145 %147
        %149 = OpFSub %6 %148 %139
               OpStore %141 %149
        %151 = OpLoad %6 %127
        %152 = OpLoad %6 %141
        %153 = OpCompositeConstruct %7 %151 %152
               OpStore %150 %153
               OpStore %154 %104
        %158 = OpAccessChain %157 %122 %156
        %159 = OpLoad %6 %158
               OpStore %155 %159
        %164 = OpAccessChain %163 %122 %162
        %165 = OpLoad %119 %164
        %166 = OpCompositeExtract %118 %165 0
        %167 = OpCompositeExtract %118 %165 1
        %168 = OpCompositeConstruct %119 %166 %167
               OpStore %161 %168
        %170 = OpLoad %6 %155
        %172 = OpFMul %6 %170 %171
               OpStore %169 %172
               OpStore %173 %131
               OpBranch %174
        %174 = OpLabel
               OpLoopMerge %176 %177 None
               OpBranch %178
        %178 = OpLabel
        %179 = OpLoad %63 %173
        %180 = OpLoad %63 %117
        %182 = OpULessThan %181 %179 %180
               OpBranchConditional %182 %175 %176
        %175 = OpLabel
        %190 = OpLoad %63 %173
        %192 = OpAccessChain %191 %189 %123 %190
        %193 = OpLoad %185 %192
        %194 = OpFConvert %9 %193
               OpStore %183 %194
        %197 = OpAccessChain %195 %161 %142
        %198 = OpLoad %118 %197
        %199 = OpSNegate %118 %198
               OpStore %196 %199
               OpBranch %200
        %200 = OpLabel
               OpLoopMerge %202 %203 None
               OpBranch %204
        %204 = OpLabel
        %205 = OpLoad %118 %196
        %206 = OpAccessChain %195 %161 %142
        %207 = OpLoad %118 %206
        %208 = OpSLessThanEqual %181 %205 %207
               OpBranchConditional %208 %201 %202
        %201 = OpLabel
        %210 = OpAccessChain %195 %161 %131
        %211 = OpLoad %118 %210
        %212 = OpSNegate %118 %211
               OpStore %209 %212
               OpBranch %213
        %213 = OpLabel
               OpLoopMerge %215 %216 None
               OpBranch %217
        %217 = OpLabel
        %218 = OpLoad %118 %209
        %219 = OpAccessChain %195 %161 %131
        %220 = OpLoad %118 %219
        %221 = OpSLessThanEqual %181 %218 %220
               OpBranchConditional %221 %214 %215
        %214 = OpLabel
        %222 = OpLoad %7 %150
        %223 = OpLoad %6 %169
        %224 = OpLoad %118 %209
        %225 = OpConvertSToF %6 %224
        %226 = OpLoad %118 %196
        %227 = OpConvertSToF %6 %226
        %228 = OpCompositeConstruct %7 %225 %227
        %229 = OpVectorTimesScalar %7 %228 %223
        %230 = OpFAdd %7 %222 %229
               OpStore %231 %230
        %233 = OpLoad %9 %183
               OpStore %232 %233
        %235 = OpLoad %6 %155
               OpStore %234 %235
        %236 = OpFunctionCall %9 %16 %231 %232 %234
        %237 = OpLoad %9 %154
        %238 = OpFAdd %9 %237 %236
               OpStore %154 %238
               OpBranch %216
        %216 = OpLabel
        %239 = OpLoad %118 %209
        %240 = OpIAdd %118 %239 %156
               OpStore %209 %240
               OpBranch %213
        %215 = OpLabel
               OpBranch %203
        %203 = OpLabel
        %241 = OpLoad %118 %196
        %242 = OpIAdd %118 %241 %156
               OpStore %196 %242
               OpBranch %200
        %202 = OpLabel
               OpBranch %177
        %177 = OpLabel
        %243 = OpLoad %63 %173
        %244 = OpIAdd %63 %243 %156
               OpStore %173 %244
               OpBranch %174
        %176 = OpLabel
        %248 = OpLoad %245 %247
        %250 = OpLoad %128 %130
        %251 = OpVectorShuffle %249 %250 %250 0 1
        %252 = OpBitcast %119 %251
        %253 = OpLoad %9 %154
               OpImageWrite %248 %252 %253
               OpReturn
               OpFunctionEnd
         %16 = OpFunction %9 None %12
         %13 = OpFunctionParameter %8
         %14 = OpFunctionParameter %10
         %15 = OpFunctionParameter %11
         %17 = OpLabel
         %18 = OpVariable %8 Function
         %23 = OpVariable %8 Function
         %29 = OpVariable %8 Function
         %33 = OpVariable %8 Function
         %37 = OpVariable %8 Function
         %41 = OpVariable %11 Function
         %45 = OpVariable %11 Function
         %49 = OpVariable %11 Function
         %53 = OpVariable %11 Function
         %57 = OpVariable %10 Function
         %98 = OpVariable %10 Function
         %19 = OpLoad %7 %13
         %20 = OpLoad %9 %14
         %21 = OpVectorShuffle %7 %20 %20 0 1
         %22 = OpFSub %7 %19 %21
               OpStore %18 %22
         %24 = OpLoad %7 %18
         %26 = OpLoad %6 %15
         %27 = OpFAdd %6 %25 %26
         %28 = OpVectorTimesScalar %7 %24 %27
               OpStore %23 %28
         %30 = OpLoad %7 %18
         %32 = OpVectorTimesScalar %7 %30 %31
               OpStore %29 %32
         %34 = OpLoad %7 %18
         %36 = OpVectorTimesScalar %7 %34 %35
               OpStore %33 %36
         %38 = OpLoad %7 %18
         %40 = OpVectorTimesScalar %7 %38 %39
               OpStore %37 %40
         %42 = OpLoad %7 %29
         %43 = OpLoad %7 %29
         %44 = OpDot %6 %42 %43
               OpStore %41 %44
         %46 = OpLoad %7 %33
         %47 = OpLoad %7 %33
         %48 = OpDot %6 %46 %47
               OpStore %45 %48
         %50 = OpLoad %7 %37
         %51 = OpLoad %7 %37
         %52 = OpDot %6 %50 %51
               OpStore %49 %52
         %54 = OpLoad %7 %23
         %55 = OpLoad %7 %23
         %56 = OpDot %6 %54 %55
               OpStore %53 %56
         %58 = OpLoad %6 %41
         %59 = OpLoad %6 %45
         %60 = OpLoad %6 %49
         %61 = OpLoad %6 %53
         %62 = OpCompositeConstruct %9 %58 %59 %60 %61
         %65 = OpAccessChain %11 %14 %64
         %66 = OpLoad %6 %65
         %67 = OpVectorTimesScalar %9 %62 %66
               OpStore %57 %67
         %68 = OpLoad %9 %57
         %69 = OpLoad %9 %57
         %70 = OpFMul %9 %68 %69
         %71 = OpLoad %9 %57
         %72 = OpFAdd %9 %70 %71
               OpStore %57 %72
         %73 = OpLoad %9 %57
         %74 = OpLoad %9 %57
         %75 = OpFMul %9 %73 %74
         %76 = OpLoad %9 %57
         %77 = OpFAdd %9 %75 %76
               OpStore %57 %77
         %78 = OpLoad %9 %57
         %79 = OpLoad %9 %57
         %80 = OpFMul %9 %78 %79
         %81 = OpLoad %9 %57
         %82 = OpFAdd %9 %80 %81
               OpStore %57 %82
         %83 = OpLoad %9 %57
         %84 = OpLoad %9 %57
         %85 = OpFMul %9 %83 %84
         %86 = OpLoad %9 %57
         %87 = OpFAdd %9 %85 %86
               OpStore %57 %87
         %88 = OpLoad %9 %57
         %89 = OpLoad %9 %57
         %90 = OpFMul %9 %88 %89
         %91 = OpLoad %9 %57
         %92 = OpFAdd %9 %90 %91
               OpStore %57 %92
         %93 = OpLoad %9 %57
         %94 = OpLoad %9 %57
         %95 = OpFMul %9 %93 %94
         %96 = OpLoad %9 %57
         %97 = OpFAdd %9 %95 %96
               OpStore %57 %97
        %101 = OpLoad %9 %57
        %102 = OpFSub %9 %100 %101
        %105 = OpExtInst %9 %1 FMax %102 %104
               OpStore %98 %105
        %106 = OpAccessChain %11 %98 %64
        %107 = OpLoad %6 %106
        %108 = OpLoad %9 %98
        %109 = OpCompositeConstruct %9 %107 %107 %107 %107
        %110 = OpFSub %9 %108 %109
               OpStore %98 %110
        %111 = OpLoad %9 %98
        %112 = OpExtInst %9 %1 FMax %111 %104
               OpStore %98 %112
        %113 = OpLoad %9 %98
               OpReturnValue %113
               OpFunctionEnd
