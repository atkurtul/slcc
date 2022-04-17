; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 145
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %inNormal %inLightVec %inViewVec %outColor0 %outColor1 %inPos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %Push_Constants "Push_Constants"
               OpMemberName %Push_Constants 0 "offset"
               OpMemberName %Push_Constants 1 "color"
               OpMemberName %Push_Constants 2 "object_type"
               OpName %push_constants "push_constants"
               OpName %color "color"
               OpName %samplerEnvMap "samplerEnvMap"
               OpName %inUV "inUV"
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
               OpName %outColor0 "outColor0"
               OpName %l "l"
               OpName %threshold "threshold"
               OpName %outColor1 "outColor1"
               OpName %inPos "inPos"
               OpMemberDecorate %Push_Constants 0 Offset 0
               OpMemberDecorate %Push_Constants 1 Offset 16
               OpMemberDecorate %Push_Constants 2 Offset 32
               OpDecorate %Push_Constants Block
               OpDecorate %samplerEnvMap DescriptorSet 0
               OpDecorate %samplerEnvMap Binding 1
               OpDecorate %inUV Location 0
               OpDecorate %inNormal Location 2
               OpDecorate %inLightVec Location 4
               OpDecorate %inViewVec Location 3
               OpDecorate %outColor0 Location 0
               OpDecorate %outColor1 Location 1
               OpDecorate %inPos Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
        %int = OpTypeInt 32 1
%Push_Constants = OpTypeStruct %v4float %v4float %int
%_ptr_PushConstant_Push_Constants = OpTypePointer PushConstant %Push_Constants
%push_constants = OpVariable %_ptr_PushConstant_Push_Constants PushConstant
      %int_2 = OpConstant %int 2
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
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_v4float = OpTypePointer PushConstant %v4float
  %float_0_5 = OpConstant %float 0.5
         %51 = OpConstantComposite %v3float %float_0_5 %float_0_5 %float_0_5
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
    %float_8 = OpConstant %float 8
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %outColor0 = OpVariable %_ptr_Output_v4float Output
%_ptr_Output_float = OpTypePointer Output %float
     %uint_2 = OpConstant %uint 2
%_ptr_Function_float = OpTypePointer Function %float
%float_0_212599993 = OpConstant %float 0.212599993
%float_0_715200007 = OpConstant %float 0.715200007
%float_0_0722000003 = OpConstant %float 0.0722000003
        %119 = OpConstantComposite %v3float %float_0_212599993 %float_0_715200007 %float_0_0722000003
 %float_0_75 = OpConstant %float 0.75
  %outColor1 = OpVariable %_ptr_Output_v4float Output
       %bool = OpTypeBool
        %134 = OpConstantComposite %v3float %float_0 %float_0 %float_0
     %uint_3 = OpConstant %uint 3
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
          %l = OpVariable %_ptr_Function_float Function
  %threshold = OpVariable %_ptr_Function_float Function
        %128 = OpVariable %_ptr_Function_v3float Function
         %14 = OpAccessChain %_ptr_PushConstant_int %push_constants %int_2
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
         %47 = OpAccessChain %_ptr_PushConstant_v4float %push_constants %int_1
         %48 = OpLoad %v4float %47
         %49 = OpVectorShuffle %v3float %48 %48 0 1 2
         %52 = OpFMul %v3float %49 %51
               OpStore %ambient %52
         %56 = OpLoad %v3float %inNormal
         %57 = OpExtInst %v3float %1 Normalize %56
               OpStore %N %57
         %60 = OpLoad %v3float %inLightVec
         %61 = OpExtInst %v3float %1 Normalize %60
               OpStore %L %61
         %64 = OpLoad %v3float %inViewVec
         %65 = OpExtInst %v3float %1 Normalize %64
               OpStore %V %65
         %67 = OpLoad %v3float %L
         %68 = OpFNegate %v3float %67
         %69 = OpLoad %v3float %N
         %70 = OpExtInst %v3float %1 Reflect %68 %69
               OpStore %R %70
         %72 = OpLoad %v3float %N
         %73 = OpLoad %v3float %L
         %74 = OpDot %float %72 %73
         %76 = OpExtInst %float %1 FMax %74 %float_0
         %77 = OpAccessChain %_ptr_PushConstant_v4float %push_constants %int_1
         %78 = OpLoad %v4float %77
         %79 = OpVectorShuffle %v3float %78 %78 0 1 2
         %80 = OpVectorTimesScalar %v3float %79 %76
               OpStore %diffuse %80
         %82 = OpLoad %v3float %R
         %83 = OpLoad %v3float %V
         %84 = OpDot %float %82 %83
         %85 = OpExtInst %float %1 FMax %84 %float_0
         %87 = OpExtInst %float %1 Pow %85 %float_8
         %88 = OpCompositeConstruct %v3float %87 %87 %87
               OpStore %specular %88
         %89 = OpLoad %v3float %ambient
         %90 = OpLoad %v3float %diffuse
         %91 = OpFAdd %v3float %89 %90
         %92 = OpLoad %v3float %specular
         %93 = OpFAdd %v3float %91 %92
         %94 = OpCompositeExtract %float %93 0
         %95 = OpCompositeExtract %float %93 1
         %96 = OpCompositeExtract %float %93 2
         %97 = OpCompositeConstruct %v4float %94 %95 %96 %float_1
               OpStore %color %97
               OpBranch %18
         %18 = OpLabel
        %102 = OpLoad %v4float %color
        %103 = OpVectorShuffle %v3float %102 %102 0 1 2
        %105 = OpAccessChain %_ptr_Output_float %outColor0 %uint_0
        %106 = OpCompositeExtract %float %103 0
               OpStore %105 %106
        %107 = OpAccessChain %_ptr_Output_float %outColor0 %uint_1
        %108 = OpCompositeExtract %float %103 1
               OpStore %107 %108
        %110 = OpAccessChain %_ptr_Output_float %outColor0 %uint_2
        %111 = OpCompositeExtract %float %103 2
               OpStore %110 %111
        %114 = OpLoad %v4float %outColor0
        %115 = OpVectorShuffle %v3float %114 %114 0 1 2
        %120 = OpDot %float %115 %119
               OpStore %l %120
               OpStore %threshold %float_0_75
        %124 = OpLoad %float %l
        %125 = OpLoad %float %threshold
        %127 = OpFOrdGreaterThan %bool %124 %125
               OpSelectionMerge %130 None
               OpBranchConditional %127 %129 %133
        %129 = OpLabel
        %131 = OpLoad %v4float %outColor0
        %132 = OpVectorShuffle %v3float %131 %131 0 1 2
               OpStore %128 %132
               OpBranch %130
        %133 = OpLabel
               OpStore %128 %134
               OpBranch %130
        %130 = OpLabel
        %135 = OpLoad %v3float %128
        %136 = OpAccessChain %_ptr_Output_float %outColor1 %uint_0
        %137 = OpCompositeExtract %float %135 0
               OpStore %136 %137
        %138 = OpAccessChain %_ptr_Output_float %outColor1 %uint_1
        %139 = OpCompositeExtract %float %135 1
               OpStore %138 %139
        %140 = OpAccessChain %_ptr_Output_float %outColor1 %uint_2
        %141 = OpCompositeExtract %float %135 2
               OpStore %140 %141
        %143 = OpAccessChain %_ptr_Output_float %outColor1 %uint_3
               OpStore %143 %float_1
               OpReturn
               OpFunctionEnd
