; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 249
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %17 %34 %44 %49 %66 %191 %227 %248
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %7 "type"
               OpName %15 "normal"
               OpName %17 "inUVW"
               OpName %22 "color"
               OpName %26 "samplerEnvMap"
               OpName %31 "wViewVec"
               OpName %34 "inInvModelView"
               OpName %44 "inViewVec"
               OpName %48 "normal"
               OpName %49 "inNormal"
               OpName %52 "wNormal"
               OpName %64 "NdotL"
               OpName %66 "inLightVec"
               OpName %71 "eyeDir"
               OpName %74 "halfVec"
               OpName %79 "NdotH"
               OpName %84 "NdotV"
               OpName %89 "VdotH"
               OpName %94 "NH2"
               OpName %98 "g1"
               OpName %104 "g2"
               OpName %110 "geoAtt"
               OpName %116 "fresnel"
               OpName %127 "spec"
               OpName %158 "wViewVec"
               OpName %170 "wNormal"
               OpName %191 "outColor0"
               OpName %196 "UBO"
               OpMemberName %196 0 "exposure"
               OpName %198 "ubo"
               OpName %217 "l"
               OpName %225 "threshold"
               OpName %227 "outColor1"
               OpName %248 "inPos"
               OpDecorate %7 SpecId 0
               OpDecorate %17 Location 0
               OpDecorate %26 DescriptorSet 0
               OpDecorate %26 Binding 1
               OpDecorate %34 Location 5
               OpDecorate %44 Location 3
               OpDecorate %49 Location 2
               OpDecorate %66 Location 4
               OpDecorate %191 Location 0
               OpMemberDecorate %196 0 Offset 0
               OpDecorate %196 Block
               OpDecorate %198 DescriptorSet 0
               OpDecorate %198 Binding 2
               OpDecorate %227 Location 1
               OpDecorate %248 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 1
          %7 = OpSpecConstant %6 0
         %12 = OpTypeFloat 32
         %13 = OpTypeVector %12 3
         %14 = OpTypePointer Function %13
         %16 = OpTypePointer Input %13
         %17 = OpVariable %16 Input
         %20 = OpTypeVector %12 4
         %21 = OpTypePointer Function %20
         %23 = OpTypeImage %12 Cube 0 0 0 1 Unknown
         %24 = OpTypeSampledImage %23
         %25 = OpTypePointer UniformConstant %24
         %26 = OpVariable %25 UniformConstant
         %32 = OpTypeMatrix %20 4
         %33 = OpTypePointer Input %32
         %34 = OpVariable %33 Input
         %36 = OpTypeMatrix %13 3
         %44 = OpVariable %16 Input
         %49 = OpVariable %16 Input
         %63 = OpTypePointer Function %12
         %66 = OpVariable %16 Input
         %69 = OpConstant %12 0
         %95 = OpConstant %12 2
        %111 = OpConstant %12 1
        %119 = OpConstant %12 5
        %121 = OpConstant %12 0.400000006
        %124 = OpConstant %12 0.600000024
        %134 = OpConstant %12 3.1400001
        %147 = OpConstant %12 0.200000003
        %149 = OpConstant %12 0.800000012
        %185 = OpConstant %12 0.625
        %190 = OpTypePointer Output %20
        %191 = OpVariable %190 Output
        %192 = OpConstantComposite %13 %111 %111 %111
        %196 = OpTypeStruct %12
        %197 = OpTypePointer Uniform %196
        %198 = OpVariable %197 Uniform
        %199 = OpConstant %6 0
        %200 = OpTypePointer Uniform %12
        %206 = OpTypeInt 32 0
        %207 = OpConstant %206 0
        %208 = OpTypePointer Output %12
        %211 = OpConstant %206 1
        %214 = OpConstant %206 2
        %220 = OpConstant %12 0.212599993
        %221 = OpConstant %12 0.715200007
        %222 = OpConstant %12 0.0722000003
        %223 = OpConstantComposite %13 %220 %221 %222
        %226 = OpConstant %12 0.75
        %227 = OpVariable %190 Output
        %230 = OpTypeBool
        %238 = OpConstantComposite %13 %69 %69 %69
        %246 = OpConstant %206 3
        %248 = OpVariable %16 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %15 = OpVariable %14 Function
         %22 = OpVariable %21 Function
         %31 = OpVariable %14 Function
         %48 = OpVariable %14 Function
         %52 = OpVariable %14 Function
         %64 = OpVariable %63 Function
         %71 = OpVariable %14 Function
         %74 = OpVariable %14 Function
         %79 = OpVariable %63 Function
         %84 = OpVariable %63 Function
         %89 = OpVariable %63 Function
         %94 = OpVariable %63 Function
         %98 = OpVariable %63 Function
        %104 = OpVariable %63 Function
        %110 = OpVariable %63 Function
        %116 = OpVariable %63 Function
        %127 = OpVariable %63 Function
        %158 = OpVariable %14 Function
        %170 = OpVariable %14 Function
        %217 = OpVariable %63 Function
        %225 = OpVariable %63 Function
        %232 = OpVariable %14 Function
               OpSelectionMerge %11 None
               OpSwitch %7 %11 0 %8 1 %9 2 %10
          %8 = OpLabel
         %18 = OpLoad %13 %17
         %19 = OpExtInst %13 %1 Normalize %18
               OpStore %15 %19
         %27 = OpLoad %24 %26
         %28 = OpLoad %13 %15
         %29 = OpImageSampleImplicitLod %20 %27 %28
               OpStore %22 %29
               OpBranch %11
          %9 = OpLabel
         %35 = OpLoad %32 %34
         %37 = OpCompositeExtract %20 %35 0
         %38 = OpVectorShuffle %13 %37 %37 0 1 2
         %39 = OpCompositeExtract %20 %35 1
         %40 = OpVectorShuffle %13 %39 %39 0 1 2
         %41 = OpCompositeExtract %20 %35 2
         %42 = OpVectorShuffle %13 %41 %41 0 1 2
         %43 = OpCompositeConstruct %36 %38 %40 %42
         %45 = OpLoad %13 %44
         %46 = OpExtInst %13 %1 Normalize %45
         %47 = OpMatrixTimesVector %13 %43 %46
               OpStore %31 %47
         %50 = OpLoad %13 %49
         %51 = OpExtInst %13 %1 Normalize %50
               OpStore %48 %51
         %53 = OpLoad %32 %34
         %54 = OpCompositeExtract %20 %53 0
         %55 = OpVectorShuffle %13 %54 %54 0 1 2
         %56 = OpCompositeExtract %20 %53 1
         %57 = OpVectorShuffle %13 %56 %56 0 1 2
         %58 = OpCompositeExtract %20 %53 2
         %59 = OpVectorShuffle %13 %58 %58 0 1 2
         %60 = OpCompositeConstruct %36 %55 %57 %59
         %61 = OpLoad %13 %48
         %62 = OpMatrixTimesVector %13 %60 %61
               OpStore %52 %62
         %65 = OpLoad %13 %48
         %67 = OpLoad %13 %66
         %68 = OpDot %12 %65 %67
         %70 = OpExtInst %12 %1 FMax %68 %69
               OpStore %64 %70
         %72 = OpLoad %13 %44
         %73 = OpExtInst %13 %1 Normalize %72
               OpStore %71 %73
         %75 = OpLoad %13 %66
         %76 = OpLoad %13 %71
         %77 = OpFAdd %13 %75 %76
         %78 = OpExtInst %13 %1 Normalize %77
               OpStore %74 %78
         %80 = OpLoad %13 %48
         %81 = OpLoad %13 %74
         %82 = OpDot %12 %80 %81
         %83 = OpExtInst %12 %1 FMax %82 %69
               OpStore %79 %83
         %85 = OpLoad %13 %48
         %86 = OpLoad %13 %71
         %87 = OpDot %12 %85 %86
         %88 = OpExtInst %12 %1 FMax %87 %69
               OpStore %84 %88
         %90 = OpLoad %13 %71
         %91 = OpLoad %13 %74
         %92 = OpDot %12 %90 %91
         %93 = OpExtInst %12 %1 FMax %92 %69
               OpStore %89 %93
         %96 = OpLoad %12 %79
         %97 = OpFMul %12 %95 %96
               OpStore %94 %97
         %99 = OpLoad %12 %94
        %100 = OpLoad %12 %84
        %101 = OpFMul %12 %99 %100
        %102 = OpLoad %12 %89
        %103 = OpFDiv %12 %101 %102
               OpStore %98 %103
        %105 = OpLoad %12 %94
        %106 = OpLoad %12 %64
        %107 = OpFMul %12 %105 %106
        %108 = OpLoad %12 %89
        %109 = OpFDiv %12 %107 %108
               OpStore %104 %109
        %112 = OpLoad %12 %98
        %113 = OpLoad %12 %104
        %114 = OpExtInst %12 %1 FMin %112 %113
        %115 = OpExtInst %12 %1 FMin %111 %114
               OpStore %110 %115
        %117 = OpLoad %12 %89
        %118 = OpFSub %12 %111 %117
        %120 = OpExtInst %12 %1 Pow %118 %119
               OpStore %116 %120
        %122 = OpLoad %12 %116
        %123 = OpFMul %12 %122 %121
               OpStore %116 %123
        %125 = OpLoad %12 %116
        %126 = OpFAdd %12 %125 %124
               OpStore %116 %126
        %128 = OpLoad %12 %116
        %129 = OpLoad %12 %110
        %130 = OpFMul %12 %128 %129
        %131 = OpLoad %12 %84
        %132 = OpLoad %12 %64
        %133 = OpFMul %12 %131 %132
        %135 = OpFMul %12 %133 %134
        %136 = OpFDiv %12 %130 %135
               OpStore %127 %136
        %137 = OpLoad %24 %26
        %138 = OpLoad %13 %31
        %139 = OpFNegate %13 %138
        %140 = OpLoad %13 %52
        %141 = OpExtInst %13 %1 Reflect %139 %140
        %142 = OpImageSampleImplicitLod %20 %137 %141
               OpStore %22 %142
        %143 = OpLoad %20 %22
        %144 = OpVectorShuffle %13 %143 %143 0 1 2
        %145 = OpLoad %12 %64
        %146 = OpVectorTimesScalar %13 %144 %145
        %148 = OpLoad %12 %127
        %150 = OpFMul %12 %148 %149
        %151 = OpFAdd %12 %147 %150
        %152 = OpVectorTimesScalar %13 %146 %151
        %153 = OpCompositeExtract %12 %152 0
        %154 = OpCompositeExtract %12 %152 1
        %155 = OpCompositeExtract %12 %152 2
        %156 = OpCompositeConstruct %20 %153 %154 %155 %111
               OpStore %22 %156
               OpBranch %11
         %10 = OpLabel
        %159 = OpLoad %32 %34
        %160 = OpCompositeExtract %20 %159 0
        %161 = OpVectorShuffle %13 %160 %160 0 1 2
        %162 = OpCompositeExtract %20 %159 1
        %163 = OpVectorShuffle %13 %162 %162 0 1 2
        %164 = OpCompositeExtract %20 %159 2
        %165 = OpVectorShuffle %13 %164 %164 0 1 2
        %166 = OpCompositeConstruct %36 %161 %163 %165
        %167 = OpLoad %13 %44
        %168 = OpExtInst %13 %1 Normalize %167
        %169 = OpMatrixTimesVector %13 %166 %168
               OpStore %158 %169
        %171 = OpLoad %32 %34
        %172 = OpCompositeExtract %20 %171 0
        %173 = OpVectorShuffle %13 %172 %172 0 1 2
        %174 = OpCompositeExtract %20 %171 1
        %175 = OpVectorShuffle %13 %174 %174 0 1 2
        %176 = OpCompositeExtract %20 %171 2
        %177 = OpVectorShuffle %13 %176 %176 0 1 2
        %178 = OpCompositeConstruct %36 %173 %175 %177
        %179 = OpLoad %13 %49
        %180 = OpMatrixTimesVector %13 %178 %179
               OpStore %170 %180
        %181 = OpLoad %24 %26
        %182 = OpLoad %13 %158
        %183 = OpFNegate %13 %182
        %184 = OpLoad %13 %170
        %186 = OpExtInst %13 %1 Refract %183 %184 %185
        %187 = OpImageSampleImplicitLod %20 %181 %186
               OpStore %22 %187
               OpBranch %11
         %11 = OpLabel
        %193 = OpLoad %20 %22
        %194 = OpVectorShuffle %13 %193 %193 0 1 2
        %195 = OpFNegate %13 %194
        %201 = OpAccessChain %200 %198 %199
        %202 = OpLoad %12 %201
        %203 = OpVectorTimesScalar %13 %195 %202
        %204 = OpExtInst %13 %1 Exp %203
        %205 = OpFSub %13 %192 %204
        %209 = OpAccessChain %208 %191 %207
        %210 = OpCompositeExtract %12 %205 0
               OpStore %209 %210
        %212 = OpAccessChain %208 %191 %211
        %213 = OpCompositeExtract %12 %205 1
               OpStore %212 %213
        %215 = OpAccessChain %208 %191 %214
        %216 = OpCompositeExtract %12 %205 2
               OpStore %215 %216
        %218 = OpLoad %20 %191
        %219 = OpVectorShuffle %13 %218 %218 0 1 2
        %224 = OpDot %12 %219 %223
               OpStore %217 %224
               OpStore %225 %226
        %228 = OpLoad %12 %217
        %229 = OpLoad %12 %225
        %231 = OpFOrdGreaterThan %230 %228 %229
               OpSelectionMerge %234 None
               OpBranchConditional %231 %233 %237
        %233 = OpLabel
        %235 = OpLoad %20 %191
        %236 = OpVectorShuffle %13 %235 %235 0 1 2
               OpStore %232 %236
               OpBranch %234
        %237 = OpLabel
               OpStore %232 %238
               OpBranch %234
        %234 = OpLabel
        %239 = OpLoad %13 %232
        %240 = OpAccessChain %208 %227 %207
        %241 = OpCompositeExtract %12 %239 0
               OpStore %240 %241
        %242 = OpAccessChain %208 %227 %211
        %243 = OpCompositeExtract %12 %239 1
               OpStore %242 %243
        %244 = OpAccessChain %208 %227 %214
        %245 = OpCompositeExtract %12 %239 2
               OpStore %244 %245
        %247 = OpAccessChain %208 %227 %246
               OpStore %247 %111
               OpReturn
               OpFunctionEnd
