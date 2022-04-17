; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 77
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %o_pos %position %o_uv %texcoord_0 %o_normal %normal %_
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %o_pos "o_pos"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "model"
               OpMemberName %GlobalUniform 1 "view_proj"
               OpMemberName %GlobalUniform 2 "camera_position"
               OpName %global_uniform "global_uniform"
               OpName %position "position"
               OpName %o_uv "o_uv"
               OpName %texcoord_0 "texcoord_0"
               OpName %o_normal "o_normal"
               OpName %normal "normal"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpName %_ ""
               OpName %Light "Light"
               OpMemberName %Light 0 "position"
               OpMemberName %Light 1 "color"
               OpName %LightsInfo "LightsInfo"
               OpMemberName %LightsInfo 0 "count"
               OpMemberName %LightsInfo 1 "lights"
               OpName %lights "lights"
               OpDecorate %o_pos Location 0
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
               OpDecorate %position Location 0
               OpDecorate %o_uv Location 1
               OpDecorate %texcoord_0 Location 1
               OpDecorate %o_normal Location 2
               OpDecorate %normal Location 2
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpDecorate %gl_PerVertex Block
               OpMemberDecorate %Light 0 Offset 0
               OpMemberDecorate %Light 1 Offset 16
               OpDecorate %_arr_Light_uint_16 ArrayStride 32
               OpMemberDecorate %LightsInfo 0 Offset 0
               OpMemberDecorate %LightsInfo 1 Offset 16
               OpDecorate %LightsInfo Block
               OpDecorate %lights DescriptorSet 0
               OpDecorate %lights Binding 4
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Output_v3float = OpTypePointer Output %v3float
      %o_pos = OpVariable %_ptr_Output_v3float Output
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %v3float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %position = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
    %v2float = OpTypeVector %float 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
       %o_uv = OpVariable %_ptr_Output_v2float Output
%_ptr_Input_v2float = OpTypePointer Input %v2float
 %texcoord_0 = OpVariable %_ptr_Input_v2float Input
   %o_normal = OpVariable %_ptr_Output_v3float Output
%mat3v3float = OpTypeMatrix %v3float 3
     %normal = OpVariable %_ptr_Input_v3float Input
%gl_PerVertex = OpTypeStruct %v4float %float
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
      %int_1 = OpConstant %int 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %uint = OpTypeInt 32 0
      %Light = OpTypeStruct %v4float %v4float
    %uint_16 = OpConstant %uint 16
%_arr_Light_uint_16 = OpTypeArray %Light %uint_16
 %LightsInfo = OpTypeStruct %uint %_arr_Light_uint_16
%_ptr_Uniform_LightsInfo = OpTypePointer Uniform %LightsInfo
     %lights = OpVariable %_ptr_Uniform_LightsInfo Uniform
       %main = OpFunction %void None %3
          %5 = OpLabel
         %18 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %19 = OpLoad %mat4v4float %18
         %22 = OpLoad %v3float %position
         %24 = OpCompositeExtract %float %22 0
         %25 = OpCompositeExtract %float %22 1
         %26 = OpCompositeExtract %float %22 2
         %27 = OpCompositeConstruct %v4float %24 %25 %26 %float_1
         %28 = OpMatrixTimesVector %v4float %19 %27
         %29 = OpCompositeExtract %float %28 0
         %30 = OpCompositeExtract %float %28 1
         %31 = OpCompositeExtract %float %28 2
         %32 = OpCompositeConstruct %v3float %29 %30 %31
               OpStore %o_pos %32
         %38 = OpLoad %v2float %texcoord_0
               OpStore %o_uv %38
         %40 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %41 = OpLoad %mat4v4float %40
         %43 = OpCompositeExtract %v4float %41 0
         %44 = OpVectorShuffle %v3float %43 %43 0 1 2
         %45 = OpCompositeExtract %v4float %41 1
         %46 = OpVectorShuffle %v3float %45 %45 0 1 2
         %47 = OpCompositeExtract %v4float %41 2
         %48 = OpVectorShuffle %v3float %47 %47 0 1 2
         %49 = OpCompositeConstruct %mat3v3float %44 %46 %48
         %51 = OpLoad %v3float %normal
         %52 = OpMatrixTimesVector %v3float %49 %51
               OpStore %o_normal %52
         %57 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_1
         %58 = OpLoad %mat4v4float %57
         %59 = OpAccessChain %_ptr_Uniform_mat4v4float %global_uniform %int_0
         %60 = OpLoad %mat4v4float %59
         %61 = OpMatrixTimesMatrix %mat4v4float %58 %60
         %62 = OpLoad %v3float %position
         %63 = OpCompositeExtract %float %62 0
         %64 = OpCompositeExtract %float %62 1
         %65 = OpCompositeExtract %float %62 2
         %66 = OpCompositeConstruct %v4float %63 %64 %65 %float_1
         %67 = OpMatrixTimesVector %v4float %61 %66
         %69 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %69 %67
               OpReturn
               OpFunctionEnd
