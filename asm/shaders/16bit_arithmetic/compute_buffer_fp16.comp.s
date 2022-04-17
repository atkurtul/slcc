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
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_16bit_storage"
               OpSourceExtension "GL_EXT_shader_explicit_arithmetic_types_float16"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %compute_blob_vf162_vf164_f161_ "compute_blob(vf162;vf164;f161;"
               OpName %pos "pos"
               OpName %blob "blob"
               OpName %seed "seed"
               OpName %offset "offset"
               OpName %rg_offset "rg_offset"
               OpName %bs_offset "bs_offset"
               OpName %rg_dot "rg_dot"
               OpName %bs_dot "bs_dot"
               OpName %dots "dots"
               OpName %parabolas "parabolas"
               OpName %num_blobs "num_blobs"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "num_blobs"
               OpMemberName %Registers 1 "seed"
               OpMemberName %Registers 2 "range"
               OpName %registers "registers"
               OpName %x "x"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %WIDTH "WIDTH"
               OpName %y "y"
               OpName %HEIGHT "HEIGHT"
               OpName %pos_0 "pos"
               OpName %result "result"
               OpName %seed_0 "seed"
               OpName %range "range"
               OpName %stride "stride"
               OpName %i "i"
               OpName %blob_0 "blob"
               OpName %SSBO "SSBO"
               OpMemberName %SSBO 0 "blob_data"
               OpName %_ ""
               OpName %y_0 "y"
               OpName %x_0 "x"
               OpName %param "param"
               OpName %param_0 "param"
               OpName %param_1 "param"
               OpName %o_results "o_results"
               OpMemberDecorate %Registers 0 Offset 0
               OpMemberDecorate %Registers 1 Offset 4
               OpMemberDecorate %Registers 2 Offset 8
               OpDecorate %Registers Block
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpDecorate %WIDTH SpecId 0
               OpDecorate %HEIGHT SpecId 1
               OpDecorate %_runtimearr_v4half ArrayStride 8
               OpMemberDecorate %SSBO 0 NonWritable
               OpMemberDecorate %SSBO 0 Offset 0
               OpDecorate %SSBO BufferBlock
               OpDecorate %_ DescriptorSet 0
               OpDecorate %_ Binding 0
               OpDecorate %o_results RelaxedPrecision
               OpDecorate %o_results DescriptorSet 0
               OpDecorate %o_results Binding 1
               OpDecorate %o_results NonReadable
               OpDecorate %252 RelaxedPrecision
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %half = OpTypeFloat 16
     %v2half = OpTypeVector %half 2
%_ptr_Function_v2half = OpTypePointer Function %v2half
     %v4half = OpTypeVector %half 4
%_ptr_Function_v4half = OpTypePointer Function %v4half
%_ptr_Function_half = OpTypePointer Function %half
         %12 = OpTypeFunction %v4half %_ptr_Function_v2half %_ptr_Function_v4half %_ptr_Function_half
%half_0x1_e64pn1 = OpConstant %half 0x1.e64p-1
%half_0x1p_0 = OpConstant %half 0x1p+0
         %28 = OpConstantComposite %v4half %half_0x1_e64pn1 %half_0x1p_0 %half_0x1_e64pn1 %half_0x1p_0
%half_0x1_0ccp_0 = OpConstant %half 0x1.0ccp+0
%half_0x1_198p_0 = OpConstant %half 0x1.198p+0
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
%half_0x1_cccpn1 = OpConstant %half 0x1.cccp-1
        %102 = OpConstantComposite %v4half %half_0x1p_0 %half_0x1p_0 %half_0x1p_0 %half_0x1_cccpn1
%half_0x0p_0 = OpConstant %half 0x0p+0
        %106 = OpConstantComposite %v4half %half_0x0p_0 %half_0x0p_0 %half_0x0p_0 %half_0x0p_0
%_ptr_Function_uint = OpTypePointer Function %uint
      %float = OpTypeFloat 32
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
  %Registers = OpTypeStruct %uint %float %v2int
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_uint = OpTypePointer PushConstant %uint
%_ptr_Function_float = OpTypePointer Function %float
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %uint_0 = OpConstant %uint 0
%_ptr_Input_uint = OpTypePointer Input %uint
      %WIDTH = OpSpecConstant %uint 1
  %float_0_5 = OpConstant %float 0.5
     %uint_1 = OpConstant %uint 1
     %HEIGHT = OpSpecConstant %uint 1
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_float = OpTypePointer PushConstant %float
%_ptr_Function_v2int = OpTypePointer Function %v2int
      %int_2 = OpConstant %int 2
%_ptr_PushConstant_v2int = OpTypePointer PushConstant %v2int
%half_0x1_33pn2 = OpConstant %half 0x1.33p-2
       %bool = OpTypeBool
%_runtimearr_v4half = OpTypeRuntimeArray %v4half
       %SSBO = OpTypeStruct %_runtimearr_v4half
%_ptr_Uniform_SSBO = OpTypePointer Uniform %SSBO
          %_ = OpVariable %_ptr_Uniform_SSBO Uniform
%_ptr_Uniform_v4half = OpTypePointer Uniform %v4half
%_ptr_Function_int = OpTypePointer Function %int
        %249 = OpTypeImage %float 2D 0 0 0 2 Rgba16f
%_ptr_UniformConstant_249 = OpTypePointer UniformConstant %249
  %o_results = OpVariable %_ptr_UniformConstant_249 UniformConstant
     %v2uint = OpTypeVector %uint 2
    %v4float = OpTypeVector %float 4
     %uint_8 = OpConstant %uint 8
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
  %num_blobs = OpVariable %_ptr_Function_uint Function
          %x = OpVariable %_ptr_Function_float Function
          %y = OpVariable %_ptr_Function_float Function
      %pos_0 = OpVariable %_ptr_Function_v2half Function
     %result = OpVariable %_ptr_Function_v4half Function
     %seed_0 = OpVariable %_ptr_Function_half Function
      %range = OpVariable %_ptr_Function_v2int Function
     %stride = OpVariable %_ptr_Function_half Function
          %i = OpVariable %_ptr_Function_uint Function
     %blob_0 = OpVariable %_ptr_Function_v4half Function
        %y_0 = OpVariable %_ptr_Function_int Function
        %x_0 = OpVariable %_ptr_Function_int Function
      %param = OpVariable %_ptr_Function_v2half Function
    %param_0 = OpVariable %_ptr_Function_v4half Function
    %param_1 = OpVariable %_ptr_Function_half Function
        %128 = OpAccessChain %_ptr_PushConstant_uint %registers %int_0
        %129 = OpLoad %uint %128
               OpStore %num_blobs %129
        %137 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_0
        %138 = OpLoad %uint %137
        %139 = OpConvertUToF %float %138
        %141 = OpConvertUToF %float %WIDTH
        %142 = OpFDiv %float %139 %141
        %144 = OpFSub %float %142 %float_0_5
               OpStore %x %144
        %147 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_1
        %148 = OpLoad %uint %147
        %149 = OpConvertUToF %float %148
        %151 = OpConvertUToF %float %HEIGHT
        %152 = OpFDiv %float %149 %151
        %153 = OpFSub %float %152 %float_0_5
               OpStore %y %153
        %155 = OpLoad %float %x
        %156 = OpFConvert %half %155
        %157 = OpLoad %float %y
        %158 = OpFConvert %half %157
        %159 = OpCompositeConstruct %v2half %156 %158
               OpStore %pos_0 %159
               OpStore %result %106
        %164 = OpAccessChain %_ptr_PushConstant_float %registers %int_1
        %165 = OpLoad %float %164
        %166 = OpFConvert %half %165
               OpStore %seed_0 %166
        %171 = OpAccessChain %_ptr_PushConstant_v2int %registers %int_2
        %172 = OpLoad %v2int %171
        %173 = OpCompositeExtract %int %172 0
        %174 = OpCompositeExtract %int %172 1
        %175 = OpCompositeConstruct %v2int %173 %174
               OpStore %range %175
        %177 = OpLoad %half %seed_0
        %179 = OpFMul %half %177 %half_0x1_33pn2
               OpStore %stride %179
               OpStore %i %uint_0
               OpBranch %181
        %181 = OpLabel
               OpLoopMerge %183 %184 None
               OpBranch %185
        %185 = OpLabel
        %186 = OpLoad %uint %i
        %187 = OpLoad %uint %num_blobs
        %189 = OpULessThan %bool %186 %187
               OpBranchConditional %189 %182 %183
        %182 = OpLabel
        %195 = OpLoad %uint %i
        %197 = OpAccessChain %_ptr_Uniform_v4half %_ %int_0 %195
        %198 = OpLoad %v4half %197
               OpStore %blob_0 %198
        %201 = OpAccessChain %_ptr_Function_int %range %uint_1
        %202 = OpLoad %int %201
        %203 = OpSNegate %int %202
               OpStore %y_0 %203
               OpBranch %204
        %204 = OpLabel
               OpLoopMerge %206 %207 None
               OpBranch %208
        %208 = OpLabel
        %209 = OpLoad %int %y_0
        %210 = OpAccessChain %_ptr_Function_int %range %uint_1
        %211 = OpLoad %int %210
        %212 = OpSLessThanEqual %bool %209 %211
               OpBranchConditional %212 %205 %206
        %205 = OpLabel
        %214 = OpAccessChain %_ptr_Function_int %range %uint_0
        %215 = OpLoad %int %214
        %216 = OpSNegate %int %215
               OpStore %x_0 %216
               OpBranch %217
        %217 = OpLabel
               OpLoopMerge %219 %220 None
               OpBranch %221
        %221 = OpLabel
        %222 = OpLoad %int %x_0
        %223 = OpAccessChain %_ptr_Function_int %range %uint_0
        %224 = OpLoad %int %223
        %225 = OpSLessThanEqual %bool %222 %224
               OpBranchConditional %225 %218 %219
        %218 = OpLabel
        %226 = OpLoad %v2half %pos_0
        %227 = OpLoad %half %stride
        %228 = OpLoad %int %x_0
        %229 = OpConvertSToF %half %228
        %230 = OpLoad %int %y_0
        %231 = OpConvertSToF %half %230
        %232 = OpCompositeConstruct %v2half %229 %231
        %233 = OpVectorTimesScalar %v2half %232 %227
        %234 = OpFAdd %v2half %226 %233
               OpStore %param %234
        %237 = OpLoad %v4half %blob_0
               OpStore %param_0 %237
        %239 = OpLoad %half %seed_0
               OpStore %param_1 %239
        %240 = OpFunctionCall %v4half %compute_blob_vf162_vf164_f161_ %param %param_0 %param_1
        %241 = OpLoad %v4half %result
        %242 = OpFAdd %v4half %241 %240
               OpStore %result %242
               OpBranch %220
        %220 = OpLabel
        %243 = OpLoad %int %x_0
        %244 = OpIAdd %int %243 %int_1
               OpStore %x_0 %244
               OpBranch %217
        %219 = OpLabel
               OpBranch %207
        %207 = OpLabel
        %245 = OpLoad %int %y_0
        %246 = OpIAdd %int %245 %int_1
               OpStore %y_0 %246
               OpBranch %204
        %206 = OpLabel
               OpBranch %184
        %184 = OpLabel
        %247 = OpLoad %uint %i
        %248 = OpIAdd %uint %247 %int_1
               OpStore %i %248
               OpBranch %181
        %183 = OpLabel
        %252 = OpLoad %249 %o_results
        %254 = OpLoad %v3uint %gl_GlobalInvocationID
        %255 = OpVectorShuffle %v2uint %254 %254 0 1
        %256 = OpBitcast %v2int %255
        %257 = OpLoad %v4half %result
        %259 = OpFConvert %v4float %257
               OpImageWrite %252 %256 %259
               OpReturn
               OpFunctionEnd
%compute_blob_vf162_vf164_f161_ = OpFunction %v4half None %12
        %pos = OpFunctionParameter %_ptr_Function_v2half
       %blob = OpFunctionParameter %_ptr_Function_v4half
       %seed = OpFunctionParameter %_ptr_Function_half
         %17 = OpLabel
     %offset = OpVariable %_ptr_Function_v2half Function
  %rg_offset = OpVariable %_ptr_Function_v4half Function
  %bs_offset = OpVariable %_ptr_Function_v4half Function
     %rg_dot = OpVariable %_ptr_Function_v4half Function
     %bs_dot = OpVariable %_ptr_Function_v4half Function
       %dots = OpVariable %_ptr_Function_v4half Function
  %parabolas = OpVariable %_ptr_Function_v4half Function
         %19 = OpLoad %v2half %pos
         %20 = OpLoad %v4half %blob
         %21 = OpVectorShuffle %v2half %20 %20 0 1
         %22 = OpFSub %v2half %19 %21
               OpStore %offset %22
         %24 = OpLoad %v2half %offset
         %25 = OpVectorShuffle %v4half %24 %24 0 0 1 1
         %29 = OpFMul %v4half %25 %28
               OpStore %rg_offset %29
         %31 = OpLoad %v2half %offset
         %32 = OpVectorShuffle %v4half %31 %31 0 0 1 1
         %35 = OpLoad %half %seed
         %36 = OpFAdd %half %half_0x1_198p_0 %35
         %37 = OpLoad %half %seed
         %38 = OpFAdd %half %half_0x1_198p_0 %37
         %39 = OpCompositeConstruct %v4half %half_0x1_0ccp_0 %36 %half_0x1_0ccp_0 %38
         %40 = OpFMul %v4half %32 %39
               OpStore %bs_offset %40
         %42 = OpLoad %v4half %rg_offset
         %43 = OpLoad %v4half %rg_offset
         %44 = OpFMul %v4half %42 %43
               OpStore %rg_dot %44
         %46 = OpLoad %v4half %bs_offset
         %47 = OpLoad %v4half %bs_offset
         %48 = OpFMul %v4half %46 %47
               OpStore %bs_dot %48
         %50 = OpLoad %v4half %rg_dot
         %51 = OpVectorShuffle %v2half %50 %50 0 1
         %52 = OpLoad %v4half %rg_dot
         %53 = OpVectorShuffle %v2half %52 %52 2 3
         %54 = OpFAdd %v2half %51 %53
         %55 = OpLoad %v4half %bs_dot
         %56 = OpVectorShuffle %v2half %55 %55 0 1
         %57 = OpLoad %v4half %bs_dot
         %58 = OpVectorShuffle %v2half %57 %57 2 3
         %59 = OpFAdd %v2half %56 %58
         %60 = OpCompositeExtract %half %54 0
         %61 = OpCompositeExtract %half %54 1
         %62 = OpCompositeExtract %half %59 0
         %63 = OpCompositeExtract %half %59 1
         %64 = OpCompositeConstruct %v4half %60 %61 %62 %63
         %67 = OpAccessChain %_ptr_Function_half %blob %uint_3
         %68 = OpLoad %half %67
         %69 = OpVectorTimesScalar %v4half %64 %68
               OpStore %dots %69
         %70 = OpLoad %v4half %dots
         %71 = OpLoad %v4half %dots
         %72 = OpFMul %v4half %70 %71
         %73 = OpLoad %v4half %dots
         %74 = OpFAdd %v4half %72 %73
               OpStore %dots %74
         %75 = OpLoad %v4half %dots
         %76 = OpLoad %v4half %dots
         %77 = OpFMul %v4half %75 %76
         %78 = OpLoad %v4half %dots
         %79 = OpFAdd %v4half %77 %78
               OpStore %dots %79
         %80 = OpLoad %v4half %dots
         %81 = OpLoad %v4half %dots
         %82 = OpFMul %v4half %80 %81
         %83 = OpLoad %v4half %dots
         %84 = OpFAdd %v4half %82 %83
               OpStore %dots %84
         %85 = OpLoad %v4half %dots
         %86 = OpLoad %v4half %dots
         %87 = OpFMul %v4half %85 %86
         %88 = OpLoad %v4half %dots
         %89 = OpFAdd %v4half %87 %88
               OpStore %dots %89
         %90 = OpLoad %v4half %dots
         %91 = OpLoad %v4half %dots
         %92 = OpFMul %v4half %90 %91
         %93 = OpLoad %v4half %dots
         %94 = OpFAdd %v4half %92 %93
               OpStore %dots %94
         %95 = OpLoad %v4half %dots
         %96 = OpLoad %v4half %dots
         %97 = OpFMul %v4half %95 %96
         %98 = OpLoad %v4half %dots
         %99 = OpFAdd %v4half %97 %98
               OpStore %dots %99
        %103 = OpLoad %v4half %dots
        %104 = OpFSub %v4half %102 %103
        %107 = OpExtInst %v4half %1 FMax %104 %106
               OpStore %parabolas %107
        %108 = OpAccessChain %_ptr_Function_half %parabolas %uint_3
        %109 = OpLoad %half %108
        %110 = OpLoad %v4half %parabolas
        %111 = OpCompositeConstruct %v4half %109 %109 %109 %109
        %112 = OpFSub %v4half %110 %111
               OpStore %parabolas %112
        %113 = OpLoad %v4half %parabolas
        %114 = OpExtInst %v4half %1 FMax %113 %106
               OpStore %parabolas %114
        %115 = OpLoad %v4half %parabolas
               OpReturnValue %115
               OpFunctionEnd
