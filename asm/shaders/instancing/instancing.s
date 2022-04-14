; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 246
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %12 %15 %19 %28 %157 %168 %171 %181 %194 %209 %237 %242
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outColor"
               OpName %12 "outUV"
               OpName %15 "inUV"
               OpName %19 "instanceTexIndex"
               OpName %26 "s"
               OpName %28 "instanceRot"
               OpName %36 "UBO"
               OpMemberName %36 0 "projection"
               OpMemberName %36 1 "modelview"
               OpMemberName %36 2 "lightPos"
               OpMemberName %36 3 "locSpeed"
               OpMemberName %36 4 "globSpeed"
               OpName %38 "ubo"
               OpName %45 "c"
               OpName %54 "mx"
               OpName %84 "my"
               OpName %109 "mz"
               OpName %121 "rotMat"
               OpName %141 "gRotMat"
               OpName %156 "locPos"
               OpName %157 "inPos"
               OpName %165 "pos"
               OpName %168 "instanceScale"
               OpName %171 "instancePos"
               OpName %179 "gl_PerVertex"
               OpMemberName %179 0 "gl_Position"
               OpMemberName %179 1 "gl_PointSize"
               OpMemberName %179 2 "gl_ClipDistance"
               OpMemberName %179 3 "gl_CullDistance"
               OpName %181 ""
               OpName %194 "outNormal"
               OpName %209 "inNormal"
               OpName %222 "lPos"
               OpName %237 "outLightVec"
               OpName %242 "outViewVec"
               OpDecorate %9 Location 1
               OpDecorate %12 Location 2
               OpDecorate %15 Location 2
               OpDecorate %19 Location 7
               OpDecorate %28 Location 5
               OpMemberDecorate %36 0 ColMajor
               OpMemberDecorate %36 0 Offset 0
               OpMemberDecorate %36 0 MatrixStride 16
               OpMemberDecorate %36 1 ColMajor
               OpMemberDecorate %36 1 Offset 64
               OpMemberDecorate %36 1 MatrixStride 16
               OpMemberDecorate %36 2 Offset 128
               OpMemberDecorate %36 3 Offset 144
               OpMemberDecorate %36 4 Offset 148
               OpDecorate %36 Block
               OpDecorate %38 DescriptorSet 0
               OpDecorate %38 Binding 0
               OpDecorate %157 Location 0
               OpDecorate %168 Location 6
               OpDecorate %171 Location 4
               OpMemberDecorate %179 0 BuiltIn Position
               OpMemberDecorate %179 1 BuiltIn PointSize
               OpMemberDecorate %179 2 BuiltIn ClipDistance
               OpMemberDecorate %179 3 BuiltIn CullDistance
               OpDecorate %179 Block
               OpDecorate %194 Location 0
               OpDecorate %209 Location 1
               OpDecorate %237 Location 4
               OpDecorate %242 Location 3
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpConstant %6 1
         %11 = OpConstantComposite %7 %10 %10 %10
         %12 = OpVariable %8 Output
         %13 = OpTypeVector %6 2
         %14 = OpTypePointer Input %13
         %15 = OpVariable %14 Input
         %17 = OpTypeInt 32 1
         %18 = OpTypePointer Input %17
         %19 = OpVariable %18 Input
         %25 = OpTypePointer Function %6
         %27 = OpTypePointer Input %7
         %28 = OpVariable %27 Input
         %29 = OpTypeInt 32 0
         %30 = OpConstant %29 0
         %31 = OpTypePointer Input %6
         %34 = OpTypeVector %6 4
         %35 = OpTypeMatrix %34 4
         %36 = OpTypeStruct %35 %35 %34 %6 %6
         %37 = OpTypePointer Uniform %36
         %38 = OpVariable %37 Uniform
         %39 = OpConstant %17 3
         %40 = OpTypePointer Uniform %6
         %52 = OpTypeMatrix %7 3
         %53 = OpTypePointer Function %52
         %55 = OpConstant %17 0
         %58 = OpConstant %6 0
         %60 = OpTypePointer Function %7
         %62 = OpConstant %17 1
         %68 = OpConstant %17 2
         %69 = OpConstantComposite %7 %58 %58 %10
         %71 = OpConstant %29 1
         %89 = OpConstantComposite %7 %58 %10 %58
         %96 = OpConstant %29 2
        %110 = OpConstantComposite %7 %10 %58 %58
        %129 = OpConstant %17 4
        %140 = OpTypePointer Function %35
        %145 = OpTypePointer Function %34
        %147 = OpConstantComposite %34 %58 %10 %58 %58
        %154 = OpConstantComposite %34 %58 %58 %58 %10
        %157 = OpVariable %27 Input
        %168 = OpVariable %31 Input
        %171 = OpVariable %27 Input
        %178 = OpTypeArray %6 %71
        %179 = OpTypeStruct %34 %6 %178 %178
        %180 = OpTypePointer Output %179
        %181 = OpVariable %180 Output
        %182 = OpTypePointer Uniform %35
        %192 = OpTypePointer Output %34
        %194 = OpVariable %8 Output
        %209 = OpVariable %27 Input
        %232 = OpTypePointer Uniform %34
        %237 = OpVariable %8 Output
        %242 = OpVariable %8 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %26 = OpVariable %25 Function
         %45 = OpVariable %25 Function
         %54 = OpVariable %53 Function
         %84 = OpVariable %53 Function
        %109 = OpVariable %53 Function
        %121 = OpVariable %53 Function
        %141 = OpVariable %140 Function
        %156 = OpVariable %145 Function
        %165 = OpVariable %145 Function
        %222 = OpVariable %60 Function
               OpStore %9 %11
         %16 = OpLoad %13 %15
         %20 = OpLoad %17 %19
         %21 = OpConvertSToF %6 %20
         %22 = OpCompositeExtract %6 %16 0
         %23 = OpCompositeExtract %6 %16 1
         %24 = OpCompositeConstruct %7 %22 %23 %21
               OpStore %12 %24
         %32 = OpAccessChain %31 %28 %30
         %33 = OpLoad %6 %32
         %41 = OpAccessChain %40 %38 %39
         %42 = OpLoad %6 %41
         %43 = OpFAdd %6 %33 %42
         %44 = OpExtInst %6 %1 Sin %43
               OpStore %26 %44
         %46 = OpAccessChain %31 %28 %30
         %47 = OpLoad %6 %46
         %48 = OpAccessChain %40 %38 %39
         %49 = OpLoad %6 %48
         %50 = OpFAdd %6 %47 %49
         %51 = OpExtInst %6 %1 Cos %50
               OpStore %45 %51
         %56 = OpLoad %6 %45
         %57 = OpLoad %6 %26
         %59 = OpCompositeConstruct %7 %56 %57 %58
         %61 = OpAccessChain %60 %54 %55
               OpStore %61 %59
         %63 = OpLoad %6 %26
         %64 = OpFNegate %6 %63
         %65 = OpLoad %6 %45
         %66 = OpCompositeConstruct %7 %64 %65 %58
         %67 = OpAccessChain %60 %54 %62
               OpStore %67 %66
         %70 = OpAccessChain %60 %54 %68
               OpStore %70 %69
         %72 = OpAccessChain %31 %28 %71
         %73 = OpLoad %6 %72
         %74 = OpAccessChain %40 %38 %39
         %75 = OpLoad %6 %74
         %76 = OpFAdd %6 %73 %75
         %77 = OpExtInst %6 %1 Sin %76
               OpStore %26 %77
         %78 = OpAccessChain %31 %28 %71
         %79 = OpLoad %6 %78
         %80 = OpAccessChain %40 %38 %39
         %81 = OpLoad %6 %80
         %82 = OpFAdd %6 %79 %81
         %83 = OpExtInst %6 %1 Cos %82
               OpStore %45 %83
         %85 = OpLoad %6 %45
         %86 = OpLoad %6 %26
         %87 = OpCompositeConstruct %7 %85 %58 %86
         %88 = OpAccessChain %60 %84 %55
               OpStore %88 %87
         %90 = OpAccessChain %60 %84 %62
               OpStore %90 %89
         %91 = OpLoad %6 %26
         %92 = OpFNegate %6 %91
         %93 = OpLoad %6 %45
         %94 = OpCompositeConstruct %7 %92 %58 %93
         %95 = OpAccessChain %60 %84 %68
               OpStore %95 %94
         %97 = OpAccessChain %31 %28 %96
         %98 = OpLoad %6 %97
         %99 = OpAccessChain %40 %38 %39
        %100 = OpLoad %6 %99
        %101 = OpFAdd %6 %98 %100
        %102 = OpExtInst %6 %1 Sin %101
               OpStore %26 %102
        %103 = OpAccessChain %31 %28 %96
        %104 = OpLoad %6 %103
        %105 = OpAccessChain %40 %38 %39
        %106 = OpLoad %6 %105
        %107 = OpFAdd %6 %104 %106
        %108 = OpExtInst %6 %1 Cos %107
               OpStore %45 %108
        %111 = OpAccessChain %60 %109 %55
               OpStore %111 %110
        %112 = OpLoad %6 %45
        %113 = OpLoad %6 %26
        %114 = OpCompositeConstruct %7 %58 %112 %113
        %115 = OpAccessChain %60 %109 %62
               OpStore %115 %114
        %116 = OpLoad %6 %26
        %117 = OpFNegate %6 %116
        %118 = OpLoad %6 %45
        %119 = OpCompositeConstruct %7 %58 %117 %118
        %120 = OpAccessChain %60 %109 %68
               OpStore %120 %119
        %122 = OpLoad %52 %109
        %123 = OpLoad %52 %84
        %124 = OpMatrixTimesMatrix %52 %122 %123
        %125 = OpLoad %52 %54
        %126 = OpMatrixTimesMatrix %52 %124 %125
               OpStore %121 %126
        %127 = OpAccessChain %31 %28 %71
        %128 = OpLoad %6 %127
        %130 = OpAccessChain %40 %38 %129
        %131 = OpLoad %6 %130
        %132 = OpFAdd %6 %128 %131
        %133 = OpExtInst %6 %1 Sin %132
               OpStore %26 %133
        %134 = OpAccessChain %31 %28 %71
        %135 = OpLoad %6 %134
        %136 = OpAccessChain %40 %38 %129
        %137 = OpLoad %6 %136
        %138 = OpFAdd %6 %135 %137
        %139 = OpExtInst %6 %1 Cos %138
               OpStore %45 %139
        %142 = OpLoad %6 %45
        %143 = OpLoad %6 %26
        %144 = OpCompositeConstruct %34 %142 %58 %143 %58
        %146 = OpAccessChain %145 %141 %55
               OpStore %146 %144
        %148 = OpAccessChain %145 %141 %62
               OpStore %148 %147
        %149 = OpLoad %6 %26
        %150 = OpFNegate %6 %149
        %151 = OpLoad %6 %45
        %152 = OpCompositeConstruct %34 %150 %58 %151 %58
        %153 = OpAccessChain %145 %141 %68
               OpStore %153 %152
        %155 = OpAccessChain %145 %141 %39
               OpStore %155 %154
        %158 = OpLoad %7 %157
        %159 = OpLoad %52 %121
        %160 = OpVectorTimesMatrix %7 %158 %159
        %161 = OpCompositeExtract %6 %160 0
        %162 = OpCompositeExtract %6 %160 1
        %163 = OpCompositeExtract %6 %160 2
        %164 = OpCompositeConstruct %34 %161 %162 %163 %10
               OpStore %156 %164
        %166 = OpLoad %34 %156
        %167 = OpVectorShuffle %7 %166 %166 0 1 2
        %169 = OpLoad %6 %168
        %170 = OpVectorTimesScalar %7 %167 %169
        %172 = OpLoad %7 %171
        %173 = OpFAdd %7 %170 %172
        %174 = OpCompositeExtract %6 %173 0
        %175 = OpCompositeExtract %6 %173 1
        %176 = OpCompositeExtract %6 %173 2
        %177 = OpCompositeConstruct %34 %174 %175 %176 %10
               OpStore %165 %177
        %183 = OpAccessChain %182 %38 %55
        %184 = OpLoad %35 %183
        %185 = OpAccessChain %182 %38 %62
        %186 = OpLoad %35 %185
        %187 = OpMatrixTimesMatrix %35 %184 %186
        %188 = OpLoad %35 %141
        %189 = OpMatrixTimesMatrix %35 %187 %188
        %190 = OpLoad %34 %165
        %191 = OpMatrixTimesVector %34 %189 %190
        %193 = OpAccessChain %192 %181 %55
               OpStore %193 %191
        %195 = OpAccessChain %182 %38 %62
        %196 = OpLoad %35 %195
        %197 = OpLoad %35 %141
        %198 = OpMatrixTimesMatrix %35 %196 %197
        %199 = OpCompositeExtract %34 %198 0
        %200 = OpVectorShuffle %7 %199 %199 0 1 2
        %201 = OpCompositeExtract %34 %198 1
        %202 = OpVectorShuffle %7 %201 %201 0 1 2
        %203 = OpCompositeExtract %34 %198 2
        %204 = OpVectorShuffle %7 %203 %203 0 1 2
        %205 = OpCompositeConstruct %52 %200 %202 %204
        %206 = OpLoad %52 %121
        %207 = OpExtInst %52 %1 MatrixInverse %206
        %208 = OpMatrixTimesMatrix %52 %205 %207
        %210 = OpLoad %7 %209
        %211 = OpMatrixTimesVector %7 %208 %210
               OpStore %194 %211
        %212 = OpAccessChain %182 %38 %62
        %213 = OpLoad %35 %212
        %214 = OpLoad %7 %157
        %215 = OpLoad %7 %171
        %216 = OpFAdd %7 %214 %215
        %217 = OpCompositeExtract %6 %216 0
        %218 = OpCompositeExtract %6 %216 1
        %219 = OpCompositeExtract %6 %216 2
        %220 = OpCompositeConstruct %34 %217 %218 %219 %10
        %221 = OpMatrixTimesVector %34 %213 %220
               OpStore %165 %221
        %223 = OpAccessChain %182 %38 %62
        %224 = OpLoad %35 %223
        %225 = OpCompositeExtract %34 %224 0
        %226 = OpVectorShuffle %7 %225 %225 0 1 2
        %227 = OpCompositeExtract %34 %224 1
        %228 = OpVectorShuffle %7 %227 %227 0 1 2
        %229 = OpCompositeExtract %34 %224 2
        %230 = OpVectorShuffle %7 %229 %229 0 1 2
        %231 = OpCompositeConstruct %52 %226 %228 %230
        %233 = OpAccessChain %232 %38 %68
        %234 = OpLoad %34 %233
        %235 = OpVectorShuffle %7 %234 %234 0 1 2
        %236 = OpMatrixTimesVector %7 %231 %235
               OpStore %222 %236
        %238 = OpLoad %7 %222
        %239 = OpLoad %34 %165
        %240 = OpVectorShuffle %7 %239 %239 0 1 2
        %241 = OpFSub %7 %238 %240
               OpStore %237 %241
        %243 = OpLoad %34 %165
        %244 = OpVectorShuffle %7 %243 %243 0 1 2
        %245 = OpFNegate %7 %244
               OpStore %242 %245
               OpReturn
               OpFunctionEnd
