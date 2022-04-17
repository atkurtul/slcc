; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 255
; Schema: 0
               OpCapability Shader
               OpCapability RayQueryKHR
               OpExtension "SPV_KHR_ray_query"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %in_scene_pos %in_normal %o_color %in_pos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 460
               OpSourceExtension "GL_EXT_ray_query"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %calculate_ambient_occlusion_vf3_vf3_ "calculate_ambient_occlusion(vf3;vf3;"
               OpName %object_point "object_point"
               OpName %object_normal "object_normal"
               OpName %intersects_light_vf3_vf3_ "intersects_light(vf3;vf3;"
               OpName %light_origin "light_origin"
               OpName %pos "pos"
               OpName %max_ao_each "max_ao_each"
               OpName %max_ao "max_ao"
               OpName %accumulated_ao "accumulated_ao"
               OpName %u "u"
               OpName %v "v"
               OpName %accumulated_factor "accumulated_factor"
               OpName %j "j"
               OpName %phi "phi"
               OpName %k "k"
               OpName %theta "theta"
               OpName %x "x"
               OpName %y "y"
               OpName %z "z"
               OpName %direction "direction"
               OpName %query "query"
               OpName %topLevelAS "topLevelAS"
               OpName %dist "dist"
               OpName %ao "ao"
               OpName %factor "factor"
               OpName %direction_0 "direction"
               OpName %distance "distance"
               OpName %query_0 "query"
               OpName %ao_0 "ao"
               OpName %in_scene_pos "in_scene_pos"
               OpName %in_normal "in_normal"
               OpName %param "param"
               OpName %param_0 "param"
               OpName %lighting "lighting"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "model"
               OpMemberName %GlobalUniform 1 "view_proj"
               OpMemberName %GlobalUniform 2 "camera_position"
               OpMemberName %GlobalUniform 3 "light_position"
               OpName %global_uniform "global_uniform"
               OpName %param_1 "param"
               OpName %param_2 "param"
               OpName %o_color "o_color"
               OpName %in_pos "in_pos"
               OpDecorate %topLevelAS DescriptorSet 0
               OpDecorate %topLevelAS Binding 0
               OpDecorate %in_scene_pos Location 2
               OpDecorate %in_normal Location 1
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpMemberDecorate %GlobalUniform 3 Offset 144
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
               OpDecorate %o_color Location 0
               OpDecorate %in_pos Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
          %9 = OpTypeFunction %float %_ptr_Function_v3float %_ptr_Function_v3float
       %bool = OpTypeBool
         %15 = OpTypeFunction %bool %_ptr_Function_v3float %_ptr_Function_v3float
       %uint = OpTypeInt 32 0
%_ptr_Function_uint = OpTypePointer Function %uint
     %uint_3 = OpConstant %uint 3
%_ptr_Function_float = OpTypePointer Function %float
    %float_0 = OpConstant %float 0
    %float_1 = OpConstant %float 1
         %34 = OpConstantComposite %v3float %float_0 %float_0 %float_1
%float_0_899999976 = OpConstant %float 0.899999976
         %43 = OpConstantComposite %v3float %float_1 %float_0 %float_0
     %uint_0 = OpConstant %uint 0
  %float_0_5 = OpConstant %float 0.5
%float_n3_14159012 = OpConstant %float -3.14159012
%float_6_28318024 = OpConstant %float 6.28318024
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
        %127 = OpTypeRayQueryKHR
%_ptr_Private_127 = OpTypePointer Private %127
      %query = OpVariable %_ptr_Private_127 Private
        %130 = OpTypeAccelerationStructureKHR
%_ptr_UniformConstant_130 = OpTypePointer UniformConstant %130
 %topLevelAS = OpVariable %_ptr_UniformConstant_130 UniformConstant
     %uint_4 = OpConstant %uint 4
   %uint_255 = OpConstant %uint 255
%float_0_00999999978 = OpConstant %float 0.00999999978
    %float_2 = OpConstant %float 2
       %true = OpConstantTrue %bool
        %int = OpTypeInt 32 1
      %int_1 = OpConstant %int 1
      %false = OpConstantFalse %bool
      %int_0 = OpConstant %int 0
%float_0_200000003 = OpConstant %float 0.200000003
%float_0_800000012 = OpConstant %float 0.800000012
    %query_0 = OpVariable %_ptr_Private_127 Private
    %v4float = OpTypeVector %float 4
%_ptr_Input_v4float = OpTypePointer Input %v4float
%in_scene_pos = OpVariable %_ptr_Input_v4float Input
%_ptr_Input_v3float = OpTypePointer Input %v3float
  %in_normal = OpVariable %_ptr_Input_v3float Input
%_ptr_Function_v4float = OpTypePointer Function %v4float
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %v3float %v3float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
      %int_3 = OpConstant %int 3
%_ptr_Uniform_v3float = OpTypePointer Uniform %v3float
        %238 = OpConstantComposite %v4float %float_0_200000003 %float_0_200000003 %float_0_200000003 %float_1
        %239 = OpConstantComposite %v4float %float_1 %float_1 %float_1 %float_1
     %v4bool = OpTypeVector %bool 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
    %o_color = OpVariable %_ptr_Output_v4float Output
        %247 = OpConstantComposite %v3float %float_1 %float_1 %float_1
     %in_pos = OpVariable %_ptr_Input_v4float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
       %ao_0 = OpVariable %_ptr_Function_float Function
      %param = OpVariable %_ptr_Function_v3float Function
    %param_0 = OpVariable %_ptr_Function_v3float Function
   %lighting = OpVariable %_ptr_Function_v4float Function
    %param_1 = OpVariable %_ptr_Function_v3float Function
    %param_2 = OpVariable %_ptr_Function_v3float Function
        %218 = OpLoad %v4float %in_scene_pos
        %219 = OpVectorShuffle %v3float %218 %218 0 1 2
               OpStore %param %219
        %221 = OpLoad %v3float %in_normal
               OpStore %param_0 %221
        %222 = OpFunctionCall %float %calculate_ambient_occlusion_vf3_vf3_ %param %param_0
               OpStore %ao_0 %222
        %232 = OpAccessChain %_ptr_Uniform_v3float %global_uniform %int_3
        %233 = OpLoad %v3float %232
               OpStore %param_1 %233
        %235 = OpLoad %v4float %in_scene_pos
        %236 = OpVectorShuffle %v3float %235 %235 0 1 2
               OpStore %param_2 %236
        %237 = OpFunctionCall %bool %intersects_light_vf3_vf3_ %param_1 %param_2
        %241 = OpCompositeConstruct %v4bool %237 %237 %237 %237
        %242 = OpSelect %v4float %241 %238 %239
               OpStore %lighting %242
        %245 = OpLoad %v4float %lighting
        %246 = OpLoad %float %ao_0
        %248 = OpVectorTimesScalar %v3float %247 %246
        %249 = OpCompositeExtract %float %248 0
        %250 = OpCompositeExtract %float %248 1
        %251 = OpCompositeExtract %float %248 2
        %252 = OpCompositeConstruct %v4float %249 %250 %251 %float_1
        %253 = OpFMul %v4float %245 %252
               OpStore %o_color %253
               OpReturn
               OpFunctionEnd
%calculate_ambient_occlusion_vf3_vf3_ = OpFunction %float None %9
%object_point = OpFunctionParameter %_ptr_Function_v3float
%object_normal = OpFunctionParameter %_ptr_Function_v3float
         %13 = OpLabel
%max_ao_each = OpVariable %_ptr_Function_uint Function
     %max_ao = OpVariable %_ptr_Function_uint Function
%accumulated_ao = OpVariable %_ptr_Function_float Function
          %u = OpVariable %_ptr_Function_v3float Function
         %39 = OpVariable %_ptr_Function_v3float Function
          %v = OpVariable %_ptr_Function_v3float Function
%accumulated_factor = OpVariable %_ptr_Function_float Function
          %j = OpVariable %_ptr_Function_uint Function
        %phi = OpVariable %_ptr_Function_float Function
          %k = OpVariable %_ptr_Function_uint Function
      %theta = OpVariable %_ptr_Function_float Function
          %x = OpVariable %_ptr_Function_float Function
          %y = OpVariable %_ptr_Function_float Function
          %z = OpVariable %_ptr_Function_float Function
  %direction = OpVariable %_ptr_Function_v3float Function
       %dist = OpVariable %_ptr_Function_float Function
         %ao = OpVariable %_ptr_Function_float Function
     %factor = OpVariable %_ptr_Function_float Function
               OpStore %max_ao_each %uint_3
         %25 = OpLoad %uint %max_ao_each
         %26 = OpLoad %uint %max_ao_each
         %27 = OpIMul %uint %25 %26
               OpStore %max_ao %27
               OpStore %accumulated_ao %float_0
         %32 = OpLoad %v3float %object_normal
         %35 = OpDot %float %32 %34
         %36 = OpExtInst %float %1 FAbs %35
         %38 = OpFOrdGreaterThan %bool %36 %float_0_899999976
               OpSelectionMerge %41 None
               OpBranchConditional %38 %40 %45
         %40 = OpLabel
         %42 = OpLoad %v3float %object_normal
         %44 = OpExtInst %v3float %1 Cross %42 %43
               OpStore %39 %44
               OpBranch %41
         %45 = OpLabel
         %46 = OpLoad %v3float %object_normal
         %47 = OpExtInst %v3float %1 Cross %46 %34
               OpStore %39 %47
               OpBranch %41
         %41 = OpLabel
         %48 = OpLoad %v3float %39
               OpStore %u %48
         %50 = OpLoad %v3float %object_normal
         %51 = OpLoad %v3float %u
         %52 = OpExtInst %v3float %1 Cross %50 %51
               OpStore %v %52
               OpStore %accumulated_factor %float_0
               OpStore %j %uint_0
               OpBranch %56
         %56 = OpLabel
               OpLoopMerge %58 %59 None
               OpBranch %60
         %60 = OpLabel
         %61 = OpLoad %uint %j
         %62 = OpLoad %uint %max_ao_each
         %63 = OpULessThan %bool %61 %62
               OpBranchConditional %63 %57 %58
         %57 = OpLabel
         %68 = OpLoad %uint %j
         %70 = OpIAdd %uint %68 %uint_1
         %71 = OpConvertUToF %float %70
         %72 = OpLoad %uint %max_ao_each
         %74 = OpIAdd %uint %72 %uint_2
         %75 = OpConvertUToF %float %74
         %76 = OpFDiv %float %71 %75
         %77 = OpFMul %float %float_6_28318024 %76
         %78 = OpFAdd %float %float_n3_14159012 %77
         %79 = OpFMul %float %float_0_5 %78
               OpStore %phi %79
               OpStore %k %uint_0
               OpBranch %81
         %81 = OpLabel
               OpLoopMerge %83 %84 None
               OpBranch %85
         %85 = OpLabel
         %86 = OpLoad %uint %k
         %87 = OpLoad %uint %max_ao_each
         %88 = OpULessThan %bool %86 %87
               OpBranchConditional %88 %82 %83
         %82 = OpLabel
         %90 = OpLoad %uint %k
         %91 = OpIAdd %uint %90 %uint_1
         %92 = OpConvertUToF %float %91
         %93 = OpLoad %uint %max_ao_each
         %94 = OpIAdd %uint %93 %uint_2
         %95 = OpConvertUToF %float %94
         %96 = OpFDiv %float %92 %95
         %97 = OpFMul %float %float_6_28318024 %96
         %98 = OpFAdd %float %float_n3_14159012 %97
         %99 = OpFMul %float %float_0_5 %98
               OpStore %theta %99
        %101 = OpLoad %float %phi
        %102 = OpExtInst %float %1 Cos %101
        %103 = OpLoad %float %theta
        %104 = OpExtInst %float %1 Sin %103
        %105 = OpFMul %float %102 %104
               OpStore %x %105
        %107 = OpLoad %float %phi
        %108 = OpExtInst %float %1 Sin %107
        %109 = OpLoad %float %theta
        %110 = OpExtInst %float %1 Sin %109
        %111 = OpFMul %float %108 %110
               OpStore %y %111
        %113 = OpLoad %float %theta
        %114 = OpExtInst %float %1 Cos %113
               OpStore %z %114
        %116 = OpLoad %float %x
        %117 = OpLoad %v3float %u
        %118 = OpVectorTimesScalar %v3float %117 %116
        %119 = OpLoad %float %y
        %120 = OpLoad %v3float %v
        %121 = OpVectorTimesScalar %v3float %120 %119
        %122 = OpFAdd %v3float %118 %121
        %123 = OpLoad %float %z
        %124 = OpLoad %v3float %object_normal
        %125 = OpVectorTimesScalar %v3float %124 %123
        %126 = OpFAdd %v3float %122 %125
               OpStore %direction %126
        %133 = OpLoad %130 %topLevelAS
        %136 = OpLoad %v3float %object_point
        %138 = OpLoad %v3float %direction
               OpRayQueryInitializeKHR %query %133 %uint_4 %uint_255 %136 %float_0_00999999978 %138 %float_2
        %140 = OpRayQueryProceedKHR %bool %query
               OpStore %dist %float_2
        %145 = OpRayQueryGetIntersectionTypeKHR %uint %query %int_1
        %146 = OpINotEqual %bool %145 %uint_0
               OpSelectionMerge %148 None
               OpBranchConditional %146 %147 %148
        %147 = OpLabel
        %151 = OpRayQueryGetIntersectionTKHR %float %query %int_0
               OpStore %dist %151
               OpBranch %148
        %148 = OpLabel
        %153 = OpLoad %float %dist
        %154 = OpExtInst %float %1 FMin %153 %float_2
               OpStore %ao %154
        %158 = OpLoad %float %z
        %159 = OpFMul %float %float_0_800000012 %158
        %160 = OpLoad %float %z
        %161 = OpFMul %float %159 %160
        %162 = OpFAdd %float %float_0_200000003 %161
               OpStore %factor %162
        %163 = OpLoad %float %factor
        %164 = OpLoad %float %accumulated_factor
        %165 = OpFAdd %float %164 %163
               OpStore %accumulated_factor %165
        %166 = OpLoad %float %ao
        %167 = OpLoad %float %factor
        %168 = OpFMul %float %166 %167
        %169 = OpLoad %float %accumulated_ao
        %170 = OpFAdd %float %169 %168
               OpStore %accumulated_ao %170
               OpBranch %84
         %84 = OpLabel
        %171 = OpLoad %uint %k
        %172 = OpIAdd %uint %171 %int_1
               OpStore %k %172
               OpBranch %81
         %83 = OpLabel
               OpBranch %59
         %59 = OpLabel
        %173 = OpLoad %uint %j
        %174 = OpIAdd %uint %173 %int_1
               OpStore %j %174
               OpBranch %56
         %58 = OpLabel
        %175 = OpLoad %float %accumulated_factor
        %176 = OpFMul %float %float_2 %175
        %177 = OpLoad %float %accumulated_ao
        %178 = OpFDiv %float %177 %176
               OpStore %accumulated_ao %178
        %179 = OpLoad %float %accumulated_ao
        %180 = OpLoad %float %accumulated_ao
        %181 = OpFMul %float %180 %179
               OpStore %accumulated_ao %181
        %182 = OpLoad %float %accumulated_ao
        %183 = OpFMul %float %182 %float_1
        %184 = OpExtInst %float %1 FMin %183 %float_1
        %185 = OpExtInst %float %1 FMax %184 %float_0
               OpStore %accumulated_ao %185
        %186 = OpLoad %float %accumulated_ao
               OpReturnValue %186
               OpFunctionEnd
%intersects_light_vf3_vf3_ = OpFunction %bool None %15
%light_origin = OpFunctionParameter %_ptr_Function_v3float
        %pos = OpFunctionParameter %_ptr_Function_v3float
         %19 = OpLabel
%direction_0 = OpVariable %_ptr_Function_v3float Function
   %distance = OpVariable %_ptr_Function_float Function
        %190 = OpLoad %v3float %light_origin
        %191 = OpLoad %v3float %pos
        %192 = OpFSub %v3float %190 %191
               OpStore %direction_0 %192
        %194 = OpLoad %v3float %direction_0
        %195 = OpLoad %v3float %direction_0
        %196 = OpDot %float %194 %195
        %197 = OpExtInst %float %1 Sqrt %196
               OpStore %distance %197
        %199 = OpLoad %130 %topLevelAS
        %200 = OpLoad %v3float %pos
        %201 = OpLoad %v3float %direction_0
        %202 = OpLoad %float %distance
               OpRayQueryInitializeKHR %query_0 %199 %uint_4 %uint_255 %200 %float_0_00999999978 %201 %202
        %203 = OpRayQueryProceedKHR %bool %query_0
        %204 = OpRayQueryGetIntersectionTypeKHR %uint %query_0 %int_1
        %205 = OpINotEqual %bool %204 %uint_0
               OpSelectionMerge %207 None
               OpBranchConditional %205 %206 %207
        %206 = OpLabel
               OpReturnValue %true
        %207 = OpLabel
               OpReturnValue %false
               OpFunctionEnd
