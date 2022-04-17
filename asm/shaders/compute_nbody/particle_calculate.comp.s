; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 191
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID %gl_LocalInvocationID
               OpExecutionMode %main LocalSize 1 1 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %index "index"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "deltaT"
               OpMemberName %UBO 1 "particleCount"
               OpName %ubo "ubo"
               OpName %position "position"
               OpName %Particle "Particle"
               OpMemberName %Particle 0 "pos"
               OpMemberName %Particle 1 "vel"
               OpName %Pos "Pos"
               OpMemberName %Pos 0 "particles"
               OpName %_ ""
               OpName %velocity "velocity"
               OpName %acceleration "acceleration"
               OpName %i "i"
               OpName %gl_LocalInvocationID "gl_LocalInvocationID"
               OpName %SHARED_DATA_SIZE "SHARED_DATA_SIZE"
               OpName %sharedData "sharedData"
               OpName %j "j"
               OpName %other "other"
               OpName %len "len"
               OpName %GRAVITY "GRAVITY"
               OpName %SOFTEN "SOFTEN"
               OpName %POWER "POWER"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 1 Offset 4
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 1
               OpMemberDecorate %Particle 0 Offset 0
               OpMemberDecorate %Particle 1 Offset 16
               OpDecorate %_runtimearr_Particle ArrayStride 32
               OpMemberDecorate %Pos 0 Offset 0
               OpDecorate %Pos BufferBlock
               OpDecorate %_ DescriptorSet 0
               OpDecorate %_ Binding 0
               OpDecorate %gl_LocalInvocationID BuiltIn LocalInvocationId
               OpDecorate %SHARED_DATA_SIZE SpecId 1
               OpDecorate %104 SpecId 0
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
               OpDecorate %GRAVITY SpecId 2
               OpDecorate %SOFTEN SpecId 4
               OpDecorate %POWER SpecId 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %uint = OpTypeInt 32 0
%_ptr_Function_uint = OpTypePointer Function %uint
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %uint_0 = OpConstant %uint 0
%_ptr_Input_uint = OpTypePointer Input %uint
      %float = OpTypeFloat 32
        %int = OpTypeInt 32 1
        %UBO = OpTypeStruct %float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_1 = OpConstant %int 1
%_ptr_Uniform_int = OpTypePointer Uniform %int
       %bool = OpTypeBool
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
   %Particle = OpTypeStruct %v4float %v4float
%_runtimearr_Particle = OpTypeRuntimeArray %Particle
        %Pos = OpTypeStruct %_runtimearr_Particle
%_ptr_Uniform_Pos = OpTypePointer Uniform %Pos
          %_ = OpVariable %_ptr_Uniform_Pos Uniform
      %int_0 = OpConstant %int 0
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
    %float_0 = OpConstant %float 0
         %51 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
%_ptr_Function_int = OpTypePointer Function %int
%gl_LocalInvocationID = OpVariable %_ptr_Input_v3uint Input
%SHARED_DATA_SIZE = OpSpecConstant %int 1024
%_arr_v4float_SHARED_DATA_SIZE = OpTypeArray %v4float %SHARED_DATA_SIZE
%_ptr_Workgroup__arr_v4float_SHARED_DATA_SIZE = OpTypePointer Workgroup %_arr_v4float_SHARED_DATA_SIZE
 %sharedData = OpVariable %_ptr_Workgroup__arr_v4float_SHARED_DATA_SIZE Workgroup
%_ptr_Workgroup_v4float = OpTypePointer Workgroup %v4float
     %uint_2 = OpConstant %uint 2
   %uint_264 = OpConstant %uint 264
        %104 = OpSpecConstant %uint 1
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpSpecConstantComposite %v3uint %104 %uint_1 %uint_1
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %GRAVITY = OpSpecConstant %float 0.00200000009
     %uint_3 = OpConstant %uint 3
%_ptr_Function_float = OpTypePointer Function %float
     %SOFTEN = OpSpecConstant %float 0.0500000007
      %POWER = OpSpecConstant %float 0.75
%_ptr_Uniform_float = OpTypePointer Uniform %float
%float_0_0500000007 = OpConstant %float 0.0500000007
%float_0_00499999989 = OpConstant %float 0.00499999989
    %float_1 = OpConstant %float 1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %index = OpVariable %_ptr_Function_uint Function
   %position = OpVariable %_ptr_Function_v4float Function
   %velocity = OpVariable %_ptr_Function_v4float Function
%acceleration = OpVariable %_ptr_Function_v4float Function
          %i = OpVariable %_ptr_Function_int Function
          %j = OpVariable %_ptr_Function_int Function
      %other = OpVariable %_ptr_Function_v4float Function
        %len = OpVariable %_ptr_Function_v3float Function
         %14 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_0
         %15 = OpLoad %uint %14
               OpStore %index %15
         %16 = OpLoad %uint %index
         %24 = OpAccessChain %_ptr_Uniform_int %ubo %int_1
         %25 = OpLoad %int %24
         %26 = OpBitcast %uint %25
         %28 = OpUGreaterThanEqual %bool %16 %26
               OpSelectionMerge %30 None
               OpBranchConditional %28 %29 %30
         %29 = OpLabel
               OpReturn
         %30 = OpLabel
         %41 = OpLoad %uint %index
         %43 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %41 %int_0
         %44 = OpLoad %v4float %43
               OpStore %position %44
         %46 = OpLoad %uint %index
         %47 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %46 %int_1
         %48 = OpLoad %v4float %47
               OpStore %velocity %48
               OpStore %acceleration %51
               OpStore %i %int_0
               OpBranch %54
         %54 = OpLabel
               OpLoopMerge %56 %57 None
               OpBranch %58
         %58 = OpLabel
         %59 = OpLoad %int %i
         %60 = OpAccessChain %_ptr_Uniform_int %ubo %int_1
         %61 = OpLoad %int %60
         %62 = OpSLessThan %bool %59 %61
               OpBranchConditional %62 %55 %56
         %55 = OpLabel
         %63 = OpLoad %int %i
         %64 = OpBitcast %uint %63
         %66 = OpAccessChain %_ptr_Input_uint %gl_LocalInvocationID %uint_0
         %67 = OpLoad %uint %66
         %68 = OpIAdd %uint %64 %67
         %69 = OpAccessChain %_ptr_Uniform_int %ubo %int_1
         %70 = OpLoad %int %69
         %71 = OpBitcast %uint %70
         %72 = OpULessThan %bool %68 %71
               OpSelectionMerge %74 None
               OpBranchConditional %72 %73 %90
         %73 = OpLabel
         %79 = OpAccessChain %_ptr_Input_uint %gl_LocalInvocationID %uint_0
         %80 = OpLoad %uint %79
         %81 = OpLoad %int %i
         %82 = OpBitcast %uint %81
         %83 = OpAccessChain %_ptr_Input_uint %gl_LocalInvocationID %uint_0
         %84 = OpLoad %uint %83
         %85 = OpIAdd %uint %82 %84
         %86 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %85 %int_0
         %87 = OpLoad %v4float %86
         %89 = OpAccessChain %_ptr_Workgroup_v4float %sharedData %80
               OpStore %89 %87
               OpBranch %74
         %90 = OpLabel
         %91 = OpAccessChain %_ptr_Input_uint %gl_LocalInvocationID %uint_0
         %92 = OpLoad %uint %91
         %93 = OpAccessChain %_ptr_Workgroup_v4float %sharedData %92
               OpStore %93 %51
               OpBranch %74
         %74 = OpLabel
               OpControlBarrier %uint_2 %uint_2 %uint_264
               OpStore %j %int_0
               OpBranch %97
         %97 = OpLabel
               OpLoopMerge %99 %100 None
               OpBranch %101
        %101 = OpLabel
        %102 = OpLoad %int %j
        %103 = OpBitcast %uint %102
        %107 = OpCompositeExtract %uint %gl_WorkGroupSize 0
        %108 = OpULessThan %bool %103 %107
               OpBranchConditional %108 %98 %99
         %98 = OpLabel
        %110 = OpLoad %int %j
        %111 = OpAccessChain %_ptr_Workgroup_v4float %sharedData %110
        %112 = OpLoad %v4float %111
               OpStore %other %112
        %116 = OpLoad %v4float %other
        %117 = OpVectorShuffle %v3float %116 %116 0 1 2
        %118 = OpLoad %v4float %position
        %119 = OpVectorShuffle %v3float %118 %118 0 1 2
        %120 = OpFSub %v3float %117 %119
               OpStore %len %120
        %122 = OpLoad %v3float %len
        %123 = OpVectorTimesScalar %v3float %122 %GRAVITY
        %126 = OpAccessChain %_ptr_Function_float %other %uint_3
        %127 = OpLoad %float %126
        %128 = OpVectorTimesScalar %v3float %123 %127
        %129 = OpLoad %v3float %len
        %130 = OpLoad %v3float %len
        %131 = OpDot %float %129 %130
        %133 = OpFAdd %float %131 %SOFTEN
        %135 = OpExtInst %float %1 Pow %133 %POWER
        %136 = OpCompositeConstruct %v3float %135 %135 %135
        %137 = OpFDiv %v3float %128 %136
        %138 = OpLoad %v4float %acceleration
        %139 = OpVectorShuffle %v3float %138 %138 0 1 2
        %140 = OpFAdd %v3float %139 %137
        %141 = OpAccessChain %_ptr_Function_float %acceleration %uint_0
        %142 = OpCompositeExtract %float %140 0
               OpStore %141 %142
        %143 = OpAccessChain %_ptr_Function_float %acceleration %uint_1
        %144 = OpCompositeExtract %float %140 1
               OpStore %143 %144
        %145 = OpAccessChain %_ptr_Function_float %acceleration %uint_2
        %146 = OpCompositeExtract %float %140 2
               OpStore %145 %146
               OpBranch %100
        %100 = OpLabel
        %147 = OpLoad %int %j
        %148 = OpIAdd %int %147 %int_1
               OpStore %j %148
               OpBranch %97
         %99 = OpLabel
               OpControlBarrier %uint_2 %uint_2 %uint_264
               OpBranch %57
         %57 = OpLabel
        %149 = OpLoad %int %i
        %150 = OpIAdd %int %149 %SHARED_DATA_SIZE
               OpStore %i %150
               OpBranch %54
         %56 = OpLabel
        %151 = OpLoad %uint %index
        %153 = OpAccessChain %_ptr_Uniform_float %ubo %int_0
        %154 = OpLoad %float %153
        %156 = OpFMul %float %154 %float_0_0500000007
        %157 = OpLoad %v4float %acceleration
        %158 = OpVectorShuffle %v3float %157 %157 0 1 2
        %159 = OpVectorTimesScalar %v3float %158 %156
        %160 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %151 %int_1
        %161 = OpLoad %v4float %160
        %162 = OpVectorShuffle %v3float %161 %161 0 1 2
        %163 = OpFAdd %v3float %162 %159
        %164 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %151 %int_1 %uint_0
        %165 = OpCompositeExtract %float %163 0
               OpStore %164 %165
        %166 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %151 %int_1 %uint_1
        %167 = OpCompositeExtract %float %163 1
               OpStore %166 %167
        %168 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %151 %int_1 %uint_2
        %169 = OpCompositeExtract %float %163 2
               OpStore %168 %169
        %170 = OpLoad %uint %index
        %172 = OpAccessChain %_ptr_Uniform_float %ubo %int_0
        %173 = OpLoad %float %172
        %174 = OpFMul %float %float_0_00499999989 %173
        %175 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %170 %int_1 %uint_3
        %176 = OpLoad %float %175
        %177 = OpFAdd %float %176 %174
        %178 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %170 %int_1 %uint_3
               OpStore %178 %177
        %179 = OpLoad %uint %index
        %180 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %179 %int_1 %uint_3
        %181 = OpLoad %float %180
        %183 = OpFOrdGreaterThan %bool %181 %float_1
               OpSelectionMerge %185 None
               OpBranchConditional %183 %184 %185
        %184 = OpLabel
        %186 = OpLoad %uint %index
        %187 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %186 %int_1 %uint_3
        %188 = OpLoad %float %187
        %189 = OpFSub %float %188 %float_1
        %190 = OpAccessChain %_ptr_Uniform_float %_ %int_0 %186 %int_1 %uint_3
               OpStore %190 %189
               OpBranch %185
        %185 = OpLabel
               OpReturn
               OpFunctionEnd
