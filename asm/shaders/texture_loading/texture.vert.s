; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 118
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %inUV %outLodBias %inPos %_ %outNormal %inNormal %outLightVec %outViewVec
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %outLodBias "outLodBias"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "model"
               OpMemberName %UBO 2 "viewPos"
               OpMemberName %UBO 3 "lodBias"
               OpName %ubo "ubo"
               OpName %worldPos "worldPos"
               OpName %inPos "inPos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpName %pos "pos"
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %lightPos "lightPos"
               OpName %lPos "lPos"
               OpName %outLightVec "outLightVec"
               OpName %outViewVec "outViewVec"
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 1
               OpDecorate %outLodBias Location 1
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 144
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inPos Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
               OpDecorate %outNormal Location 2
               OpDecorate %inNormal Location 2
               OpDecorate %outLightVec Location 4
               OpDecorate %outViewVec Location 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_float = OpTypePointer Output %float
 %outLodBias = OpVariable %_ptr_Output_float Output
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v4float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
        %int = OpTypeInt 32 1
      %int_3 = OpConstant %int 3
%_ptr_Uniform_float = OpTypePointer Uniform %float
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Function_v4float = OpTypePointer Function %v4float
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %outNormal = OpVariable %_ptr_Output_v3float Output
%mat3v3float = OpTypeMatrix %v3float 3
   %inNormal = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
         %91 = OpConstantComposite %v3float %float_0 %float_0 %float_0
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
      %int_2 = OpConstant %int 2
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
   %worldPos = OpVariable %_ptr_Function_v3float Function
        %pos = OpVariable %_ptr_Function_v4float Function
   %lightPos = OpVariable %_ptr_Function_v3float Function
       %lPos = OpVariable %_ptr_Function_v3float Function
         %12 = OpLoad %v2float %inUV
               OpStore %outUV %12
         %23 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %24 = OpLoad %float %23
               OpStore %outLodBias %24
         %30 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %31 = OpLoad %mat4v4float %30
         %34 = OpLoad %v3float %inPos
         %36 = OpCompositeExtract %float %34 0
         %37 = OpCompositeExtract %float %34 1
         %38 = OpCompositeExtract %float %34 2
         %39 = OpCompositeConstruct %v4float %36 %37 %38 %float_1
         %40 = OpMatrixTimesVector %v4float %31 %39
         %41 = OpCompositeExtract %float %40 0
         %42 = OpCompositeExtract %float %40 1
         %43 = OpCompositeExtract %float %40 2
         %44 = OpCompositeConstruct %v3float %41 %42 %43
               OpStore %worldPos %44
         %49 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %50 = OpLoad %mat4v4float %49
         %51 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %52 = OpLoad %mat4v4float %51
         %53 = OpMatrixTimesMatrix %mat4v4float %50 %52
         %54 = OpLoad %v3float %inPos
         %55 = OpCompositeExtract %float %54 0
         %56 = OpCompositeExtract %float %54 1
         %57 = OpCompositeExtract %float %54 2
         %58 = OpCompositeConstruct %v4float %55 %56 %57 %float_1
         %59 = OpMatrixTimesVector %v4float %53 %58
         %61 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %61 %59
         %64 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %65 = OpLoad %mat4v4float %64
         %66 = OpLoad %v3float %inPos
         %67 = OpCompositeExtract %float %66 0
         %68 = OpCompositeExtract %float %66 1
         %69 = OpCompositeExtract %float %66 2
         %70 = OpCompositeConstruct %v4float %67 %68 %69 %float_1
         %71 = OpMatrixTimesVector %v4float %65 %70
               OpStore %pos %71
         %74 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %75 = OpLoad %mat4v4float %74
         %76 = OpTranspose %mat4v4float %75
         %77 = OpExtInst %mat4v4float %1 MatrixInverse %76
         %79 = OpCompositeExtract %v4float %77 0
         %80 = OpVectorShuffle %v3float %79 %79 0 1 2
         %81 = OpCompositeExtract %v4float %77 1
         %82 = OpVectorShuffle %v3float %81 %81 0 1 2
         %83 = OpCompositeExtract %v4float %77 2
         %84 = OpVectorShuffle %v3float %83 %83 0 1 2
         %85 = OpCompositeConstruct %mat3v3float %80 %82 %84
         %87 = OpLoad %v3float %inNormal
         %88 = OpMatrixTimesVector %v3float %85 %87
               OpStore %outNormal %88
               OpStore %lightPos %91
         %93 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %94 = OpLoad %mat4v4float %93
         %95 = OpCompositeExtract %v4float %94 0
         %96 = OpVectorShuffle %v3float %95 %95 0 1 2
         %97 = OpCompositeExtract %v4float %94 1
         %98 = OpVectorShuffle %v3float %97 %97 0 1 2
         %99 = OpCompositeExtract %v4float %94 2
        %100 = OpVectorShuffle %v3float %99 %99 0 1 2
        %101 = OpCompositeConstruct %mat3v3float %96 %98 %100
        %102 = OpLoad %v3float %lightPos
        %103 = OpMatrixTimesVector %v3float %101 %102
               OpStore %lPos %103
        %105 = OpLoad %v3float %lPos
        %106 = OpLoad %v4float %pos
        %107 = OpVectorShuffle %v3float %106 %106 0 1 2
        %108 = OpFSub %v3float %105 %107
               OpStore %outLightVec %108
        %112 = OpAccessChain %_ptr_Uniform_v4float %ubo %int_2
        %113 = OpLoad %v4float %112
        %114 = OpVectorShuffle %v3float %113 %113 0 1 2
        %115 = OpLoad %v4float %pos
        %116 = OpVectorShuffle %v3float %115 %115 0 1 2
        %117 = OpFSub %v3float %114 %116
               OpStore %outViewVec %117
               OpReturn
               OpFunctionEnd
