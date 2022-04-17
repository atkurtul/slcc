; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 177
; Schema: 0
               OpCapability Tessellation
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint TessellationEvaluation %main "main" %inUV %gl_TessCoord %outUV %inNormal %outNormal %gl_in %_ %outViewVec %outLightVec %outWorldPos %outEyePos
               OpExecutionMode %main Quads
               OpExecutionMode %main SpacingEqual
               OpExecutionMode %main VertexOrderCw
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %uv1 "uv1"
               OpName %inUV "inUV"
               OpName %gl_TessCoord "gl_TessCoord"
               OpName %uv2 "uv2"
               OpName %outUV "outUV"
               OpName %n1 "n1"
               OpName %inNormal "inNormal"
               OpName %n2 "n2"
               OpName %outNormal "outNormal"
               OpName %pos1 "pos1"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %gl_in "gl_in"
               OpName %pos2 "pos2"
               OpName %pos "pos"
               OpName %displacementMap "displacementMap"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lightPos"
               OpMemberName %UBO 3 "frustumPlanes"
               OpMemberName %UBO 4 "displacementFactor"
               OpMemberName %UBO 5 "tessellationFactor"
               OpMemberName %UBO 6 "viewportDim"
               OpMemberName %UBO 7 "tessellatedEdgeSize"
               OpName %ubo "ubo"
               OpName %gl_PerVertex_0 "gl_PerVertex"
               OpMemberName %gl_PerVertex_0 0 "gl_Position"
               OpMemberName %gl_PerVertex_0 1 "gl_PointSize"
               OpMemberName %gl_PerVertex_0 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex_0 3 "gl_CullDistance"
               OpName %_ ""
               OpName %outViewVec "outViewVec"
               OpName %outLightVec "outLightVec"
               OpName %outWorldPos "outWorldPos"
               OpName %outEyePos "outEyePos"
               OpDecorate %inUV Location 1
               OpDecorate %gl_TessCoord BuiltIn TessCoord
               OpDecorate %outUV Location 1
               OpDecorate %inNormal Location 0
               OpDecorate %outNormal Location 0
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %displacementMap DescriptorSet 0
               OpDecorate %displacementMap Binding 1
               OpDecorate %_arr_v4float_uint_6 ArrayStride 16
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 144
               OpMemberDecorate %UBO 4 Offset 240
               OpMemberDecorate %UBO 5 Offset 244
               OpMemberDecorate %UBO 6 Offset 248
               OpMemberDecorate %UBO 7 Offset 256
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpMemberDecorate %gl_PerVertex_0 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex_0 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex_0 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex_0 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex_0 Block
               OpDecorate %outViewVec Location 2
               OpDecorate %outLightVec Location 3
               OpDecorate %outWorldPos Location 5
               OpDecorate %outEyePos Location 4
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
       %uint = OpTypeInt 32 0
    %uint_32 = OpConstant %uint 32
%_arr_v2float_uint_32 = OpTypeArray %v2float %uint_32
%_ptr_Input__arr_v2float_uint_32 = OpTypePointer Input %_arr_v2float_uint_32
       %inUV = OpVariable %_ptr_Input__arr_v2float_uint_32 Input
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %int_1 = OpConstant %int 1
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
%gl_TessCoord = OpVariable %_ptr_Input_v3float Input
     %uint_0 = OpConstant %uint 0
%_ptr_Input_float = OpTypePointer Input %float
      %int_3 = OpConstant %int 3
      %int_2 = OpConstant %int 2
%_ptr_Output_v2float = OpTypePointer Output %v2float
      %outUV = OpVariable %_ptr_Output_v2float Output
     %uint_1 = OpConstant %uint 1
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_arr_v3float_uint_32 = OpTypeArray %v3float %uint_32
%_ptr_Input__arr_v3float_uint_32 = OpTypePointer Input %_arr_v3float_uint_32
   %inNormal = OpVariable %_ptr_Input__arr_v3float_uint_32 Input
%_ptr_Output_v3float = OpTypePointer Output %v3float
  %outNormal = OpVariable %_ptr_Output_v3float Output
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_arr_gl_PerVertex_uint_32 = OpTypeArray %gl_PerVertex %uint_32
%_ptr_Input__arr_gl_PerVertex_uint_32 = OpTypePointer Input %_arr_gl_PerVertex_uint_32
      %gl_in = OpVariable %_ptr_Input__arr_gl_PerVertex_uint_32 Input
%_ptr_Input_v4float = OpTypePointer Input %v4float
        %115 = OpTypeImage %float 2D 0 0 0 1 Unknown
        %116 = OpTypeSampledImage %115
%_ptr_UniformConstant_116 = OpTypePointer UniformConstant %116
%displacementMap = OpVariable %_ptr_UniformConstant_116 UniformConstant
    %float_0 = OpConstant %float 0
%mat4v4float = OpTypeMatrix %v4float 4
     %uint_6 = OpConstant %uint 6
%_arr_v4float_uint_6 = OpTypeArray %v4float %uint_6
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v4float %_arr_v4float_uint_6 %float %float %v2float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_4 = OpConstant %int 4
%_ptr_Uniform_float = OpTypePointer Uniform %float
%_ptr_Function_float = OpTypePointer Function %float
%gl_PerVertex_0 = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex_0 = OpTypePointer Output %gl_PerVertex_0
          %_ = OpVariable %_ptr_Output_gl_PerVertex_0 Output
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Output_v4float = OpTypePointer Output %v4float
 %outViewVec = OpVariable %_ptr_Output_v3float Output
%outLightVec = OpVariable %_ptr_Output_v3float Output
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
%outWorldPos = OpVariable %_ptr_Output_v3float Output
  %outEyePos = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
        %uv1 = OpVariable %_ptr_Function_v2float Function
        %uv2 = OpVariable %_ptr_Function_v2float Function
         %n1 = OpVariable %_ptr_Function_v3float Function
         %n2 = OpVariable %_ptr_Function_v3float Function
       %pos1 = OpVariable %_ptr_Function_v4float Function
       %pos2 = OpVariable %_ptr_Function_v4float Function
        %pos = OpVariable %_ptr_Function_v4float Function
         %18 = OpAccessChain %_ptr_Input_v2float %inUV %int_0
         %19 = OpLoad %v2float %18
         %21 = OpAccessChain %_ptr_Input_v2float %inUV %int_1
         %22 = OpLoad %v2float %21
         %28 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
         %29 = OpLoad %float %28
         %30 = OpCompositeConstruct %v2float %29 %29
         %31 = OpExtInst %v2float %1 FMix %19 %22 %30
               OpStore %uv1 %31
         %34 = OpAccessChain %_ptr_Input_v2float %inUV %int_3
         %35 = OpLoad %v2float %34
         %37 = OpAccessChain %_ptr_Input_v2float %inUV %int_2
         %38 = OpLoad %v2float %37
         %39 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
         %40 = OpLoad %float %39
         %41 = OpCompositeConstruct %v2float %40 %40
         %42 = OpExtInst %v2float %1 FMix %35 %38 %41
               OpStore %uv2 %42
         %45 = OpLoad %v2float %uv1
         %46 = OpLoad %v2float %uv2
         %48 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_1
         %49 = OpLoad %float %48
         %50 = OpCompositeConstruct %v2float %49 %49
         %51 = OpExtInst %v2float %1 FMix %45 %46 %50
               OpStore %outUV %51
         %57 = OpAccessChain %_ptr_Input_v3float %inNormal %int_0
         %58 = OpLoad %v3float %57
         %59 = OpAccessChain %_ptr_Input_v3float %inNormal %int_1
         %60 = OpLoad %v3float %59
         %61 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
         %62 = OpLoad %float %61
         %63 = OpCompositeConstruct %v3float %62 %62 %62
         %64 = OpExtInst %v3float %1 FMix %58 %60 %63
               OpStore %n1 %64
         %66 = OpAccessChain %_ptr_Input_v3float %inNormal %int_3
         %67 = OpLoad %v3float %66
         %68 = OpAccessChain %_ptr_Input_v3float %inNormal %int_2
         %69 = OpLoad %v3float %68
         %70 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
         %71 = OpLoad %float %70
         %72 = OpCompositeConstruct %v3float %71 %71 %71
         %73 = OpExtInst %v3float %1 FMix %67 %69 %72
               OpStore %n2 %73
         %76 = OpLoad %v3float %n1
         %77 = OpLoad %v3float %n2
         %78 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_1
         %79 = OpLoad %float %78
         %80 = OpCompositeConstruct %v3float %79 %79 %79
         %81 = OpExtInst %v3float %1 FMix %76 %77 %80
               OpStore %outNormal %81
         %91 = OpAccessChain %_ptr_Input_v4float %gl_in %int_0 %int_0
         %92 = OpLoad %v4float %91
         %93 = OpAccessChain %_ptr_Input_v4float %gl_in %int_1 %int_0
         %94 = OpLoad %v4float %93
         %95 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
         %96 = OpLoad %float %95
         %97 = OpCompositeConstruct %v4float %96 %96 %96 %96
         %98 = OpExtInst %v4float %1 FMix %92 %94 %97
               OpStore %pos1 %98
        %100 = OpAccessChain %_ptr_Input_v4float %gl_in %int_3 %int_0
        %101 = OpLoad %v4float %100
        %102 = OpAccessChain %_ptr_Input_v4float %gl_in %int_2 %int_0
        %103 = OpLoad %v4float %102
        %104 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_0
        %105 = OpLoad %float %104
        %106 = OpCompositeConstruct %v4float %105 %105 %105 %105
        %107 = OpExtInst %v4float %1 FMix %101 %103 %106
               OpStore %pos2 %107
        %109 = OpLoad %v4float %pos1
        %110 = OpLoad %v4float %pos2
        %111 = OpAccessChain %_ptr_Input_float %gl_TessCoord %uint_1
        %112 = OpLoad %float %111
        %113 = OpCompositeConstruct %v4float %112 %112 %112 %112
        %114 = OpExtInst %v4float %1 FMix %109 %110 %113
               OpStore %pos %114
        %119 = OpLoad %116 %displacementMap
        %120 = OpLoad %v2float %outUV
        %122 = OpImageSampleExplicitLod %v4float %119 %120 Lod %float_0
        %123 = OpCompositeExtract %float %122 0
        %132 = OpAccessChain %_ptr_Uniform_float %ubo %int_4
        %133 = OpLoad %float %132
        %134 = OpFMul %float %123 %133
        %136 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %137 = OpLoad %float %136
        %138 = OpFSub %float %137 %134
        %139 = OpAccessChain %_ptr_Function_float %pos %uint_1
               OpStore %139 %138
        %144 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
        %145 = OpLoad %mat4v4float %144
        %146 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %147 = OpLoad %mat4v4float %146
        %148 = OpMatrixTimesMatrix %mat4v4float %145 %147
        %149 = OpLoad %v4float %pos
        %150 = OpMatrixTimesVector %v4float %148 %149
        %152 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %152 %150
        %154 = OpLoad %v4float %pos
        %155 = OpVectorShuffle %v3float %154 %154 0 1 2
        %156 = OpFNegate %v3float %155
               OpStore %outViewVec %156
        %159 = OpAccessChain %_ptr_Uniform_v4float %ubo %int_2
        %160 = OpLoad %v4float %159
        %161 = OpVectorShuffle %v3float %160 %160 0 1 2
        %162 = OpLoad %v3float %outViewVec
        %163 = OpFAdd %v3float %161 %162
        %164 = OpExtInst %v3float %1 Normalize %163
               OpStore %outLightVec %164
        %166 = OpLoad %v4float %pos
        %167 = OpVectorShuffle %v3float %166 %166 0 1 2
               OpStore %outWorldPos %167
        %169 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %170 = OpLoad %mat4v4float %169
        %171 = OpLoad %v4float %pos
        %172 = OpMatrixTimesVector %v4float %170 %171
        %173 = OpCompositeExtract %float %172 0
        %174 = OpCompositeExtract %float %172 1
        %175 = OpCompositeExtract %float %172 2
        %176 = OpCompositeConstruct %v3float %173 %174 %175
               OpStore %outEyePos %176
               OpReturn
               OpFunctionEnd
