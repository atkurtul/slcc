; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 177
; Schema: 0
               OpCapability Tessellation
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint TessellationEvaluation %4 "main" %14 %25 %44 %56 %75 %89 %142 %153 %157 %165 %168
               OpExecutionMode %4 Quads
               OpExecutionMode %4 SpacingEqual
               OpExecutionMode %4 VertexOrderCw
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "uv1"
               OpName %14 "inUV"
               OpName %25 "gl_TessCoord"
               OpName %32 "uv2"
               OpName %44 "outUV"
               OpName %53 "n1"
               OpName %56 "inNormal"
               OpName %65 "n2"
               OpName %75 "outNormal"
               OpName %84 "pos1"
               OpName %86 "gl_PerVertex"
               OpMemberName %86 0 "gl_Position"
               OpMemberName %86 1 "gl_PointSize"
               OpMemberName %86 2 "gl_ClipDistance"
               OpMemberName %86 3 "gl_CullDistance"
               OpName %89 "gl_in"
               OpName %99 "pos2"
               OpName %108 "pos"
               OpName %118 "displacementMap"
               OpName %127 "UBO"
               OpMemberName %127 0 "projection"
               OpMemberName %127 1 "modelview"
               OpMemberName %127 2 "lightPos"
               OpMemberName %127 3 "frustumPlanes"
               OpMemberName %127 4 "displacementFactor"
               OpMemberName %127 5 "tessellationFactor"
               OpMemberName %127 6 "viewportDim"
               OpMemberName %127 7 "tessellatedEdgeSize"
               OpName %129 "ubo"
               OpName %140 "gl_PerVertex"
               OpMemberName %140 0 "gl_Position"
               OpMemberName %140 1 "gl_PointSize"
               OpMemberName %140 2 "gl_ClipDistance"
               OpMemberName %140 3 "gl_CullDistance"
               OpName %142 ""
               OpName %153 "outViewVec"
               OpName %157 "outLightVec"
               OpName %165 "outWorldPos"
               OpName %168 "outEyePos"
               OpDecorate %14 Location 1
               OpDecorate %25 BuiltIn TessCoord
               OpDecorate %44 Location 1
               OpDecorate %56 Location 0
               OpDecorate %75 Location 0
               OpMemberDecorate %86 0 BuiltIn Position
               OpMemberDecorate %86 1 BuiltIn PointSize
               OpMemberDecorate %86 2 BuiltIn ClipDistance
               OpMemberDecorate %86 3 BuiltIn CullDistance
               OpDecorate %86 Block
               OpDecorate %118 DescriptorSet 0
               OpDecorate %118 Binding 1
               OpDecorate %126 ArrayStride 16
               OpMemberDecorate %127 0 ColMajor
               OpMemberDecorate %127 0 Offset 0
               OpMemberDecorate %127 0 MatrixStride 16
               OpMemberDecorate %127 1 ColMajor
               OpMemberDecorate %127 1 Offset 64
               OpMemberDecorate %127 1 MatrixStride 16
               OpMemberDecorate %127 2 Offset 128
               OpMemberDecorate %127 3 Offset 144
               OpMemberDecorate %127 4 Offset 240
               OpMemberDecorate %127 5 Offset 244
               OpMemberDecorate %127 6 Offset 248
               OpMemberDecorate %127 7 Offset 256
               OpDecorate %127 Block
               OpDecorate %129 DescriptorSet 0
               OpDecorate %129 Binding 0
               OpMemberDecorate %140 0 BuiltIn Position
               OpMemberDecorate %140 1 BuiltIn PointSize
               OpMemberDecorate %140 2 BuiltIn ClipDistance
               OpMemberDecorate %140 3 BuiltIn CullDistance
               OpDecorate %140 Block
               OpDecorate %153 Location 2
               OpDecorate %157 Location 3
               OpDecorate %165 Location 5
               OpDecorate %168 Location 4
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
         %10 = OpTypeInt 32 0
         %11 = OpConstant %10 32
         %12 = OpTypeArray %7 %11
         %13 = OpTypePointer Input %12
         %14 = OpVariable %13 Input
         %15 = OpTypeInt 32 1
         %16 = OpConstant %15 0
         %17 = OpTypePointer Input %7
         %20 = OpConstant %15 1
         %23 = OpTypeVector %6 3
         %24 = OpTypePointer Input %23
         %25 = OpVariable %24 Input
         %26 = OpConstant %10 0
         %27 = OpTypePointer Input %6
         %33 = OpConstant %15 3
         %36 = OpConstant %15 2
         %43 = OpTypePointer Output %7
         %44 = OpVariable %43 Output
         %47 = OpConstant %10 1
         %52 = OpTypePointer Function %23
         %54 = OpTypeArray %23 %11
         %55 = OpTypePointer Input %54
         %56 = OpVariable %55 Input
         %74 = OpTypePointer Output %23
         %75 = OpVariable %74 Output
         %82 = OpTypeVector %6 4
         %83 = OpTypePointer Function %82
         %85 = OpTypeArray %6 %47
         %86 = OpTypeStruct %82 %6 %85 %85
         %87 = OpTypeArray %86 %11
         %88 = OpTypePointer Input %87
         %89 = OpVariable %88 Input
         %90 = OpTypePointer Input %82
        %115 = OpTypeImage %6 2D 0 0 0 1 Unknown
        %116 = OpTypeSampledImage %115
        %117 = OpTypePointer UniformConstant %116
        %118 = OpVariable %117 UniformConstant
        %121 = OpConstant %6 0
        %124 = OpTypeMatrix %82 4
        %125 = OpConstant %10 6
        %126 = OpTypeArray %82 %125
        %127 = OpTypeStruct %124 %124 %82 %126 %6 %6 %7 %6
        %128 = OpTypePointer Uniform %127
        %129 = OpVariable %128 Uniform
        %130 = OpConstant %15 4
        %131 = OpTypePointer Uniform %6
        %135 = OpTypePointer Function %6
        %140 = OpTypeStruct %82 %6 %85 %85
        %141 = OpTypePointer Output %140
        %142 = OpVariable %141 Output
        %143 = OpTypePointer Uniform %124
        %151 = OpTypePointer Output %82
        %153 = OpVariable %74 Output
        %157 = OpVariable %74 Output
        %158 = OpTypePointer Uniform %82
        %165 = OpVariable %74 Output
        %168 = OpVariable %74 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %32 = OpVariable %8 Function
         %53 = OpVariable %52 Function
         %65 = OpVariable %52 Function
         %84 = OpVariable %83 Function
         %99 = OpVariable %83 Function
        %108 = OpVariable %83 Function
         %18 = OpAccessChain %17 %14 %16
         %19 = OpLoad %7 %18
         %21 = OpAccessChain %17 %14 %20
         %22 = OpLoad %7 %21
         %28 = OpAccessChain %27 %25 %26
         %29 = OpLoad %6 %28
         %30 = OpCompositeConstruct %7 %29 %29
         %31 = OpExtInst %7 %1 FMix %19 %22 %30
               OpStore %9 %31
         %34 = OpAccessChain %17 %14 %33
         %35 = OpLoad %7 %34
         %37 = OpAccessChain %17 %14 %36
         %38 = OpLoad %7 %37
         %39 = OpAccessChain %27 %25 %26
         %40 = OpLoad %6 %39
         %41 = OpCompositeConstruct %7 %40 %40
         %42 = OpExtInst %7 %1 FMix %35 %38 %41
               OpStore %32 %42
         %45 = OpLoad %7 %9
         %46 = OpLoad %7 %32
         %48 = OpAccessChain %27 %25 %47
         %49 = OpLoad %6 %48
         %50 = OpCompositeConstruct %7 %49 %49
         %51 = OpExtInst %7 %1 FMix %45 %46 %50
               OpStore %44 %51
         %57 = OpAccessChain %24 %56 %16
         %58 = OpLoad %23 %57
         %59 = OpAccessChain %24 %56 %20
         %60 = OpLoad %23 %59
         %61 = OpAccessChain %27 %25 %26
         %62 = OpLoad %6 %61
         %63 = OpCompositeConstruct %23 %62 %62 %62
         %64 = OpExtInst %23 %1 FMix %58 %60 %63
               OpStore %53 %64
         %66 = OpAccessChain %24 %56 %33
         %67 = OpLoad %23 %66
         %68 = OpAccessChain %24 %56 %36
         %69 = OpLoad %23 %68
         %70 = OpAccessChain %27 %25 %26
         %71 = OpLoad %6 %70
         %72 = OpCompositeConstruct %23 %71 %71 %71
         %73 = OpExtInst %23 %1 FMix %67 %69 %72
               OpStore %65 %73
         %76 = OpLoad %23 %53
         %77 = OpLoad %23 %65
         %78 = OpAccessChain %27 %25 %47
         %79 = OpLoad %6 %78
         %80 = OpCompositeConstruct %23 %79 %79 %79
         %81 = OpExtInst %23 %1 FMix %76 %77 %80
               OpStore %75 %81
         %91 = OpAccessChain %90 %89 %16 %16
         %92 = OpLoad %82 %91
         %93 = OpAccessChain %90 %89 %20 %16
         %94 = OpLoad %82 %93
         %95 = OpAccessChain %27 %25 %26
         %96 = OpLoad %6 %95
         %97 = OpCompositeConstruct %82 %96 %96 %96 %96
         %98 = OpExtInst %82 %1 FMix %92 %94 %97
               OpStore %84 %98
        %100 = OpAccessChain %90 %89 %33 %16
        %101 = OpLoad %82 %100
        %102 = OpAccessChain %90 %89 %36 %16
        %103 = OpLoad %82 %102
        %104 = OpAccessChain %27 %25 %26
        %105 = OpLoad %6 %104
        %106 = OpCompositeConstruct %82 %105 %105 %105 %105
        %107 = OpExtInst %82 %1 FMix %101 %103 %106
               OpStore %99 %107
        %109 = OpLoad %82 %84
        %110 = OpLoad %82 %99
        %111 = OpAccessChain %27 %25 %47
        %112 = OpLoad %6 %111
        %113 = OpCompositeConstruct %82 %112 %112 %112 %112
        %114 = OpExtInst %82 %1 FMix %109 %110 %113
               OpStore %108 %114
        %119 = OpLoad %116 %118
        %120 = OpLoad %7 %44
        %122 = OpImageSampleExplicitLod %82 %119 %120 Lod %121
        %123 = OpCompositeExtract %6 %122 0
        %132 = OpAccessChain %131 %129 %130
        %133 = OpLoad %6 %132
        %134 = OpFMul %6 %123 %133
        %136 = OpAccessChain %135 %108 %47
        %137 = OpLoad %6 %136
        %138 = OpFSub %6 %137 %134
        %139 = OpAccessChain %135 %108 %47
               OpStore %139 %138
        %144 = OpAccessChain %143 %129 %16
        %145 = OpLoad %124 %144
        %146 = OpAccessChain %143 %129 %20
        %147 = OpLoad %124 %146
        %148 = OpMatrixTimesMatrix %124 %145 %147
        %149 = OpLoad %82 %108
        %150 = OpMatrixTimesVector %82 %148 %149
        %152 = OpAccessChain %151 %142 %16
               OpStore %152 %150
        %154 = OpLoad %82 %108
        %155 = OpVectorShuffle %23 %154 %154 0 1 2
        %156 = OpFNegate %23 %155
               OpStore %153 %156
        %159 = OpAccessChain %158 %129 %36
        %160 = OpLoad %82 %159
        %161 = OpVectorShuffle %23 %160 %160 0 1 2
        %162 = OpLoad %23 %153
        %163 = OpFAdd %23 %161 %162
        %164 = OpExtInst %23 %1 Normalize %163
               OpStore %157 %164
        %166 = OpLoad %82 %108
        %167 = OpVectorShuffle %23 %166 %166 0 1 2
               OpStore %165 %167
        %169 = OpAccessChain %143 %129 %20
        %170 = OpLoad %124 %169
        %171 = OpLoad %82 %108
        %172 = OpMatrixTimesVector %82 %170 %171
        %173 = OpCompositeExtract %6 %172 0
        %174 = OpCompositeExtract %6 %172 1
        %175 = OpCompositeExtract %6 %172 2
        %176 = OpCompositeConstruct %23 %173 %174 %175
               OpStore %168 %176
               OpReturn
               OpFunctionEnd
