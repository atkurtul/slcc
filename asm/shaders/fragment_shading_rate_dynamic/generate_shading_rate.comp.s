; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 255
; Schema: 0
               OpCapability Shader
               OpCapability StorageImageExtendedFormats
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %x0 "x0"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %y0 "y0"
               OpName %frame_width "frame_width"
               OpName %FrequencyInformation "FrequencyInformation"
               OpMemberName %FrequencyInformation 0 "settings"
               OpMemberName %FrequencyInformation 1 "frequency_information"
               OpMemberName %FrequencyInformation 2 "rates"
               OpName %params "params"
               OpName %frame_height "frame_height"
               OpName %output_width "output_width"
               OpName %output_height "output_height"
               OpName %delta_x "delta_x"
               OpName %delta_y "delta_y"
               OpName %max_freqs "max_freqs"
               OpName %i "i"
               OpName %j "j"
               OpName %coord "coord"
               OpName %freq "freq"
               OpName %input_frequency "input_frequency"
               OpName %freqs "freqs"
               OpName %max_rate "max_rate"
               OpName %optimal_rate "optimal_rate"
               OpName %n_rates "n_rates"
               OpName %optimal_rate_index "optimal_rate_index"
               OpName %current_cost "current_cost"
               OpName %i_0 "i"
               OpName %rate "rate"
               OpName %cost "cost"
               OpName %optimal_rate_x "optimal_rate_x"
               OpName %optimal_rate_y "optimal_rate_y"
               OpName %rate_code "rate_code"
               OpName %output_sampling_rate "output_sampling_rate"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpDecorate %_runtimearr_v2uint ArrayStride 8
               OpMemberDecorate %FrequencyInformation 0 Offset 0
               OpMemberDecorate %FrequencyInformation 1 Offset 16
               OpMemberDecorate %FrequencyInformation 2 Offset 32
               OpDecorate %FrequencyInformation BufferBlock
               OpDecorate %params DescriptorSet 0
               OpDecorate %params Binding 2
               OpDecorate %input_frequency DescriptorSet 0
               OpDecorate %input_frequency Binding 0
               OpDecorate %input_frequency NonWritable
               OpDecorate %output_sampling_rate DescriptorSet 0
               OpDecorate %output_sampling_rate Binding 1
               OpDecorate %output_sampling_rate NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %uint = OpTypeInt 32 0
%_ptr_Function_uint = OpTypePointer Function %uint
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %uint_0 = OpConstant %uint 0
%_ptr_Input_uint = OpTypePointer Input %uint
     %uint_1 = OpConstant %uint 1
     %v4uint = OpTypeVector %uint 4
     %v2uint = OpTypeVector %uint 2
%_runtimearr_v2uint = OpTypeRuntimeArray %v2uint
%FrequencyInformation = OpTypeStruct %v4uint %v4uint %_runtimearr_v2uint
%_ptr_Uniform_FrequencyInformation = OpTypePointer Uniform %FrequencyInformation
     %params = OpVariable %_ptr_Uniform_FrequencyInformation Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Uniform_uint = OpTypePointer Uniform %uint
     %uint_2 = OpConstant %uint 2
     %uint_3 = OpConstant %uint 3
      %float = OpTypeFloat 32
       %bool = OpTypeBool
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
    %float_0 = OpConstant %float 0
         %77 = OpConstantComposite %v2float %float_0 %float_0
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
        %113 = OpTypeImage %uint 2D 0 0 0 2 Rg8ui
%_ptr_UniformConstant_113 = OpTypePointer UniformConstant %113
%input_frequency = OpVariable %_ptr_UniformConstant_113 UniformConstant
    %v4float = OpTypeVector %float 4
  %float_255 = OpConstant %float 255
      %int_1 = OpConstant %int 1
 %float_1_25 = OpConstant %float 1.25
    %float_1 = OpConstant %float 1
        %141 = OpConstantComposite %v2float %float_1 %float_1
%_ptr_Function_float = OpTypePointer Function %float
    %float_2 = OpConstant %float 2
%_ptr_Function_v2uint = OpTypePointer Function %v2uint
      %int_2 = OpConstant %int 2
%_ptr_Uniform_v2uint = OpTypePointer Uniform %v2uint
    %uint_12 = OpConstant %uint 12
        %242 = OpTypeImage %uint 2D 0 0 0 2 R8ui
%_ptr_UniformConstant_242 = OpTypePointer UniformConstant %242
%output_sampling_rate = OpVariable %_ptr_UniformConstant_242 UniformConstant
     %uint_8 = OpConstant %uint 8
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %x0 = OpVariable %_ptr_Function_uint Function
         %y0 = OpVariable %_ptr_Function_uint Function
%frame_width = OpVariable %_ptr_Function_uint Function
%frame_height = OpVariable %_ptr_Function_uint Function
%output_width = OpVariable %_ptr_Function_uint Function
%output_height = OpVariable %_ptr_Function_uint Function
    %delta_x = OpVariable %_ptr_Function_uint Function
    %delta_y = OpVariable %_ptr_Function_uint Function
  %max_freqs = OpVariable %_ptr_Function_v2float Function
          %i = OpVariable %_ptr_Function_uint Function
          %j = OpVariable %_ptr_Function_uint Function
      %coord = OpVariable %_ptr_Function_v2int Function
       %freq = OpVariable %_ptr_Function_v2float Function
      %freqs = OpVariable %_ptr_Function_v2float Function
   %max_rate = OpVariable %_ptr_Function_float Function
%optimal_rate = OpVariable %_ptr_Function_v2float Function
    %n_rates = OpVariable %_ptr_Function_uint Function
%optimal_rate_index = OpVariable %_ptr_Function_uint Function
%current_cost = OpVariable %_ptr_Function_float Function
        %i_0 = OpVariable %_ptr_Function_uint Function
       %rate = OpVariable %_ptr_Function_v2uint Function
       %cost = OpVariable %_ptr_Function_float Function
%optimal_rate_x = OpVariable %_ptr_Function_uint Function
%optimal_rate_y = OpVariable %_ptr_Function_uint Function
  %rate_code = OpVariable %_ptr_Function_uint Function
         %14 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_0
         %15 = OpLoad %uint %14
               OpStore %x0 %15
         %18 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_1
         %19 = OpLoad %uint %18
               OpStore %y0 %19
         %30 = OpAccessChain %_ptr_Uniform_uint %params %int_0 %uint_0
         %31 = OpLoad %uint %30
               OpStore %frame_width %31
         %33 = OpAccessChain %_ptr_Uniform_uint %params %int_0 %uint_1
         %34 = OpLoad %uint %33
               OpStore %frame_height %34
         %37 = OpAccessChain %_ptr_Uniform_uint %params %int_0 %uint_2
         %38 = OpLoad %uint %37
               OpStore %output_width %38
         %41 = OpAccessChain %_ptr_Uniform_uint %params %int_0 %uint_3
         %42 = OpLoad %uint %41
               OpStore %output_height %42
         %44 = OpLoad %uint %frame_width
         %46 = OpConvertUToF %float %44
         %47 = OpLoad %uint %output_width
         %48 = OpConvertUToF %float %47
         %49 = OpFDiv %float %46 %48
         %50 = OpExtInst %float %1 Round %49
         %51 = OpConvertFToU %uint %50
         %52 = OpExtInst %uint %1 UMax %uint_1 %51
               OpStore %delta_x %52
         %54 = OpLoad %uint %frame_height
         %55 = OpConvertUToF %float %54
         %56 = OpLoad %uint %output_height
         %57 = OpConvertUToF %float %56
         %58 = OpFDiv %float %55 %57
         %59 = OpExtInst %float %1 Round %58
         %60 = OpConvertFToU %uint %59
         %61 = OpExtInst %uint %1 UMax %uint_1 %60
               OpStore %delta_y %61
         %62 = OpLoad %uint %x0
         %63 = OpLoad %uint %output_width
         %65 = OpUGreaterThanEqual %bool %62 %63
         %66 = OpLoad %uint %y0
         %67 = OpLoad %uint %output_height
         %68 = OpUGreaterThanEqual %bool %66 %67
         %69 = OpLogicalOr %bool %65 %68
               OpSelectionMerge %71 None
               OpBranchConditional %69 %70 %71
         %70 = OpLabel
               OpReturn
         %71 = OpLabel
               OpStore %max_freqs %77
               OpStore %i %uint_0
               OpBranch %79
         %79 = OpLabel
               OpLoopMerge %81 %82 None
               OpBranch %83
         %83 = OpLabel
         %84 = OpLoad %uint %i
         %85 = OpLoad %uint %delta_x
         %86 = OpULessThan %bool %84 %85
               OpBranchConditional %86 %80 %81
         %80 = OpLabel
               OpStore %j %uint_0
               OpBranch %88
         %88 = OpLabel
               OpLoopMerge %90 %91 None
               OpBranch %92
         %92 = OpLabel
         %93 = OpLoad %uint %j
         %94 = OpLoad %uint %delta_y
         %95 = OpULessThan %bool %93 %94
               OpBranchConditional %95 %89 %90
         %89 = OpLabel
         %99 = OpLoad %uint %delta_x
        %100 = OpLoad %uint %x0
        %101 = OpIMul %uint %99 %100
        %102 = OpLoad %uint %i
        %103 = OpIAdd %uint %101 %102
        %104 = OpBitcast %int %103
        %105 = OpLoad %uint %delta_y
        %106 = OpLoad %uint %y0
        %107 = OpIMul %uint %105 %106
        %108 = OpLoad %uint %j
        %109 = OpIAdd %uint %107 %108
        %110 = OpBitcast %int %109
        %111 = OpCompositeConstruct %v2int %104 %110
               OpStore %coord %111
        %116 = OpLoad %113 %input_frequency
        %117 = OpLoad %v2int %coord
        %118 = OpImageRead %v4uint %116 %117
        %120 = OpConvertUToF %v4float %118
        %121 = OpCompositeExtract %float %120 0
        %122 = OpCompositeExtract %float %120 1
        %123 = OpCompositeConstruct %v2float %121 %122
        %125 = OpCompositeConstruct %v2float %float_255 %float_255
        %126 = OpFDiv %v2float %123 %125
               OpStore %freq %126
        %127 = OpLoad %v2float %max_freqs
        %128 = OpLoad %v2float %freq
        %129 = OpExtInst %v2float %1 FMax %127 %128
               OpStore %max_freqs %129
               OpBranch %91
         %91 = OpLabel
        %130 = OpLoad %uint %j
        %132 = OpIAdd %uint %130 %int_1
               OpStore %j %132
               OpBranch %88
         %90 = OpLabel
               OpBranch %82
         %82 = OpLabel
        %133 = OpLoad %uint %i
        %134 = OpIAdd %uint %133 %int_1
               OpStore %i %134
               OpBranch %79
         %81 = OpLabel
        %137 = OpLoad %v2float %max_freqs
        %138 = OpExtInst %v2float %1 Sqrt %137
        %139 = OpVectorTimesScalar %v2float %138 %float_1_25
        %142 = OpExtInst %v2float %1 FMin %139 %141
               OpStore %freqs %142
        %145 = OpAccessChain %_ptr_Uniform_uint %params %int_1 %uint_0
        %146 = OpLoad %uint %145
        %147 = OpAccessChain %_ptr_Uniform_uint %params %int_1 %uint_1
        %148 = OpLoad %uint %147
        %149 = OpExtInst %uint %1 UMax %146 %148
        %150 = OpConvertUToF %float %149
               OpStore %max_rate %150
        %152 = OpLoad %v2float %freqs
        %153 = OpFMul %v2float %152 %141
        %154 = OpLoad %v2float %freqs
        %155 = OpCompositeConstruct %v2float %float_1 %float_1
        %156 = OpFSub %v2float %155 %154
        %157 = OpLoad %float %max_rate
        %158 = OpLoad %float %max_rate
        %159 = OpCompositeConstruct %v2float %157 %158
        %160 = OpFMul %v2float %156 %159
        %161 = OpFAdd %v2float %153 %160
               OpStore %optimal_rate %161
        %163 = OpAccessChain %_ptr_Uniform_uint %params %int_1 %uint_2
        %164 = OpLoad %uint %163
               OpStore %n_rates %164
               OpStore %optimal_rate_index %uint_0
        %168 = OpLoad %float %max_rate
        %169 = OpFMul %float %float_2 %168
        %170 = OpLoad %float %max_rate
        %171 = OpFMul %float %169 %170
        %172 = OpFAdd %float %float_1 %171
               OpStore %current_cost %172
               OpStore %i_0 %uint_0
               OpBranch %174
        %174 = OpLabel
               OpLoopMerge %176 %177 None
               OpBranch %178
        %178 = OpLabel
        %179 = OpLoad %uint %i_0
        %180 = OpLoad %uint %n_rates
        %181 = OpULessThan %bool %179 %180
               OpBranchConditional %181 %175 %176
        %175 = OpLabel
        %185 = OpLoad %uint %i_0
        %187 = OpAccessChain %_ptr_Uniform_v2uint %params %int_2 %185
        %188 = OpLoad %v2uint %187
               OpStore %rate %188
        %190 = OpAccessChain %_ptr_Function_uint %rate %uint_0
        %191 = OpLoad %uint %190
        %192 = OpConvertUToF %float %191
        %193 = OpAccessChain %_ptr_Function_float %optimal_rate %uint_0
        %194 = OpLoad %float %193
        %195 = OpFSub %float %192 %194
        %196 = OpAccessChain %_ptr_Function_uint %rate %uint_0
        %197 = OpLoad %uint %196
        %198 = OpConvertUToF %float %197
        %199 = OpAccessChain %_ptr_Function_float %optimal_rate %uint_0
        %200 = OpLoad %float %199
        %201 = OpFSub %float %198 %200
        %202 = OpFMul %float %195 %201
        %203 = OpAccessChain %_ptr_Function_uint %rate %uint_1
        %204 = OpLoad %uint %203
        %205 = OpConvertUToF %float %204
        %206 = OpAccessChain %_ptr_Function_float %optimal_rate %uint_1
        %207 = OpLoad %float %206
        %208 = OpFSub %float %205 %207
        %209 = OpAccessChain %_ptr_Function_uint %rate %uint_1
        %210 = OpLoad %uint %209
        %211 = OpConvertUToF %float %210
        %212 = OpAccessChain %_ptr_Function_float %optimal_rate %uint_1
        %213 = OpLoad %float %212
        %214 = OpFSub %float %211 %213
        %215 = OpFMul %float %208 %214
        %216 = OpFAdd %float %202 %215
               OpStore %cost %216
        %217 = OpLoad %float %cost
        %218 = OpLoad %float %current_cost
        %219 = OpFOrdLessThan %bool %217 %218
               OpSelectionMerge %221 None
               OpBranchConditional %219 %220 %221
        %220 = OpLabel
        %222 = OpLoad %float %cost
               OpStore %current_cost %222
        %223 = OpLoad %uint %i_0
               OpStore %optimal_rate_index %223
               OpBranch %221
        %221 = OpLabel
               OpBranch %177
        %177 = OpLabel
        %224 = OpLoad %uint %i_0
        %225 = OpIAdd %uint %224 %int_1
               OpStore %i_0 %225
               OpBranch %174
        %176 = OpLabel
        %227 = OpLoad %uint %optimal_rate_index
        %228 = OpAccessChain %_ptr_Uniform_uint %params %int_2 %227 %uint_0
        %229 = OpLoad %uint %228
               OpStore %optimal_rate_x %229
        %231 = OpLoad %uint %optimal_rate_index
        %232 = OpAccessChain %_ptr_Uniform_uint %params %int_2 %231 %uint_1
        %233 = OpLoad %uint %232
               OpStore %optimal_rate_y %233
        %235 = OpLoad %uint %optimal_rate_y
        %236 = OpShiftRightLogical %uint %235 %int_1
        %237 = OpLoad %uint %optimal_rate_x
        %238 = OpShiftLeftLogical %uint %237 %int_1
        %240 = OpBitwiseAnd %uint %238 %uint_12
        %241 = OpBitwiseOr %uint %236 %240
               OpStore %rate_code %241
        %245 = OpLoad %242 %output_sampling_rate
        %246 = OpLoad %uint %x0
        %247 = OpBitcast %int %246
        %248 = OpLoad %uint %y0
        %249 = OpBitcast %int %248
        %250 = OpCompositeConstruct %v2int %247 %249
        %251 = OpLoad %uint %rate_code
        %252 = OpCompositeConstruct %v4uint %251 %251 %251 %251
               OpImageWrite %245 %250 %252
               OpReturn
               OpFunctionEnd
