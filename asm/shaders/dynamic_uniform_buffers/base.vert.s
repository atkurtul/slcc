; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 62
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outColor %inColor %inPos %_
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outColor "outColor"
               OpName %inColor "inColor"
               OpName %modelView "modelView"
               OpName %UboView "UboView"
               OpMemberName %UboView 0 "projection"
               OpMemberName %UboView 1 "view"
               OpName %uboView "uboView"
               OpName %UboInstance "UboInstance"
               OpMemberName %UboInstance 0 "model"
               OpName %uboInstance "uboInstance"
               OpName %worldPos "worldPos"
               OpName %inPos "inPos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpName %_ ""
               OpDecorate %outColor Location 0
               OpDecorate %inColor Location 1
               OpMemberDecorate %UboView 0 ColMajor
               OpMemberDecorate %UboView 0 Offset 0
               OpMemberDecorate %UboView 0 MatrixStride 16
               OpMemberDecorate %UboView 1 ColMajor
               OpMemberDecorate %UboView 1 Offset 64
               OpMemberDecorate %UboView 1 MatrixStride 16
               OpDecorate %UboView Block
               OpDecorate %uboView DescriptorSet 0
               OpDecorate %uboView Binding 0
               OpMemberDecorate %UboInstance 0 ColMajor
               OpMemberDecorate %UboInstance 0 Offset 0
               OpMemberDecorate %UboInstance 0 MatrixStride 16
               OpDecorate %UboInstance Block
               OpDecorate %uboInstance DescriptorSet 0
               OpDecorate %uboInstance Binding 1
               OpDecorate %inPos Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
   %outColor = OpVariable %_ptr_Output_v3float Output
%_ptr_Input_v3float = OpTypePointer Input %v3float
    %inColor = OpVariable %_ptr_Input_v3float Input
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
%_ptr_Function_mat4v4float = OpTypePointer Function %mat4v4float
    %UboView = OpTypeStruct %mat4v4float %mat4v4float
%_ptr_Uniform_UboView = OpTypePointer Uniform %UboView
    %uboView = OpVariable %_ptr_Uniform_UboView Uniform
        %int = OpTypeInt 32 1
      %int_1 = OpConstant %int 1
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%UboInstance = OpTypeStruct %mat4v4float
%_ptr_Uniform_UboInstance = OpTypePointer Uniform %UboInstance
%uboInstance = OpVariable %_ptr_Uniform_UboInstance Uniform
      %int_0 = OpConstant %int 0
%_ptr_Function_v3float = OpTypePointer Function %v3float
      %inPos = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%gl_PerVertex = OpTypeStruct %v4float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
  %modelView = OpVariable %_ptr_Function_mat4v4float Function
   %worldPos = OpVariable %_ptr_Function_v3float Function
         %12 = OpLoad %v3float %inColor
               OpStore %outColor %12
         %23 = OpAccessChain %_ptr_Uniform_mat4v4float %uboView %int_1
         %24 = OpLoad %mat4v4float %23
         %29 = OpAccessChain %_ptr_Uniform_mat4v4float %uboInstance %int_0
         %30 = OpLoad %mat4v4float %29
         %31 = OpMatrixTimesMatrix %mat4v4float %24 %30
               OpStore %modelView %31
         %34 = OpLoad %mat4v4float %modelView
         %36 = OpLoad %v3float %inPos
         %38 = OpCompositeExtract %float %36 0
         %39 = OpCompositeExtract %float %36 1
         %40 = OpCompositeExtract %float %36 2
         %41 = OpCompositeConstruct %v4float %38 %39 %40 %float_1
         %42 = OpMatrixTimesVector %v4float %34 %41
         %43 = OpCompositeExtract %float %42 0
         %44 = OpCompositeExtract %float %42 1
         %45 = OpCompositeExtract %float %42 2
         %46 = OpCompositeConstruct %v3float %43 %44 %45
               OpStore %worldPos %46
         %50 = OpAccessChain %_ptr_Uniform_mat4v4float %uboView %int_0
         %51 = OpLoad %mat4v4float %50
         %52 = OpLoad %mat4v4float %modelView
         %53 = OpMatrixTimesMatrix %mat4v4float %51 %52
         %54 = OpLoad %v3float %inPos
         %55 = OpCompositeExtract %float %54 0
         %56 = OpCompositeExtract %float %54 1
         %57 = OpCompositeExtract %float %54 2
         %58 = OpCompositeConstruct %v4float %55 %56 %57 %float_1
         %59 = OpMatrixTimesVector %v4float %53 %58
         %61 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %61 %59
               OpReturn
               OpFunctionEnd
