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
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_shader_16bit_storage"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %compute_blob_vf2_vf4_f1_ "compute_blob(vf2;vf4;f1;"
               OpName %pos "pos"
               OpName %blob "blob"
               OpName %seed "seed"
               OpName %offset "offset"
               OpName %s_offset "s_offset"
               OpName %r_offset "r_offset"
               OpName %g_offset "g_offset"
               OpName %b_offset "b_offset"
               OpName %r_dot "r_dot"
               OpName %g_dot "g_dot"
               OpName %b_dot "b_dot"
               OpName %s_dot "s_dot"
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
               OpDecorate %248 RelaxedPrecision
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
%_ptr_Function_float = OpTypePointer Function %float
         %12 = OpTypeFunction %v4float %_ptr_Function_v2float %_ptr_Function_v4float %_ptr_Function_float
%float_1_10000002 = OpConstant %float 1.10000002
%float_0_949999988 = OpConstant %float 0.949999988
    %float_1 = OpConstant %float 1
%float_1_04999995 = OpConstant %float 1.04999995
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
%float_0_899999976 = OpConstant %float 0.899999976
        %100 = OpConstantComposite %v4float %float_1 %float_1 %float_1 %float_0_899999976
    %float_0 = OpConstant %float 0
        %104 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
%_ptr_Function_uint = OpTypePointer Function %uint
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
  %Registers = OpTypeStruct %uint %float %v2int
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_uint = OpTypePointer PushConstant %uint
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
%float_0_300000012 = OpConstant %float 0.300000012
       %bool = OpTypeBool
       %half = OpTypeFloat 16
     %v4half = OpTypeVector %half 4
%_runtimearr_v4half = OpTypeRuntimeArray %v4half
       %SSBO = OpTypeStruct %_runtimearr_v4half
%_ptr_Uniform_SSBO = OpTypePointer Uniform %SSBO
          %_ = OpVariable %_ptr_Uniform_SSBO Uniform
%_ptr_Uniform_v4half = OpTypePointer Uniform %v4half
%_ptr_Function_int = OpTypePointer Function %int
        %245 = OpTypeImage %float 2D 0 0 0 2 Rgba16f
%_ptr_UniformConstant_245 = OpTypePointer UniformConstant %245
  %o_results = OpVariable %_ptr_UniformConstant_245 UniformConstant
     %v2uint = OpTypeVector %uint 2
     %uint_8 = OpConstant %uint 8
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
  %num_blobs = OpVariable %_ptr_Function_uint Function
          %x = OpVariable %_ptr_Function_float Function
          %y = OpVariable %_ptr_Function_float Function
      %pos_0 = OpVariable %_ptr_Function_v2float Function
     %result = OpVariable %_ptr_Function_v4float Function
     %seed_0 = OpVariable %_ptr_Function_float Function
      %range = OpVariable %_ptr_Function_v2int Function
     %stride = OpVariable %_ptr_Function_float Function
          %i = OpVariable %_ptr_Function_uint Function
     %blob_0 = OpVariable %_ptr_Function_v4float Function
        %y_0 = OpVariable %_ptr_Function_int Function
        %x_0 = OpVariable %_ptr_Function_int Function
      %param = OpVariable %_ptr_Function_v2float Function
    %param_0 = OpVariable %_ptr_Function_v4float Function
    %param_1 = OpVariable %_ptr_Function_float Function
        %125 = OpAccessChain %_ptr_PushConstant_uint %registers %int_0
        %126 = OpLoad %uint %125
               OpStore %num_blobs %126
        %133 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_0
        %134 = OpLoad %uint %133
        %135 = OpConvertUToF %float %134
        %137 = OpConvertUToF %float %WIDTH
        %138 = OpFDiv %float %135 %137
        %140 = OpFSub %float %138 %float_0_5
               OpStore %x %140
        %143 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_1
        %144 = OpLoad %uint %143
        %145 = OpConvertUToF %float %144
        %147 = OpConvertUToF %float %HEIGHT
        %148 = OpFDiv %float %145 %147
        %149 = OpFSub %float %148 %float_0_5
               OpStore %y %149
        %151 = OpLoad %float %x
        %152 = OpLoad %float %y
        %153 = OpCompositeConstruct %v2float %151 %152
               OpStore %pos_0 %153
               OpStore %result %104
        %158 = OpAccessChain %_ptr_PushConstant_float %registers %int_1
        %159 = OpLoad %float %158
               OpStore %seed_0 %159
        %164 = OpAccessChain %_ptr_PushConstant_v2int %registers %int_2
        %165 = OpLoad %v2int %164
        %166 = OpCompositeExtract %int %165 0
        %167 = OpCompositeExtract %int %165 1
        %168 = OpCompositeConstruct %v2int %166 %167
               OpStore %range %168
        %170 = OpLoad %float %seed_0
        %172 = OpFMul %float %170 %float_0_300000012
               OpStore %stride %172
               OpStore %i %uint_0
               OpBranch %174
        %174 = OpLabel
               OpLoopMerge %176 %177 None
               OpBranch %178
        %178 = OpLabel
        %179 = OpLoad %uint %i
        %180 = OpLoad %uint %num_blobs
        %182 = OpULessThan %bool %179 %180
               OpBranchConditional %182 %175 %176
        %175 = OpLabel
        %190 = OpLoad %uint %i
        %192 = OpAccessChain %_ptr_Uniform_v4half %_ %int_0 %190
        %193 = OpLoad %v4half %192
        %194 = OpFConvert %v4float %193
               OpStore %blob_0 %194
        %197 = OpAccessChain %_ptr_Function_int %range %uint_1
        %198 = OpLoad %int %197
        %199 = OpSNegate %int %198
               OpStore %y_0 %199
               OpBranch %200
        %200 = OpLabel
               OpLoopMerge %202 %203 None
               OpBranch %204
        %204 = OpLabel
        %205 = OpLoad %int %y_0
        %206 = OpAccessChain %_ptr_Function_int %range %uint_1
        %207 = OpLoad %int %206
        %208 = OpSLessThanEqual %bool %205 %207
               OpBranchConditional %208 %201 %202
        %201 = OpLabel
        %210 = OpAccessChain %_ptr_Function_int %range %uint_0
        %211 = OpLoad %int %210
        %212 = OpSNegate %int %211
               OpStore %x_0 %212
               OpBranch %213
        %213 = OpLabel
               OpLoopMerge %215 %216 None
               OpBranch %217
        %217 = OpLabel
        %218 = OpLoad %int %x_0
        %219 = OpAccessChain %_ptr_Function_int %range %uint_0
        %220 = OpLoad %int %219
        %221 = OpSLessThanEqual %bool %218 %220
               OpBranchConditional %221 %214 %215
        %214 = OpLabel
        %222 = OpLoad %v2float %pos_0
        %223 = OpLoad %float %stride
        %224 = OpLoad %int %x_0
        %225 = OpConvertSToF %float %224
        %226 = OpLoad %int %y_0
        %227 = OpConvertSToF %float %226
        %228 = OpCompositeConstruct %v2float %225 %227
        %229 = OpVectorTimesScalar %v2float %228 %223
        %230 = OpFAdd %v2float %222 %229
               OpStore %param %230
        %233 = OpLoad %v4float %blob_0
               OpStore %param_0 %233
        %235 = OpLoad %float %seed_0
               OpStore %param_1 %235
        %236 = OpFunctionCall %v4float %compute_blob_vf2_vf4_f1_ %param %param_0 %param_1
        %237 = OpLoad %v4float %result
        %238 = OpFAdd %v4float %237 %236
               OpStore %result %238
               OpBranch %216
        %216 = OpLabel
        %239 = OpLoad %int %x_0
        %240 = OpIAdd %int %239 %int_1
               OpStore %x_0 %240
               OpBranch %213
        %215 = OpLabel
               OpBranch %203
        %203 = OpLabel
        %241 = OpLoad %int %y_0
        %242 = OpIAdd %int %241 %int_1
               OpStore %y_0 %242
               OpBranch %200
        %202 = OpLabel
               OpBranch %177
        %177 = OpLabel
        %243 = OpLoad %uint %i
        %244 = OpIAdd %uint %243 %int_1
               OpStore %i %244
               OpBranch %174
        %176 = OpLabel
        %248 = OpLoad %245 %o_results
        %250 = OpLoad %v3uint %gl_GlobalInvocationID
        %251 = OpVectorShuffle %v2uint %250 %250 0 1
        %252 = OpBitcast %v2int %251
        %253 = OpLoad %v4float %result
               OpImageWrite %248 %252 %253
               OpReturn
               OpFunctionEnd
%compute_blob_vf2_vf4_f1_ = OpFunction %v4float None %12
        %pos = OpFunctionParameter %_ptr_Function_v2float
       %blob = OpFunctionParameter %_ptr_Function_v4float
       %seed = OpFunctionParameter %_ptr_Function_float
         %17 = OpLabel
     %offset = OpVariable %_ptr_Function_v2float Function
   %s_offset = OpVariable %_ptr_Function_v2float Function
   %r_offset = OpVariable %_ptr_Function_v2float Function
   %g_offset = OpVariable %_ptr_Function_v2float Function
   %b_offset = OpVariable %_ptr_Function_v2float Function
      %r_dot = OpVariable %_ptr_Function_float Function
      %g_dot = OpVariable %_ptr_Function_float Function
      %b_dot = OpVariable %_ptr_Function_float Function
      %s_dot = OpVariable %_ptr_Function_float Function
       %dots = OpVariable %_ptr_Function_v4float Function
  %parabolas = OpVariable %_ptr_Function_v4float Function
         %19 = OpLoad %v2float %pos
         %20 = OpLoad %v4float %blob
         %21 = OpVectorShuffle %v2float %20 %20 0 1
         %22 = OpFSub %v2float %19 %21
               OpStore %offset %22
         %24 = OpLoad %v2float %offset
         %26 = OpLoad %float %seed
         %27 = OpFAdd %float %float_1_10000002 %26
         %28 = OpVectorTimesScalar %v2float %24 %27
               OpStore %s_offset %28
         %30 = OpLoad %v2float %offset
         %32 = OpVectorTimesScalar %v2float %30 %float_0_949999988
               OpStore %r_offset %32
         %34 = OpLoad %v2float %offset
         %36 = OpVectorTimesScalar %v2float %34 %float_1
               OpStore %g_offset %36
         %38 = OpLoad %v2float %offset
         %40 = OpVectorTimesScalar %v2float %38 %float_1_04999995
               OpStore %b_offset %40
         %42 = OpLoad %v2float %r_offset
         %43 = OpLoad %v2float %r_offset
         %44 = OpDot %float %42 %43
               OpStore %r_dot %44
         %46 = OpLoad %v2float %g_offset
         %47 = OpLoad %v2float %g_offset
         %48 = OpDot %float %46 %47
               OpStore %g_dot %48
         %50 = OpLoad %v2float %b_offset
         %51 = OpLoad %v2float %b_offset
         %52 = OpDot %float %50 %51
               OpStore %b_dot %52
         %54 = OpLoad %v2float %s_offset
         %55 = OpLoad %v2float %s_offset
         %56 = OpDot %float %54 %55
               OpStore %s_dot %56
         %58 = OpLoad %float %r_dot
         %59 = OpLoad %float %g_dot
         %60 = OpLoad %float %b_dot
         %61 = OpLoad %float %s_dot
         %62 = OpCompositeConstruct %v4float %58 %59 %60 %61
         %65 = OpAccessChain %_ptr_Function_float %blob %uint_3
         %66 = OpLoad %float %65
         %67 = OpVectorTimesScalar %v4float %62 %66
               OpStore %dots %67
         %68 = OpLoad %v4float %dots
         %69 = OpLoad %v4float %dots
         %70 = OpFMul %v4float %68 %69
         %71 = OpLoad %v4float %dots
         %72 = OpFAdd %v4float %70 %71
               OpStore %dots %72
         %73 = OpLoad %v4float %dots
         %74 = OpLoad %v4float %dots
         %75 = OpFMul %v4float %73 %74
         %76 = OpLoad %v4float %dots
         %77 = OpFAdd %v4float %75 %76
               OpStore %dots %77
         %78 = OpLoad %v4float %dots
         %79 = OpLoad %v4float %dots
         %80 = OpFMul %v4float %78 %79
         %81 = OpLoad %v4float %dots
         %82 = OpFAdd %v4float %80 %81
               OpStore %dots %82
         %83 = OpLoad %v4float %dots
         %84 = OpLoad %v4float %dots
         %85 = OpFMul %v4float %83 %84
         %86 = OpLoad %v4float %dots
         %87 = OpFAdd %v4float %85 %86
               OpStore %dots %87
         %88 = OpLoad %v4float %dots
         %89 = OpLoad %v4float %dots
         %90 = OpFMul %v4float %88 %89
         %91 = OpLoad %v4float %dots
         %92 = OpFAdd %v4float %90 %91
               OpStore %dots %92
         %93 = OpLoad %v4float %dots
         %94 = OpLoad %v4float %dots
         %95 = OpFMul %v4float %93 %94
         %96 = OpLoad %v4float %dots
         %97 = OpFAdd %v4float %95 %96
               OpStore %dots %97
        %101 = OpLoad %v4float %dots
        %102 = OpFSub %v4float %100 %101
        %105 = OpExtInst %v4float %1 FMax %102 %104
               OpStore %parabolas %105
        %106 = OpAccessChain %_ptr_Function_float %parabolas %uint_3
        %107 = OpLoad %float %106
        %108 = OpLoad %v4float %parabolas
        %109 = OpCompositeConstruct %v4float %107 %107 %107 %107
        %110 = OpFSub %v4float %108 %109
               OpStore %parabolas %110
        %111 = OpLoad %v4float %parabolas
        %112 = OpExtInst %v4float %1 FMax %111 %104
               OpStore %parabolas %112
        %113 = OpLoad %v4float %parabolas
               OpReturnValue %113
               OpFunctionEnd
