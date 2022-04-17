; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 200
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %gl_FragCoord %inNormal %inLightVec %outFragColor %inViewVec %inEyePos %inWorldPos
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %sampleTerrainLayer_ "sampleTerrainLayer("
               OpName %fog_f1_ "fog(f1;"
               OpName %density "density"
               OpName %layers "layers"
               OpName %color "color"
               OpName %height "height"
               OpName %samplerHeight "samplerHeight"
               OpName %inUV "inUV"
               OpName %i "i"
               OpName %range "range"
               OpName %weight "weight"
               OpName %samplerLayers "samplerLayers"
               OpName %dist "dist"
               OpName %gl_FragCoord "gl_FragCoord"
               OpName %d "d"
               OpName %N "N"
               OpName %inNormal "inNormal"
               OpName %L "L"
               OpName %inLightVec "inLightVec"
               OpName %ambient "ambient"
               OpName %diffuse "diffuse"
               OpName %color_0 "color"
               OpName %outFragColor "outFragColor"
               OpName %param "param"
               OpName %inViewVec "inViewVec"
               OpName %inEyePos "inEyePos"
               OpName %inWorldPos "inWorldPos"
               OpDecorate %samplerHeight DescriptorSet 0
               OpDecorate %samplerHeight Binding 1
               OpDecorate %inUV Location 1
               OpDecorate %samplerLayers DescriptorSet 0
               OpDecorate %samplerLayers Binding 2
               OpDecorate %gl_FragCoord BuiltIn FragCoord
               OpDecorate %inNormal Location 0
               OpDecorate %inLightVec Location 3
               OpDecorate %outFragColor Location 0
               OpDecorate %inViewVec Location 2
               OpDecorate %inEyePos Location 4
               OpDecorate %inWorldPos Location 5
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
          %8 = OpTypeFunction %v3float
%_ptr_Function_float = OpTypePointer Function %float
         %12 = OpTypeFunction %float %_ptr_Function_float
    %v2float = OpTypeVector %float 2
       %uint = OpTypeInt 32 0
     %uint_6 = OpConstant %uint 6
%_arr_v2float_uint_6 = OpTypeArray %v2float %uint_6
%_ptr_Function__arr_v2float_uint_6 = OpTypePointer Function %_arr_v2float_uint_6
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
  %float_n10 = OpConstant %float -10
   %float_10 = OpConstant %float 10
         %26 = OpConstantComposite %v2float %float_n10 %float_10
%_ptr_Function_v2float = OpTypePointer Function %v2float
      %int_1 = OpConstant %int 1
    %float_5 = OpConstant %float 5
   %float_45 = OpConstant %float 45
         %32 = OpConstantComposite %v2float %float_5 %float_45
      %int_2 = OpConstant %int 2
   %float_80 = OpConstant %float 80
         %36 = OpConstantComposite %v2float %float_45 %float_80
      %int_3 = OpConstant %int 3
   %float_75 = OpConstant %float 75
  %float_100 = OpConstant %float 100
         %41 = OpConstantComposite %v2float %float_75 %float_100
      %int_4 = OpConstant %int 4
   %float_95 = OpConstant %float 95
  %float_140 = OpConstant %float 140
         %46 = OpConstantComposite %v2float %float_95 %float_140
      %int_5 = OpConstant %int 5
  %float_190 = OpConstant %float 190
         %50 = OpConstantComposite %v2float %float_140 %float_190
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %float_0 = OpConstant %float 0
         %55 = OpConstantComposite %v3float %float_0 %float_0 %float_0
         %57 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %58 = OpTypeSampledImage %57
%_ptr_UniformConstant_58 = OpTypePointer UniformConstant %58
%samplerHeight = OpVariable %_ptr_UniformConstant_58 UniformConstant
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %v4float = OpTypeVector %float 4
     %uint_0 = OpConstant %uint 0
  %float_255 = OpConstant %float 255
%_ptr_Function_int = OpTypePointer Function %int
      %int_6 = OpConstant %int 6
       %bool = OpTypeBool
     %uint_1 = OpConstant %uint 1
        %105 = OpTypeImage %float 2D 0 1 0 1 Unknown
        %106 = OpTypeSampledImage %105
%_ptr_UniformConstant_106 = OpTypePointer UniformConstant %106
%samplerLayers = OpVariable %_ptr_UniformConstant_106 UniformConstant
   %float_16 = OpConstant %float 16
%_ptr_Input_v4float = OpTypePointer Input %v4float
%gl_FragCoord = OpVariable %_ptr_Input_v4float Input
     %uint_2 = OpConstant %uint 2
%_ptr_Input_float = OpTypePointer Input %float
     %uint_3 = OpConstant %uint 3
%float_0_100000001 = OpConstant %float 0.100000001
    %float_1 = OpConstant %float 1
%float_n1_44269502 = OpConstant %float -1.44269502
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %float_0_5 = OpConstant %float 0.5
        %167 = OpConstantComposite %v3float %float_0_5 %float_0_5 %float_0_5
        %173 = OpConstantComposite %v3float %float_1 %float_1 %float_1
%_ptr_Function_v4float = OpTypePointer Function %v4float
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
%float_0_469999999 = OpConstant %float 0.469999999
%float_0_670000017 = OpConstant %float 0.670000017
        %191 = OpConstantComposite %v4float %float_0_469999999 %float_0_5 %float_0_670000017 %float_0
 %float_0_25 = OpConstant %float 0.25
  %inViewVec = OpVariable %_ptr_Input_v3float Input
   %inEyePos = OpVariable %_ptr_Input_v3float Input
 %inWorldPos = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
          %N = OpVariable %_ptr_Function_v3float Function
          %L = OpVariable %_ptr_Function_v3float Function
    %ambient = OpVariable %_ptr_Function_v3float Function
    %diffuse = OpVariable %_ptr_Function_v3float Function
    %color_0 = OpVariable %_ptr_Function_v4float Function
      %param = OpVariable %_ptr_Function_float Function
        %159 = OpLoad %v3float %inNormal
        %160 = OpExtInst %v3float %1 Normalize %159
               OpStore %N %160
        %163 = OpLoad %v3float %inLightVec
        %164 = OpExtInst %v3float %1 Normalize %163
               OpStore %L %164
               OpStore %ambient %167
        %169 = OpLoad %v3float %N
        %170 = OpLoad %v3float %L
        %171 = OpDot %float %169 %170
        %172 = OpExtInst %float %1 FMax %171 %float_0
        %174 = OpVectorTimesScalar %v3float %173 %172
               OpStore %diffuse %174
        %177 = OpLoad %v3float %ambient
        %178 = OpLoad %v3float %diffuse
        %179 = OpFAdd %v3float %177 %178
        %180 = OpFunctionCall %v3float %sampleTerrainLayer_
        %181 = OpFMul %v3float %179 %180
        %182 = OpCompositeExtract %float %181 0
        %183 = OpCompositeExtract %float %181 1
        %184 = OpCompositeExtract %float %181 2
        %185 = OpCompositeConstruct %v4float %182 %183 %184 %float_1
               OpStore %color_0 %185
        %188 = OpLoad %v4float %color_0
               OpStore %param %float_0_25
        %194 = OpFunctionCall %float %fog_f1_ %param
        %195 = OpCompositeConstruct %v4float %194 %194 %194 %194
        %196 = OpExtInst %v4float %1 FMix %188 %191 %195
               OpStore %outFragColor %196
               OpReturn
               OpFunctionEnd
%sampleTerrainLayer_ = OpFunction %v3float None %8
         %10 = OpLabel
     %layers = OpVariable %_ptr_Function__arr_v2float_uint_6 Function
      %color = OpVariable %_ptr_Function_v3float Function
     %height = OpVariable %_ptr_Function_float Function
          %i = OpVariable %_ptr_Function_int Function
      %range = OpVariable %_ptr_Function_float Function
     %weight = OpVariable %_ptr_Function_float Function
         %28 = OpAccessChain %_ptr_Function_v2float %layers %int_0
               OpStore %28 %26
         %33 = OpAccessChain %_ptr_Function_v2float %layers %int_1
               OpStore %33 %32
         %37 = OpAccessChain %_ptr_Function_v2float %layers %int_2
               OpStore %37 %36
         %42 = OpAccessChain %_ptr_Function_v2float %layers %int_3
               OpStore %42 %41
         %47 = OpAccessChain %_ptr_Function_v2float %layers %int_4
               OpStore %47 %46
         %51 = OpAccessChain %_ptr_Function_v2float %layers %int_5
               OpStore %51 %50
               OpStore %color %55
         %61 = OpLoad %58 %samplerHeight
         %64 = OpLoad %v2float %inUV
         %66 = OpImageSampleExplicitLod %v4float %61 %64 Lod %float_0
         %68 = OpCompositeExtract %float %66 0
         %70 = OpFMul %float %68 %float_255
               OpStore %height %70
               OpStore %i %int_0
               OpBranch %73
         %73 = OpLabel
               OpLoopMerge %75 %76 None
               OpBranch %77
         %77 = OpLabel
         %78 = OpLoad %int %i
         %81 = OpSLessThan %bool %78 %int_6
               OpBranchConditional %81 %74 %75
         %74 = OpLabel
         %83 = OpLoad %int %i
         %85 = OpAccessChain %_ptr_Function_float %layers %83 %uint_1
         %86 = OpLoad %float %85
         %87 = OpLoad %int %i
         %88 = OpAccessChain %_ptr_Function_float %layers %87 %uint_0
         %89 = OpLoad %float %88
         %90 = OpFSub %float %86 %89
               OpStore %range %90
         %92 = OpLoad %float %range
         %93 = OpLoad %float %height
         %94 = OpLoad %int %i
         %95 = OpAccessChain %_ptr_Function_float %layers %94 %uint_1
         %96 = OpLoad %float %95
         %97 = OpFSub %float %93 %96
         %98 = OpExtInst %float %1 FAbs %97
         %99 = OpFSub %float %92 %98
        %100 = OpLoad %float %range
        %101 = OpFDiv %float %99 %100
               OpStore %weight %101
        %102 = OpLoad %float %weight
        %103 = OpExtInst %float %1 FMax %float_0 %102
               OpStore %weight %103
        %104 = OpLoad %float %weight
        %109 = OpLoad %106 %samplerLayers
        %110 = OpLoad %v2float %inUV
        %112 = OpVectorTimesScalar %v2float %110 %float_16
        %113 = OpLoad %int %i
        %114 = OpConvertSToF %float %113
        %115 = OpCompositeExtract %float %112 0
        %116 = OpCompositeExtract %float %112 1
        %117 = OpCompositeConstruct %v3float %115 %116 %114
        %118 = OpImageSampleImplicitLod %v4float %109 %117
        %119 = OpVectorShuffle %v3float %118 %118 0 1 2
        %120 = OpVectorTimesScalar %v3float %119 %104
        %121 = OpLoad %v3float %color
        %122 = OpFAdd %v3float %121 %120
               OpStore %color %122
               OpBranch %76
         %76 = OpLabel
        %123 = OpLoad %int %i
        %124 = OpIAdd %int %123 %int_1
               OpStore %i %124
               OpBranch %73
         %75 = OpLabel
        %125 = OpLoad %v3float %color
               OpReturnValue %125
               OpFunctionEnd
    %fog_f1_ = OpFunction %float None %12
    %density = OpFunctionParameter %_ptr_Function_float
         %15 = OpLabel
       %dist = OpVariable %_ptr_Function_float Function
          %d = OpVariable %_ptr_Function_float Function
        %133 = OpAccessChain %_ptr_Input_float %gl_FragCoord %uint_2
        %134 = OpLoad %float %133
        %136 = OpAccessChain %_ptr_Input_float %gl_FragCoord %uint_3
        %137 = OpLoad %float %136
        %138 = OpFDiv %float %134 %137
        %140 = OpFMul %float %138 %float_0_100000001
               OpStore %dist %140
        %142 = OpLoad %float %density
        %143 = OpLoad %float %dist
        %144 = OpFMul %float %142 %143
               OpStore %d %144
        %146 = OpLoad %float %d
        %147 = OpLoad %float %d
        %148 = OpFMul %float %146 %147
        %150 = OpFMul %float %148 %float_n1_44269502
        %151 = OpExtInst %float %1 Exp2 %150
        %152 = OpExtInst %float %1 FClamp %151 %float_0 %float_1
        %153 = OpFSub %float %float_1 %152
               OpReturnValue %153
               OpFunctionEnd
