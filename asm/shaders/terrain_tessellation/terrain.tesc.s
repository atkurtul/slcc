; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 298
; Schema: 0
               OpCapability Tessellation
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint TessellationControl %main "main" %gl_in %gl_InvocationID %inUV %gl_TessLevelInner %gl_TessLevelOuter %gl_out %outNormal %inNormal %outUV
               OpExecutionMode %main OutputVertices 4
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %screenSpaceTessFactor_vf4_vf4_ "screenSpaceTessFactor(vf4;vf4;"
               OpName %p0 "p0"
               OpName %p1 "p1"
               OpName %frustumCheck_ "frustumCheck("
               OpName %midPoint "midPoint"
               OpName %radius "radius"
               OpName %v0 "v0"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lightPos"
               OpMemberName %UBO 3 "frustumPlanes"
               OpMemberName %UBO 4 "displacementFactor"
               OpMemberName %UBO 5 "tessellationFactor"
               OpMemberName %UBO 6 "viewportDim"
               OpMemberName %UBO 7 "tessellatedEdgeSize"
               OpName %ubo "ubo"
               OpName %clip0 "clip0"
               OpName %clip1 "clip1"
               OpName %pos "pos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %gl_in "gl_in"
               OpName %gl_InvocationID "gl_InvocationID"
               OpName %samplerHeight "samplerHeight"
               OpName %inUV "inUV"
               OpName %i "i"
               OpName %gl_TessLevelInner "gl_TessLevelInner"
               OpName %gl_TessLevelOuter "gl_TessLevelOuter"
               OpName %param "param"
               OpName %param_0 "param"
               OpName %param_1 "param"
               OpName %param_2 "param"
               OpName %param_3 "param"
               OpName %param_4 "param"
               OpName %param_5 "param"
               OpName %param_6 "param"
               OpName %gl_PerVertex_0 "gl_PerVertex"
               OpMemberName %gl_PerVertex_0 0 "gl_Position"
               OpMemberName %gl_PerVertex_0 1 "gl_PointSize"
               OpMemberName %gl_PerVertex_0 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex_0 3 "gl_CullDistance"
               OpName %gl_out "gl_out"
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %outUV "outUV"
               OpDecorate %_arr_v4float_uint_6 ArrayStride 16
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 144
               OpMemberDecorate %UBO 4 Offset 240
               OpMemberDecorate %UBO 5 Offset 244
               OpMemberDecorate %UBO 6 Offset 248
               OpMemberDecorate %UBO 7 Offset 256
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %gl_InvocationID BuiltIn InvocationId
               OpDecorate %samplerHeight DescriptorSet 0
               OpDecorate %samplerHeight Binding 1
               OpDecorate %inUV Location 1
               OpDecorate %gl_TessLevelInner Patch
               OpDecorate %gl_TessLevelInner BuiltIn TessLevelInner
               OpDecorate %gl_TessLevelOuter Patch
               OpDecorate %gl_TessLevelOuter BuiltIn TessLevelOuter
               OpMemberDecorate %gl_PerVertex_0 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex_0 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex_0 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex_0 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex_0 Block
               OpDecorate %outNormal Location 0
               OpDecorate %inNormal Location 0
               OpDecorate %outUV Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
          %9 = OpTypeFunction %float %_ptr_Function_v4float %_ptr_Function_v4float
       %bool = OpTypeBool
         %15 = OpTypeFunction %bool
  %float_0_5 = OpConstant %float 0.5
%_ptr_Function_float = OpTypePointer Function %float
    %float_2 = OpConstant %float 2
%mat4v4float = OpTypeMatrix %v4float 4
       %uint = OpTypeInt 32 0
     %uint_6 = OpConstant %uint 6
%_arr_v4float_uint_6 = OpTypeArray %v4float %uint_6
    %v2float = OpTypeVector %float 2
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v4float %_arr_v4float_uint_6 %float %float %v2float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
        %int = OpTypeInt 32 1
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
      %int_0 = OpConstant %int 0
    %v3float = OpTypeVector %float 3
    %float_0 = OpConstant %float 0
         %55 = OpConstantComposite %v3float %float_0 %float_0 %float_0
     %uint_3 = OpConstant %uint 3
      %int_6 = OpConstant %int 6
%_ptr_Uniform_v2float = OpTypePointer Uniform %v2float
     %uint_0 = OpConstant %uint 0
     %uint_1 = OpConstant %uint 1
      %int_7 = OpConstant %int 7
%_ptr_Uniform_float = OpTypePointer Uniform %float
      %int_5 = OpConstant %int 5
    %float_1 = OpConstant %float 1
   %float_64 = OpConstant %float 64
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
    %uint_32 = OpConstant %uint 32
%_arr_gl_PerVertex_uint_32 = OpTypeArray %gl_PerVertex %uint_32
%_ptr_Input__arr_gl_PerVertex_uint_32 = OpTypePointer Input %_arr_gl_PerVertex_uint_32
      %gl_in = OpVariable %_ptr_Input__arr_gl_PerVertex_uint_32 Input
%_ptr_Input_int = OpTypePointer Input %int
%gl_InvocationID = OpVariable %_ptr_Input_int Input
%_ptr_Input_v4float = OpTypePointer Input %v4float
        %136 = OpTypeImage %float 2D 0 0 0 1 Unknown
        %137 = OpTypeSampledImage %136
%_ptr_UniformConstant_137 = OpTypePointer UniformConstant %137
%samplerHeight = OpVariable %_ptr_UniformConstant_137 UniformConstant
%_arr_v2float_uint_32 = OpTypeArray %v2float %uint_32
%_ptr_Input__arr_v2float_uint_32 = OpTypePointer Input %_arr_v2float_uint_32
       %inUV = OpVariable %_ptr_Input__arr_v2float_uint_32 Input
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %int_4 = OpConstant %int 4
%_ptr_Function_int = OpTypePointer Function %int
      %int_3 = OpConstant %int 3
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
    %float_8 = OpConstant %float 8
      %false = OpConstantFalse %bool
       %true = OpConstantTrue %bool
     %uint_2 = OpConstant %uint 2
%_arr_float_uint_2 = OpTypeArray %float %uint_2
%_ptr_Output__arr_float_uint_2 = OpTypePointer Output %_arr_float_uint_2
%gl_TessLevelInner = OpVariable %_ptr_Output__arr_float_uint_2 Output
%_ptr_Output_float = OpTypePointer Output %float
     %uint_4 = OpConstant %uint 4
%_arr_float_uint_4 = OpTypeArray %float %uint_4
%_ptr_Output__arr_float_uint_4 = OpTypePointer Output %_arr_float_uint_4
%gl_TessLevelOuter = OpVariable %_ptr_Output__arr_float_uint_4 Output
      %int_2 = OpConstant %int 2
%gl_PerVertex_0 = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_arr_gl_PerVertex_0_uint_4 = OpTypeArray %gl_PerVertex_0 %uint_4
%_ptr_Output__arr_gl_PerVertex_0_uint_4 = OpTypePointer Output %_arr_gl_PerVertex_0_uint_4
     %gl_out = OpVariable %_ptr_Output__arr_gl_PerVertex_0_uint_4 Output
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_arr_v3float_uint_4 = OpTypeArray %v3float %uint_4
%_ptr_Output__arr_v3float_uint_4 = OpTypePointer Output %_arr_v3float_uint_4
  %outNormal = OpVariable %_ptr_Output__arr_v3float_uint_4 Output
%_arr_v3float_uint_32 = OpTypeArray %v3float %uint_32
%_ptr_Input__arr_v3float_uint_32 = OpTypePointer Input %_arr_v3float_uint_32
   %inNormal = OpVariable %_ptr_Input__arr_v3float_uint_32 Input
%_ptr_Input_v3float = OpTypePointer Input %v3float
%_ptr_Output_v3float = OpTypePointer Output %v3float
%_arr_v2float_uint_4 = OpTypeArray %v2float %uint_4
%_ptr_Output__arr_v2float_uint_4 = OpTypePointer Output %_arr_v2float_uint_4
      %outUV = OpVariable %_ptr_Output__arr_v2float_uint_4 Output
%_ptr_Output_v2float = OpTypePointer Output %v2float
       %main = OpFunction %void None %3
          %5 = OpLabel
      %param = OpVariable %_ptr_Function_v4float Function
    %param_0 = OpVariable %_ptr_Function_v4float Function
    %param_1 = OpVariable %_ptr_Function_v4float Function
    %param_2 = OpVariable %_ptr_Function_v4float Function
    %param_3 = OpVariable %_ptr_Function_v4float Function
    %param_4 = OpVariable %_ptr_Function_v4float Function
    %param_5 = OpVariable %_ptr_Function_v4float Function
    %param_6 = OpVariable %_ptr_Function_v4float Function
        %185 = OpLoad %int %gl_InvocationID
        %186 = OpIEqual %bool %185 %int_0
               OpSelectionMerge %188 None
               OpBranchConditional %186 %187 %188
        %187 = OpLabel
        %189 = OpFunctionCall %bool %frustumCheck_
        %190 = OpLogicalNot %bool %189
               OpSelectionMerge %192 None
               OpBranchConditional %190 %191 %209
        %191 = OpLabel
        %198 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_0
               OpStore %198 %float_0
        %199 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_1
               OpStore %199 %float_0
        %204 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_0
               OpStore %204 %float_0
        %205 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_1
               OpStore %205 %float_0
        %207 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_2
               OpStore %207 %float_0
        %208 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_3
               OpStore %208 %float_0
               OpBranch %192
        %209 = OpLabel
        %210 = OpAccessChain %_ptr_Uniform_float %ubo %int_5
        %211 = OpLoad %float %210
        %212 = OpFOrdGreaterThan %bool %211 %float_0
               OpSelectionMerge %214 None
               OpBranchConditional %212 %213 %259
        %213 = OpLabel
        %216 = OpAccessChain %_ptr_Input_v4float %gl_in %int_3 %int_0
        %217 = OpLoad %v4float %216
               OpStore %param %217
        %219 = OpAccessChain %_ptr_Input_v4float %gl_in %int_0 %int_0
        %220 = OpLoad %v4float %219
               OpStore %param_0 %220
        %221 = OpFunctionCall %float %screenSpaceTessFactor_vf4_vf4_ %param %param_0
        %222 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_0
               OpStore %222 %221
        %224 = OpAccessChain %_ptr_Input_v4float %gl_in %int_0 %int_0
        %225 = OpLoad %v4float %224
               OpStore %param_1 %225
        %227 = OpAccessChain %_ptr_Input_v4float %gl_in %int_1 %int_0
        %228 = OpLoad %v4float %227
               OpStore %param_2 %228
        %229 = OpFunctionCall %float %screenSpaceTessFactor_vf4_vf4_ %param_1 %param_2
        %230 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_1
               OpStore %230 %229
        %232 = OpAccessChain %_ptr_Input_v4float %gl_in %int_1 %int_0
        %233 = OpLoad %v4float %232
               OpStore %param_3 %233
        %235 = OpAccessChain %_ptr_Input_v4float %gl_in %int_2 %int_0
        %236 = OpLoad %v4float %235
               OpStore %param_4 %236
        %237 = OpFunctionCall %float %screenSpaceTessFactor_vf4_vf4_ %param_3 %param_4
        %238 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_2
               OpStore %238 %237
        %240 = OpAccessChain %_ptr_Input_v4float %gl_in %int_2 %int_0
        %241 = OpLoad %v4float %240
               OpStore %param_5 %241
        %243 = OpAccessChain %_ptr_Input_v4float %gl_in %int_3 %int_0
        %244 = OpLoad %v4float %243
               OpStore %param_6 %244
        %245 = OpFunctionCall %float %screenSpaceTessFactor_vf4_vf4_ %param_5 %param_6
        %246 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_3
               OpStore %246 %245
        %247 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_0
        %248 = OpLoad %float %247
        %249 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_3
        %250 = OpLoad %float %249
        %251 = OpExtInst %float %1 FMix %248 %250 %float_0_5
        %252 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_0
               OpStore %252 %251
        %253 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_2
        %254 = OpLoad %float %253
        %255 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_1
        %256 = OpLoad %float %255
        %257 = OpExtInst %float %1 FMix %254 %256 %float_0_5
        %258 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_1
               OpStore %258 %257
               OpBranch %214
        %259 = OpLabel
        %260 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_0
               OpStore %260 %float_1
        %261 = OpAccessChain %_ptr_Output_float %gl_TessLevelInner %int_1
               OpStore %261 %float_1
        %262 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_0
               OpStore %262 %float_1
        %263 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_1
               OpStore %263 %float_1
        %264 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_2
               OpStore %264 %float_1
        %265 = OpAccessChain %_ptr_Output_float %gl_TessLevelOuter %int_3
               OpStore %265 %float_1
               OpBranch %214
        %214 = OpLabel
               OpBranch %192
        %192 = OpLabel
               OpBranch %188
        %188 = OpLabel
        %270 = OpLoad %int %gl_InvocationID
        %271 = OpLoad %int %gl_InvocationID
        %272 = OpAccessChain %_ptr_Input_v4float %gl_in %271 %int_0
        %273 = OpLoad %v4float %272
        %275 = OpAccessChain %_ptr_Output_v4float %gl_out %270 %int_0
               OpStore %275 %273
        %279 = OpLoad %int %gl_InvocationID
        %283 = OpLoad %int %gl_InvocationID
        %285 = OpAccessChain %_ptr_Input_v3float %inNormal %283
        %286 = OpLoad %v3float %285
        %288 = OpAccessChain %_ptr_Output_v3float %outNormal %279
               OpStore %288 %286
        %292 = OpLoad %int %gl_InvocationID
        %293 = OpLoad %int %gl_InvocationID
        %294 = OpAccessChain %_ptr_Input_v2float %inUV %293
        %295 = OpLoad %v2float %294
        %297 = OpAccessChain %_ptr_Output_v2float %outUV %292
               OpStore %297 %295
               OpReturn
               OpFunctionEnd
%screenSpaceTessFactor_vf4_vf4_ = OpFunction %float None %9
         %p0 = OpFunctionParameter %_ptr_Function_v4float
         %p1 = OpFunctionParameter %_ptr_Function_v4float
         %13 = OpLabel
   %midPoint = OpVariable %_ptr_Function_v4float Function
     %radius = OpVariable %_ptr_Function_float Function
         %v0 = OpVariable %_ptr_Function_v4float Function
      %clip0 = OpVariable %_ptr_Function_v4float Function
      %clip1 = OpVariable %_ptr_Function_v4float Function
         %20 = OpLoad %v4float %p0
         %21 = OpLoad %v4float %p1
         %22 = OpFAdd %v4float %20 %21
         %23 = OpVectorTimesScalar %v4float %22 %float_0_5
               OpStore %midPoint %23
         %26 = OpLoad %v4float %p0
         %27 = OpLoad %v4float %p1
         %28 = OpExtInst %float %1 Distance %26 %27
         %30 = OpFDiv %float %28 %float_2
               OpStore %radius %30
         %43 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %44 = OpLoad %mat4v4float %43
         %45 = OpLoad %v4float %midPoint
         %46 = OpMatrixTimesVector %v4float %44 %45
               OpStore %v0 %46
         %49 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %50 = OpLoad %mat4v4float %49
         %51 = OpLoad %v4float %v0
         %52 = OpLoad %float %radius
         %56 = OpCompositeExtract %float %55 0
         %57 = OpCompositeExtract %float %55 1
         %58 = OpCompositeExtract %float %55 2
         %59 = OpCompositeConstruct %v4float %52 %56 %57 %58
         %60 = OpFSub %v4float %51 %59
         %61 = OpMatrixTimesVector %v4float %50 %60
               OpStore %clip0 %61
         %63 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %64 = OpLoad %mat4v4float %63
         %65 = OpLoad %v4float %v0
         %66 = OpLoad %float %radius
         %67 = OpCompositeExtract %float %55 0
         %68 = OpCompositeExtract %float %55 1
         %69 = OpCompositeExtract %float %55 2
         %70 = OpCompositeConstruct %v4float %66 %67 %68 %69
         %71 = OpFAdd %v4float %65 %70
         %72 = OpMatrixTimesVector %v4float %64 %71
               OpStore %clip1 %72
         %74 = OpAccessChain %_ptr_Function_float %clip0 %uint_3
         %75 = OpLoad %float %74
         %76 = OpLoad %v4float %clip0
         %77 = OpCompositeConstruct %v4float %75 %75 %75 %75
         %78 = OpFDiv %v4float %76 %77
               OpStore %clip0 %78
         %79 = OpAccessChain %_ptr_Function_float %clip1 %uint_3
         %80 = OpLoad %float %79
         %81 = OpLoad %v4float %clip1
         %82 = OpCompositeConstruct %v4float %80 %80 %80 %80
         %83 = OpFDiv %v4float %81 %82
               OpStore %clip1 %83
         %86 = OpAccessChain %_ptr_Uniform_v2float %ubo %int_6
         %87 = OpLoad %v2float %86
         %88 = OpLoad %v4float %clip0
         %89 = OpVectorShuffle %v2float %88 %88 0 1
         %90 = OpFMul %v2float %89 %87
         %92 = OpAccessChain %_ptr_Function_float %clip0 %uint_0
         %93 = OpCompositeExtract %float %90 0
               OpStore %92 %93
         %95 = OpAccessChain %_ptr_Function_float %clip0 %uint_1
         %96 = OpCompositeExtract %float %90 1
               OpStore %95 %96
         %97 = OpAccessChain %_ptr_Uniform_v2float %ubo %int_6
         %98 = OpLoad %v2float %97
         %99 = OpLoad %v4float %clip1
        %100 = OpVectorShuffle %v2float %99 %99 0 1
        %101 = OpFMul %v2float %100 %98
        %102 = OpAccessChain %_ptr_Function_float %clip1 %uint_0
        %103 = OpCompositeExtract %float %101 0
               OpStore %102 %103
        %104 = OpAccessChain %_ptr_Function_float %clip1 %uint_1
        %105 = OpCompositeExtract %float %101 1
               OpStore %104 %105
        %106 = OpLoad %v4float %clip0
        %107 = OpLoad %v4float %clip1
        %108 = OpExtInst %float %1 Distance %106 %107
        %111 = OpAccessChain %_ptr_Uniform_float %ubo %int_7
        %112 = OpLoad %float %111
        %113 = OpFDiv %float %108 %112
        %115 = OpAccessChain %_ptr_Uniform_float %ubo %int_5
        %116 = OpLoad %float %115
        %117 = OpFMul %float %113 %116
        %120 = OpExtInst %float %1 FClamp %117 %float_1 %float_64
               OpReturnValue %120
               OpFunctionEnd
%frustumCheck_ = OpFunction %bool None %15
         %17 = OpLabel
        %pos = OpVariable %_ptr_Function_v4float Function
          %i = OpVariable %_ptr_Function_int Function
        %132 = OpLoad %int %gl_InvocationID
        %134 = OpAccessChain %_ptr_Input_v4float %gl_in %132 %int_0
        %135 = OpLoad %v4float %134
               OpStore %pos %135
        %140 = OpLoad %137 %samplerHeight
        %145 = OpAccessChain %_ptr_Input_v2float %inUV %int_0
        %146 = OpLoad %v2float %145
        %147 = OpImageSampleExplicitLod %v4float %140 %146 Lod %float_0
        %148 = OpCompositeExtract %float %147 0
        %150 = OpAccessChain %_ptr_Uniform_float %ubo %int_4
        %151 = OpLoad %float %150
        %152 = OpFMul %float %148 %151
        %153 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %154 = OpLoad %float %153
        %155 = OpFSub %float %154 %152
        %156 = OpAccessChain %_ptr_Function_float %pos %uint_1
               OpStore %156 %155
               OpStore %i %int_0
               OpBranch %159
        %159 = OpLabel
               OpLoopMerge %161 %162 None
               OpBranch %163
        %163 = OpLabel
        %164 = OpLoad %int %i
        %165 = OpSLessThan %bool %164 %int_6
               OpBranchConditional %165 %160 %161
        %160 = OpLabel
        %166 = OpLoad %v4float %pos
        %168 = OpLoad %int %i
        %170 = OpAccessChain %_ptr_Uniform_v4float %ubo %int_3 %168
        %171 = OpLoad %v4float %170
        %172 = OpDot %float %166 %171
        %174 = OpFAdd %float %172 %float_8
        %175 = OpFOrdLessThan %bool %174 %float_0
               OpSelectionMerge %177 None
               OpBranchConditional %175 %176 %177
        %176 = OpLabel
               OpReturnValue %false
        %177 = OpLabel
               OpBranch %162
        %162 = OpLabel
        %180 = OpLoad %int %i
        %181 = OpIAdd %int %180 %int_1
               OpStore %i %181
               OpBranch %159
        %161 = OpLabel
               OpReturnValue %true
               OpFunctionEnd
