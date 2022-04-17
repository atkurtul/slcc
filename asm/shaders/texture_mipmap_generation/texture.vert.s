; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 61
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %inUV %inPos %_ %outNormal
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %worldPos "worldPos"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lodBias"
               OpMemberName %UBO 3 "samplerIndex"
               OpName %ubo "ubo"
               OpName %inPos "inPos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %outNormal "outNormal"
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 1
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 132
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inPos Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %outNormal Location 3
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
        %int = OpTypeInt 32 1
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
%_ptr_Output_v4float = OpTypePointer Output %v4float
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %outNormal = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
   %worldPos = OpVariable %_ptr_Function_v3float Function
         %12 = OpLoad %v2float %inUV
               OpStore %outUV %12
         %24 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %25 = OpLoad %mat4v4float %24
         %28 = OpLoad %v3float %inPos
         %30 = OpCompositeExtract %float %28 0
         %31 = OpCompositeExtract %float %28 1
         %32 = OpCompositeExtract %float %28 2
         %33 = OpCompositeConstruct %v4float %30 %31 %32 %float_1
         %34 = OpMatrixTimesVector %v4float %25 %33
         %35 = OpCompositeExtract %float %34 0
         %36 = OpCompositeExtract %float %34 1
         %37 = OpCompositeExtract %float %34 2
         %38 = OpCompositeConstruct %v3float %35 %36 %37
               OpStore %worldPos %38
         %46 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %47 = OpLoad %mat4v4float %46
         %48 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %49 = OpLoad %mat4v4float %48
         %50 = OpMatrixTimesMatrix %mat4v4float %47 %49
         %51 = OpLoad %v3float %inPos
         %52 = OpCompositeExtract %float %51 0
         %53 = OpCompositeExtract %float %51 1
         %54 = OpCompositeExtract %float %51 2
         %55 = OpCompositeConstruct %v4float %52 %53 %54 %float_1
         %56 = OpMatrixTimesVector %v4float %50 %55
         %58 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %58 %56
               OpReturn
               OpFunctionEnd
