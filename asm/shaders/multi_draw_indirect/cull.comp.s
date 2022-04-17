; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 201
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 64 1 1
               OpSource GLSL 460
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %check_is_visible_mf44_vf3_f1_ "check_is_visible(mf44;vf3;f1;"
               OpName %mat "mat"
               OpName %origin "origin"
               OpName %radius "radius"
               OpName %plane_index "plane_index"
               OpName %i "i"
               OpName %j "j"
               OpName %sign "sign"
               OpName %plane "plane"
               OpName %k "k"
               OpName %id "id"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "view"
               OpMemberName %GlobalUniform 1 "proj"
               OpMemberName %GlobalUniform 2 "proj_view"
               OpMemberName %GlobalUniform 3 "model_count"
               OpName %global_uniform "global_uniform"
               OpName %ModelInformation "ModelInformation"
               OpMemberName %ModelInformation 0 "x"
               OpMemberName %ModelInformation 1 "y"
               OpMemberName %ModelInformation 2 "z"
               OpMemberName %ModelInformation 3 "r"
               OpMemberName %ModelInformation 4 "texture_index"
               OpMemberName %ModelInformation 5 "firstIndex"
               OpMemberName %ModelInformation 6 "indexCount"
               OpMemberName %ModelInformation 7 "_pad"
               OpName %model "model"
               OpName %ModelInformation_0 "ModelInformation"
               OpMemberName %ModelInformation_0 0 "x"
               OpMemberName %ModelInformation_0 1 "y"
               OpMemberName %ModelInformation_0 2 "z"
               OpMemberName %ModelInformation_0 3 "r"
               OpMemberName %ModelInformation_0 4 "texture_index"
               OpMemberName %ModelInformation_0 5 "firstIndex"
               OpMemberName %ModelInformation_0 6 "indexCount"
               OpMemberName %ModelInformation_0 7 "_pad"
               OpName %ModelInformationBuffer "ModelInformationBuffer"
               OpMemberName %ModelInformationBuffer 0 "arr"
               OpName %model_buffer "model_buffer"
               OpName %is_visible "is_visible"
               OpName %param "param"
               OpName %param_0 "param"
               OpName %param_1 "param"
               OpName %VkDrawIndexedIndirectCommand "VkDrawIndexedIndirectCommand"
               OpMemberName %VkDrawIndexedIndirectCommand 0 "indexCount"
               OpMemberName %VkDrawIndexedIndirectCommand 1 "instanceCount"
               OpMemberName %VkDrawIndexedIndirectCommand 2 "firstIndex"
               OpMemberName %VkDrawIndexedIndirectCommand 3 "vertexOffset"
               OpMemberName %VkDrawIndexedIndirectCommand 4 "firstInstance"
               OpName %CommandBuffer "CommandBuffer"
               OpMemberName %CommandBuffer 0 "commands"
               OpName %command_buffer "command_buffer"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 ColMajor
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpMemberDecorate %GlobalUniform 2 MatrixStride 16
               OpMemberDecorate %GlobalUniform 3 Offset 192
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 2
               OpMemberDecorate %ModelInformation_0 0 Offset 0
               OpMemberDecorate %ModelInformation_0 1 Offset 4
               OpMemberDecorate %ModelInformation_0 2 Offset 8
               OpMemberDecorate %ModelInformation_0 3 Offset 12
               OpMemberDecorate %ModelInformation_0 4 Offset 16
               OpMemberDecorate %ModelInformation_0 5 Offset 20
               OpMemberDecorate %ModelInformation_0 6 Offset 24
               OpMemberDecorate %ModelInformation_0 7 Offset 28
               OpDecorate %_runtimearr_ModelInformation_0 ArrayStride 32
               OpMemberDecorate %ModelInformationBuffer 0 NonWritable
               OpMemberDecorate %ModelInformationBuffer 0 Offset 0
               OpDecorate %ModelInformationBuffer BufferBlock
               OpDecorate %model_buffer DescriptorSet 0
               OpDecorate %model_buffer Binding 0
               OpMemberDecorate %VkDrawIndexedIndirectCommand 0 Offset 0
               OpMemberDecorate %VkDrawIndexedIndirectCommand 1 Offset 4
               OpMemberDecorate %VkDrawIndexedIndirectCommand 2 Offset 8
               OpMemberDecorate %VkDrawIndexedIndirectCommand 3 Offset 12
               OpMemberDecorate %VkDrawIndexedIndirectCommand 4 Offset 16
               OpDecorate %_runtimearr_VkDrawIndexedIndirectCommand ArrayStride 20
               OpMemberDecorate %CommandBuffer 0 NonReadable
               OpMemberDecorate %CommandBuffer 0 Offset 0
               OpDecorate %CommandBuffer BufferBlock
               OpDecorate %command_buffer DescriptorSet 0
               OpDecorate %command_buffer Binding 3
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
%_ptr_Function_mat4v4float = OpTypePointer Function %mat4v4float
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_Function_float = OpTypePointer Function %float
       %bool = OpTypeBool
         %14 = OpTypeFunction %bool %_ptr_Function_mat4v4float %_ptr_Function_v3float %_ptr_Function_float
       %uint = OpTypeInt 32 0
%_ptr_Function_uint = OpTypePointer Function %uint
     %uint_0 = OpConstant %uint 0
     %uint_3 = OpConstant %uint 3
     %uint_2 = OpConstant %uint 2
    %float_1 = OpConstant %float 1
   %float_n1 = OpConstant %float -1
%_ptr_Function_v4float = OpTypePointer Function %v4float
    %float_0 = OpConstant %float 0
         %59 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
     %uint_4 = OpConstant %uint 4
        %int = OpTypeInt 32 1
      %int_1 = OpConstant %int 1
      %false = OpConstantFalse %bool
       %true = OpConstantTrue %bool
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
%_ptr_Input_uint = OpTypePointer Input %uint
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %uint
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
      %int_3 = OpConstant %int 3
%_ptr_Uniform_uint = OpTypePointer Uniform %uint
%ModelInformation = OpTypeStruct %float %float %float %float %uint %uint %uint %uint
%_ptr_Function_ModelInformation = OpTypePointer Function %ModelInformation
%ModelInformation_0 = OpTypeStruct %float %float %float %float %uint %uint %uint %uint
%_runtimearr_ModelInformation_0 = OpTypeRuntimeArray %ModelInformation_0
%ModelInformationBuffer = OpTypeStruct %_runtimearr_ModelInformation_0
%_ptr_Uniform_ModelInformationBuffer = OpTypePointer Uniform %ModelInformationBuffer
%model_buffer = OpVariable %_ptr_Uniform_ModelInformationBuffer Uniform
      %int_0 = OpConstant %int 0
%_ptr_Uniform_ModelInformation_0 = OpTypePointer Uniform %ModelInformation_0
      %int_2 = OpConstant %int 2
      %int_4 = OpConstant %int 4
      %int_5 = OpConstant %int 5
      %int_6 = OpConstant %int 6
      %int_7 = OpConstant %int 7
%_ptr_Function_bool = OpTypePointer Function %bool
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%VkDrawIndexedIndirectCommand = OpTypeStruct %uint %uint %uint %int %uint
%_runtimearr_VkDrawIndexedIndirectCommand = OpTypeRuntimeArray %VkDrawIndexedIndirectCommand
%CommandBuffer = OpTypeStruct %_runtimearr_VkDrawIndexedIndirectCommand
%_ptr_Uniform_CommandBuffer = OpTypePointer Uniform %CommandBuffer
%command_buffer = OpVariable %_ptr_Uniform_CommandBuffer Uniform
    %uint_64 = OpConstant %uint 64
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_64 %uint_1 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %id = OpVariable %_ptr_Function_uint Function
      %model = OpVariable %_ptr_Function_ModelInformation Function
 %is_visible = OpVariable %_ptr_Function_bool Function
      %param = OpVariable %_ptr_Function_mat4v4float Function
    %param_0 = OpVariable %_ptr_Function_v3float Function
    %param_1 = OpVariable %_ptr_Function_float Function
        %122 = OpAccessChain %_ptr_Input_uint %gl_GlobalInvocationID %uint_0
        %123 = OpLoad %uint %122
               OpStore %id %123
        %124 = OpLoad %uint %id
        %130 = OpAccessChain %_ptr_Uniform_uint %global_uniform %int_3
        %131 = OpLoad %uint %130
        %132 = OpUGreaterThanEqual %bool %124 %131
               OpSelectionMerge %134 None
               OpBranchConditional %132 %133 %134
        %133 = OpLabel
               OpReturn
        %134 = OpLabel
        %145 = OpLoad %uint %id
        %147 = OpAccessChain %_ptr_Uniform_ModelInformation_0 %model_buffer %int_0 %145
        %148 = OpLoad %ModelInformation_0 %147
        %149 = OpCompositeExtract %float %148 0
        %150 = OpAccessChain %_ptr_Function_float %model %int_0
               OpStore %150 %149
        %151 = OpCompositeExtract %float %148 1
        %152 = OpAccessChain %_ptr_Function_float %model %int_1
               OpStore %152 %151
        %153 = OpCompositeExtract %float %148 2
        %155 = OpAccessChain %_ptr_Function_float %model %int_2
               OpStore %155 %153
        %156 = OpCompositeExtract %float %148 3
        %157 = OpAccessChain %_ptr_Function_float %model %int_3
               OpStore %157 %156
        %158 = OpCompositeExtract %uint %148 4
        %160 = OpAccessChain %_ptr_Function_uint %model %int_4
               OpStore %160 %158
        %161 = OpCompositeExtract %uint %148 5
        %163 = OpAccessChain %_ptr_Function_uint %model %int_5
               OpStore %163 %161
        %164 = OpCompositeExtract %uint %148 6
        %166 = OpAccessChain %_ptr_Function_uint %model %int_6
               OpStore %166 %164
        %167 = OpCompositeExtract %uint %148 7
        %169 = OpAccessChain %_ptr_Function_uint %model %int_7
               OpStore %169 %167
        %172 = OpAccessChain %_ptr_Function_float %model %int_0
        %173 = OpLoad %float %172
        %174 = OpAccessChain %_ptr_Function_float %model %int_1
        %175 = OpLoad %float %174
        %176 = OpAccessChain %_ptr_Function_float %model %int_2
        %177 = OpLoad %float %176
        %178 = OpCompositeConstruct %v3float %173 %175 %177
        %181 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_2
        %182 = OpLoad %mat4v4float %181
               OpStore %param %182
               OpStore %param_0 %178
        %185 = OpAccessChain %_ptr_Function_float %model %int_3
        %186 = OpLoad %float %185
               OpStore %param_1 %186
        %187 = OpFunctionCall %bool %check_is_visible_mf44_vf3_f1_ %param %param_0 %param_1
               OpStore %is_visible %187
        %193 = OpLoad %uint %id
        %194 = OpLoad %bool %is_visible
        %195 = OpSelect %int %194 %int_1 %int_0
        %196 = OpBitcast %uint %195
        %197 = OpAccessChain %_ptr_Uniform_uint %command_buffer %int_0 %193 %int_1
               OpStore %197 %196
               OpReturn
               OpFunctionEnd
%check_is_visible_mf44_vf3_f1_ = OpFunction %bool None %14
        %mat = OpFunctionParameter %_ptr_Function_mat4v4float
     %origin = OpFunctionParameter %_ptr_Function_v3float
     %radius = OpFunctionParameter %_ptr_Function_float
         %19 = OpLabel
%plane_index = OpVariable %_ptr_Function_uint Function
          %i = OpVariable %_ptr_Function_uint Function
          %j = OpVariable %_ptr_Function_uint Function
       %sign = OpVariable %_ptr_Function_float Function
      %plane = OpVariable %_ptr_Function_v4float Function
          %k = OpVariable %_ptr_Function_uint Function
               OpStore %plane_index %uint_0
               OpStore %i %uint_0
               OpBranch %25
         %25 = OpLabel
               OpLoopMerge %27 %28 None
               OpBranch %29
         %29 = OpLabel
         %30 = OpLoad %uint %i
         %32 = OpULessThan %bool %30 %uint_3
               OpBranchConditional %32 %26 %27
         %26 = OpLabel
               OpStore %j %uint_0
               OpBranch %34
         %34 = OpLabel
               OpLoopMerge %36 %37 None
               OpBranch %38
         %38 = OpLabel
         %39 = OpLoad %uint %j
         %41 = OpULessThan %bool %39 %uint_2
               OpBranchConditional %41 %35 %36
         %35 = OpLabel
         %42 = OpLoad %uint %plane_index
         %43 = OpIEqual %bool %42 %uint_2
         %44 = OpLoad %uint %plane_index
         %45 = OpIEqual %bool %44 %uint_3
         %46 = OpLogicalOr %bool %43 %45
               OpSelectionMerge %48 None
               OpBranchConditional %46 %47 %48
         %47 = OpLabel
               OpBranch %37
         %48 = OpLabel
         %51 = OpLoad %uint %j
         %52 = OpUGreaterThan %bool %51 %uint_0
         %55 = OpSelect %float %52 %float_1 %float_n1
               OpStore %sign %55
               OpStore %plane %59
               OpStore %k %uint_0
               OpBranch %61
         %61 = OpLabel
               OpLoopMerge %63 %64 None
               OpBranch %65
         %65 = OpLabel
         %66 = OpLoad %uint %k
         %68 = OpULessThan %bool %66 %uint_4
               OpBranchConditional %68 %62 %63
         %62 = OpLabel
         %69 = OpLoad %uint %k
         %70 = OpLoad %uint %k
         %71 = OpAccessChain %_ptr_Function_float %mat %70 %uint_3
         %72 = OpLoad %float %71
         %73 = OpLoad %float %sign
         %74 = OpLoad %uint %k
         %75 = OpLoad %uint %i
         %76 = OpAccessChain %_ptr_Function_float %mat %74 %75
         %77 = OpLoad %float %76
         %78 = OpFMul %float %73 %77
         %79 = OpFAdd %float %72 %78
         %80 = OpAccessChain %_ptr_Function_float %plane %69
               OpStore %80 %79
               OpBranch %64
         %64 = OpLabel
         %81 = OpLoad %uint %k
         %84 = OpIAdd %uint %81 %int_1
               OpStore %k %84
               OpBranch %61
         %63 = OpLabel
         %85 = OpLoad %v4float %plane
         %86 = OpVectorShuffle %v3float %85 %85 0 1 2
         %87 = OpLoad %v4float %plane
         %88 = OpVectorShuffle %v3float %87 %87 0 1 2
         %89 = OpDot %float %86 %88
         %90 = OpExtInst %float %1 Sqrt %89
         %91 = OpLoad %v4float %plane
         %92 = OpCompositeConstruct %v4float %90 %90 %90 %90
         %93 = OpFDiv %v4float %91 %92
               OpStore %plane %93
         %94 = OpLoad %v3float %origin
         %95 = OpLoad %v4float %plane
         %96 = OpVectorShuffle %v3float %95 %95 0 1 2
         %97 = OpDot %float %94 %96
         %98 = OpAccessChain %_ptr_Function_float %plane %uint_3
         %99 = OpLoad %float %98
        %100 = OpFAdd %float %97 %99
        %101 = OpLoad %float %radius
        %102 = OpFAdd %float %100 %101
        %103 = OpFOrdLessThan %bool %102 %float_0
               OpSelectionMerge %105 None
               OpBranchConditional %103 %104 %105
        %104 = OpLabel
               OpReturnValue %false
        %105 = OpLabel
               OpBranch %37
         %37 = OpLabel
        %108 = OpLoad %uint %j
        %109 = OpIAdd %uint %108 %int_1
               OpStore %j %109
        %110 = OpLoad %uint %plane_index
        %111 = OpIAdd %uint %110 %int_1
               OpStore %plane_index %111
               OpBranch %34
         %36 = OpLabel
               OpBranch %28
         %28 = OpLabel
        %112 = OpLoad %uint %i
        %113 = OpIAdd %uint %112 %int_1
               OpStore %i %113
               OpBranch %25
         %27 = OpLabel
               OpReturnValue %true
               OpFunctionEnd
