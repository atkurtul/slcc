; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 251
; Schema: 0
               OpCapability Shader
               OpCapability ImageQuery
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %index "index"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %uv "uv"
               OpName %ImageInput "ImageInput"
               OpName %neighbors "neighbors"
               OpName %self "self"
               OpName %is_alive "is_alive"
               OpName %total "total"
               OpName %tmp "tmp"
               OpName %tmp_0 "tmp"
               OpName %tmp_1 "tmp"
               OpName %tmp_2 "tmp"
               OpName %tmp_3 "tmp"
               OpName %tmp_4 "tmp"
               OpName %tmp_5 "tmp"
               OpName %tmp_6 "tmp"
               OpName %fresh_color "fresh_color"
               OpName %ImageOutput "ImageOutput"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpDecorate %ImageInput DescriptorSet 1
               OpDecorate %ImageInput Binding 0
               OpDecorate %ImageOutput DescriptorSet 0
               OpDecorate %ImageOutput Binding 0
               OpDecorate %ImageOutput NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %v2uint = OpTypeVector %uint 2
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
  %float_0_5 = OpConstant %float 0.5
         %27 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %28 = OpTypeSampledImage %27
%_ptr_UniformConstant_28 = OpTypePointer UniformConstant %28
 %ImageInput = OpVariable %_ptr_UniformConstant_28 UniformConstant
      %int_0 = OpConstant %int 0
%_ptr_Function_int = OpTypePointer Function %int
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
    %float_0 = OpConstant %float 0
       %bool = OpTypeBool
%_ptr_Function_bool = OpTypePointer Function %bool
    %v3float = OpTypeVector %float 3
         %52 = OpConstantComposite %v3float %float_0 %float_0 %float_0
     %v3bool = OpTypeVector %bool 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
     %int_n1 = OpConstant %int -1
         %64 = OpConstantComposite %v2int %int_n1 %int_n1
      %int_1 = OpConstant %int 1
         %81 = OpConstantComposite %v2int %int_0 %int_n1
         %97 = OpConstantComposite %v2int %int_1 %int_n1
        %113 = OpConstantComposite %v2int %int_n1 %int_0
        %129 = OpConstantComposite %v2int %int_1 %int_0
        %145 = OpConstantComposite %v2int %int_n1 %int_1
        %161 = OpConstantComposite %v2int %int_0 %int_1
        %177 = OpConstantComposite %v2int %int_1 %int_1
     %uint_2 = OpConstant %uint 2
     %uint_3 = OpConstant %uint 3
     %uint_0 = OpConstant %uint 0
%_ptr_Function_float = OpTypePointer Function %float
     %uint_1 = OpConstant %uint 1
    %float_1 = OpConstant %float 1
        %239 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_239 = OpTypePointer UniformConstant %239
%ImageOutput = OpVariable %_ptr_UniformConstant_239 UniformConstant
     %uint_8 = OpConstant %uint 8
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %index = OpVariable %_ptr_Function_v2int Function
         %uv = OpVariable %_ptr_Function_v2float Function
  %neighbors = OpVariable %_ptr_Function_int Function
       %self = OpVariable %_ptr_Function_v4float Function
   %is_alive = OpVariable %_ptr_Function_bool Function
      %total = OpVariable %_ptr_Function_v3float Function
        %tmp = OpVariable %_ptr_Function_v3float Function
      %tmp_0 = OpVariable %_ptr_Function_v3float Function
      %tmp_1 = OpVariable %_ptr_Function_v3float Function
      %tmp_2 = OpVariable %_ptr_Function_v3float Function
      %tmp_3 = OpVariable %_ptr_Function_v3float Function
      %tmp_4 = OpVariable %_ptr_Function_v3float Function
      %tmp_5 = OpVariable %_ptr_Function_v3float Function
      %tmp_6 = OpVariable %_ptr_Function_v3float Function
%fresh_color = OpVariable %_ptr_Function_v3float Function
         %15 = OpLoad %v3uint %gl_GlobalInvocationID
         %16 = OpVectorShuffle %v2uint %15 %15 0 1
         %17 = OpBitcast %v2int %16
               OpStore %index %17
         %22 = OpLoad %v2int %index
         %23 = OpConvertSToF %v2float %22
         %25 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
         %26 = OpFAdd %v2float %23 %25
         %31 = OpLoad %28 %ImageInput
         %33 = OpImage %27 %31
         %34 = OpImageQuerySizeLod %v2int %33 %int_0
         %35 = OpConvertSToF %v2float %34
         %36 = OpFDiv %v2float %26 %35
               OpStore %uv %36
               OpStore %neighbors %int_0
         %42 = OpLoad %28 %ImageInput
         %43 = OpLoad %v2float %uv
         %45 = OpImageSampleExplicitLod %v4float %42 %43 Lod %float_0
               OpStore %self %45
         %50 = OpLoad %v4float %self
         %51 = OpVectorShuffle %v3float %50 %50 0 1 2
         %54 = OpFUnordNotEqual %v3bool %51 %52
         %55 = OpAny %bool %54
               OpStore %is_alive %55
         %58 = OpLoad %v4float %self
         %59 = OpVectorShuffle %v3float %58 %58 0 1 2
               OpStore %total %59
         %61 = OpLoad %28 %ImageInput
         %62 = OpLoad %v2float %uv
         %65 = OpImageSampleExplicitLod %v4float %61 %62 Lod|ConstOffset %float_0 %64
         %66 = OpVectorShuffle %v3float %65 %65 0 1 2
               OpStore %tmp %66
         %67 = OpLoad %v3float %tmp
         %68 = OpFUnordNotEqual %v3bool %67 %52
         %69 = OpAny %bool %68
               OpSelectionMerge %71 None
               OpBranchConditional %69 %70 %71
         %70 = OpLabel
         %72 = OpLoad %int %neighbors
         %74 = OpIAdd %int %72 %int_1
               OpStore %neighbors %74
         %75 = OpLoad %v3float %tmp
         %76 = OpLoad %v3float %total
         %77 = OpFAdd %v3float %76 %75
               OpStore %total %77
               OpBranch %71
         %71 = OpLabel
         %79 = OpLoad %28 %ImageInput
         %80 = OpLoad %v2float %uv
         %82 = OpImageSampleExplicitLod %v4float %79 %80 Lod|ConstOffset %float_0 %81
         %83 = OpVectorShuffle %v3float %82 %82 0 1 2
               OpStore %tmp_0 %83
         %84 = OpLoad %v3float %tmp_0
         %85 = OpFUnordNotEqual %v3bool %84 %52
         %86 = OpAny %bool %85
               OpSelectionMerge %88 None
               OpBranchConditional %86 %87 %88
         %87 = OpLabel
         %89 = OpLoad %int %neighbors
         %90 = OpIAdd %int %89 %int_1
               OpStore %neighbors %90
         %91 = OpLoad %v3float %tmp_0
         %92 = OpLoad %v3float %total
         %93 = OpFAdd %v3float %92 %91
               OpStore %total %93
               OpBranch %88
         %88 = OpLabel
         %95 = OpLoad %28 %ImageInput
         %96 = OpLoad %v2float %uv
         %98 = OpImageSampleExplicitLod %v4float %95 %96 Lod|ConstOffset %float_0 %97
         %99 = OpVectorShuffle %v3float %98 %98 0 1 2
               OpStore %tmp_1 %99
        %100 = OpLoad %v3float %tmp_1
        %101 = OpFUnordNotEqual %v3bool %100 %52
        %102 = OpAny %bool %101
               OpSelectionMerge %104 None
               OpBranchConditional %102 %103 %104
        %103 = OpLabel
        %105 = OpLoad %int %neighbors
        %106 = OpIAdd %int %105 %int_1
               OpStore %neighbors %106
        %107 = OpLoad %v3float %tmp_1
        %108 = OpLoad %v3float %total
        %109 = OpFAdd %v3float %108 %107
               OpStore %total %109
               OpBranch %104
        %104 = OpLabel
        %111 = OpLoad %28 %ImageInput
        %112 = OpLoad %v2float %uv
        %114 = OpImageSampleExplicitLod %v4float %111 %112 Lod|ConstOffset %float_0 %113
        %115 = OpVectorShuffle %v3float %114 %114 0 1 2
               OpStore %tmp_2 %115
        %116 = OpLoad %v3float %tmp_2
        %117 = OpFUnordNotEqual %v3bool %116 %52
        %118 = OpAny %bool %117
               OpSelectionMerge %120 None
               OpBranchConditional %118 %119 %120
        %119 = OpLabel
        %121 = OpLoad %int %neighbors
        %122 = OpIAdd %int %121 %int_1
               OpStore %neighbors %122
        %123 = OpLoad %v3float %tmp_2
        %124 = OpLoad %v3float %total
        %125 = OpFAdd %v3float %124 %123
               OpStore %total %125
               OpBranch %120
        %120 = OpLabel
        %127 = OpLoad %28 %ImageInput
        %128 = OpLoad %v2float %uv
        %130 = OpImageSampleExplicitLod %v4float %127 %128 Lod|ConstOffset %float_0 %129
        %131 = OpVectorShuffle %v3float %130 %130 0 1 2
               OpStore %tmp_3 %131
        %132 = OpLoad %v3float %tmp_3
        %133 = OpFUnordNotEqual %v3bool %132 %52
        %134 = OpAny %bool %133
               OpSelectionMerge %136 None
               OpBranchConditional %134 %135 %136
        %135 = OpLabel
        %137 = OpLoad %int %neighbors
        %138 = OpIAdd %int %137 %int_1
               OpStore %neighbors %138
        %139 = OpLoad %v3float %tmp_3
        %140 = OpLoad %v3float %total
        %141 = OpFAdd %v3float %140 %139
               OpStore %total %141
               OpBranch %136
        %136 = OpLabel
        %143 = OpLoad %28 %ImageInput
        %144 = OpLoad %v2float %uv
        %146 = OpImageSampleExplicitLod %v4float %143 %144 Lod|ConstOffset %float_0 %145
        %147 = OpVectorShuffle %v3float %146 %146 0 1 2
               OpStore %tmp_4 %147
        %148 = OpLoad %v3float %tmp_4
        %149 = OpFUnordNotEqual %v3bool %148 %52
        %150 = OpAny %bool %149
               OpSelectionMerge %152 None
               OpBranchConditional %150 %151 %152
        %151 = OpLabel
        %153 = OpLoad %int %neighbors
        %154 = OpIAdd %int %153 %int_1
               OpStore %neighbors %154
        %155 = OpLoad %v3float %tmp_4
        %156 = OpLoad %v3float %total
        %157 = OpFAdd %v3float %156 %155
               OpStore %total %157
               OpBranch %152
        %152 = OpLabel
        %159 = OpLoad %28 %ImageInput
        %160 = OpLoad %v2float %uv
        %162 = OpImageSampleExplicitLod %v4float %159 %160 Lod|ConstOffset %float_0 %161
        %163 = OpVectorShuffle %v3float %162 %162 0 1 2
               OpStore %tmp_5 %163
        %164 = OpLoad %v3float %tmp_5
        %165 = OpFUnordNotEqual %v3bool %164 %52
        %166 = OpAny %bool %165
               OpSelectionMerge %168 None
               OpBranchConditional %166 %167 %168
        %167 = OpLabel
        %169 = OpLoad %int %neighbors
        %170 = OpIAdd %int %169 %int_1
               OpStore %neighbors %170
        %171 = OpLoad %v3float %tmp_5
        %172 = OpLoad %v3float %total
        %173 = OpFAdd %v3float %172 %171
               OpStore %total %173
               OpBranch %168
        %168 = OpLabel
        %175 = OpLoad %28 %ImageInput
        %176 = OpLoad %v2float %uv
        %178 = OpImageSampleExplicitLod %v4float %175 %176 Lod|ConstOffset %float_0 %177
        %179 = OpVectorShuffle %v3float %178 %178 0 1 2
               OpStore %tmp_6 %179
        %180 = OpLoad %v3float %tmp_6
        %181 = OpFUnordNotEqual %v3bool %180 %52
        %182 = OpAny %bool %181
               OpSelectionMerge %184 None
               OpBranchConditional %182 %183 %184
        %183 = OpLabel
        %185 = OpLoad %int %neighbors
        %186 = OpIAdd %int %185 %int_1
               OpStore %neighbors %186
        %187 = OpLoad %v3float %tmp_6
        %188 = OpLoad %v3float %total
        %189 = OpFAdd %v3float %188 %187
               OpStore %total %189
               OpBranch %184
        %184 = OpLabel
        %190 = OpLoad %bool %is_alive
               OpSelectionMerge %192 None
               OpBranchConditional %190 %191 %214
        %191 = OpLabel
        %193 = OpLoad %int %neighbors
        %194 = OpBitcast %uint %193
        %196 = OpIEqual %bool %194 %uint_2
        %197 = OpLogicalNot %bool %196
               OpSelectionMerge %199 None
               OpBranchConditional %197 %198 %199
        %198 = OpLabel
        %200 = OpLoad %int %neighbors
        %201 = OpBitcast %uint %200
        %203 = OpIEqual %bool %201 %uint_3
               OpBranch %199
        %199 = OpLabel
        %204 = OpPhi %bool %196 %191 %203 %198
               OpStore %is_alive %204
        %205 = OpLoad %bool %is_alive
               OpSelectionMerge %207 None
               OpBranchConditional %205 %206 %213
        %206 = OpLabel
        %208 = OpLoad %int %neighbors
        %209 = OpConvertSToF %float %208
        %210 = OpLoad %v3float %total
        %211 = OpCompositeConstruct %v3float %209 %209 %209
        %212 = OpFDiv %v3float %210 %211
               OpStore %total %212
               OpBranch %207
        %213 = OpLabel
               OpStore %total %52
               OpBranch %207
        %207 = OpLabel
               OpBranch %192
        %214 = OpLabel
        %215 = OpLoad %int %neighbors
        %216 = OpBitcast %uint %215
        %217 = OpIEqual %bool %216 %uint_3
               OpStore %is_alive %217
        %218 = OpLoad %bool %is_alive
               OpSelectionMerge %220 None
               OpBranchConditional %218 %219 %238
        %219 = OpLabel
        %224 = OpAccessChain %_ptr_Function_float %uv %uint_0
        %225 = OpLoad %float %224
        %227 = OpAccessChain %_ptr_Function_float %uv %uint_1
        %228 = OpLoad %float %227
        %230 = OpAccessChain %_ptr_Function_float %uv %uint_0
        %231 = OpLoad %float %230
        %232 = OpFSub %float %float_1 %231
        %233 = OpAccessChain %_ptr_Function_float %uv %uint_1
        %234 = OpLoad %float %233
        %235 = OpFSub %float %232 %234
        %236 = OpCompositeConstruct %v3float %225 %228 %235
               OpStore %fresh_color %236
        %237 = OpLoad %v3float %fresh_color
               OpStore %total %237
               OpBranch %220
        %238 = OpLabel
               OpStore %total %52
               OpBranch %220
        %220 = OpLabel
               OpBranch %192
        %192 = OpLabel
        %242 = OpLoad %239 %ImageOutput
        %243 = OpLoad %v2int %index
        %244 = OpLoad %v3float %total
        %245 = OpCompositeExtract %float %244 0
        %246 = OpCompositeExtract %float %244 1
        %247 = OpCompositeExtract %float %244 2
        %248 = OpCompositeConstruct %v4float %245 %246 %247 %float_0
               OpImageWrite %242 %243 %248
               OpReturn
               OpFunctionEnd
