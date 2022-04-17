; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 86
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %inPos %_ %outGradientPos %inVel
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %spriteSize "spriteSize"
               OpName %inPos "inPos"
               OpName %eyePos "eyePos"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "screendim"
               OpName %ubo "ubo"
               OpName %projectedCorner "projectedCorner"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %outGradientPos "outGradientPos"
               OpName %inVel "inVel"
               OpDecorate %inPos Location 0
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %outGradientPos Location 0
               OpDecorate %inVel Location 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
%_ptr_Function_float = OpTypePointer Function %float
%float_0_00499999989 = OpConstant %float 0.00499999989
    %v4float = OpTypeVector %float 4
%_ptr_Input_v4float = OpTypePointer Input %v4float
      %inPos = OpVariable %_ptr_Input_v4float Input
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
%_ptr_Input_float = OpTypePointer Input %float
%_ptr_Function_v4float = OpTypePointer Function %v4float
%mat4v4float = OpTypeMatrix %v4float 4
    %v2float = OpTypeVector %float 2
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v2float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
        %int = OpTypeInt 32 1
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
     %uint_0 = OpConstant %uint 0
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
    %float_1 = OpConstant %float 1
      %int_0 = OpConstant %int 0
  %float_0_5 = OpConstant %float 0.5
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_2 = OpConstant %int 2
%_ptr_Uniform_float = OpTypePointer Uniform %float
  %float_128 = OpConstant %float 128
%_ptr_Output_float = OpTypePointer Output %float
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outGradientPos = OpVariable %_ptr_Output_float Output
      %inVel = OpVariable %_ptr_Input_v4float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
 %spriteSize = OpVariable %_ptr_Function_float Function
     %eyePos = OpVariable %_ptr_Function_v4float Function
%projectedCorner = OpVariable %_ptr_Function_v4float Function
         %16 = OpAccessChain %_ptr_Input_float %inPos %uint_3
         %17 = OpLoad %float %16
         %18 = OpFMul %float %float_0_00499999989 %17
               OpStore %spriteSize %18
         %29 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
         %30 = OpLoad %mat4v4float %29
         %32 = OpAccessChain %_ptr_Input_float %inPos %uint_0
         %33 = OpLoad %float %32
         %35 = OpAccessChain %_ptr_Input_float %inPos %uint_1
         %36 = OpLoad %float %35
         %38 = OpAccessChain %_ptr_Input_float %inPos %uint_2
         %39 = OpLoad %float %38
         %41 = OpCompositeConstruct %v4float %33 %36 %39 %float_1
         %42 = OpMatrixTimesVector %v4float %30 %41
               OpStore %eyePos %42
         %45 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %46 = OpLoad %mat4v4float %45
         %48 = OpLoad %float %spriteSize
         %49 = OpFMul %float %float_0_5 %48
         %50 = OpLoad %float %spriteSize
         %51 = OpFMul %float %float_0_5 %50
         %52 = OpAccessChain %_ptr_Function_float %eyePos %uint_2
         %53 = OpLoad %float %52
         %54 = OpAccessChain %_ptr_Function_float %eyePos %uint_3
         %55 = OpLoad %float %54
         %56 = OpCompositeConstruct %v4float %49 %51 %53 %55
         %57 = OpMatrixTimesVector %v4float %46 %56
               OpStore %projectedCorner %57
         %64 = OpAccessChain %_ptr_Uniform_float %ubo %int_2 %uint_0
         %65 = OpLoad %float %64
         %66 = OpAccessChain %_ptr_Function_float %projectedCorner %uint_0
         %67 = OpLoad %float %66
         %68 = OpFMul %float %65 %67
         %69 = OpAccessChain %_ptr_Function_float %projectedCorner %uint_3
         %70 = OpLoad %float %69
         %71 = OpFDiv %float %68 %70
         %73 = OpExtInst %float %1 FClamp %71 %float_1 %float_128
         %75 = OpAccessChain %_ptr_Output_float %_ %int_1
               OpStore %75 %73
         %76 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
         %77 = OpLoad %mat4v4float %76
         %78 = OpLoad %v4float %eyePos
         %79 = OpMatrixTimesVector %v4float %77 %78
         %81 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %81 %79
         %84 = OpAccessChain %_ptr_Input_float %inVel %uint_3
         %85 = OpLoad %float %84
               OpStore %outGradientPos %85
               OpReturn
               OpFunctionEnd
