; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 129
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUVW %inPos %outPos %_ %outNormal %inNormal %outInvModelView %outLightVec %outViewVec
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUVW "outUVW"
               OpName %inPos "inPos"
               OpName %type "type"
               OpName %outPos "outPos"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "skybox_modelview"
               OpMemberName %UBO 3 "modelscale"
               OpName %ubo "ubo"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %outInvModelView "outInvModelView"
               OpName %lightPos "lightPos"
               OpName %outLightVec "outLightVec"
               OpName %outViewVec "outViewVec"
               OpDecorate %outUVW Location 0
               OpDecorate %inPos Location 0
               OpDecorate %type SpecId 0
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
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
               OpDecorate %outNormal Location 2
               OpDecorate %inNormal Location 1
               OpDecorate %outInvModelView Location 5
               OpDecorate %outLightVec Location 4
               OpDecorate %outViewVec Location 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
     %outUVW = OpVariable %_ptr_Output_v3float Output
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
        %int = OpTypeInt 32 1
       %type = OpSpecConstant %int 0
     %outPos = OpVariable %_ptr_Output_v3float Output
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %mat4v4float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_2 = OpConstant %int 2
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%mat3v3float = OpTypeMatrix %v3float 3
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
      %int_1 = OpConstant %int 1
      %int_3 = OpConstant %int 3
%_ptr_Uniform_float = OpTypePointer Uniform %float
  %outNormal = OpVariable %_ptr_Output_v3float Output
   %inNormal = OpVariable %_ptr_Input_v3float Input
%_ptr_Output_mat4v4float = OpTypePointer Output %mat4v4float
%outInvModelView = OpVariable %_ptr_Output_mat4v4float Output
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %float_0 = OpConstant %float 0
   %float_n5 = OpConstant %float -5
    %float_5 = OpConstant %float 5
        %121 = OpConstantComposite %v3float %float_0 %float_n5 %float_5
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
   %lightPos = OpVariable %_ptr_Function_v3float Function
         %12 = OpLoad %v3float %inPos
               OpStore %outUVW %12
               OpSelectionMerge %17 None
               OpSwitch %type %17 0 %15 1 %16
         %15 = OpLabel
         %26 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_2
         %27 = OpLoad %mat4v4float %26
         %29 = OpCompositeExtract %v4float %27 0
         %30 = OpVectorShuffle %v3float %29 %29 0 1 2
         %31 = OpCompositeExtract %v4float %27 1
         %32 = OpVectorShuffle %v3float %31 %31 0 1 2
         %33 = OpCompositeExtract %v4float %27 2
         %34 = OpVectorShuffle %v3float %33 %33 0 1 2
         %35 = OpCompositeConstruct %mat3v3float %30 %32 %34
         %36 = OpLoad %v3float %inPos
         %37 = OpMatrixTimesVector %v3float %35 %36
         %38 = OpCompositeExtract %float %37 0
         %39 = OpCompositeExtract %float %37 1
         %40 = OpCompositeExtract %float %37 2
         %41 = OpCompositeConstruct %v3float %38 %39 %40
               OpStore %outPos %41
         %46 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %47 = OpLoad %mat4v4float %46
         %48 = OpLoad %v3float %outPos
         %50 = OpCompositeExtract %float %48 0
         %51 = OpCompositeExtract %float %48 1
         %52 = OpCompositeExtract %float %48 2
         %53 = OpCompositeConstruct %v4float %50 %51 %52 %float_1
         %54 = OpMatrixTimesVector %v4float %47 %53
         %55 = OpCompositeExtract %float %54 0
         %56 = OpCompositeExtract %float %54 1
         %57 = OpCompositeExtract %float %54 2
         %58 = OpCompositeExtract %float %54 3
         %59 = OpCompositeConstruct %v4float %55 %56 %57 %58
         %61 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %61 %59
               OpBranch %17
         %16 = OpLabel
         %64 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %65 = OpLoad %mat4v4float %64
         %66 = OpLoad %v3float %inPos
         %69 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %70 = OpLoad %float %69
         %71 = OpVectorTimesScalar %v3float %66 %70
         %72 = OpCompositeExtract %float %71 0
         %73 = OpCompositeExtract %float %71 1
         %74 = OpCompositeExtract %float %71 2
         %75 = OpCompositeConstruct %v4float %72 %73 %74 %float_1
         %76 = OpMatrixTimesVector %v4float %65 %75
         %77 = OpCompositeExtract %float %76 0
         %78 = OpCompositeExtract %float %76 1
         %79 = OpCompositeExtract %float %76 2
         %80 = OpCompositeConstruct %v3float %77 %78 %79
               OpStore %outPos %80
         %81 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %82 = OpLoad %mat4v4float %81
         %83 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %84 = OpLoad %mat4v4float %83
         %85 = OpMatrixTimesMatrix %mat4v4float %82 %84
         %86 = OpLoad %v3float %inPos
         %87 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %88 = OpLoad %float %87
         %89 = OpVectorTimesScalar %v3float %86 %88
         %90 = OpCompositeExtract %float %89 0
         %91 = OpCompositeExtract %float %89 1
         %92 = OpCompositeExtract %float %89 2
         %93 = OpCompositeConstruct %v4float %90 %91 %92 %float_1
         %94 = OpMatrixTimesVector %v4float %85 %93
         %95 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %95 %94
               OpBranch %17
         %17 = OpLabel
         %99 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %100 = OpLoad %mat4v4float %99
        %101 = OpCompositeExtract %v4float %100 0
        %102 = OpVectorShuffle %v3float %101 %101 0 1 2
        %103 = OpCompositeExtract %v4float %100 1
        %104 = OpVectorShuffle %v3float %103 %103 0 1 2
        %105 = OpCompositeExtract %v4float %100 2
        %106 = OpVectorShuffle %v3float %105 %105 0 1 2
        %107 = OpCompositeConstruct %mat3v3float %102 %104 %106
        %109 = OpLoad %v3float %inNormal
        %110 = OpMatrixTimesVector %v3float %107 %109
               OpStore %outNormal %110
        %113 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_2
        %114 = OpLoad %mat4v4float %113
        %115 = OpExtInst %mat4v4float %1 MatrixInverse %114
               OpStore %outInvModelView %115
               OpStore %lightPos %121
        %123 = OpLoad %v3float %lightPos
        %124 = OpLoad %v3float %outPos
        %125 = OpFSub %v3float %123 %124
               OpStore %outLightVec %125
        %127 = OpLoad %v3float %outPos
        %128 = OpFNegate %v3float %127
               OpStore %outViewVec %128
               OpReturn
               OpFunctionEnd
