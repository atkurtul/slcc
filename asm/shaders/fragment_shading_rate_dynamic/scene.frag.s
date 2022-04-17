; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 234
; Schema: 0
               OpCapability Shader
               OpCapability StorageImageExtendedFormats
               OpCapability FragmentShadingRateKHR
               OpExtension "SPV_KHR_fragment_shading_rate"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %inNormal %inLightVec %inViewVec %gl_ShadingRateEXT %outColor %gl_FragCoord %outFrequency %inPos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_fragment_shading_rate"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %freq_x "freq_x"
               OpName %freq_y "freq_y"
               OpName %Push_Constants "Push_Constants"
               OpMemberName %Push_Constants 0 "offset"
               OpMemberName %Push_Constants 1 "object_type"
               OpName %push_constants "push_constants"
               OpName %color "color"
               OpName %samplerEnvMap "samplerEnvMap"
               OpName %inUV "inUV"
               OpName %dx "dx"
               OpName %dy "dy"
               OpName %tex_value "tex_value"
               OpName %samplerSphere "samplerSphere"
               OpName %ambient "ambient"
               OpName %N "N"
               OpName %inNormal "inNormal"
               OpName %L "L"
               OpName %inLightVec "inLightVec"
               OpName %V "V"
               OpName %inViewVec "inViewVec"
               OpName %R "R"
               OpName %diffuse "diffuse"
               OpName %specular "specular"
               OpName %dx_0 "dx"
               OpName %dy_0 "dy"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "skybox_modelview"
               OpMemberName %UBO 3 "color_shading_rates"
               OpName %ubo "ubo"
               OpName %v "v"
               OpName %h "h"
               OpName %gl_ShadingRateEXT "gl_ShadingRateEXT"
               OpName %outColor "outColor"
               OpName %coord "coord"
               OpName %gl_FragCoord "gl_FragCoord"
               OpName %input_frequency "input_frequency"
               OpName %outFrequency "outFrequency"
               OpName %output_sampling_rate "output_sampling_rate"
               OpName %inPos "inPos"
               OpMemberDecorate %Push_Constants 0 Offset 0
               OpMemberDecorate %Push_Constants 1 Offset 16
               OpDecorate %Push_Constants Block
               OpDecorate %samplerEnvMap DescriptorSet 0
               OpDecorate %samplerEnvMap Binding 1
               OpDecorate %inUV Location 0
               OpDecorate %samplerSphere DescriptorSet 0
               OpDecorate %samplerSphere Binding 2
               OpDecorate %inNormal Location 2
               OpDecorate %inLightVec Location 4
               OpDecorate %inViewVec Location 3
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 ColMajor
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 2 MatrixStride 16
               OpMemberDecorate %UBO 3 Offset 192
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %gl_ShadingRateEXT Flat
               OpDecorate %gl_ShadingRateEXT BuiltIn ShadingRateKHR
               OpDecorate %outColor Location 0
               OpDecorate %gl_FragCoord BuiltIn FragCoord
               OpDecorate %input_frequency DescriptorSet 0
               OpDecorate %input_frequency Binding 3
               OpDecorate %input_frequency NonWritable
               OpDecorate %outFrequency Location 1
               OpDecorate %output_sampling_rate DescriptorSet 0
               OpDecorate %output_sampling_rate Binding 4
               OpDecorate %output_sampling_rate NonWritable
               OpDecorate %inPos Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
%_ptr_Function_float = OpTypePointer Function %float
    %float_0 = OpConstant %float 0
    %v4float = OpTypeVector %float 4
        %int = OpTypeInt 32 1
%Push_Constants = OpTypeStruct %v4float %int
%_ptr_PushConstant_Push_Constants = OpTypePointer PushConstant %Push_Constants
%push_constants = OpVariable %_ptr_PushConstant_Push_Constants PushConstant
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_int = OpTypePointer PushConstant %int
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %25 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %26 = OpTypeSampledImage %25
%_ptr_UniformConstant_26 = OpTypePointer UniformConstant %26
%samplerEnvMap = OpVariable %_ptr_UniformConstant_26 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Input_float = OpTypePointer Input %float
    %float_1 = OpConstant %float 1
     %uint_1 = OpConstant %uint 1
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%samplerSphere = OpVariable %_ptr_UniformConstant_26 UniformConstant
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
    %float_8 = OpConstant %float 8
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_3 = OpConstant %int 3
%_ptr_Uniform_int = OpTypePointer Uniform %int
       %bool = OpTypeBool
%_ptr_Function_int = OpTypePointer Function %int
%_ptr_Input_int = OpTypePointer Input %int
%gl_ShadingRateEXT = OpVariable %_ptr_Input_int Input
      %int_2 = OpConstant %int 2
      %int_4 = OpConstant %int 4
      %int_8 = OpConstant %int 8
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %outColor = OpVariable %_ptr_Output_v4float Output
        %174 = OpConstantComposite %v3float %float_1 %float_1 %float_1
   %float_16 = OpConstant %float 16
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
%_ptr_Input_v4float = OpTypePointer Input %v4float
%gl_FragCoord = OpVariable %_ptr_Input_v4float Input
      %v4int = OpTypeVector %int 4
        %204 = OpTypeImage %uint 2D 0 0 0 2 Rg8ui
%_ptr_UniformConstant_204 = OpTypePointer UniformConstant %204
%input_frequency = OpVariable %_ptr_UniformConstant_204 UniformConstant
     %v4uint = OpTypeVector %uint 4
     %v2uint = OpTypeVector %uint 2
%_ptr_Output_v2uint = OpTypePointer Output %v2uint
%outFrequency = OpVariable %_ptr_Output_v2uint Output
  %float_255 = OpConstant %float 255
        %230 = OpTypeImage %uint 2D 0 0 0 2 R8ui
%_ptr_UniformConstant_230 = OpTypePointer UniformConstant %230
%output_sampling_rate = OpVariable %_ptr_UniformConstant_230 UniformConstant
      %inPos = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
     %freq_x = OpVariable %_ptr_Function_float Function
     %freq_y = OpVariable %_ptr_Function_float Function
      %color = OpVariable %_ptr_Function_v4float Function
         %dx = OpVariable %_ptr_Function_v3float Function
         %dy = OpVariable %_ptr_Function_v3float Function
  %tex_value = OpVariable %_ptr_Function_v4float Function
    %ambient = OpVariable %_ptr_Function_v3float Function
          %N = OpVariable %_ptr_Function_v3float Function
          %L = OpVariable %_ptr_Function_v3float Function
          %V = OpVariable %_ptr_Function_v3float Function
          %R = OpVariable %_ptr_Function_v3float Function
    %diffuse = OpVariable %_ptr_Function_v3float Function
   %specular = OpVariable %_ptr_Function_v3float Function
       %dx_0 = OpVariable %_ptr_Function_v3float Function
       %dy_0 = OpVariable %_ptr_Function_v3float Function
          %v = OpVariable %_ptr_Function_int Function
          %h = OpVariable %_ptr_Function_int Function
      %coord = OpVariable %_ptr_Function_v2int Function
               OpStore %freq_x %float_0
               OpStore %freq_y %float_0
         %18 = OpAccessChain %_ptr_PushConstant_int %push_constants %int_1
         %19 = OpLoad %int %18
               OpSelectionMerge %22 None
               OpSwitch %19 %22 0 %20 1 %21
         %20 = OpLabel
         %29 = OpLoad %26 %samplerEnvMap
         %36 = OpAccessChain %_ptr_Input_float %inUV %uint_0
         %37 = OpLoad %float %36
         %40 = OpAccessChain %_ptr_Input_float %inUV %uint_1
         %41 = OpLoad %float %40
         %42 = OpFSub %float %float_1 %41
         %43 = OpCompositeConstruct %v2float %37 %42
         %44 = OpImageSampleImplicitLod %v4float %29 %43
               OpStore %color %44
         %48 = OpLoad %v4float %color
         %49 = OpVectorShuffle %v3float %48 %48 0 1 2
         %50 = OpDPdx %v3float %49
               OpStore %dx %50
         %52 = OpLoad %v4float %color
         %53 = OpVectorShuffle %v3float %52 %52 0 1 2
         %54 = OpDPdx %v3float %53
               OpStore %dy %54
         %55 = OpLoad %v3float %dx
         %56 = OpLoad %v3float %dx
         %57 = OpDot %float %55 %56
               OpStore %freq_x %57
         %58 = OpLoad %v3float %dy
         %59 = OpLoad %v3float %dy
         %60 = OpDot %float %58 %59
               OpStore %freq_y %60
               OpBranch %22
         %21 = OpLabel
         %64 = OpLoad %26 %samplerSphere
         %65 = OpAccessChain %_ptr_Input_float %inUV %uint_0
         %66 = OpLoad %float %65
         %67 = OpAccessChain %_ptr_Input_float %inUV %uint_1
         %68 = OpLoad %float %67
         %69 = OpFSub %float %float_1 %68
         %70 = OpCompositeConstruct %v2float %66 %69
         %71 = OpImageSampleImplicitLod %v4float %64 %70
               OpStore %tex_value %71
         %73 = OpLoad %v4float %tex_value
         %74 = OpVectorShuffle %v3float %73 %73 0 1 2
               OpStore %ambient %74
         %78 = OpLoad %v3float %inNormal
         %79 = OpExtInst %v3float %1 Normalize %78
               OpStore %N %79
         %82 = OpLoad %v3float %inLightVec
         %83 = OpExtInst %v3float %1 Normalize %82
               OpStore %L %83
         %86 = OpLoad %v3float %inViewVec
         %87 = OpExtInst %v3float %1 Normalize %86
               OpStore %V %87
         %89 = OpLoad %v3float %L
         %90 = OpFNegate %v3float %89
         %91 = OpLoad %v3float %N
         %92 = OpExtInst %v3float %1 Reflect %90 %91
               OpStore %R %92
         %94 = OpLoad %v3float %N
         %95 = OpLoad %v3float %L
         %96 = OpDot %float %94 %95
         %97 = OpExtInst %float %1 FMax %96 %float_0
         %98 = OpCompositeConstruct %v3float %97 %97 %97
               OpStore %diffuse %98
        %100 = OpLoad %v3float %R
        %101 = OpLoad %v3float %V
        %102 = OpDot %float %100 %101
        %103 = OpExtInst %float %1 FMax %102 %float_0
        %105 = OpExtInst %float %1 Pow %103 %float_8
        %106 = OpCompositeConstruct %v3float %105 %105 %105
               OpStore %specular %106
        %107 = OpLoad %v3float %ambient
        %108 = OpLoad %v3float %diffuse
        %109 = OpFAdd %v3float %107 %108
        %110 = OpLoad %v3float %specular
        %111 = OpFAdd %v3float %109 %110
        %112 = OpCompositeExtract %float %111 0
        %113 = OpCompositeExtract %float %111 1
        %114 = OpCompositeExtract %float %111 2
        %115 = OpCompositeConstruct %v4float %112 %113 %114 %float_1
               OpStore %color %115
        %117 = OpLoad %v4float %color
        %118 = OpVectorShuffle %v3float %117 %117 0 1 2
        %119 = OpDPdx %v3float %118
               OpStore %dx_0 %119
        %121 = OpLoad %v4float %color
        %122 = OpVectorShuffle %v3float %121 %121 0 1 2
        %123 = OpDPdy %v3float %122
               OpStore %dy_0 %123
        %124 = OpLoad %v3float %dx_0
        %125 = OpLoad %v3float %dx_0
        %126 = OpDot %float %124 %125
               OpStore %freq_x %126
        %127 = OpLoad %v3float %dy_0
        %128 = OpLoad %v3float %dy_0
        %129 = OpDot %float %127 %128
               OpStore %freq_y %129
               OpBranch %22
         %22 = OpLabel
        %138 = OpAccessChain %_ptr_Uniform_int %ubo %int_3
        %139 = OpLoad %int %138
        %141 = OpIEqual %bool %139 %int_1
               OpSelectionMerge %143 None
               OpBranchConditional %141 %142 %187
        %142 = OpLabel
               OpStore %v %int_1
               OpStore %h %int_1
        %149 = OpLoad %int %gl_ShadingRateEXT
        %150 = OpBitwiseAnd %int %149 %int_1
        %151 = OpIEqual %bool %150 %int_1
               OpSelectionMerge %153 None
               OpBranchConditional %151 %152 %153
        %152 = OpLabel
               OpStore %v %int_2
               OpBranch %153
        %153 = OpLabel
        %155 = OpLoad %int %gl_ShadingRateEXT
        %156 = OpBitwiseAnd %int %155 %int_2
        %157 = OpIEqual %bool %156 %int_2
               OpSelectionMerge %159 None
               OpBranchConditional %157 %158 %159
        %158 = OpLabel
               OpStore %v %int_4
               OpBranch %159
        %159 = OpLabel
        %161 = OpLoad %int %gl_ShadingRateEXT
        %162 = OpBitwiseAnd %int %161 %int_4
        %163 = OpIEqual %bool %162 %int_4
               OpSelectionMerge %165 None
               OpBranchConditional %163 %164 %165
        %164 = OpLabel
               OpStore %h %int_2
               OpBranch %165
        %165 = OpLabel
        %166 = OpLoad %int %gl_ShadingRateEXT
        %168 = OpBitwiseAnd %int %166 %int_8
        %169 = OpIEqual %bool %168 %int_8
               OpSelectionMerge %171 None
               OpBranchConditional %169 %170 %171
        %170 = OpLabel
               OpStore %h %int_4
               OpBranch %171
        %171 = OpLabel
        %175 = OpLoad %int %h
        %176 = OpLoad %int %v
        %177 = OpIMul %int %175 %176
        %178 = OpConvertSToF %float %177
        %180 = OpFDiv %float %178 %float_16
        %181 = OpFSub %float %float_1 %180
        %182 = OpVectorTimesScalar %v3float %174 %181
        %183 = OpCompositeExtract %float %182 0
        %184 = OpCompositeExtract %float %182 1
        %185 = OpCompositeExtract %float %182 2
        %186 = OpCompositeConstruct %v4float %183 %184 %185 %float_1
               OpStore %outColor %186
               OpBranch %143
        %187 = OpLabel
        %188 = OpAccessChain %_ptr_Uniform_int %ubo %int_3
        %189 = OpLoad %int %188
        %190 = OpIEqual %bool %189 %int_2
               OpSelectionMerge %192 None
               OpBranchConditional %190 %191 %212
        %191 = OpLabel
        %198 = OpLoad %v4float %gl_FragCoord
        %200 = OpConvertFToS %v4int %198
        %201 = OpCompositeExtract %int %200 0
        %202 = OpCompositeExtract %int %200 1
        %203 = OpCompositeConstruct %v2int %201 %202
               OpStore %coord %203
        %207 = OpLoad %204 %input_frequency
        %208 = OpLoad %v2int %coord
        %210 = OpImageRead %v4uint %207 %208
        %211 = OpConvertUToF %v4float %210
               OpStore %outColor %211
               OpBranch %192
        %212 = OpLabel
        %213 = OpLoad %v4float %color
        %214 = OpVectorShuffle %v3float %213 %213 0 1 2
        %215 = OpCompositeExtract %float %214 0
        %216 = OpCompositeExtract %float %214 1
        %217 = OpCompositeExtract %float %214 2
        %218 = OpCompositeConstruct %v4float %215 %216 %217 %float_1
               OpStore %outColor %218
               OpBranch %192
        %192 = OpLabel
               OpBranch %143
        %143 = OpLabel
        %223 = OpLoad %float %freq_x
        %224 = OpFMul %float %float_255 %223
        %225 = OpConvertFToU %uint %224
        %226 = OpLoad %float %freq_y
        %227 = OpFMul %float %float_255 %226
        %228 = OpConvertFToU %uint %227
        %229 = OpCompositeConstruct %v2uint %225 %228
               OpStore %outFrequency %229
               OpReturn
               OpFunctionEnd
