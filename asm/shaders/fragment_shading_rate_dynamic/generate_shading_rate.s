; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 255
; Schema: 0
               OpCapability Shader
               OpCapability StorageImageExtendedFormats
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %11
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "x0"
               OpName %11 "gl_GlobalInvocationID"
               OpName %16 "y0"
               OpName %20 "frame_width"
               OpName %24 "FrequencyInformation"
               OpMemberName %24 0 "settings"
               OpMemberName %24 1 "frequency_information"
               OpMemberName %24 2 "rates"
               OpName %26 "params"
               OpName %32 "frame_height"
               OpName %35 "output_width"
               OpName %39 "output_height"
               OpName %43 "delta_x"
               OpName %53 "delta_y"
               OpName %75 "max_freqs"
               OpName %78 "i"
               OpName %87 "j"
               OpName %98 "coord"
               OpName %112 "freq"
               OpName %115 "input_frequency"
               OpName %135 "freqs"
               OpName %144 "max_rate"
               OpName %151 "optimal_rate"
               OpName %162 "n_rates"
               OpName %165 "optimal_rate_index"
               OpName %166 "current_cost"
               OpName %173 "i"
               OpName %183 "rate"
               OpName %189 "cost"
               OpName %226 "optimal_rate_x"
               OpName %230 "optimal_rate_y"
               OpName %234 "rate_code"
               OpName %244 "output_sampling_rate"
               OpDecorate %11 BuiltIn GlobalInvocationId
               OpDecorate %23 ArrayStride 8
               OpMemberDecorate %24 0 Offset 0
               OpMemberDecorate %24 1 Offset 16
               OpMemberDecorate %24 2 Offset 32
               OpDecorate %24 BufferBlock
               OpDecorate %26 DescriptorSet 0
               OpDecorate %26 Binding 2
               OpDecorate %115 DescriptorSet 0
               OpDecorate %115 Binding 0
               OpDecorate %115 NonWritable
               OpDecorate %244 DescriptorSet 0
               OpDecorate %244 Binding 1
               OpDecorate %244 NonReadable
               OpDecorate %254 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 0
          %7 = OpTypePointer Function %6
          %9 = OpTypeVector %6 3
         %10 = OpTypePointer Input %9
         %11 = OpVariable %10 Input
         %12 = OpConstant %6 0
         %13 = OpTypePointer Input %6
         %17 = OpConstant %6 1
         %21 = OpTypeVector %6 4
         %22 = OpTypeVector %6 2
         %23 = OpTypeRuntimeArray %22
         %24 = OpTypeStruct %21 %21 %23
         %25 = OpTypePointer Uniform %24
         %26 = OpVariable %25 Uniform
         %27 = OpTypeInt 32 1
         %28 = OpConstant %27 0
         %29 = OpTypePointer Uniform %6
         %36 = OpConstant %6 2
         %40 = OpConstant %6 3
         %45 = OpTypeFloat 32
         %64 = OpTypeBool
         %73 = OpTypeVector %45 2
         %74 = OpTypePointer Function %73
         %76 = OpConstant %45 0
         %77 = OpConstantComposite %73 %76 %76
         %96 = OpTypeVector %27 2
         %97 = OpTypePointer Function %96
        %113 = OpTypeImage %6 2D 0 0 0 2 Rg8ui
        %114 = OpTypePointer UniformConstant %113
        %115 = OpVariable %114 UniformConstant
        %119 = OpTypeVector %45 4
        %124 = OpConstant %45 255
        %131 = OpConstant %27 1
        %136 = OpConstant %45 1.25
        %140 = OpConstant %45 1
        %141 = OpConstantComposite %73 %140 %140
        %143 = OpTypePointer Function %45
        %167 = OpConstant %45 2
        %182 = OpTypePointer Function %22
        %184 = OpConstant %27 2
        %186 = OpTypePointer Uniform %22
        %239 = OpConstant %6 12
        %242 = OpTypeImage %6 2D 0 0 0 2 R8ui
        %243 = OpTypePointer UniformConstant %242
        %244 = OpVariable %243 UniformConstant
        %253 = OpConstant %6 8
        %254 = OpConstantComposite %9 %253 %253 %17
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %8 = OpVariable %7 Function
         %16 = OpVariable %7 Function
         %20 = OpVariable %7 Function
         %32 = OpVariable %7 Function
         %35 = OpVariable %7 Function
         %39 = OpVariable %7 Function
         %43 = OpVariable %7 Function
         %53 = OpVariable %7 Function
         %75 = OpVariable %74 Function
         %78 = OpVariable %7 Function
         %87 = OpVariable %7 Function
         %98 = OpVariable %97 Function
        %112 = OpVariable %74 Function
        %135 = OpVariable %74 Function
        %144 = OpVariable %143 Function
        %151 = OpVariable %74 Function
        %162 = OpVariable %7 Function
        %165 = OpVariable %7 Function
        %166 = OpVariable %143 Function
        %173 = OpVariable %7 Function
        %183 = OpVariable %182 Function
        %189 = OpVariable %143 Function
        %226 = OpVariable %7 Function
        %230 = OpVariable %7 Function
        %234 = OpVariable %7 Function
         %14 = OpAccessChain %13 %11 %12
         %15 = OpLoad %6 %14
               OpStore %8 %15
         %18 = OpAccessChain %13 %11 %17
         %19 = OpLoad %6 %18
               OpStore %16 %19
         %30 = OpAccessChain %29 %26 %28 %12
         %31 = OpLoad %6 %30
               OpStore %20 %31
         %33 = OpAccessChain %29 %26 %28 %17
         %34 = OpLoad %6 %33
               OpStore %32 %34
         %37 = OpAccessChain %29 %26 %28 %36
         %38 = OpLoad %6 %37
               OpStore %35 %38
         %41 = OpAccessChain %29 %26 %28 %40
         %42 = OpLoad %6 %41
               OpStore %39 %42
         %44 = OpLoad %6 %20
         %46 = OpConvertUToF %45 %44
         %47 = OpLoad %6 %35
         %48 = OpConvertUToF %45 %47
         %49 = OpFDiv %45 %46 %48
         %50 = OpExtInst %45 %1 Round %49
         %51 = OpConvertFToU %6 %50
         %52 = OpExtInst %6 %1 UMax %17 %51
               OpStore %43 %52
         %54 = OpLoad %6 %32
         %55 = OpConvertUToF %45 %54
         %56 = OpLoad %6 %39
         %57 = OpConvertUToF %45 %56
         %58 = OpFDiv %45 %55 %57
         %59 = OpExtInst %45 %1 Round %58
         %60 = OpConvertFToU %6 %59
         %61 = OpExtInst %6 %1 UMax %17 %60
               OpStore %53 %61
         %62 = OpLoad %6 %8
         %63 = OpLoad %6 %35
         %65 = OpUGreaterThanEqual %64 %62 %63
         %66 = OpLoad %6 %16
         %67 = OpLoad %6 %39
         %68 = OpUGreaterThanEqual %64 %66 %67
         %69 = OpLogicalOr %64 %65 %68
               OpSelectionMerge %71 None
               OpBranchConditional %69 %70 %71
         %70 = OpLabel
               OpReturn
         %71 = OpLabel
               OpStore %75 %77
               OpStore %78 %12
               OpBranch %79
         %79 = OpLabel
               OpLoopMerge %81 %82 None
               OpBranch %83
         %83 = OpLabel
         %84 = OpLoad %6 %78
         %85 = OpLoad %6 %43
         %86 = OpULessThan %64 %84 %85
               OpBranchConditional %86 %80 %81
         %80 = OpLabel
               OpStore %87 %12
               OpBranch %88
         %88 = OpLabel
               OpLoopMerge %90 %91 None
               OpBranch %92
         %92 = OpLabel
         %93 = OpLoad %6 %87
         %94 = OpLoad %6 %53
         %95 = OpULessThan %64 %93 %94
               OpBranchConditional %95 %89 %90
         %89 = OpLabel
         %99 = OpLoad %6 %43
        %100 = OpLoad %6 %8
        %101 = OpIMul %6 %99 %100
        %102 = OpLoad %6 %78
        %103 = OpIAdd %6 %101 %102
        %104 = OpBitcast %27 %103
        %105 = OpLoad %6 %53
        %106 = OpLoad %6 %16
        %107 = OpIMul %6 %105 %106
        %108 = OpLoad %6 %87
        %109 = OpIAdd %6 %107 %108
        %110 = OpBitcast %27 %109
        %111 = OpCompositeConstruct %96 %104 %110
               OpStore %98 %111
        %116 = OpLoad %113 %115
        %117 = OpLoad %96 %98
        %118 = OpImageRead %21 %116 %117
        %120 = OpConvertUToF %119 %118
        %121 = OpCompositeExtract %45 %120 0
        %122 = OpCompositeExtract %45 %120 1
        %123 = OpCompositeConstruct %73 %121 %122
        %125 = OpCompositeConstruct %73 %124 %124
        %126 = OpFDiv %73 %123 %125
               OpStore %112 %126
        %127 = OpLoad %73 %75
        %128 = OpLoad %73 %112
        %129 = OpExtInst %73 %1 FMax %127 %128
               OpStore %75 %129
               OpBranch %91
         %91 = OpLabel
        %130 = OpLoad %6 %87
        %132 = OpIAdd %6 %130 %131
               OpStore %87 %132
               OpBranch %88
         %90 = OpLabel
               OpBranch %82
         %82 = OpLabel
        %133 = OpLoad %6 %78
        %134 = OpIAdd %6 %133 %131
               OpStore %78 %134
               OpBranch %79
         %81 = OpLabel
        %137 = OpLoad %73 %75
        %138 = OpExtInst %73 %1 Sqrt %137
        %139 = OpVectorTimesScalar %73 %138 %136
        %142 = OpExtInst %73 %1 FMin %139 %141
               OpStore %135 %142
        %145 = OpAccessChain %29 %26 %131 %12
        %146 = OpLoad %6 %145
        %147 = OpAccessChain %29 %26 %131 %17
        %148 = OpLoad %6 %147
        %149 = OpExtInst %6 %1 UMax %146 %148
        %150 = OpConvertUToF %45 %149
               OpStore %144 %150
        %152 = OpLoad %73 %135
        %153 = OpFMul %73 %152 %141
        %154 = OpLoad %73 %135
        %155 = OpCompositeConstruct %73 %140 %140
        %156 = OpFSub %73 %155 %154
        %157 = OpLoad %45 %144
        %158 = OpLoad %45 %144
        %159 = OpCompositeConstruct %73 %157 %158
        %160 = OpFMul %73 %156 %159
        %161 = OpFAdd %73 %153 %160
               OpStore %151 %161
        %163 = OpAccessChain %29 %26 %131 %36
        %164 = OpLoad %6 %163
               OpStore %162 %164
               OpStore %165 %12
        %168 = OpLoad %45 %144
        %169 = OpFMul %45 %167 %168
        %170 = OpLoad %45 %144
        %171 = OpFMul %45 %169 %170
        %172 = OpFAdd %45 %140 %171
               OpStore %166 %172
               OpStore %173 %12
               OpBranch %174
        %174 = OpLabel
               OpLoopMerge %176 %177 None
               OpBranch %178
        %178 = OpLabel
        %179 = OpLoad %6 %173
        %180 = OpLoad %6 %162
        %181 = OpULessThan %64 %179 %180
               OpBranchConditional %181 %175 %176
        %175 = OpLabel
        %185 = OpLoad %6 %173
        %187 = OpAccessChain %186 %26 %184 %185
        %188 = OpLoad %22 %187
               OpStore %183 %188
        %190 = OpAccessChain %7 %183 %12
        %191 = OpLoad %6 %190
        %192 = OpConvertUToF %45 %191
        %193 = OpAccessChain %143 %151 %12
        %194 = OpLoad %45 %193
        %195 = OpFSub %45 %192 %194
        %196 = OpAccessChain %7 %183 %12
        %197 = OpLoad %6 %196
        %198 = OpConvertUToF %45 %197
        %199 = OpAccessChain %143 %151 %12
        %200 = OpLoad %45 %199
        %201 = OpFSub %45 %198 %200
        %202 = OpFMul %45 %195 %201
        %203 = OpAccessChain %7 %183 %17
        %204 = OpLoad %6 %203
        %205 = OpConvertUToF %45 %204
        %206 = OpAccessChain %143 %151 %17
        %207 = OpLoad %45 %206
        %208 = OpFSub %45 %205 %207
        %209 = OpAccessChain %7 %183 %17
        %210 = OpLoad %6 %209
        %211 = OpConvertUToF %45 %210
        %212 = OpAccessChain %143 %151 %17
        %213 = OpLoad %45 %212
        %214 = OpFSub %45 %211 %213
        %215 = OpFMul %45 %208 %214
        %216 = OpFAdd %45 %202 %215
               OpStore %189 %216
        %217 = OpLoad %45 %189
        %218 = OpLoad %45 %166
        %219 = OpFOrdLessThan %64 %217 %218
               OpSelectionMerge %221 None
               OpBranchConditional %219 %220 %221
        %220 = OpLabel
        %222 = OpLoad %45 %189
               OpStore %166 %222
        %223 = OpLoad %6 %173
               OpStore %165 %223
               OpBranch %221
        %221 = OpLabel
               OpBranch %177
        %177 = OpLabel
        %224 = OpLoad %6 %173
        %225 = OpIAdd %6 %224 %131
               OpStore %173 %225
               OpBranch %174
        %176 = OpLabel
        %227 = OpLoad %6 %165
        %228 = OpAccessChain %29 %26 %184 %227 %12
        %229 = OpLoad %6 %228
               OpStore %226 %229
        %231 = OpLoad %6 %165
        %232 = OpAccessChain %29 %26 %184 %231 %17
        %233 = OpLoad %6 %232
               OpStore %230 %233
        %235 = OpLoad %6 %230
        %236 = OpShiftRightLogical %6 %235 %131
        %237 = OpLoad %6 %226
        %238 = OpShiftLeftLogical %6 %237 %131
        %240 = OpBitwiseAnd %6 %238 %239
        %241 = OpBitwiseOr %6 %236 %240
               OpStore %234 %241
        %245 = OpLoad %242 %244
        %246 = OpLoad %6 %8
        %247 = OpBitcast %27 %246
        %248 = OpLoad %6 %16
        %249 = OpBitcast %27 %248
        %250 = OpCompositeConstruct %96 %247 %249
        %251 = OpLoad %6 %234
        %252 = OpCompositeConstruct %21 %251 %251 %251 %251
               OpImageWrite %245 %250 %252
               OpReturn
               OpFunctionEnd
