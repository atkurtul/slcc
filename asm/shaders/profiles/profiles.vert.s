; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 50
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outUV %inUV %outTexIndex %inTexIndex %_ %inPos
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %outTexIndex "outTexIndex"
               OpName %inTexIndex "inTexIndex"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "view"
               OpName %ubo "ubo"
               OpName %inPos "inPos"
               OpDecorate %outUV Location 0
               OpDecorate %inUV Location 1
               OpDecorate %outTexIndex Flat
               OpDecorate %outTexIndex Location 1
               OpDecorate %inTexIndex Location 2
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
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inPos Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
        %int = OpTypeInt 32 1
%_ptr_Output_int = OpTypePointer Output %int
%outTexIndex = OpVariable %_ptr_Output_int Output
%_ptr_Input_int = OpTypePointer Input %int
 %inTexIndex = OpVariable %_ptr_Input_int Input
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_0 = OpConstant %int 0
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
      %int_1 = OpConstant %int 1
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
         %12 = OpLoad %v2float %inUV
               OpStore %outUV %12
         %18 = OpLoad %int %inTexIndex
               OpStore %outTexIndex %18
         %32 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %33 = OpLoad %mat4v4float %32
         %35 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %36 = OpLoad %mat4v4float %35
         %37 = OpMatrixTimesMatrix %mat4v4float %33 %36
         %41 = OpLoad %v3float %inPos
         %43 = OpCompositeExtract %float %41 0
         %44 = OpCompositeExtract %float %41 1
         %45 = OpCompositeExtract %float %41 2
         %46 = OpCompositeConstruct %v4float %43 %44 %45 %float_1
         %47 = OpMatrixTimesVector %v4float %37 %46
         %49 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %49 %47
               OpReturn
               OpFunctionEnd
