; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 149
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
               OpMemberName %Push_Constants 1 "color"
               OpMemberName %Push_Constants 2 "object_type"
               OpName %push_constants "push_constants"
               OpName %outPos "outPos"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "skybox_modelview"
               OpMemberName %UBO 3 "modelscale"
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
               OpMemberDecorate %Push_Constants 2 Offset 32
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
%Push_Constants = OpTypeStruct %v4float %v4float %int
%_ptr_PushConstant_Push_Constants = OpTypePointer PushConstant %Push_Constants
%push_constants = OpVariable %_ptr_PushConstant_Push_Constants PushConstant
      %int_2 = OpConstant %int 2
%_ptr_PushConstant_int = OpTypePointer PushConstant %int
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
     %outPos = OpVariable %_ptr_Output_v3float Output
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
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
      %int_3 = OpConstant %int 3
%_ptr_Uniform_float = OpTypePointer Uniform %float
%_ptr_PushConstant_v4float = OpTypePointer PushConstant %v4float
      %int_1 = OpConstant %int 1
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
  %outNormal = OpVariable %_ptr_Output_v3float Output
   %inNormal = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
  %float_n10 = OpConstant %float -10
        %140 = OpConstantComposite %v3float %float_0 %float_n10 %float_n10
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
   %localPos = OpVariable %_ptr_Function_v3float Function
   %lightPos = OpVariable %_ptr_Function_v3float Function
         %14 = OpAccessChain %_ptr_PushConstant_int %push_constants %int_2
         %15 = OpLoad %int %14
               OpSelectionMerge %18 None
               OpSwitch %15 %18 0 %16 1 %17
         %16 = OpLabel
         %27 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_2
         %28 = OpLoad %mat4v4float %27
         %30 = OpCompositeExtract %v4float %28 0
         %31 = OpVectorShuffle %v3float %30 %30 0 1 2
         %32 = OpCompositeExtract %v4float %28 1
         %33 = OpVectorShuffle %v3float %32 %32 0 1 2
         %34 = OpCompositeExtract %v4float %28 2
         %35 = OpVectorShuffle %v3float %34 %34 0 1 2
         %36 = OpCompositeConstruct %mat3v3float %31 %33 %35
         %39 = OpLoad %v3float %inPos
         %40 = OpMatrixTimesVector %v3float %36 %39
         %41 = OpCompositeExtract %float %40 0
         %42 = OpCompositeExtract %float %40 1
         %43 = OpCompositeExtract %float %40 2
         %44 = OpCompositeConstruct %v3float %41 %42 %43
               OpStore %outPos %44
         %52 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %53 = OpLoad %mat4v4float %52
         %54 = OpLoad %v3float %outPos
         %56 = OpCompositeExtract %float %54 0
         %57 = OpCompositeExtract %float %54 1
         %58 = OpCompositeExtract %float %54 2
         %59 = OpCompositeConstruct %v4float %56 %57 %58 %float_1
         %60 = OpMatrixTimesVector %v4float %53 %59
         %61 = OpCompositeExtract %float %60 0
         %62 = OpCompositeExtract %float %60 1
         %63 = OpCompositeExtract %float %60 2
         %64 = OpCompositeExtract %float %60 3
         %65 = OpCompositeConstruct %v4float %61 %62 %63 %64
         %67 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %67 %65
               OpBranch %18
         %17 = OpLabel
         %71 = OpLoad %v3float %inPos
         %74 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %75 = OpLoad %float %74
         %76 = OpVectorTimesScalar %v3float %71 %75
         %78 = OpAccessChain %_ptr_PushConstant_v4float %push_constants %int_0
         %79 = OpLoad %v4float %78
         %80 = OpVectorShuffle %v3float %79 %79 0 1 2
         %81 = OpFAdd %v3float %76 %80
               OpStore %localPos %81
         %83 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %84 = OpLoad %mat4v4float %83
         %85 = OpLoad %v3float %localPos
         %86 = OpCompositeExtract %float %85 0
         %87 = OpCompositeExtract %float %85 1
         %88 = OpCompositeExtract %float %85 2
         %89 = OpCompositeConstruct %v4float %86 %87 %88 %float_1
         %90 = OpMatrixTimesVector %v4float %84 %89
         %91 = OpCompositeExtract %float %90 0
         %92 = OpCompositeExtract %float %90 1
         %93 = OpCompositeExtract %float %90 2
         %94 = OpCompositeConstruct %v3float %91 %92 %93
               OpStore %outPos %94
         %95 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %96 = OpLoad %mat4v4float %95
         %97 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %98 = OpLoad %mat4v4float %97
         %99 = OpMatrixTimesMatrix %mat4v4float %96 %98
        %100 = OpLoad %v3float %localPos
        %101 = OpCompositeExtract %float %100 0
        %102 = OpCompositeExtract %float %100 1
        %103 = OpCompositeExtract %float %100 2
        %104 = OpCompositeConstruct %v4float %101 %102 %103 %float_1
        %105 = OpMatrixTimesVector %v4float %99 %104
        %106 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %106 %105
               OpBranch %18
         %18 = OpLabel
        %114 = OpLoad %v2float %inUV
               OpStore %outUV %114
        %116 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %117 = OpLoad %mat4v4float %116
        %118 = OpCompositeExtract %v4float %117 0
        %119 = OpVectorShuffle %v3float %118 %118 0 1 2
        %120 = OpCompositeExtract %v4float %117 1
        %121 = OpVectorShuffle %v3float %120 %120 0 1 2
        %122 = OpCompositeExtract %v4float %117 2
        %123 = OpVectorShuffle %v3float %122 %122 0 1 2
        %124 = OpCompositeConstruct %mat3v3float %119 %121 %123
        %126 = OpLoad %v3float %inNormal
        %127 = OpMatrixTimesVector %v3float %124 %126
               OpStore %outNormal %127
        %129 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %130 = OpLoad %mat4v4float %129
        %131 = OpCompositeExtract %v4float %130 0
        %132 = OpVectorShuffle %v3float %131 %131 0 1 2
        %133 = OpCompositeExtract %v4float %130 1
        %134 = OpVectorShuffle %v3float %133 %133 0 1 2
        %135 = OpCompositeExtract %v4float %130 2
        %136 = OpVectorShuffle %v3float %135 %135 0 1 2
        %137 = OpCompositeConstruct %mat3v3float %132 %134 %136
        %141 = OpMatrixTimesVector %v3float %137 %140
               OpStore %lightPos %141
        %143 = OpLoad %v3float %lightPos
        %144 = OpLoad %v3float %outPos
        %145 = OpFSub %v3float %143 %144
               OpStore %outLightVec %145
        %147 = OpLoad %v3float %outPos
        %148 = OpFNegate %v3float %147
               OpStore %outViewVec %148
               OpReturn
               OpFunctionEnd
