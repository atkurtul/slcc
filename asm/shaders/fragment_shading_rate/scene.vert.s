; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 144
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outPos %inPos %_ %outUV %inUV %outNormal %inNormal %outLightVec %outViewVec
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %Push_Constants "Push_Constants"
               OpMemberName %Push_Constants 0 "offset"
               OpMemberName %Push_Constants 1 "object_type"
               OpName %push_constants "push_constants"
               OpName %outPos "outPos"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "skybox_modelview"
               OpMemberName %UBO 3 "color_shading_rates"
               OpName %ubo "ubo"
               OpName %inPos "inPos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %localPos "localPos"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %lightPos "lightPos"
               OpName %outLightVec "outLightVec"
               OpName %outViewVec "outViewVec"
               OpMemberDecorate %Push_Constants 0 Offset 0
               OpMemberDecorate %Push_Constants 1 Offset 16
               OpDecorate %Push_Constants Block
               OpDecorate %outPos Location 1
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
               OpDecorate %inPos Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 2
               OpDecorate %outNormal Location 2
               OpDecorate %inNormal Location 1
               OpDecorate %outLightVec Location 4
               OpDecorate %outViewVec Location 3
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
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
     %outPos = OpVariable %_ptr_Output_v3float Output
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_2 = OpConstant %int 2
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%mat3v3float = OpTypeMatrix %v3float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_PushConstant_v4float = OpTypePointer PushConstant %v4float
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
  %outNormal = OpVariable %_ptr_Output_v3float Output
   %inNormal = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
  %float_n10 = OpConstant %float -10
        %135 = OpConstantComposite %v3float %float_0 %float_n10 %float_n10
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
   %localPos = OpVariable %_ptr_Function_v3float Function
   %lightPos = OpVariable %_ptr_Function_v3float Function
         %14 = OpAccessChain %_ptr_PushConstant_int %push_constants %int_1
         %15 = OpLoad %int %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %28 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_2
         %29 = OpLoad %mat4v4float %28
         %31 = OpCompositeExtract %v4float %29 0
         %32 = OpVectorShuffle %v3float %31 %31 0 1 2
         %33 = OpCompositeExtract %v4float %29 1
         %34 = OpVectorShuffle %v3float %33 %33 0 1 2
         %35 = OpCompositeExtract %v4float %29 2
         %36 = OpVectorShuffle %v3float %35 %35 0 1 2
         %37 = OpCompositeConstruct %mat3v3float %32 %34 %36
         %40 = OpLoad %v3float %inPos
         %41 = OpMatrixTimesVector %v3float %37 %40
         %42 = OpCompositeExtract %float %41 0
         %43 = OpCompositeExtract %float %41 1
         %44 = OpCompositeExtract %float %41 2
         %45 = OpCompositeConstruct %v3float %42 %43 %44
               OpStore %outPos %45
         %53 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %54 = OpLoad %mat4v4float %53
         %55 = OpLoad %v3float %outPos
         %57 = OpCompositeExtract %float %55 0
         %58 = OpCompositeExtract %float %55 1
         %59 = OpCompositeExtract %float %55 2
         %60 = OpCompositeConstruct %v4float %57 %58 %59 %float_1
         %61 = OpMatrixTimesVector %v4float %54 %60
         %62 = OpCompositeExtract %float %61 0
         %63 = OpCompositeExtract %float %61 1
         %64 = OpCompositeExtract %float %61 2
         %65 = OpCompositeExtract %float %61 3
         %66 = OpCompositeConstruct %v4float %62 %63 %64 %65
         %68 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %68 %66
               OpBranch %18
         %17 = OpLabel
         %72 = OpLoad %v3float %inPos
         %74 = OpAccessChain %_ptr_PushConstant_v4float %push_constants %int_0
         %75 = OpLoad %v4float %74
         %76 = OpVectorShuffle %v3float %75 %75 0 1 2
         %77 = OpFAdd %v3float %72 %76
               OpStore %localPos %77
         %78 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %79 = OpLoad %mat4v4float %78
         %80 = OpLoad %v3float %localPos
         %81 = OpCompositeExtract %float %80 0
         %82 = OpCompositeExtract %float %80 1
         %83 = OpCompositeExtract %float %80 2
         %84 = OpCompositeConstruct %v4float %81 %82 %83 %float_1
         %85 = OpMatrixTimesVector %v4float %79 %84
         %86 = OpCompositeExtract %float %85 0
         %87 = OpCompositeExtract %float %85 1
         %88 = OpCompositeExtract %float %85 2
         %89 = OpCompositeConstruct %v3float %86 %87 %88
               OpStore %outPos %89
         %90 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %91 = OpLoad %mat4v4float %90
         %92 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %93 = OpLoad %mat4v4float %92
         %94 = OpMatrixTimesMatrix %mat4v4float %91 %93
         %95 = OpLoad %v3float %localPos
         %96 = OpCompositeExtract %float %95 0
         %97 = OpCompositeExtract %float %95 1
         %98 = OpCompositeExtract %float %95 2
         %99 = OpCompositeConstruct %v4float %96 %97 %98 %float_1
        %100 = OpMatrixTimesVector %v4float %94 %99
        %101 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %101 %100
               OpBranch %18
         %18 = OpLabel
        %109 = OpLoad %v2float %inUV
               OpStore %outUV %109
        %111 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %112 = OpLoad %mat4v4float %111
        %113 = OpCompositeExtract %v4float %112 0
        %114 = OpVectorShuffle %v3float %113 %113 0 1 2
        %115 = OpCompositeExtract %v4float %112 1
        %116 = OpVectorShuffle %v3float %115 %115 0 1 2
        %117 = OpCompositeExtract %v4float %112 2
        %118 = OpVectorShuffle %v3float %117 %117 0 1 2
        %119 = OpCompositeConstruct %mat3v3float %114 %116 %118
        %121 = OpLoad %v3float %inNormal
        %122 = OpMatrixTimesVector %v3float %119 %121
               OpStore %outNormal %122
        %124 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %125 = OpLoad %mat4v4float %124
        %126 = OpCompositeExtract %v4float %125 0
        %127 = OpVectorShuffle %v3float %126 %126 0 1 2
        %128 = OpCompositeExtract %v4float %125 1
        %129 = OpVectorShuffle %v3float %128 %128 0 1 2
        %130 = OpCompositeExtract %v4float %125 2
        %131 = OpVectorShuffle %v3float %130 %130 0 1 2
        %132 = OpCompositeConstruct %mat3v3float %127 %129 %131
        %136 = OpMatrixTimesVector %v3float %132 %135
               OpStore %lightPos %136
        %138 = OpLoad %v3float %lightPos
        %139 = OpLoad %v3float %outPos
        %140 = OpFSub %v3float %138 %139
               OpStore %outLightVec %140
        %142 = OpLoad %v3float %outPos
        %143 = OpFNegate %v3float %142
               OpStore %outViewVec %143
               OpReturn
               OpFunctionEnd
