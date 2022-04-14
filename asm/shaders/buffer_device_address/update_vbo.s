; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 166
; Schema: 0
               OpCapability Shader
               OpCapability PhysicalStorageBufferAddresses
               OpExtension "SPV_KHR_physical_storage_buffer"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel PhysicalStorageBuffer64 GLSL450
               OpEntryPoint GLCompute %4 "main" %12 %22 %32
               OpExecutionMode %4 LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "local_offset"
               OpName %12 "gl_GlobalInvocationID"
               OpName %16 "local_index"
               OpName %22 "gl_NumWorkGroups"
               OpName %31 "slice"
               OpName %32 "gl_WorkGroupID"
               OpName %40 "Position"
               OpMemberName %40 0 "positions"
               OpName %42 "positions"
               OpName %44 "Registers"
               OpMemberName %44 0 "references"
               OpMemberName %44 1 "fract_time"
               OpName %46 "PositionReferences"
               OpMemberName %46 0 "buffers"
               OpName %48 "registers"
               OpName %59 "offset"
               OpName %73 "pos"
               OpDecorate %12 BuiltIn GlobalInvocationId
               OpDecorate %22 BuiltIn NumWorkgroups
               OpDecorate %32 BuiltIn WorkgroupId
               OpDecorate %39 ArrayStride 8
               OpMemberDecorate %40 0 NonReadable
               OpMemberDecorate %40 0 Offset 0
               OpDecorate %40 Block
               OpDecorate %42 RestrictPointer
               OpMemberDecorate %44 0 Offset 0
               OpMemberDecorate %44 1 Offset 8
               OpDecorate %44 Block
               OpDecorate %45 ArrayStride 8
               OpMemberDecorate %46 0 NonWritable
               OpMemberDecorate %46 0 Offset 0
               OpDecorate %46 Block
               OpDecorate %165 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 0
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
         %10 = OpTypeVector %6 3
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %15 = OpTypePointer Function %6
         %17 = OpConstant %6 1
         %20 = OpConstant %6 8
         %22 = OpVariable %11 Input
         %23 = OpConstant %6 0
         %24 = OpTypePointer Input %6
         %32 = OpVariable %11 Input
         %33 = OpConstant %6 2
               OpTypeForwardPointer %36 PhysicalStorageBuffer
         %37 = OpTypeFloat 32
         %38 = OpTypeVector %37 2
         %39 = OpTypeRuntimeArray %38
         %40 = OpTypeStruct %39
         %36 = OpTypePointer PhysicalStorageBuffer %40
         %41 = OpTypePointer Function %36
               OpTypeForwardPointer %43 PhysicalStorageBuffer
         %44 = OpTypeStruct %43 %37
         %45 = OpTypeRuntimeArray %36
         %46 = OpTypeStruct %45
         %43 = OpTypePointer PhysicalStorageBuffer %46
         %47 = OpTypePointer PushConstant %44
         %48 = OpVariable %47 PushConstant
         %49 = OpTypeInt 32 1
         %50 = OpConstant %49 0
         %51 = OpTypePointer PushConstant %43
         %55 = OpTypePointer PhysicalStorageBuffer %36
         %58 = OpTypePointer Function %37
         %60 = OpConstant %37 6.2831254
         %61 = OpConstant %49 1
         %62 = OpTypePointer PushConstant %37
         %67 = OpConstant %37 0.100000001
         %72 = OpTypePointer Function %38
         %76 = OpConstant %37 0.200000003
         %77 = OpConstant %37 2.20000005
         %89 = OpConstant %37 2.25
         %93 = OpConstant %37 2
        %103 = OpConstant %37 1.79999995
        %107 = OpConstant %37 3
        %117 = OpConstant %37 2.8499999
        %121 = OpConstant %37 4
        %131 = OpConstant %37 0.5
        %140 = OpConstant %37 0.300000012
        %151 = OpConstant %37 8
        %152 = OpConstantComposite %38 %151 %151
        %157 = OpConstant %37 1
        %163 = OpTypePointer PhysicalStorageBuffer %38
        %165 = OpConstantComposite %10 %20 %20 %17
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %16 = OpVariable %15 Function
         %31 = OpVariable %15 Function
         %42 = OpVariable %41 Function
         %59 = OpVariable %58 Function
         %73 = OpVariable %72 Function
         %13 = OpLoad %10 %12
         %14 = OpVectorShuffle %7 %13 %13 0 1
               OpStore %9 %14
         %18 = OpAccessChain %15 %9 %17
         %19 = OpLoad %6 %18
         %21 = OpIMul %6 %19 %20
         %25 = OpAccessChain %24 %22 %23
         %26 = OpLoad %6 %25
         %27 = OpIMul %6 %21 %26
         %28 = OpAccessChain %15 %9 %23
         %29 = OpLoad %6 %28
         %30 = OpIAdd %6 %27 %29
               OpStore %16 %30
         %34 = OpAccessChain %24 %32 %33
         %35 = OpLoad %6 %34
               OpStore %31 %35
         %52 = OpAccessChain %51 %48 %50
         %53 = OpLoad %43 %52
         %54 = OpLoad %6 %31
         %56 = OpAccessChain %55 %53 %50 %54
         %57 = OpLoad %36 %56 Aligned 8
               OpStore %42 %57
         %63 = OpAccessChain %62 %48 %61
         %64 = OpLoad %37 %63
         %65 = OpLoad %6 %31
         %66 = OpConvertUToF %37 %65
         %68 = OpFMul %37 %66 %67
         %69 = OpFAdd %37 %64 %68
         %70 = OpExtInst %37 %1 Fract %69
         %71 = OpFMul %37 %60 %70
               OpStore %59 %71
         %74 = OpLoad %7 %9
         %75 = OpConvertUToF %38 %74
               OpStore %73 %75
         %78 = OpAccessChain %58 %73 %23
         %79 = OpLoad %37 %78
         %80 = OpFMul %37 %77 %79
         %81 = OpLoad %37 %59
         %82 = OpFAdd %37 %80 %81
         %83 = OpExtInst %37 %1 Sin %82
         %84 = OpFMul %37 %76 %83
         %85 = OpAccessChain %58 %73 %23
         %86 = OpLoad %37 %85
         %87 = OpFAdd %37 %86 %84
         %88 = OpAccessChain %58 %73 %23
               OpStore %88 %87
         %90 = OpAccessChain %58 %73 %17
         %91 = OpLoad %37 %90
         %92 = OpFMul %37 %89 %91
         %94 = OpLoad %37 %59
         %95 = OpFMul %37 %93 %94
         %96 = OpFAdd %37 %92 %95
         %97 = OpExtInst %37 %1 Sin %96
         %98 = OpFMul %37 %76 %97
         %99 = OpAccessChain %58 %73 %17
        %100 = OpLoad %37 %99
        %101 = OpFAdd %37 %100 %98
        %102 = OpAccessChain %58 %73 %17
               OpStore %102 %101
        %104 = OpAccessChain %58 %73 %17
        %105 = OpLoad %37 %104
        %106 = OpFMul %37 %103 %105
        %108 = OpLoad %37 %59
        %109 = OpFMul %37 %107 %108
        %110 = OpFAdd %37 %106 %109
        %111 = OpExtInst %37 %1 Cos %110
        %112 = OpFMul %37 %76 %111
        %113 = OpAccessChain %58 %73 %23
        %114 = OpLoad %37 %113
        %115 = OpFAdd %37 %114 %112
        %116 = OpAccessChain %58 %73 %23
               OpStore %116 %115
        %118 = OpAccessChain %58 %73 %23
        %119 = OpLoad %37 %118
        %120 = OpFMul %37 %117 %119
        %122 = OpLoad %37 %59
        %123 = OpFMul %37 %121 %122
        %124 = OpFAdd %37 %120 %123
        %125 = OpExtInst %37 %1 Cos %124
        %126 = OpFMul %37 %76 %125
        %127 = OpAccessChain %58 %73 %17
        %128 = OpLoad %37 %127
        %129 = OpFAdd %37 %128 %126
        %130 = OpAccessChain %58 %73 %17
               OpStore %130 %129
        %132 = OpLoad %37 %59
        %133 = OpExtInst %37 %1 Sin %132
        %134 = OpFMul %37 %131 %133
        %135 = OpAccessChain %58 %73 %23
        %136 = OpLoad %37 %135
        %137 = OpFAdd %37 %136 %134
        %138 = OpAccessChain %58 %73 %23
               OpStore %138 %137
        %139 = OpLoad %37 %59
        %141 = OpFAdd %37 %139 %140
        %142 = OpExtInst %37 %1 Sin %141
        %143 = OpFMul %37 %131 %142
        %144 = OpAccessChain %58 %73 %17
        %145 = OpLoad %37 %144
        %146 = OpFAdd %37 %145 %143
        %147 = OpAccessChain %58 %73 %17
               OpStore %147 %146
        %148 = OpLoad %36 %42
        %149 = OpLoad %6 %16
        %150 = OpLoad %38 %73
        %153 = OpLoad %10 %22
        %154 = OpVectorShuffle %7 %153 %153 0 1
        %155 = OpConvertUToF %38 %154
        %156 = OpFMul %38 %152 %155
        %158 = OpCompositeConstruct %38 %157 %157
        %159 = OpFSub %38 %156 %158
        %160 = OpFDiv %38 %150 %159
        %161 = OpCompositeConstruct %38 %131 %131
        %162 = OpFSub %38 %160 %161
        %164 = OpAccessChain %163 %148 %50 %149
               OpStore %164 %162 Aligned 8
               OpReturn
               OpFunctionEnd
