; SPIR-V
; Version: 1.3
; Generator: Khronos; 0
; Bound: 133
; Schema: 0
               OpCapability Shader
               OpCapability ImageQuery
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %1 "main" %gl_GlobalInvocationID
               OpExecutionMode %1 LocalSize 1 1 1
               OpMemberName %main_input_block_t 0 "size"
               OpName %main_input_block_t "main_input_block_t"
               OpName %main_input_block "main_input_block"
               OpName %main_input_image "main_input_image"
               OpName %main_output_image "main_output_image"
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
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
       %void = OpTypeVoid
          %9 = OpTypeFunction %void
        %int = OpTypeInt 32 1
%_ptr_Uniform_int = OpTypePointer Uniform %int
%_ptr_Function_int = OpTypePointer Function %int
%main_input_block_t = OpTypeStruct %int
%_ptr_Uniform_main_input_block_t = OpTypePointer Uniform %main_input_block_t
      %float = OpTypeFloat 32
         %26 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_26 = OpTypePointer UniformConstant %26
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
     %v2uint = OpTypeVector %uint 2
%_ptr_Function_v2uint = OpTypePointer Function %v2uint
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
%_ptr_Function_uint = OpTypePointer Function %uint
       %bool = OpTypeBool
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_1 %uint_1 %uint_1
     %uint_0 = OpConstant %uint 0
     %uint_4 = OpConstant %uint 4
      %int_1 = OpConstant %int 1
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
%main_input_block = OpVariable %_ptr_Uniform_main_input_block_t Uniform
%main_input_image = OpVariable %_ptr_UniformConstant_26 UniformConstant
%main_output_image = OpVariable %_ptr_UniformConstant_26 UniformConstant
          %1 = OpFunction %void None %9
         %10 = OpLabel
         %18 = OpVariable %_ptr_Function_int Function
         %32 = OpVariable %_ptr_Function_v2int Function
         %36 = OpVariable %_ptr_Function_int Function
         %39 = OpVariable %_ptr_Function_int Function
         %46 = OpVariable %_ptr_Function_v2uint Function
         %48 = OpVariable %_ptr_Function_v4float Function
         %53 = OpVariable %_ptr_Function_uint Function
         %58 = OpVariable %_ptr_Function_v2int Function
         %89 = OpVariable %_ptr_Function_v2int Function
         %16 = OpInBoundsAccessChain %_ptr_Uniform_int %main_input_block %uint_0
         %20 = OpLoad %int %16
               OpStore %18 %20
         %29 = OpLoad %26 %main_input_image
         %30 = OpImageQuerySize %v2int %29
               OpStore %32 %30
         %34 = OpInBoundsAccessChain %_ptr_Function_int %32 %uint_0
         %35 = OpLoad %int %34
               OpStore %36 %35
         %37 = OpInBoundsAccessChain %_ptr_Function_int %32 %uint_1
         %38 = OpLoad %int %37
               OpStore %39 %38
         %40 = OpLoad %v3uint %gl_GlobalInvocationID
         %41 = OpVectorExtractDynamic %uint %40 %uint_0
         %42 = OpLoad %v3uint %gl_GlobalInvocationID
         %43 = OpVectorExtractDynamic %uint %42 %uint_1
         %44 = OpCompositeConstruct %v2uint %41 %43
               OpStore %46 %44
         %51 = OpConvertUToF %float %uint_0
         %52 = OpCompositeConstruct %v4float %51 %51 %51 %51
               OpStore %48 %52
               OpStore %53 %uint_0
         %56 = OpBitcast %int %uint_4
         %57 = OpSNegate %int %56
         %59 = OpCompositeConstruct %v2int %57 %57
               OpStore %58 %59
         %65 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_0
         %66 = OpLoad %int %18
         %67 = OpSNegate %int %66
               OpStore %65 %67
               OpBranch %60
         %60 = OpLabel
               OpLoopMerge %64 %63 None
               OpBranch %61
         %61 = OpLabel
         %68 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_0
         %69 = OpLoad %int %68
         %70 = OpLoad %int %18
         %72 = OpSLessThanEqual %bool %69 %70
               OpBranchConditional %72 %62 %64
         %62 = OpLabel
         %78 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_1
         %79 = OpLoad %int %18
         %80 = OpSNegate %int %79
               OpStore %78 %80
               OpBranch %73
         %73 = OpLabel
               OpLoopMerge %77 %76 None
               OpBranch %74
         %74 = OpLabel
         %81 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_1
         %82 = OpLoad %int %81
         %83 = OpLoad %int %18
         %84 = OpSLessThanEqual %bool %82 %83
               OpBranchConditional %84 %75 %77
         %75 = OpLabel
         %85 = OpLoad %v2uint %46
         %86 = OpLoad %v2int %58
         %87 = OpBitcast %v2int %85
         %88 = OpIAdd %v2int %87 %86
               OpStore %89 %88
         %93 = OpInBoundsAccessChain %_ptr_Function_int %89 %uint_0
         %94 = OpLoad %int %93
         %95 = OpBitcast %int %uint_0
         %96 = OpSLessThanEqual %bool %95 %94
         %97 = OpInBoundsAccessChain %_ptr_Function_int %89 %uint_0
         %98 = OpLoad %int %97
         %99 = OpLoad %int %36
        %100 = OpSLessThan %bool %98 %99
        %101 = OpLogicalAnd %bool %96 %100
        %102 = OpInBoundsAccessChain %_ptr_Function_int %89 %uint_1
        %103 = OpLoad %int %102
        %104 = OpBitcast %int %uint_0
        %105 = OpSLessThanEqual %bool %104 %103
        %106 = OpLogicalAnd %bool %101 %105
        %107 = OpInBoundsAccessChain %_ptr_Function_int %89 %uint_1
        %108 = OpLoad %int %107
        %109 = OpLoad %int %39
        %110 = OpSLessThan %bool %108 %109
        %111 = OpLogicalAnd %bool %106 %110
               OpSelectionMerge %92 None
               OpBranchConditional %111 %90 %92
         %90 = OpLabel
        %112 = OpLoad %uint %53
        %113 = OpIAdd %uint %112 %uint_1
               OpStore %53 %113
        %114 = OpLoad %26 %main_input_image
        %115 = OpLoad %v2int %89
        %116 = OpImageRead %v4float %114 %115
        %117 = OpLoad %v4float %48
        %118 = OpFAdd %v4float %117 %116
               OpStore %48 %118
               OpBranch %92
         %92 = OpLabel
               OpBranch %76
         %76 = OpLabel
        %119 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_1
        %120 = OpLoad %int %119
        %122 = OpIAdd %int %120 %int_1
               OpStore %119 %122
               OpBranch %73
         %77 = OpLabel
               OpBranch %63
         %63 = OpLabel
        %123 = OpInBoundsAccessChain %_ptr_Function_int %58 %uint_0
        %124 = OpLoad %int %123
        %125 = OpIAdd %int %124 %int_1
               OpStore %123 %125
               OpBranch %60
         %64 = OpLabel
        %126 = OpLoad %v4float %48
        %127 = OpLoad %uint %53
        %128 = OpConvertUToF %float %127
        %129 = OpCompositeConstruct %v4float %128 %128 %128 %128
        %130 = OpFDiv %v4float %126 %129
        %131 = OpLoad %26 %main_output_image
        %132 = OpLoad %v2uint %46
               OpImageWrite %131 %132 %130
               OpReturn
               OpFunctionEnd
