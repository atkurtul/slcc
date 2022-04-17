; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 249
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUVW %inInvModelView %inViewVec %inNormal %inLightVec %outColor0 %outColor1 %inPos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %type "type"
               OpName %normal "normal"
               OpName %inUVW "inUVW"
               OpName %color "color"
               OpName %samplerEnvMap "samplerEnvMap"
               OpName %wViewVec "wViewVec"
               OpName %inInvModelView "inInvModelView"
               OpName %inViewVec "inViewVec"
               OpName %normal_0 "normal"
               OpName %inNormal "inNormal"
               OpName %wNormal "wNormal"
               OpName %NdotL "NdotL"
               OpName %inLightVec "inLightVec"
               OpName %eyeDir "eyeDir"
               OpName %halfVec "halfVec"
               OpName %NdotH "NdotH"
               OpName %NdotV "NdotV"
               OpName %VdotH "VdotH"
               OpName %NH2 "NH2"
               OpName %g1 "g1"
               OpName %g2 "g2"
               OpName %geoAtt "geoAtt"
               OpName %fresnel "fresnel"
               OpName %spec "spec"
               OpName %wViewVec_0 "wViewVec"
               OpName %wNormal_0 "wNormal"
               OpName %outColor0 "outColor0"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "exposure"
               OpName %ubo "ubo"
               OpName %l "l"
               OpName %threshold "threshold"
               OpName %outColor1 "outColor1"
               OpName %inPos "inPos"
               OpDecorate %type SpecId 0
               OpDecorate %inUVW Location 0
               OpDecorate %samplerEnvMap DescriptorSet 0
               OpDecorate %samplerEnvMap Binding 1
               OpDecorate %inInvModelView Location 5
               OpDecorate %inViewVec Location 3
               OpDecorate %inNormal Location 2
               OpDecorate %inLightVec Location 4
               OpDecorate %outColor0 Location 0
               OpMemberDecorate %UBO 0 Offset 0
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 2
               OpDecorate %outColor1 Location 1
               OpDecorate %inPos Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
       %type = OpSpecConstant %int 0
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inUVW = OpVariable %_ptr_Input_v3float Input
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %23 = OpTypeImage %float Cube 0 0 0 1 Unknown
         %24 = OpTypeSampledImage %23
%_ptr_UniformConstant_24 = OpTypePointer UniformConstant %24
%samplerEnvMap = OpVariable %_ptr_UniformConstant_24 UniformConstant
%mat4v4float = OpTypeMatrix %v4float 4
%_ptr_Input_mat4v4float = OpTypePointer Input %mat4v4float
%inInvModelView = OpVariable %_ptr_Input_mat4v4float Input
%mat3v3float = OpTypeMatrix %v3float 3
  %inViewVec = OpVariable %_ptr_Input_v3float Input
   %inNormal = OpVariable %_ptr_Input_v3float Input
%_ptr_Function_float = OpTypePointer Function %float
 %inLightVec = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
    %float_2 = OpConstant %float 2
    %float_1 = OpConstant %float 1
    %float_5 = OpConstant %float 5
%float_0_400000006 = OpConstant %float 0.400000006
%float_0_600000024 = OpConstant %float 0.600000024
%float_3_1400001 = OpConstant %float 3.1400001
%float_0_200000003 = OpConstant %float 0.200000003
%float_0_800000012 = OpConstant %float 0.800000012
%float_0_625 = OpConstant %float 0.625
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %outColor0 = OpVariable %_ptr_Output_v4float Output
        %192 = OpConstantComposite %v3float %float_1 %float_1 %float_1
        %UBO = OpTypeStruct %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_0 = OpConstant %int 0
%_ptr_Uniform_float = OpTypePointer Uniform %float
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Output_float = OpTypePointer Output %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
%float_0_212599993 = OpConstant %float 0.212599993
%float_0_715200007 = OpConstant %float 0.715200007
%float_0_0722000003 = OpConstant %float 0.0722000003
        %223 = OpConstantComposite %v3float %float_0_212599993 %float_0_715200007 %float_0_0722000003
 %float_0_75 = OpConstant %float 0.75
  %outColor1 = OpVariable %_ptr_Output_v4float Output
       %bool = OpTypeBool
        %238 = OpConstantComposite %v3float %float_0 %float_0 %float_0
     %uint_3 = OpConstant %uint 3
      %inPos = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
     %normal = OpVariable %_ptr_Function_v3float Function
      %color = OpVariable %_ptr_Function_v4float Function
   %wViewVec = OpVariable %_ptr_Function_v3float Function
   %normal_0 = OpVariable %_ptr_Function_v3float Function
    %wNormal = OpVariable %_ptr_Function_v3float Function
      %NdotL = OpVariable %_ptr_Function_float Function
     %eyeDir = OpVariable %_ptr_Function_v3float Function
    %halfVec = OpVariable %_ptr_Function_v3float Function
      %NdotH = OpVariable %_ptr_Function_float Function
      %NdotV = OpVariable %_ptr_Function_float Function
      %VdotH = OpVariable %_ptr_Function_float Function
        %NH2 = OpVariable %_ptr_Function_float Function
         %g1 = OpVariable %_ptr_Function_float Function
         %g2 = OpVariable %_ptr_Function_float Function
     %geoAtt = OpVariable %_ptr_Function_float Function
    %fresnel = OpVariable %_ptr_Function_float Function
       %spec = OpVariable %_ptr_Function_float Function
 %wViewVec_0 = OpVariable %_ptr_Function_v3float Function
  %wNormal_0 = OpVariable %_ptr_Function_v3float Function
          %l = OpVariable %_ptr_Function_float Function
  %threshold = OpVariable %_ptr_Function_float Function
        %232 = OpVariable %_ptr_Function_v3float Function
               OpSelectionMerge %11 None
               OpSwitch %type %11 0 %8 1 %9 2 %10
          %8 = OpLabel
         %18 = OpLoad %v3float %inUVW
         %19 = OpExtInst %v3float %1 Normalize %18
               OpStore %normal %19
         %27 = OpLoad %24 %samplerEnvMap
         %28 = OpLoad %v3float %normal
         %29 = OpImageSampleImplicitLod %v4float %27 %28
               OpStore %color %29
               OpBranch %11
          %9 = OpLabel
         %35 = OpLoad %mat4v4float %inInvModelView
         %37 = OpCompositeExtract %v4float %35 0
         %38 = OpVectorShuffle %v3float %37 %37 0 1 2
         %39 = OpCompositeExtract %v4float %35 1
         %40 = OpVectorShuffle %v3float %39 %39 0 1 2
         %41 = OpCompositeExtract %v4float %35 2
         %42 = OpVectorShuffle %v3float %41 %41 0 1 2
         %43 = OpCompositeConstruct %mat3v3float %38 %40 %42
         %45 = OpLoad %v3float %inViewVec
         %46 = OpExtInst %v3float %1 Normalize %45
         %47 = OpMatrixTimesVector %v3float %43 %46
               OpStore %wViewVec %47
         %50 = OpLoad %v3float %inNormal
         %51 = OpExtInst %v3float %1 Normalize %50
               OpStore %normal_0 %51
         %53 = OpLoad %mat4v4float %inInvModelView
         %54 = OpCompositeExtract %v4float %53 0
         %55 = OpVectorShuffle %v3float %54 %54 0 1 2
         %56 = OpCompositeExtract %v4float %53 1
         %57 = OpVectorShuffle %v3float %56 %56 0 1 2
         %58 = OpCompositeExtract %v4float %53 2
         %59 = OpVectorShuffle %v3float %58 %58 0 1 2
         %60 = OpCompositeConstruct %mat3v3float %55 %57 %59
         %61 = OpLoad %v3float %normal_0
         %62 = OpMatrixTimesVector %v3float %60 %61
               OpStore %wNormal %62
         %65 = OpLoad %v3float %normal_0
         %67 = OpLoad %v3float %inLightVec
         %68 = OpDot %float %65 %67
         %70 = OpExtInst %float %1 FMax %68 %float_0
               OpStore %NdotL %70
         %72 = OpLoad %v3float %inViewVec
         %73 = OpExtInst %v3float %1 Normalize %72
               OpStore %eyeDir %73
         %75 = OpLoad %v3float %inLightVec
         %76 = OpLoad %v3float %eyeDir
         %77 = OpFAdd %v3float %75 %76
         %78 = OpExtInst %v3float %1 Normalize %77
               OpStore %halfVec %78
         %80 = OpLoad %v3float %normal_0
         %81 = OpLoad %v3float %halfVec
         %82 = OpDot %float %80 %81
         %83 = OpExtInst %float %1 FMax %82 %float_0
               OpStore %NdotH %83
         %85 = OpLoad %v3float %normal_0
         %86 = OpLoad %v3float %eyeDir
         %87 = OpDot %float %85 %86
         %88 = OpExtInst %float %1 FMax %87 %float_0
               OpStore %NdotV %88
         %90 = OpLoad %v3float %eyeDir
         %91 = OpLoad %v3float %halfVec
         %92 = OpDot %float %90 %91
         %93 = OpExtInst %float %1 FMax %92 %float_0
               OpStore %VdotH %93
         %96 = OpLoad %float %NdotH
         %97 = OpFMul %float %float_2 %96
               OpStore %NH2 %97
         %99 = OpLoad %float %NH2
        %100 = OpLoad %float %NdotV
        %101 = OpFMul %float %99 %100
        %102 = OpLoad %float %VdotH
        %103 = OpFDiv %float %101 %102
               OpStore %g1 %103
        %105 = OpLoad %float %NH2
        %106 = OpLoad %float %NdotL
        %107 = OpFMul %float %105 %106
        %108 = OpLoad %float %VdotH
        %109 = OpFDiv %float %107 %108
               OpStore %g2 %109
        %112 = OpLoad %float %g1
        %113 = OpLoad %float %g2
        %114 = OpExtInst %float %1 FMin %112 %113
        %115 = OpExtInst %float %1 FMin %float_1 %114
               OpStore %geoAtt %115
        %117 = OpLoad %float %VdotH
        %118 = OpFSub %float %float_1 %117
        %120 = OpExtInst %float %1 Pow %118 %float_5
               OpStore %fresnel %120
        %122 = OpLoad %float %fresnel
        %123 = OpFMul %float %122 %float_0_400000006
               OpStore %fresnel %123
        %125 = OpLoad %float %fresnel
        %126 = OpFAdd %float %125 %float_0_600000024
               OpStore %fresnel %126
        %128 = OpLoad %float %fresnel
        %129 = OpLoad %float %geoAtt
        %130 = OpFMul %float %128 %129
        %131 = OpLoad %float %NdotV
        %132 = OpLoad %float %NdotL
        %133 = OpFMul %float %131 %132
        %135 = OpFMul %float %133 %float_3_1400001
        %136 = OpFDiv %float %130 %135
               OpStore %spec %136
        %137 = OpLoad %24 %samplerEnvMap
        %138 = OpLoad %v3float %wViewVec
        %139 = OpFNegate %v3float %138
        %140 = OpLoad %v3float %wNormal
        %141 = OpExtInst %v3float %1 Reflect %139 %140
        %142 = OpImageSampleImplicitLod %v4float %137 %141
               OpStore %color %142
        %143 = OpLoad %v4float %color
        %144 = OpVectorShuffle %v3float %143 %143 0 1 2
        %145 = OpLoad %float %NdotL
        %146 = OpVectorTimesScalar %v3float %144 %145
        %148 = OpLoad %float %spec
        %150 = OpFMul %float %148 %float_0_800000012
        %151 = OpFAdd %float %float_0_200000003 %150
        %152 = OpVectorTimesScalar %v3float %146 %151
        %153 = OpCompositeExtract %float %152 0
        %154 = OpCompositeExtract %float %152 1
        %155 = OpCompositeExtract %float %152 2
        %156 = OpCompositeConstruct %v4float %153 %154 %155 %float_1
               OpStore %color %156
               OpBranch %11
         %10 = OpLabel
        %159 = OpLoad %mat4v4float %inInvModelView
        %160 = OpCompositeExtract %v4float %159 0
        %161 = OpVectorShuffle %v3float %160 %160 0 1 2
        %162 = OpCompositeExtract %v4float %159 1
        %163 = OpVectorShuffle %v3float %162 %162 0 1 2
        %164 = OpCompositeExtract %v4float %159 2
        %165 = OpVectorShuffle %v3float %164 %164 0 1 2
        %166 = OpCompositeConstruct %mat3v3float %161 %163 %165
        %167 = OpLoad %v3float %inViewVec
        %168 = OpExtInst %v3float %1 Normalize %167
        %169 = OpMatrixTimesVector %v3float %166 %168
               OpStore %wViewVec_0 %169
        %171 = OpLoad %mat4v4float %inInvModelView
        %172 = OpCompositeExtract %v4float %171 0
        %173 = OpVectorShuffle %v3float %172 %172 0 1 2
        %174 = OpCompositeExtract %v4float %171 1
        %175 = OpVectorShuffle %v3float %174 %174 0 1 2
        %176 = OpCompositeExtract %v4float %171 2
        %177 = OpVectorShuffle %v3float %176 %176 0 1 2
        %178 = OpCompositeConstruct %mat3v3float %173 %175 %177
        %179 = OpLoad %v3float %inNormal
        %180 = OpMatrixTimesVector %v3float %178 %179
               OpStore %wNormal_0 %180
        %181 = OpLoad %24 %samplerEnvMap
        %182 = OpLoad %v3float %wViewVec_0
        %183 = OpFNegate %v3float %182
        %184 = OpLoad %v3float %wNormal_0
        %186 = OpExtInst %v3float %1 Refract %183 %184 %float_0_625
        %187 = OpImageSampleImplicitLod %v4float %181 %186
               OpStore %color %187
               OpBranch %11
         %11 = OpLabel
        %193 = OpLoad %v4float %color
        %194 = OpVectorShuffle %v3float %193 %193 0 1 2
        %195 = OpFNegate %v3float %194
        %201 = OpAccessChain %_ptr_Uniform_float %ubo %int_0
        %202 = OpLoad %float %201
        %203 = OpVectorTimesScalar %v3float %195 %202
        %204 = OpExtInst %v3float %1 Exp %203
        %205 = OpFSub %v3float %192 %204
        %209 = OpAccessChain %_ptr_Output_float %outColor0 %uint_0
        %210 = OpCompositeExtract %float %205 0
               OpStore %209 %210
        %212 = OpAccessChain %_ptr_Output_float %outColor0 %uint_1
        %213 = OpCompositeExtract %float %205 1
               OpStore %212 %213
        %215 = OpAccessChain %_ptr_Output_float %outColor0 %uint_2
        %216 = OpCompositeExtract %float %205 2
               OpStore %215 %216
        %218 = OpLoad %v4float %outColor0
        %219 = OpVectorShuffle %v3float %218 %218 0 1 2
        %224 = OpDot %float %219 %223
               OpStore %l %224
               OpStore %threshold %float_0_75
        %228 = OpLoad %float %l
        %229 = OpLoad %float %threshold
        %231 = OpFOrdGreaterThan %bool %228 %229
               OpSelectionMerge %234 None
               OpBranchConditional %231 %233 %237
        %233 = OpLabel
        %235 = OpLoad %v4float %outColor0
        %236 = OpVectorShuffle %v3float %235 %235 0 1 2
               OpStore %232 %236
               OpBranch %234
        %237 = OpLabel
               OpStore %232 %238
               OpBranch %234
        %234 = OpLabel
        %239 = OpLoad %v3float %232
        %240 = OpAccessChain %_ptr_Output_float %outColor1 %uint_0
        %241 = OpCompositeExtract %float %239 0
               OpStore %240 %241
        %242 = OpAccessChain %_ptr_Output_float %outColor1 %uint_1
        %243 = OpCompositeExtract %float %239 1
               OpStore %242 %243
        %244 = OpAccessChain %_ptr_Output_float %outColor1 %uint_2
        %245 = OpCompositeExtract %float %239 2
               OpStore %244 %245
        %247 = OpAccessChain %_ptr_Output_float %outColor1 %uint_3
               OpStore %247 %float_1
               OpReturn
               OpFunctionEnd
