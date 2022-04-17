; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 246
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %outColor %outUV %inUV %instanceTexIndex %instanceRot %inPos %instanceScale %instancePos %_ %outNormal %inNormal %outLightVec %outViewVec
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %outColor "outColor"
               OpName %outUV "outUV"
               OpName %inUV "inUV"
               OpName %instanceTexIndex "instanceTexIndex"
               OpName %s "s"
               OpName %instanceRot "instanceRot"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "projection"
               OpMemberName %UBO 1 "modelview"
               OpMemberName %UBO 2 "lightPos"
               OpMemberName %UBO 3 "locSpeed"
               OpMemberName %UBO 4 "globSpeed"
               OpName %ubo "ubo"
               OpName %c "c"
               OpName %mx "mx"
               OpName %my "my"
               OpName %mz "mz"
               OpName %rotMat "rotMat"
               OpName %gRotMat "gRotMat"
               OpName %locPos "locPos"
               OpName %inPos "inPos"
               OpName %pos "pos"
               OpName %instanceScale "instanceScale"
               OpName %instancePos "instancePos"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %outNormal "outNormal"
               OpName %inNormal "inNormal"
               OpName %lPos "lPos"
               OpName %outLightVec "outLightVec"
               OpName %outViewVec "outViewVec"
               OpDecorate %outColor Location 1
               OpDecorate %outUV Location 2
               OpDecorate %inUV Location 2
               OpDecorate %instanceTexIndex Location 7
               OpDecorate %instanceRot Location 5
               OpMemberDecorate %UBO 0 ColMajor
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 0 MatrixStride 16
               OpMemberDecorate %UBO 1 ColMajor
               OpMemberDecorate %UBO 1 Offset 64
               OpMemberDecorate %UBO 1 MatrixStride 16
               OpMemberDecorate %UBO 2 Offset 128
               OpMemberDecorate %UBO 3 Offset 144
               OpMemberDecorate %UBO 4 Offset 148
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 0
               OpDecorate %inPos Location 0
               OpDecorate %instanceScale Location 6
               OpDecorate %instancePos Location 4
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
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
      %outUV = OpVariable %_ptr_Output_v3float Output
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%instanceTexIndex = OpVariable %_ptr_Input_int Input
%_ptr_Function_float = OpTypePointer Function %float
%_ptr_Input_v3float = OpTypePointer Input %v3float
%instanceRot = OpVariable %_ptr_Input_v3float Input
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Input_float = OpTypePointer Input %float
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
        %UBO = OpTypeStruct %mat4v4float %mat4v4float %v4float %float %float
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
      %int_3 = OpConstant %int 3
%_ptr_Uniform_float = OpTypePointer Uniform %float
%mat3v3float = OpTypeMatrix %v3float 3
%_ptr_Function_mat3v3float = OpTypePointer Function %mat3v3float
      %int_0 = OpConstant %int 0
    %float_0 = OpConstant %float 0
%_ptr_Function_v3float = OpTypePointer Function %v3float
      %int_1 = OpConstant %int 1
      %int_2 = OpConstant %int 2
         %69 = OpConstantComposite %v3float %float_0 %float_0 %float_1
     %uint_1 = OpConstant %uint 1
         %89 = OpConstantComposite %v3float %float_0 %float_1 %float_0
     %uint_2 = OpConstant %uint 2
        %110 = OpConstantComposite %v3float %float_1 %float_0 %float_0
      %int_4 = OpConstant %int 4
%_ptr_Function_mat4v4float = OpTypePointer Function %mat4v4float
%_ptr_Function_v4float = OpTypePointer Function %v4float
        %147 = OpConstantComposite %v4float %float_0 %float_1 %float_0 %float_0
        %154 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_1
      %inPos = OpVariable %_ptr_Input_v3float Input
%instanceScale = OpVariable %_ptr_Input_float Input
%instancePos = OpVariable %_ptr_Input_v3float Input
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
%_ptr_Uniform_mat4v4float = OpTypePointer Uniform %mat4v4float
%_ptr_Output_v4float = OpTypePointer Output %v4float
  %outNormal = OpVariable %_ptr_Output_v3float Output
   %inNormal = OpVariable %_ptr_Input_v3float Input
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
%outLightVec = OpVariable %_ptr_Output_v3float Output
 %outViewVec = OpVariable %_ptr_Output_v3float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
          %s = OpVariable %_ptr_Function_float Function
          %c = OpVariable %_ptr_Function_float Function
         %mx = OpVariable %_ptr_Function_mat3v3float Function
         %my = OpVariable %_ptr_Function_mat3v3float Function
         %mz = OpVariable %_ptr_Function_mat3v3float Function
     %rotMat = OpVariable %_ptr_Function_mat3v3float Function
    %gRotMat = OpVariable %_ptr_Function_mat4v4float Function
     %locPos = OpVariable %_ptr_Function_v4float Function
        %pos = OpVariable %_ptr_Function_v4float Function
       %lPos = OpVariable %_ptr_Function_v3float Function
               OpStore %outColor %11
         %16 = OpLoad %v2float %inUV
         %20 = OpLoad %int %instanceTexIndex
         %21 = OpConvertSToF %float %20
         %22 = OpCompositeExtract %float %16 0
         %23 = OpCompositeExtract %float %16 1
         %24 = OpCompositeConstruct %v3float %22 %23 %21
               OpStore %outUV %24
         %32 = OpAccessChain %_ptr_Input_float %instanceRot %uint_0
         %33 = OpLoad %float %32
         %41 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %42 = OpLoad %float %41
         %43 = OpFAdd %float %33 %42
         %44 = OpExtInst %float %1 Sin %43
               OpStore %s %44
         %46 = OpAccessChain %_ptr_Input_float %instanceRot %uint_0
         %47 = OpLoad %float %46
         %48 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %49 = OpLoad %float %48
         %50 = OpFAdd %float %47 %49
         %51 = OpExtInst %float %1 Cos %50
               OpStore %c %51
         %56 = OpLoad %float %c
         %57 = OpLoad %float %s
         %59 = OpCompositeConstruct %v3float %56 %57 %float_0
         %61 = OpAccessChain %_ptr_Function_v3float %mx %int_0
               OpStore %61 %59
         %63 = OpLoad %float %s
         %64 = OpFNegate %float %63
         %65 = OpLoad %float %c
         %66 = OpCompositeConstruct %v3float %64 %65 %float_0
         %67 = OpAccessChain %_ptr_Function_v3float %mx %int_1
               OpStore %67 %66
         %70 = OpAccessChain %_ptr_Function_v3float %mx %int_2
               OpStore %70 %69
         %72 = OpAccessChain %_ptr_Input_float %instanceRot %uint_1
         %73 = OpLoad %float %72
         %74 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %75 = OpLoad %float %74
         %76 = OpFAdd %float %73 %75
         %77 = OpExtInst %float %1 Sin %76
               OpStore %s %77
         %78 = OpAccessChain %_ptr_Input_float %instanceRot %uint_1
         %79 = OpLoad %float %78
         %80 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
         %81 = OpLoad %float %80
         %82 = OpFAdd %float %79 %81
         %83 = OpExtInst %float %1 Cos %82
               OpStore %c %83
         %85 = OpLoad %float %c
         %86 = OpLoad %float %s
         %87 = OpCompositeConstruct %v3float %85 %float_0 %86
         %88 = OpAccessChain %_ptr_Function_v3float %my %int_0
               OpStore %88 %87
         %90 = OpAccessChain %_ptr_Function_v3float %my %int_1
               OpStore %90 %89
         %91 = OpLoad %float %s
         %92 = OpFNegate %float %91
         %93 = OpLoad %float %c
         %94 = OpCompositeConstruct %v3float %92 %float_0 %93
         %95 = OpAccessChain %_ptr_Function_v3float %my %int_2
               OpStore %95 %94
         %97 = OpAccessChain %_ptr_Input_float %instanceRot %uint_2
         %98 = OpLoad %float %97
         %99 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
        %100 = OpLoad %float %99
        %101 = OpFAdd %float %98 %100
        %102 = OpExtInst %float %1 Sin %101
               OpStore %s %102
        %103 = OpAccessChain %_ptr_Input_float %instanceRot %uint_2
        %104 = OpLoad %float %103
        %105 = OpAccessChain %_ptr_Uniform_float %ubo %int_3
        %106 = OpLoad %float %105
        %107 = OpFAdd %float %104 %106
        %108 = OpExtInst %float %1 Cos %107
               OpStore %c %108
        %111 = OpAccessChain %_ptr_Function_v3float %mz %int_0
               OpStore %111 %110
        %112 = OpLoad %float %c
        %113 = OpLoad %float %s
        %114 = OpCompositeConstruct %v3float %float_0 %112 %113
        %115 = OpAccessChain %_ptr_Function_v3float %mz %int_1
               OpStore %115 %114
        %116 = OpLoad %float %s
        %117 = OpFNegate %float %116
        %118 = OpLoad %float %c
        %119 = OpCompositeConstruct %v3float %float_0 %117 %118
        %120 = OpAccessChain %_ptr_Function_v3float %mz %int_2
               OpStore %120 %119
        %122 = OpLoad %mat3v3float %mz
        %123 = OpLoad %mat3v3float %my
        %124 = OpMatrixTimesMatrix %mat3v3float %122 %123
        %125 = OpLoad %mat3v3float %mx
        %126 = OpMatrixTimesMatrix %mat3v3float %124 %125
               OpStore %rotMat %126
        %127 = OpAccessChain %_ptr_Input_float %instanceRot %uint_1
        %128 = OpLoad %float %127
        %130 = OpAccessChain %_ptr_Uniform_float %ubo %int_4
        %131 = OpLoad %float %130
        %132 = OpFAdd %float %128 %131
        %133 = OpExtInst %float %1 Sin %132
               OpStore %s %133
        %134 = OpAccessChain %_ptr_Input_float %instanceRot %uint_1
        %135 = OpLoad %float %134
        %136 = OpAccessChain %_ptr_Uniform_float %ubo %int_4
        %137 = OpLoad %float %136
        %138 = OpFAdd %float %135 %137
        %139 = OpExtInst %float %1 Cos %138
               OpStore %c %139
        %142 = OpLoad %float %c
        %143 = OpLoad %float %s
        %144 = OpCompositeConstruct %v4float %142 %float_0 %143 %float_0
        %146 = OpAccessChain %_ptr_Function_v4float %gRotMat %int_0
               OpStore %146 %144
        %148 = OpAccessChain %_ptr_Function_v4float %gRotMat %int_1
               OpStore %148 %147
        %149 = OpLoad %float %s
        %150 = OpFNegate %float %149
        %151 = OpLoad %float %c
        %152 = OpCompositeConstruct %v4float %150 %float_0 %151 %float_0
        %153 = OpAccessChain %_ptr_Function_v4float %gRotMat %int_2
               OpStore %153 %152
        %155 = OpAccessChain %_ptr_Function_v4float %gRotMat %int_3
               OpStore %155 %154
        %158 = OpLoad %v3float %inPos
        %159 = OpLoad %mat3v3float %rotMat
        %160 = OpVectorTimesMatrix %v3float %158 %159
        %161 = OpCompositeExtract %float %160 0
        %162 = OpCompositeExtract %float %160 1
        %163 = OpCompositeExtract %float %160 2
        %164 = OpCompositeConstruct %v4float %161 %162 %163 %float_1
               OpStore %locPos %164
        %166 = OpLoad %v4float %locPos
        %167 = OpVectorShuffle %v3float %166 %166 0 1 2
        %169 = OpLoad %float %instanceScale
        %170 = OpVectorTimesScalar %v3float %167 %169
        %172 = OpLoad %v3float %instancePos
        %173 = OpFAdd %v3float %170 %172
        %174 = OpCompositeExtract %float %173 0
        %175 = OpCompositeExtract %float %173 1
        %176 = OpCompositeExtract %float %173 2
        %177 = OpCompositeConstruct %v4float %174 %175 %176 %float_1
               OpStore %pos %177
        %183 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_0
        %184 = OpLoad %mat4v4float %183
        %185 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %186 = OpLoad %mat4v4float %185
        %187 = OpMatrixTimesMatrix %mat4v4float %184 %186
        %188 = OpLoad %mat4v4float %gRotMat
        %189 = OpMatrixTimesMatrix %mat4v4float %187 %188
        %190 = OpLoad %v4float %pos
        %191 = OpMatrixTimesVector %v4float %189 %190
        %193 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %193 %191
        %195 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %196 = OpLoad %mat4v4float %195
        %197 = OpLoad %mat4v4float %gRotMat
        %198 = OpMatrixTimesMatrix %mat4v4float %196 %197
        %199 = OpCompositeExtract %v4float %198 0
        %200 = OpVectorShuffle %v3float %199 %199 0 1 2
        %201 = OpCompositeExtract %v4float %198 1
        %202 = OpVectorShuffle %v3float %201 %201 0 1 2
        %203 = OpCompositeExtract %v4float %198 2
        %204 = OpVectorShuffle %v3float %203 %203 0 1 2
        %205 = OpCompositeConstruct %mat3v3float %200 %202 %204
        %206 = OpLoad %mat3v3float %rotMat
        %207 = OpExtInst %mat3v3float %1 MatrixInverse %206
        %208 = OpMatrixTimesMatrix %mat3v3float %205 %207
        %210 = OpLoad %v3float %inNormal
        %211 = OpMatrixTimesVector %v3float %208 %210
               OpStore %outNormal %211
        %212 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %213 = OpLoad %mat4v4float %212
        %214 = OpLoad %v3float %inPos
        %215 = OpLoad %v3float %instancePos
        %216 = OpFAdd %v3float %214 %215
        %217 = OpCompositeExtract %float %216 0
        %218 = OpCompositeExtract %float %216 1
        %219 = OpCompositeExtract %float %216 2
        %220 = OpCompositeConstruct %v4float %217 %218 %219 %float_1
        %221 = OpMatrixTimesVector %v4float %213 %220
               OpStore %pos %221
        %223 = OpAccessChain %_ptr_Uniform_mat4v4float %ubo %int_1
        %224 = OpLoad %mat4v4float %223
        %225 = OpCompositeExtract %v4float %224 0
        %226 = OpVectorShuffle %v3float %225 %225 0 1 2
        %227 = OpCompositeExtract %v4float %224 1
        %228 = OpVectorShuffle %v3float %227 %227 0 1 2
        %229 = OpCompositeExtract %v4float %224 2
        %230 = OpVectorShuffle %v3float %229 %229 0 1 2
        %231 = OpCompositeConstruct %mat3v3float %226 %228 %230
        %233 = OpAccessChain %_ptr_Uniform_v4float %ubo %int_2
        %234 = OpLoad %v4float %233
        %235 = OpVectorShuffle %v3float %234 %234 0 1 2
        %236 = OpMatrixTimesVector %v3float %231 %235
               OpStore %lPos %236
        %238 = OpLoad %v3float %lPos
        %239 = OpLoad %v4float %pos
        %240 = OpVectorShuffle %v3float %239 %239 0 1 2
        %241 = OpFSub %v3float %238 %240
               OpStore %outLightVec %241
        %243 = OpLoad %v4float %pos
        %244 = OpVectorShuffle %v3float %243 %243 0 1 2
        %245 = OpFNegate %v3float %244
               OpStore %outViewVec %245
               OpReturn
               OpFunctionEnd
