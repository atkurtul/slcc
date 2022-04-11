OpCapability Shader
OpCapability ImageQuery
OpMemoryModel Logical GLSL450
OpEntryPoint GLCompute %1 "main" %gl_GlobalInvocationID
OpExecutionMode %1 LocalSize 1 1 1

; Debug Information
OpMemberName %main_input_block_t 0 "size"
OpName %main_input_block_t "main_input_block_t"  ; id %11
OpName %main_input_block "main_input_block"  ; id %13
OpName %main_input_image "main_input_image"  ; id %24
OpName %main_output_image "main_output_image"  ; id %28

; Annotations
OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
OpMemberDecorate %main_input_block_t 0 Offset 0
OpDecorate %main_input_block_t Block
OpDecorate %main_input_block DescriptorSet 1
OpDecorate %main_input_block Binding 0
OpDecorate %main_input_image NonWritable
OpDecorate %main_input_image DescriptorSet 0
OpDecorate %main_input_image Binding 0
OpDecorate %main_output_image NonReadable
OpDecorate %main_output_image DescriptorSet 0
OpDecorate %main_output_image Binding 1

; Types, variables and constants
%uint = OpTypeInt 32 0
%v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%void = OpTypeVoid
%9 = OpTypeFunction %void
%int = OpTypeInt 32 1
%_ptr_Uniform_int = OpTypePointer Uniform %int
%main_input_block_t = OpTypeStruct %int
%_ptr_Uniform_main_input_block_t = OpTypePointer Uniform %main_input_block_t
%float = OpTypeFloat 32
%26 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_26 = OpTypePointer UniformConstant %26
%v2int = OpTypeVector %int 2
%v2uint = OpTypeVector %uint 2
%v4float = OpTypeVector %float 4
%bool = OpTypeBool
%uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_1 %uint_1 %uint_1
%uint_0 = OpConstant %uint 0
%int_1 = OpConstant %int 1
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
%main_input_block = OpVariable %_ptr_Uniform_main_input_block_t Uniform
%main_input_image = OpVariable %_ptr_UniformConstant_26 UniformConstant
%main_output_image = OpVariable %_ptr_UniformConstant_26 UniformConstant
%float_0 = OpConstant %float 0
%152 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
%int_n4 = OpConstant %int -4
%157 = OpConstantComposite %v2int %int_n4 %int_n4
%int_0 = OpConstant %int 0

; Function 1
%1 = OpFunction %void None %9
%10 = OpLabel
%16 = OpInBoundsAccessChain %_ptr_Uniform_int %main_input_block %uint_0
%20 = OpLoad %int %16
%29 = OpLoad %26 %main_input_image
%30 = OpImageQuerySize %v2int %29
%35 = OpCompositeExtract %int %30 0
%38 = OpCompositeExtract %int %30 1
%40 = OpLoad %v3uint %gl_GlobalInvocationID
%41 = OpVectorExtractDynamic %uint %40 %uint_0
%43 = OpVectorExtractDynamic %uint %40 %uint_1
%44 = OpCompositeConstruct %v2uint %41 %43
%67 = OpSNegate %int %20
%162 = OpCompositeInsert %v2int %67 %157 0
OpBranch %60
%60 = OpLabel
%177 = OpPhi %v2int %162 %10 %176 %63
%135 = OpPhi %uint %uint_0 %10 %143 %63
%134 = OpPhi %v4float %152 %10 %144 %63
%69 = OpCompositeExtract %int %177 0
%72 = OpSLessThanEqual %bool %69 %20
OpLoopMerge %64 %63 None
OpBranchConditional %72 %62 %64
%62 = OpLabel
%165 = OpCompositeInsert %v2int %67 %177 1
OpBranch %73
%73 = OpLabel
%178 = OpPhi %v2int %165 %62 %173 %76
%144 = OpPhi %v4float %134 %62 %150 %76
%143 = OpPhi %uint %135 %62 %149 %76
%82 = OpCompositeExtract %int %178 1
%84 = OpSLessThanEqual %bool %82 %20
OpLoopMerge %77 %76 None
OpBranchConditional %84 %75 %77
%75 = OpLabel
%87 = OpBitcast %v2int %44
%88 = OpIAdd %v2int %87 %178
%94 = OpCompositeExtract %int %88 0
%96 = OpSLessThanEqual %bool %int_0 %94
%100 = OpSLessThan %bool %94 %35
%101 = OpLogicalAnd %bool %96 %100
%103 = OpCompositeExtract %int %88 1
%105 = OpSLessThanEqual %bool %int_0 %103
%106 = OpLogicalAnd %bool %101 %105
%110 = OpSLessThan %bool %103 %38
%111 = OpLogicalAnd %bool %106 %110
OpSelectionMerge %92 None
OpBranchConditional %111 %90 %92
%90 = OpLabel
%113 = OpIAdd %uint %143 %uint_1
%116 = OpImageRead %v4float %29 %88
%118 = OpFAdd %v4float %144 %116
OpBranch %92
%92 = OpLabel
%150 = OpPhi %v4float %144 %75 %118 %90
%149 = OpPhi %uint %143 %75 %113 %90
OpBranch %76
%76 = OpLabel
%122 = OpIAdd %int %82 %int_1
%173 = OpCompositeInsert %v2int %122 %178 1
OpBranch %73
%77 = OpLabel
OpBranch %63
%63 = OpLabel
%124 = OpCompositeExtract %int %178 0
%125 = OpIAdd %int %124 %int_1
%176 = OpCompositeInsert %v2int %125 %178 0
OpBranch %60
%64 = OpLabel
%128 = OpConvertUToF %float %135
%129 = OpCompositeConstruct %v4float %128 %128 %128 %128
%130 = OpFDiv %v4float %134 %129
%131 = OpLoad %26 %main_output_image
OpImageWrite %131 %44 %130
OpReturn
OpFunctionEnd
