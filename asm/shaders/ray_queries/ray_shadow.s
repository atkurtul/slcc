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
               OpEntryPoint Fragment %4 "main" %214 %216 %244 %254
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 460
               OpSourceExtension "GL_EXT_ray_query"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %12 "calculate_ambient_occlusion(vf3;vf3;"
               OpName %10 "object_point"
               OpName %11 "object_normal"
               OpName %18 "intersects_light(vf3;vf3;"
               OpName %16 "light_origin"
               OpName %17 "pos"
               OpName %22 "max_ao_each"
               OpName %24 "max_ao"
               OpName %29 "accumulated_ao"
               OpName %31 "u"
               OpName %49 "v"
               OpName %53 "accumulated_factor"
               OpName %54 "j"
               OpName %64 "phi"
               OpName %80 "k"
               OpName %89 "theta"
               OpName %100 "x"
               OpName %106 "y"
               OpName %112 "z"
               OpName %115 "direction"
               OpName %129 "query"
               OpName %132 "topLevelAS"
               OpName %141 "dist"
               OpName %152 "ao"
               OpName %155 "factor"
               OpName %189 "direction"
               OpName %193 "distance"
               OpName %198 "query"
               OpName %211 "ao"
               OpName %214 "in_scene_pos"
               OpName %216 "in_normal"
               OpName %217 "param"
               OpName %220 "param"
               OpName %224 "lighting"
               OpName %226 "GlobalUniform"
               OpMemberName %226 0 "model"
               OpMemberName %226 1 "view_proj"
               OpMemberName %226 2 "camera_position"
               OpMemberName %226 3 "light_position"
               OpName %228 "global_uniform"
               OpName %230 "param"
               OpName %234 "param"
               OpName %244 "o_color"
               OpName %254 "in_pos"
               OpDecorate %132 DescriptorSet 0
               OpDecorate %132 Binding 0
               OpDecorate %214 Location 2
               OpDecorate %216 Location 1
               OpMemberDecorate %226 0 ColMajor
               OpMemberDecorate %226 0 Offset 0
               OpMemberDecorate %226 0 MatrixStride 16
               OpMemberDecorate %226 1 ColMajor
               OpMemberDecorate %226 1 Offset 64
               OpMemberDecorate %226 1 MatrixStride 16
               OpMemberDecorate %226 2 Offset 128
               OpMemberDecorate %226 3 Offset 144
               OpDecorate %226 Block
               OpDecorate %228 DescriptorSet 0
               OpDecorate %228 Binding 1
               OpDecorate %244 Location 0
               OpDecorate %254 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Function %7
          %9 = OpTypeFunction %6 %8 %8
         %14 = OpTypeBool
         %15 = OpTypeFunction %14 %8 %8
         %20 = OpTypeInt 32 0
         %21 = OpTypePointer Function %20
         %23 = OpConstant %20 3
         %28 = OpTypePointer Function %6
         %30 = OpConstant %6 0
         %33 = OpConstant %6 1
         %34 = OpConstantComposite %7 %30 %30 %33
         %37 = OpConstant %6 0.899999976
         %43 = OpConstantComposite %7 %33 %30 %30
         %55 = OpConstant %20 0
         %65 = OpConstant %6 0.5
         %66 = OpConstant %6 -3.14159012
         %67 = OpConstant %6 6.28318024
         %69 = OpConstant %20 1
         %73 = OpConstant %20 2
        %127 = OpTypeRayQueryKHR
        %128 = OpTypePointer Private %127
        %129 = OpVariable %128 Private
        %130 = OpTypeAccelerationStructureKHR
        %131 = OpTypePointer UniformConstant %130
        %132 = OpVariable %131 UniformConstant
        %134 = OpConstant %20 4
        %135 = OpConstant %20 255
        %137 = OpConstant %6 0.00999999978
        %139 = OpConstant %6 2
        %142 = OpConstantTrue %14
        %143 = OpTypeInt 32 1
        %144 = OpConstant %143 1
        %149 = OpConstantFalse %14
        %150 = OpConstant %143 0
        %156 = OpConstant %6 0.200000003
        %157 = OpConstant %6 0.800000012
        %198 = OpVariable %128 Private
        %212 = OpTypeVector %6 4
        %213 = OpTypePointer Input %212
        %214 = OpVariable %213 Input
        %215 = OpTypePointer Input %7
        %216 = OpVariable %215 Input
        %223 = OpTypePointer Function %212
        %225 = OpTypeMatrix %212 4
        %226 = OpTypeStruct %225 %225 %7 %7
        %227 = OpTypePointer Uniform %226
        %228 = OpVariable %227 Uniform
        %229 = OpConstant %143 3
        %231 = OpTypePointer Uniform %7
        %238 = OpConstantComposite %212 %156 %156 %156 %33
        %239 = OpConstantComposite %212 %33 %33 %33 %33
        %240 = OpTypeVector %14 4
        %243 = OpTypePointer Output %212
        %244 = OpVariable %243 Output
        %247 = OpConstantComposite %7 %33 %33 %33
        %254 = OpVariable %213 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %211 = OpVariable %28 Function
        %217 = OpVariable %8 Function
        %220 = OpVariable %8 Function
        %224 = OpVariable %223 Function
        %230 = OpVariable %8 Function
        %234 = OpVariable %8 Function
        %218 = OpLoad %212 %214
        %219 = OpVectorShuffle %7 %218 %218 0 1 2
               OpStore %217 %219
        %221 = OpLoad %7 %216
               OpStore %220 %221
        %222 = OpFunctionCall %6 %12 %217 %220
               OpStore %211 %222
        %232 = OpAccessChain %231 %228 %229
        %233 = OpLoad %7 %232
               OpStore %230 %233
        %235 = OpLoad %212 %214
        %236 = OpVectorShuffle %7 %235 %235 0 1 2
               OpStore %234 %236
        %237 = OpFunctionCall %14 %18 %230 %234
        %241 = OpCompositeConstruct %240 %237 %237 %237 %237
        %242 = OpSelect %212 %241 %238 %239
               OpStore %224 %242
        %245 = OpLoad %212 %224
        %246 = OpLoad %6 %211
        %248 = OpVectorTimesScalar %7 %247 %246
        %249 = OpCompositeExtract %6 %248 0
        %250 = OpCompositeExtract %6 %248 1
        %251 = OpCompositeExtract %6 %248 2
        %252 = OpCompositeConstruct %212 %249 %250 %251 %33
        %253 = OpFMul %212 %245 %252
               OpStore %244 %253
               OpReturn
               OpFunctionEnd
         %12 = OpFunction %6 None %9
         %10 = OpFunctionParameter %8
         %11 = OpFunctionParameter %8
         %13 = OpLabel
         %22 = OpVariable %21 Function
         %24 = OpVariable %21 Function
         %29 = OpVariable %28 Function
         %31 = OpVariable %8 Function
         %39 = OpVariable %8 Function
         %49 = OpVariable %8 Function
         %53 = OpVariable %28 Function
         %54 = OpVariable %21 Function
         %64 = OpVariable %28 Function
         %80 = OpVariable %21 Function
         %89 = OpVariable %28 Function
        %100 = OpVariable %28 Function
        %106 = OpVariable %28 Function
        %112 = OpVariable %28 Function
        %115 = OpVariable %8 Function
        %141 = OpVariable %28 Function
        %152 = OpVariable %28 Function
        %155 = OpVariable %28 Function
               OpStore %22 %23
         %25 = OpLoad %20 %22
         %26 = OpLoad %20 %22
         %27 = OpIMul %20 %25 %26
               OpStore %24 %27
               OpStore %29 %30
         %32 = OpLoad %7 %11
         %35 = OpDot %6 %32 %34
         %36 = OpExtInst %6 %1 FAbs %35
         %38 = OpFOrdGreaterThan %14 %36 %37
               OpSelectionMerge %41 None
               OpBranchConditional %38 %40 %45
         %40 = OpLabel
         %42 = OpLoad %7 %11
         %44 = OpExtInst %7 %1 Cross %42 %43
               OpStore %39 %44
               OpBranch %41
         %45 = OpLabel
         %46 = OpLoad %7 %11
         %47 = OpExtInst %7 %1 Cross %46 %34
               OpStore %39 %47
               OpBranch %41
         %41 = OpLabel
         %48 = OpLoad %7 %39
               OpStore %31 %48
         %50 = OpLoad %7 %11
         %51 = OpLoad %7 %31
         %52 = OpExtInst %7 %1 Cross %50 %51
               OpStore %49 %52
               OpStore %53 %30
               OpStore %54 %55
               OpBranch %56
         %56 = OpLabel
               OpLoopMerge %58 %59 None
               OpBranch %60
         %60 = OpLabel
         %61 = OpLoad %20 %54
         %62 = OpLoad %20 %22
         %63 = OpULessThan %14 %61 %62
               OpBranchConditional %63 %57 %58
         %57 = OpLabel
         %68 = OpLoad %20 %54
         %70 = OpIAdd %20 %68 %69
         %71 = OpConvertUToF %6 %70
         %72 = OpLoad %20 %22
         %74 = OpIAdd %20 %72 %73
         %75 = OpConvertUToF %6 %74
         %76 = OpFDiv %6 %71 %75
         %77 = OpFMul %6 %67 %76
         %78 = OpFAdd %6 %66 %77
         %79 = OpFMul %6 %65 %78
               OpStore %64 %79
               OpStore %80 %55
               OpBranch %81
         %81 = OpLabel
               OpLoopMerge %83 %84 None
               OpBranch %85
         %85 = OpLabel
         %86 = OpLoad %20 %80
         %87 = OpLoad %20 %22
         %88 = OpULessThan %14 %86 %87
               OpBranchConditional %88 %82 %83
         %82 = OpLabel
         %90 = OpLoad %20 %80
         %91 = OpIAdd %20 %90 %69
         %92 = OpConvertUToF %6 %91
         %93 = OpLoad %20 %22
         %94 = OpIAdd %20 %93 %73
         %95 = OpConvertUToF %6 %94
         %96 = OpFDiv %6 %92 %95
         %97 = OpFMul %6 %67 %96
         %98 = OpFAdd %6 %66 %97
         %99 = OpFMul %6 %65 %98
               OpStore %89 %99
        %101 = OpLoad %6 %64
        %102 = OpExtInst %6 %1 Cos %101
        %103 = OpLoad %6 %89
        %104 = OpExtInst %6 %1 Sin %103
        %105 = OpFMul %6 %102 %104
               OpStore %100 %105
        %107 = OpLoad %6 %64
        %108 = OpExtInst %6 %1 Sin %107
        %109 = OpLoad %6 %89
        %110 = OpExtInst %6 %1 Sin %109
        %111 = OpFMul %6 %108 %110
               OpStore %106 %111
        %113 = OpLoad %6 %89
        %114 = OpExtInst %6 %1 Cos %113
               OpStore %112 %114
        %116 = OpLoad %6 %100
        %117 = OpLoad %7 %31
        %118 = OpVectorTimesScalar %7 %117 %116
        %119 = OpLoad %6 %106
        %120 = OpLoad %7 %49
        %121 = OpVectorTimesScalar %7 %120 %119
        %122 = OpFAdd %7 %118 %121
        %123 = OpLoad %6 %112
        %124 = OpLoad %7 %11
        %125 = OpVectorTimesScalar %7 %124 %123
        %126 = OpFAdd %7 %122 %125
               OpStore %115 %126
        %133 = OpLoad %130 %132
        %136 = OpLoad %7 %10
        %138 = OpLoad %7 %115
               OpRayQueryInitializeKHR %129 %133 %134 %135 %136 %137 %138 %139
        %140 = OpRayQueryProceedKHR %14 %129
               OpStore %141 %139
        %145 = OpRayQueryGetIntersectionTypeKHR %20 %129 %144
        %146 = OpINotEqual %14 %145 %55
               OpSelectionMerge %148 None
               OpBranchConditional %146 %147 %148
        %147 = OpLabel
        %151 = OpRayQueryGetIntersectionTKHR %6 %129 %150
               OpStore %141 %151
               OpBranch %148
        %148 = OpLabel
        %153 = OpLoad %6 %141
        %154 = OpExtInst %6 %1 FMin %153 %139
               OpStore %152 %154
        %158 = OpLoad %6 %112
        %159 = OpFMul %6 %157 %158
        %160 = OpLoad %6 %112
        %161 = OpFMul %6 %159 %160
        %162 = OpFAdd %6 %156 %161
               OpStore %155 %162
        %163 = OpLoad %6 %155
        %164 = OpLoad %6 %53
        %165 = OpFAdd %6 %164 %163
               OpStore %53 %165
        %166 = OpLoad %6 %152
        %167 = OpLoad %6 %155
        %168 = OpFMul %6 %166 %167
        %169 = OpLoad %6 %29
        %170 = OpFAdd %6 %169 %168
               OpStore %29 %170
               OpBranch %84
         %84 = OpLabel
        %171 = OpLoad %20 %80
        %172 = OpIAdd %20 %171 %144
               OpStore %80 %172
               OpBranch %81
         %83 = OpLabel
               OpBranch %59
         %59 = OpLabel
        %173 = OpLoad %20 %54
        %174 = OpIAdd %20 %173 %144
               OpStore %54 %174
               OpBranch %56
         %58 = OpLabel
        %175 = OpLoad %6 %53
        %176 = OpFMul %6 %139 %175
        %177 = OpLoad %6 %29
        %178 = OpFDiv %6 %177 %176
               OpStore %29 %178
        %179 = OpLoad %6 %29
        %180 = OpLoad %6 %29
        %181 = OpFMul %6 %180 %179
               OpStore %29 %181
        %182 = OpLoad %6 %29
        %183 = OpFMul %6 %182 %33
        %184 = OpExtInst %6 %1 FMin %183 %33
        %185 = OpExtInst %6 %1 FMax %184 %30
               OpStore %29 %185
        %186 = OpLoad %6 %29
               OpReturnValue %186
               OpFunctionEnd
         %18 = OpFunction %14 None %15
         %16 = OpFunctionParameter %8
         %17 = OpFunctionParameter %8
         %19 = OpLabel
        %189 = OpVariable %8 Function
        %193 = OpVariable %28 Function
        %190 = OpLoad %7 %16
        %191 = OpLoad %7 %17
        %192 = OpFSub %7 %190 %191
               OpStore %189 %192
        %194 = OpLoad %7 %189
        %195 = OpLoad %7 %189
        %196 = OpDot %6 %194 %195
        %197 = OpExtInst %6 %1 Sqrt %196
               OpStore %193 %197
        %199 = OpLoad %130 %132
        %200 = OpLoad %7 %17
        %201 = OpLoad %7 %189
        %202 = OpLoad %6 %193
               OpRayQueryInitializeKHR %198 %199 %134 %135 %200 %137 %201 %202
        %203 = OpRayQueryProceedKHR %14 %198
        %204 = OpRayQueryGetIntersectionTypeKHR %20 %198 %144
        %205 = OpINotEqual %14 %204 %55
               OpSelectionMerge %207 None
               OpBranchConditional %205 %206 %207
        %206 = OpLabel
               OpReturnValue %142
        %207 = OpLabel
               OpReturnValue %149
               OpFunctionEnd
