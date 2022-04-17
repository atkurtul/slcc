; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 102
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outColor %outUV %inUV %_ %inPos %outNormal %inNormal %outLightVec %outViewVec
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outColor "outColor"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lightPos"
               OpName %ubo "ubo"
               OpName %inPos "inPos"
               OpName %pos "pos"
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %lPos "lPos"
               OpName %outLightVec "outLightVec"
               OpName %outViewVec "outViewVec"
               OpDecorate %outColor Location 1
               OpDecorate %outUV Location 2
               OpDecorate %inUV Location 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inPos Location 0
               OpDecorate %outNormal Location 0
               OpDecorate %inNormal Location 1
               OpDecorate %outLightVec Location 4
               OpDecorate %outViewVec Location 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
   %outColor = OpVariable %_ptr_Output_v3float Output
    %float_1 = OpConstant %float 1
         %11 = OpConstantComposite %v3float %float_1 %float_1 %float_1
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
   %float_10 = OpConstant %float 10
    %float_6 = OpConstant %float 6
         %20 = OpConstantComposite %v2float %float_10 %float_6
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v4float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
      %int_1 = OpConstant %int 1
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Function_v4float = OpTypePointer Function %v4float
  %outNormal = OpVariable %_ptr_Output_v3float Output
%mat3v3float = OpTypeMatrix %v3float 3
   %inNormal = OpVariable %_ptr_Input_v3float Input
%_ptr_Function_v3float = OpTypePointer Function %v3float
      %int_2 = OpConstant %int 2
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
        %pos = OpVariable %_ptr_Function_v4float Function
       %lPos = OpVariable %_ptr_Function_v3float Function
               OpStore %outColor %11
         %17 = OpLoad %v2float %inUV
         %21 = OpFMul %v2float %17 %20
               OpStore %outUV %21
         %36 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %37 = OpLoad %mat4v4float %36
         %39 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %40 = OpLoad %mat4v4float %39
         %41 = OpMatrixTimesMatrix %mat4v4float %37 %40
         %44 = OpLoad %v3float %inPos
         %45 = OpCompositeExtract %float %44 0
         %46 = OpCompositeExtract %float %44 1
         %47 = OpCompositeExtract %float %44 2
         %48 = OpCompositeConstruct %v4float %45 %46 %47 %float_1
         %49 = OpMatrixTimesVector %v4float %41 %48
         %51 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %51 %49
         %54 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %55 = OpLoad %mat4v4float %54
         %56 = OpLoad %v3float %inPos
         %57 = OpCompositeExtract %float %56 0
         %58 = OpCompositeExtract %float %56 1
         %59 = OpCompositeExtract %float %56 2
         %60 = OpCompositeConstruct %v4float %57 %58 %59 %float_1
         %61 = OpMatrixTimesVector %v4float %55 %60
               OpStore %pos %61
         %63 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %64 = OpLoad %mat4v4float %63
         %66 = OpCompositeExtract %v4float %64 0
         %67 = OpVectorShuffle %v3float %66 %66 0 1 2
         %68 = OpCompositeExtract %v4float %64 1
         %69 = OpVectorShuffle %v3float %68 %68 0 1 2
         %70 = OpCompositeExtract %v4float %64 2
         %71 = OpVectorShuffle %v3float %70 %70 0 1 2
         %72 = OpCompositeConstruct %mat3v3float %67 %69 %71
         %74 = OpLoad %v3float %inNormal
         %75 = OpMatrixTimesVector %v3float %72 %74
               OpStore %outNormal %75
         %78 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %79 = OpLoad %mat4v4float %78
         %80 = OpCompositeExtract %v4float %79 0
         %81 = OpVectorShuffle %v3float %80 %80 0 1 2
         %82 = OpCompositeExtract %v4float %79 1
         %83 = OpVectorShuffle %v3float %82 %82 0 1 2
         %84 = OpCompositeExtract %v4float %79 2
         %85 = OpVectorShuffle %v3float %84 %84 0 1 2
         %86 = OpCompositeConstruct %mat3v3float %81 %83 %85
         %89 = OpAccessChain %_ptr_Uniform_v4float %ubo %int_2
         %90 = OpLoad %v4float %89
         %91 = OpVectorShuffle %v3float %90 %90 0 1 2
         %92 = OpMatrixTimesVector %v3float %86 %91
               OpStore %lPos %92
         %94 = OpLoad %v3float %lPos
         %95 = OpLoad %v4float %pos
         %96 = OpVectorShuffle %v3float %95 %95 0 1 2
         %97 = OpFSub %v3float %94 %96
               OpStore %outLightVec %97
         %99 = OpLoad %v4float %pos
        %100 = OpVectorShuffle %v3float %99 %99 0 1 2
        %101 = OpFNegate %v3float %100
               OpStore %outViewVec %101
               OpReturn
               OpFunctionEnd
