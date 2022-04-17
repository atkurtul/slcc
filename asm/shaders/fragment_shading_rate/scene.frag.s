; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 174
; Schema: 0
               OpCapability Shader
               OpCapability FragmentShadingRateKHR
               OpExtension "SPV_KHR_fragment_shading_rate"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %inNormal %inLightVec %inViewVec %gl_ShadingRateEXT %outColor %inPos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_fragment_shading_rate"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %Push_Constants "Push_Constants"
               OpMemberName %Push_Constants 0 "offset"
               OpMemberName %Push_Constants 1 "object_type"
               OpName %push_constants "push_constants"
               OpName %color "color"
               OpName %samplerEnvMap "samplerEnvMap"
               OpName %inUV "inUV"
               OpName %ambient "ambient"
               OpName %samplerSphere "samplerSphere"
               OpName %N "N"
               OpName %inNormal "inNormal"
               OpName %L "L"
               OpName %inLightVec "inLightVec"
               OpName %V "V"
               OpName %inViewVec "inViewVec"
               OpName %R "R"
               OpName %diffuse "diffuse"
               OpName %specular "specular"
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
               OpDecorate %inPos Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
        %int = OpTypeInt 32 1
%Push_Constants = OpTypeStruct %v4float %int
%_ptr_PushConstant_Push_Constants = OpTypePointer PushConstant %Push_Constants
%push_constants = OpVariable %_ptr_PushConstant_Push_Constants PushConstant
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_int = OpTypePointer PushConstant %int
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %21 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %22 = OpTypeSampledImage %21
%_ptr_UniformConstant_22 = OpTypePointer UniformConstant %22
%samplerEnvMap = OpVariable %_ptr_UniformConstant_22 UniformConstant
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
%samplerSphere = OpVariable %_ptr_UniformConstant_22 UniformConstant
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
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
%float_0_0500000007 = OpConstant %float 0.0500000007
      %inPos = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
    %ambient = OpVariable %_ptr_Function_v3float Function
          %N = OpVariable %_ptr_Function_v3float Function
          %L = OpVariable %_ptr_Function_v3float Function
          %V = OpVariable %_ptr_Function_v3float Function
          %R = OpVariable %_ptr_Function_v3float Function
    %diffuse = OpVariable %_ptr_Function_v3float Function
   %specular = OpVariable %_ptr_Function_v3float Function
          %v = OpVariable %_ptr_Function_int Function
          %h = OpVariable %_ptr_Function_int Function
         %14 = OpAccessChain %_ptr_PushConstant_int %push_constants %int_1
         %15 = OpLoad %int %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %25 = OpLoad %22 %samplerEnvMap
         %32 = OpAccessChain %_ptr_Input_float %inUV %uint_0
         %33 = OpLoad %float %32
         %36 = OpAccessChain %_ptr_Input_float %inUV %uint_1
         %37 = OpLoad %float %36
         %38 = OpFSub %float %float_1 %37
         %39 = OpCompositeConstruct %v2float %33 %38
         %40 = OpImageSampleImplicitLod %v4float %25 %39
               OpStore %color %40
               OpBranch %18
         %17 = OpLabel
         %46 = OpLoad %22 %samplerSphere
         %47 = OpLoad %v2float %inUV
         %48 = OpImageSampleImplicitLod %v4float %46 %47
         %49 = OpVectorShuffle %v3float %48 %48 0 1 2
               OpStore %ambient %49
         %53 = OpLoad %v3float %inNormal
         %54 = OpExtInst %v3float %1 Normalize %53
               OpStore %N %54
         %57 = OpLoad %v3float %inLightVec
         %58 = OpExtInst %v3float %1 Normalize %57
               OpStore %L %58
         %61 = OpLoad %v3float %inViewVec
         %62 = OpExtInst %v3float %1 Normalize %61
               OpStore %V %62
         %64 = OpLoad %v3float %L
         %65 = OpFNegate %v3float %64
         %66 = OpLoad %v3float %N
         %67 = OpExtInst %v3float %1 Reflect %65 %66
               OpStore %R %67
         %69 = OpLoad %v3float %N
         %70 = OpLoad %v3float %L
         %71 = OpDot %float %69 %70
         %73 = OpExtInst %float %1 FMax %71 %float_0
         %74 = OpCompositeConstruct %v3float %73 %73 %73
               OpStore %diffuse %74
         %76 = OpLoad %v3float %R
         %77 = OpLoad %v3float %V
         %78 = OpDot %float %76 %77
         %79 = OpExtInst %float %1 FMax %78 %float_0
         %81 = OpExtInst %float %1 Pow %79 %float_8
         %82 = OpCompositeConstruct %v3float %81 %81 %81
               OpStore %specular %82
         %83 = OpLoad %v3float %ambient
         %84 = OpLoad %v3float %diffuse
         %85 = OpFAdd %v3float %83 %84
         %86 = OpLoad %v3float %specular
         %87 = OpFAdd %v3float %85 %86
         %88 = OpCompositeExtract %float %87 0
         %89 = OpCompositeExtract %float %87 1
         %90 = OpCompositeExtract %float %87 2
         %91 = OpCompositeConstruct %v4float %88 %89 %90 %float_1
               OpStore %color %91
               OpBranch %18
         %18 = OpLabel
        %100 = OpAccessChain %_ptr_Uniform_int %ubo %int_3
        %101 = OpLoad %int %100
        %103 = OpIEqual %bool %101 %int_1
               OpSelectionMerge %105 None
               OpBranchConditional %103 %104 %166
        %104 = OpLabel
               OpStore %v %int_1
               OpStore %h %int_1
        %111 = OpLoad %int %gl_ShadingRateEXT
        %112 = OpBitwiseAnd %int %111 %int_1
        %113 = OpIEqual %bool %112 %int_1
               OpSelectionMerge %115 None
               OpBranchConditional %113 %114 %115
        %114 = OpLabel
               OpStore %v %int_2
               OpBranch %115
        %115 = OpLabel
        %117 = OpLoad %int %gl_ShadingRateEXT
        %118 = OpBitwiseAnd %int %117 %int_2
        %119 = OpIEqual %bool %118 %int_2
               OpSelectionMerge %121 None
               OpBranchConditional %119 %120 %121
        %120 = OpLabel
               OpStore %v %int_4
               OpBranch %121
        %121 = OpLabel
        %123 = OpLoad %int %gl_ShadingRateEXT
        %124 = OpBitwiseAnd %int %123 %int_4
        %125 = OpIEqual %bool %124 %int_4
               OpSelectionMerge %127 None
               OpBranchConditional %125 %126 %127
        %126 = OpLabel
               OpStore %h %int_2
               OpBranch %127
        %127 = OpLabel
        %128 = OpLoad %int %gl_ShadingRateEXT
        %130 = OpBitwiseAnd %int %128 %int_8
        %131 = OpIEqual %bool %130 %int_8
               OpSelectionMerge %133 None
               OpBranchConditional %131 %132 %133
        %132 = OpLabel
               OpStore %h %int_4
               OpBranch %133
        %133 = OpLabel
        %134 = OpLoad %int %v
        %135 = OpIEqual %bool %134 %int_1
        %136 = OpLoad %int %h
        %137 = OpIEqual %bool %136 %int_1
        %138 = OpLogicalAnd %bool %135 %137
               OpSelectionMerge %140 None
               OpBranchConditional %138 %139 %150
        %139 = OpLabel
        %143 = OpLoad %v4float %color
        %144 = OpVectorShuffle %v3float %143 %143 0 0 0
        %145 = OpVectorTimesScalar %v3float %144 %float_1
        %146 = OpCompositeExtract %float %145 0
        %147 = OpCompositeExtract %float %145 1
        %148 = OpCompositeExtract %float %145 2
        %149 = OpCompositeConstruct %v4float %146 %147 %148 %float_1
               OpStore %outColor %149
               OpBranch %140
        %150 = OpLabel
        %151 = OpLoad %v4float %color
        %152 = OpVectorShuffle %v3float %151 %151 0 0 0
        %153 = OpVectorTimesScalar %v3float %152 %float_1
        %154 = OpLoad %int %v
        %155 = OpLoad %int %h
        %156 = OpIAdd %int %154 %155
        %157 = OpConvertSToF %float %156
        %159 = OpFMul %float %157 %float_0_0500000007
        %160 = OpCompositeConstruct %v3float %159 %159 %159
        %161 = OpFSub %v3float %153 %160
        %162 = OpCompositeExtract %float %161 0
        %163 = OpCompositeExtract %float %161 1
        %164 = OpCompositeExtract %float %161 2
        %165 = OpCompositeConstruct %v4float %162 %163 %164 %float_1
               OpStore %outColor %165
               OpBranch %140
        %140 = OpLabel
               OpBranch %105
        %166 = OpLabel
        %167 = OpLoad %v4float %color
        %168 = OpVectorShuffle %v3float %167 %167 0 1 2
        %169 = OpCompositeExtract %float %168 0
        %170 = OpCompositeExtract %float %168 1
        %171 = OpCompositeExtract %float %168 2
        %172 = OpCompositeConstruct %v4float %169 %170 %171 %float_1
               OpStore %outColor %172
               OpBranch %105
        %105 = OpLabel
               OpReturn
               OpFunctionEnd
