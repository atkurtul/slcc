; SPIR-V
; Version: 1.3
; Generator: Khronos; 0
; Bound: 131
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
         %55 = OpVariable %_ptr_Function_v2int Function
         %87 = OpVariable %_ptr_Function_v2int Function
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
         %56 = OpBitcast %int %uint_0
         %57 = OpCompositeConstruct %v2int %56 %56
               OpStore %55 %57
         %63 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_0
         %64 = OpLoad %int %18
         %65 = OpSNegate %int %64
               OpStore %63 %65
               OpBranch %58
         %58 = OpLabel
               OpLoopMerge %62 %61 None
               OpBranch %59
         %59 = OpLabel
         %66 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_0
         %67 = OpLoad %int %66
         %68 = OpLoad %int %18
         %70 = OpSLessThanEqual %bool %67 %68
               OpBranchConditional %70 %60 %62
         %60 = OpLabel
         %76 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_1
         %77 = OpLoad %int %18
         %78 = OpSNegate %int %77
               OpStore %76 %78
               OpBranch %71
         %71 = OpLabel
               OpLoopMerge %75 %74 None
               OpBranch %72
         %72 = OpLabel
         %79 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_1
         %80 = OpLoad %int %79
         %81 = OpLoad %int %18
         %82 = OpSLessThanEqual %bool %80 %81
               OpBranchConditional %82 %73 %75
         %73 = OpLabel
         %83 = OpLoad %v2uint %46
         %84 = OpLoad %v2int %55
         %85 = OpBitcast %v2int %83
         %86 = OpIAdd %v2int %85 %84
               OpStore %87 %86
         %91 = OpInBoundsAccessChain %_ptr_Function_int %87 %uint_0
         %92 = OpLoad %int %91
         %93 = OpBitcast %int %uint_0
         %94 = OpSLessThanEqual %bool %93 %92
         %95 = OpInBoundsAccessChain %_ptr_Function_int %87 %uint_0
         %96 = OpLoad %int %95
         %97 = OpLoad %int %36
         %98 = OpSLessThan %bool %96 %97
         %99 = OpLogicalAnd %bool %94 %98
        %100 = OpInBoundsAccessChain %_ptr_Function_int %87 %uint_1
        %101 = OpLoad %int %100
        %102 = OpBitcast %int %uint_0
        %103 = OpSLessThanEqual %bool %102 %101
        %104 = OpLogicalAnd %bool %99 %103
        %105 = OpInBoundsAccessChain %_ptr_Function_int %87 %uint_1
        %106 = OpLoad %int %105
        %107 = OpLoad %int %39
        %108 = OpSLessThan %bool %106 %107
        %109 = OpLogicalAnd %bool %104 %108
               OpSelectionMerge %90 None
               OpBranchConditional %109 %88 %90
         %88 = OpLabel
        %110 = OpLoad %uint %53
        %111 = OpIAdd %uint %110 %uint_1
               OpStore %53 %111
        %112 = OpLoad %26 %main_input_image
        %113 = OpLoad %v2int %87
        %114 = OpImageRead %v4float %112 %113
        %115 = OpLoad %v4float %48
        %116 = OpFAdd %v4float %115 %114
               OpStore %48 %116
               OpBranch %90
         %90 = OpLabel
               OpBranch %74
         %74 = OpLabel
        %117 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_1
        %118 = OpLoad %int %117
        %120 = OpIAdd %int %118 %int_1
               OpStore %117 %120
               OpBranch %71
         %75 = OpLabel
               OpBranch %61
         %61 = OpLabel
        %121 = OpInBoundsAccessChain %_ptr_Function_int %55 %uint_0
        %122 = OpLoad %int %121
        %123 = OpIAdd %int %122 %int_1
               OpStore %121 %123
               OpBranch %58
         %62 = OpLabel
        %124 = OpLoad %v4float %48
        %125 = OpLoad %uint %53
        %126 = OpConvertUToF %float %125
        %127 = OpCompositeConstruct %v4float %126 %126 %126 %126
        %128 = OpFDiv %v4float %124 %127
        %129 = OpLoad %26 %main_output_image
        %130 = OpLoad %v2uint %46
               OpImageWrite %129 %130 %128
               OpReturn
               OpFunctionEnd
