; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 212
; Schema: 0
               OpCapability Shader
               OpCapability PhysicalStorageBufferAddresses
               OpExtension "SPV_KHR_physical_storage_buffer"
               OpExtension "SPV_KHR_storage_buffer_storage_class"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel PhysicalStorageBuffer64 GLSL450
               OpEntryPoint GLCompute %4 "main" %120
               OpExecutionMode %4 LocalSize 64 1 1
               OpSource GLSL 460
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %18 "check_is_visible(mf44;vf3;f1;"
               OpName %15 "mat"
               OpName %16 "origin"
               OpName %17 "radius"
               OpName %22 "plane_index"
               OpName %24 "i"
               OpName %33 "j"
               OpName %50 "sign"
               OpName %57 "plane"
               OpName %60 "k"
               OpName %117 "id"
               OpName %120 "gl_GlobalInvocationID"
               OpName %125 "GlobalUniform"
               OpMemberName %125 0 "view"
               OpMemberName %125 1 "proj"
               OpMemberName %125 2 "proj_view"
               OpMemberName %125 3 "model_count"
               OpName %127 "global_uniform"
               OpName %136 "ModelInformation"
               OpMemberName %136 0 "x"
               OpMemberName %136 1 "y"
               OpMemberName %136 2 "z"
               OpMemberName %136 3 "r"
               OpMemberName %136 4 "texture_index"
               OpMemberName %136 5 "firstIndex"
               OpMemberName %136 6 "indexCount"
               OpMemberName %136 7 "_pad"
               OpName %138 "model"
               OpName %139 "ModelInformation"
               OpMemberName %139 0 "x"
               OpMemberName %139 1 "y"
               OpMemberName %139 2 "z"
               OpMemberName %139 3 "r"
               OpMemberName %139 4 "texture_index"
               OpMemberName %139 5 "firstIndex"
               OpMemberName %139 6 "indexCount"
               OpMemberName %139 7 "_pad"
               OpName %141 "ModelInformationBuffer"
               OpMemberName %141 0 "arr"
               OpName %143 "model_buffer"
               OpName %171 "is_visible"
               OpName %179 "param"
               OpName %183 "param"
               OpName %184 "param"
               OpName %189 "Addresses"
               OpMemberName %189 0 "command_buffer"
               OpName %190 "VkDrawIndexedIndirectCommand"
               OpMemberName %190 0 "indexCount"
               OpMemberName %190 1 "instanceCount"
               OpMemberName %190 2 "firstIndex"
               OpMemberName %190 3 "vertexOffset"
               OpMemberName %190 4 "firstInstance"
               OpName %192 "CommandBuffer"
               OpMemberName %192 0 "commands"
               OpName %194 "addresses"
               OpName %207 "VkDrawIndexedIndirectCommand"
               OpMemberName %207 0 "indexCount"
               OpMemberName %207 1 "instanceCount"
               OpMemberName %207 2 "firstIndex"
               OpMemberName %207 3 "vertexOffset"
               OpMemberName %207 4 "firstInstance"
               OpName %209 "CommandBuffer"
               OpMemberName %209 0 "commands"
               OpName %211 "command_buffer"
               OpDecorate %120 BuiltIn GlobalInvocationId
               OpMemberDecorate %125 0 ColMajor
               OpMemberDecorate %125 0 Offset 0
               OpMemberDecorate %125 0 MatrixStride 16
               OpMemberDecorate %125 1 ColMajor
               OpMemberDecorate %125 1 Offset 64
               OpMemberDecorate %125 1 MatrixStride 16
               OpMemberDecorate %125 2 ColMajor
               OpMemberDecorate %125 2 Offset 128
               OpMemberDecorate %125 2 MatrixStride 16
               OpMemberDecorate %125 3 Offset 192
               OpDecorate %125 Block
               OpDecorate %127 DescriptorSet 0
               OpDecorate %127 Binding 2
               OpMemberDecorate %139 0 Offset 0
               OpMemberDecorate %139 1 Offset 4
               OpMemberDecorate %139 2 Offset 8
               OpMemberDecorate %139 3 Offset 12
               OpMemberDecorate %139 4 Offset 16
               OpMemberDecorate %139 5 Offset 20
               OpMemberDecorate %139 6 Offset 24
               OpMemberDecorate %139 7 Offset 28
               OpDecorate %140 ArrayStride 32
               OpMemberDecorate %141 0 NonWritable
               OpMemberDecorate %141 0 Offset 0
               OpDecorate %141 Block
               OpDecorate %143 DescriptorSet 0
               OpDecorate %143 Binding 0
               OpMemberDecorate %189 0 Offset 0
               OpDecorate %189 Block
               OpMemberDecorate %190 0 Offset 0
               OpMemberDecorate %190 1 Offset 4
               OpMemberDecorate %190 2 Offset 8
               OpMemberDecorate %190 3 Offset 12
               OpMemberDecorate %190 4 Offset 16
               OpDecorate %191 ArrayStride 20
               OpMemberDecorate %192 0 Offset 0
               OpDecorate %192 Block
               OpDecorate %194 DescriptorSet 0
               OpDecorate %194 Binding 4
               OpDecorate %206 BuiltIn WorkgroupSize
               OpMemberDecorate %207 0 Offset 0
               OpMemberDecorate %207 1 Offset 4
               OpMemberDecorate %207 2 Offset 8
               OpMemberDecorate %207 3 Offset 12
               OpMemberDecorate %207 4 Offset 16
               OpDecorate %208 ArrayStride 20
               OpMemberDecorate %209 0 Offset 0
               OpDecorate %209 Block
               OpDecorate %211 DescriptorSet 0
               OpDecorate %211 Binding 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypeMatrix %7 4
          %9 = OpTypePointer Function %8
         %10 = OpTypeVector %6 3
         %11 = OpTypePointer Function %10
         %12 = OpTypePointer Function %6
         %13 = OpTypeBool
         %14 = OpTypeFunction %13 %9 %11 %12
         %20 = OpTypeInt 32 0
         %21 = OpTypePointer Function %20
         %23 = OpConstant %20 0
         %31 = OpConstant %20 3
         %40 = OpConstant %20 2
         %53 = OpConstant %6 1
         %54 = OpConstant %6 -1
         %56 = OpTypePointer Function %7
         %58 = OpConstant %6 0
         %59 = OpConstantComposite %7 %58 %58 %58 %58
         %67 = OpConstant %20 4
         %82 = OpTypeInt 32 1
         %83 = OpConstant %82 1
        %106 = OpConstantFalse %13
        %114 = OpConstantTrue %13
        %118 = OpTypeVector %20 3
        %119 = OpTypePointer Input %118
        %120 = OpVariable %119 Input
        %121 = OpTypePointer Input %20
        %125 = OpTypeStruct %8 %8 %8 %20
        %126 = OpTypePointer Uniform %125
        %127 = OpVariable %126 Uniform
        %128 = OpConstant %82 3
        %129 = OpTypePointer Uniform %20
        %136 = OpTypeStruct %6 %6 %6 %6 %20 %20 %20 %20
        %137 = OpTypePointer Function %136
        %139 = OpTypeStruct %6 %6 %6 %6 %20 %20 %20 %20
        %140 = OpTypeRuntimeArray %139
        %141 = OpTypeStruct %140
        %142 = OpTypePointer StorageBuffer %141
        %143 = OpVariable %142 StorageBuffer
        %144 = OpConstant %82 0
        %146 = OpTypePointer StorageBuffer %139
        %154 = OpConstant %82 2
        %159 = OpConstant %82 4
        %162 = OpConstant %82 5
        %165 = OpConstant %82 6
        %168 = OpConstant %82 7
        %170 = OpTypePointer Function %13
        %180 = OpTypePointer Uniform %8
               OpTypeForwardPointer %188 PhysicalStorageBuffer
        %189 = OpTypeStruct %188
        %190 = OpTypeStruct %20 %20 %20 %82 %20
        %191 = OpTypeRuntimeArray %190
        %192 = OpTypeStruct %191
        %188 = OpTypePointer PhysicalStorageBuffer %192
        %193 = OpTypePointer StorageBuffer %189
        %194 = OpVariable %193 StorageBuffer
        %195 = OpTypePointer StorageBuffer %188
        %202 = OpTypePointer PhysicalStorageBuffer %20
        %204 = OpConstant %20 64
        %205 = OpConstant %20 1
        %206 = OpConstantComposite %118 %204 %205 %205
        %207 = OpTypeStruct %20 %20 %20 %82 %20
        %208 = OpTypeRuntimeArray %207
        %209 = OpTypeStruct %208
        %210 = OpTypePointer StorageBuffer %209
        %211 = OpVariable %210 StorageBuffer
          %4 = OpFunction %2 None %3
          %5 = OpLabel
        %117 = OpVariable %21 Function
        %138 = OpVariable %137 Function
        %171 = OpVariable %170 Function
        %179 = OpVariable %9 Function
        %183 = OpVariable %11 Function
        %184 = OpVariable %12 Function
        %122 = OpAccessChain %121 %120 %23
        %123 = OpLoad %20 %122
               OpStore %117 %123
        %124 = OpLoad %20 %117
        %130 = OpAccessChain %129 %127 %128
        %131 = OpLoad %20 %130
        %132 = OpUGreaterThanEqual %13 %124 %131
               OpSelectionMerge %134 None
               OpBranchConditional %132 %133 %134
        %133 = OpLabel
               OpReturn
        %134 = OpLabel
        %145 = OpLoad %20 %117
        %147 = OpAccessChain %146 %143 %144 %145
        %148 = OpLoad %139 %147
        %149 = OpCompositeExtract %6 %148 0
        %150 = OpAccessChain %12 %138 %144
               OpStore %150 %149
        %151 = OpCompositeExtract %6 %148 1
        %152 = OpAccessChain %12 %138 %83
               OpStore %152 %151
        %153 = OpCompositeExtract %6 %148 2
        %155 = OpAccessChain %12 %138 %154
               OpStore %155 %153
        %156 = OpCompositeExtract %6 %148 3
        %157 = OpAccessChain %12 %138 %128
               OpStore %157 %156
        %158 = OpCompositeExtract %20 %148 4
        %160 = OpAccessChain %21 %138 %159
               OpStore %160 %158
        %161 = OpCompositeExtract %20 %148 5
        %163 = OpAccessChain %21 %138 %162
               OpStore %163 %161
        %164 = OpCompositeExtract %20 %148 6
        %166 = OpAccessChain %21 %138 %165
               OpStore %166 %164
        %167 = OpCompositeExtract %20 %148 7
        %169 = OpAccessChain %21 %138 %168
               OpStore %169 %167
        %172 = OpAccessChain %12 %138 %144
        %173 = OpLoad %6 %172
        %174 = OpAccessChain %12 %138 %83
        %175 = OpLoad %6 %174
        %176 = OpAccessChain %12 %138 %154
        %177 = OpLoad %6 %176
        %178 = OpCompositeConstruct %10 %173 %175 %177
        %181 = OpAccessChain %180 %127 %154
        %182 = OpLoad %8 %181
               OpStore %179 %182
               OpStore %183 %178
        %185 = OpAccessChain %12 %138 %128
        %186 = OpLoad %6 %185
               OpStore %184 %186
        %187 = OpFunctionCall %13 %18 %179 %183 %184
               OpStore %171 %187
        %196 = OpAccessChain %195 %194 %144
        %197 = OpLoad %188 %196
        %198 = OpLoad %20 %117
        %199 = OpLoad %13 %171
        %200 = OpSelect %82 %199 %83 %144
        %201 = OpBitcast %20 %200
        %203 = OpAccessChain %202 %197 %144 %198 %83
               OpStore %203 %201 Aligned 4
               OpReturn
               OpFunctionEnd
         %18 = OpFunction %13 None %14
         %15 = OpFunctionParameter %9
         %16 = OpFunctionParameter %11
         %17 = OpFunctionParameter %12
         %19 = OpLabel
         %22 = OpVariable %21 Function
         %24 = OpVariable %21 Function
         %33 = OpVariable %21 Function
         %50 = OpVariable %12 Function
         %57 = OpVariable %56 Function
         %60 = OpVariable %21 Function
               OpStore %22 %23
               OpStore %24 %23
               OpBranch %25
         %25 = OpLabel
               OpLoopMerge %27 %28 None
               OpBranch %29
         %29 = OpLabel
         %30 = OpLoad %20 %24
         %32 = OpULessThan %13 %30 %31
               OpBranchConditional %32 %26 %27
         %26 = OpLabel
               OpStore %33 %23
               OpBranch %34
         %34 = OpLabel
               OpLoopMerge %36 %37 None
               OpBranch %38
         %38 = OpLabel
         %39 = OpLoad %20 %33
         %41 = OpULessThan %13 %39 %40
               OpBranchConditional %41 %35 %36
         %35 = OpLabel
         %42 = OpLoad %20 %22
         %43 = OpIEqual %13 %42 %40
         %44 = OpLoad %20 %22
         %45 = OpIEqual %13 %44 %31
         %46 = OpLogicalOr %13 %43 %45
               OpSelectionMerge %48 None
               OpBranchConditional %46 %47 %48
         %47 = OpLabel
               OpBranch %37
         %48 = OpLabel
         %51 = OpLoad %20 %33
         %52 = OpUGreaterThan %13 %51 %23
         %55 = OpSelect %6 %52 %53 %54
               OpStore %50 %55
               OpStore %57 %59
               OpStore %60 %23
               OpBranch %61
         %61 = OpLabel
               OpLoopMerge %63 %64 None
               OpBranch %65
         %65 = OpLabel
         %66 = OpLoad %20 %60
         %68 = OpULessThan %13 %66 %67
               OpBranchConditional %68 %62 %63
         %62 = OpLabel
         %69 = OpLoad %20 %60
         %70 = OpLoad %20 %60
         %71 = OpAccessChain %12 %15 %70 %31
         %72 = OpLoad %6 %71
         %73 = OpLoad %6 %50
         %74 = OpLoad %20 %60
         %75 = OpLoad %20 %24
         %76 = OpAccessChain %12 %15 %74 %75
         %77 = OpLoad %6 %76
         %78 = OpFMul %6 %73 %77
         %79 = OpFAdd %6 %72 %78
         %80 = OpAccessChain %12 %57 %69
               OpStore %80 %79
               OpBranch %64
         %64 = OpLabel
         %81 = OpLoad %20 %60
         %84 = OpIAdd %20 %81 %83
               OpStore %60 %84
               OpBranch %61
         %63 = OpLabel
         %85 = OpLoad %7 %57
         %86 = OpVectorShuffle %10 %85 %85 0 1 2
         %87 = OpLoad %7 %57
         %88 = OpVectorShuffle %10 %87 %87 0 1 2
         %89 = OpDot %6 %86 %88
         %90 = OpExtInst %6 %1 Sqrt %89
         %91 = OpLoad %7 %57
         %92 = OpCompositeConstruct %7 %90 %90 %90 %90
         %93 = OpFDiv %7 %91 %92
               OpStore %57 %93
         %94 = OpLoad %10 %16
         %95 = OpLoad %7 %57
         %96 = OpVectorShuffle %10 %95 %95 0 1 2
         %97 = OpDot %6 %94 %96
         %98 = OpAccessChain %12 %57 %31
         %99 = OpLoad %6 %98
        %100 = OpFAdd %6 %97 %99
        %101 = OpLoad %6 %17
        %102 = OpFAdd %6 %100 %101
        %103 = OpFOrdLessThan %13 %102 %58
               OpSelectionMerge %105 None
               OpBranchConditional %103 %104 %105
        %104 = OpLabel
               OpReturnValue %106
        %105 = OpLabel
               OpBranch %37
         %37 = OpLabel
        %108 = OpLoad %20 %33
        %109 = OpIAdd %20 %108 %83
               OpStore %33 %109
        %110 = OpLoad %20 %22
        %111 = OpIAdd %20 %110 %83
               OpStore %22 %111
               OpBranch %34
         %36 = OpLabel
               OpBranch %28
         %28 = OpLabel
        %112 = OpLoad %20 %24
        %113 = OpIAdd %20 %112 %83
               OpStore %24 %113
               OpBranch %25
         %27 = OpLabel
               OpReturnValue %114
               OpFunctionEnd
