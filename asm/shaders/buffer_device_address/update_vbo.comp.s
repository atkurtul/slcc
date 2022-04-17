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
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID %gl_NumWorkGroups %gl_WorkGroupID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %local_offset "local_offset"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %local_index "local_index"
               OpName %gl_NumWorkGroups "gl_NumWorkGroups"
               OpName %slice "slice"
               OpName %gl_WorkGroupID "gl_WorkGroupID"
               OpName %Position "Position"
               OpMemberName %Position 0 "positions"
               OpName %positions "positions"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "references"
               OpMemberName %Registers 1 "fract_time"
               OpName %PositionReferences "PositionReferences"
               OpMemberName %PositionReferences 0 "buffers"
               OpName %registers "registers"
               OpName %offset "offset"
               OpName %pos "pos"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpDecorate %gl_NumWorkGroups BuiltIn NumWorkgroups
               OpDecorate %gl_WorkGroupID BuiltIn WorkgroupId
               OpDecorate %_runtimearr_v2float ArrayStride 8
               OpMemberDecorate %Position 0 NonReadable
               OpMemberDecorate %Position 0 Offset 0
               OpDecorate %Position Block
               OpDecorate %positions RestrictPointer
               OpMemberDecorate %Registers 0 Offset 0
               OpMemberDecorate %Registers 1 Offset 8
               OpDecorate %Registers Block
               OpDecorate %_runtimearr__ptr_PhysicalStorageBuffer_Position ArrayStride 8
               OpMemberDecorate %PositionReferences 0 NonWritable
               OpMemberDecorate %PositionReferences 0 Offset 0
               OpDecorate %PositionReferences Block
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %uint = OpTypeInt 32 0
     %v2uint = OpTypeVector %uint 2
%_ptr_Function_v2uint = OpTypePointer Function %v2uint
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
%_ptr_Function_uint = OpTypePointer Function %uint
     %uint_1 = OpConstant %uint 1
     %uint_8 = OpConstant %uint 8
%gl_NumWorkGroups = OpVariable %_ptr_Input_v3uint Input
     %uint_0 = OpConstant %uint 0
%_ptr_Input_uint = OpTypePointer Input %uint
%gl_WorkGroupID = OpVariable %_ptr_Input_v3uint Input
     %uint_2 = OpConstant %uint 2
               OpTypeForwardPointer %_ptr_PhysicalStorageBuffer_Position PhysicalStorageBuffer
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_runtimearr_v2float = OpTypeRuntimeArray %v2float
   %Position = OpTypeStruct %_runtimearr_v2float
%_ptr_PhysicalStorageBuffer_Position = OpTypePointer PhysicalStorageBuffer %Position
%_ptr_Function__ptr_PhysicalStorageBuffer_Position = OpTypePointer Function %_ptr_PhysicalStorageBuffer_Position
               OpTypeForwardPointer %_ptr_PhysicalStorageBuffer_PositionReferences PhysicalStorageBuffer
  %Registers = OpTypeStruct %_ptr_PhysicalStorageBuffer_PositionReferences %float
%_runtimearr__ptr_PhysicalStorageBuffer_Position = OpTypeRuntimeArray %_ptr_PhysicalStorageBuffer_Position
%PositionReferences = OpTypeStruct %_runtimearr__ptr_PhysicalStorageBuffer_Position
%_ptr_PhysicalStorageBuffer_PositionReferences = OpTypePointer PhysicalStorageBuffer %PositionReferences
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant__ptr_PhysicalStorageBuffer_PositionReferences = OpTypePointer PushConstant %_ptr_PhysicalStorageBuffer_PositionReferences
%_ptr_PhysicalStorageBuffer__ptr_PhysicalStorageBuffer_Position = OpTypePointer PhysicalStorageBuffer %_ptr_PhysicalStorageBuffer_Position
%_ptr_Function_float = OpTypePointer Function %float
%float_6_2831254 = OpConstant %float 6.2831254
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_float = OpTypePointer PushConstant %float
%float_0_100000001 = OpConstant %float 0.100000001
%_ptr_Function_v2float = OpTypePointer Function %v2float
%float_0_200000003 = OpConstant %float 0.200000003
%float_2_20000005 = OpConstant %float 2.20000005
 %float_2_25 = OpConstant %float 2.25
    %float_2 = OpConstant %float 2
%float_1_79999995 = OpConstant %float 1.79999995
    %float_3 = OpConstant %float 3
%float_2_8499999 = OpConstant %float 2.8499999
    %float_4 = OpConstant %float 4
  %float_0_5 = OpConstant %float 0.5
%float_0_300000012 = OpConstant %float 0.300000012
    %float_8 = OpConstant %float 8
        %152 = OpConstantComposite %v2float %float_8 %float_8
    %float_1 = OpConstant %float 1
%_ptr_PhysicalStorageBuffer_v2float = OpTypePointer PhysicalStorageBuffer %v2float
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
%local_offset = OpVariable %_ptr_Function_v2uint Function
%local_index = OpVariable %_ptr_Function_uint Function
      %slice = OpVariable %_ptr_Function_uint Function
  %positions = OpVariable %_ptr_Function__ptr_PhysicalStorageBuffer_Position Function
     %offset = OpVariable %_ptr_Function_float Function
        %pos = OpVariable %_ptr_Function_v2float Function
         %13 = OpLoad %v3uint %gl_GlobalInvocationID
         %14 = OpVectorShuffle %v2uint %13 %13 0 1
               OpStore %local_offset %14
         %18 = OpAccessChain %_ptr_Function_uint %local_offset %uint_1
         %19 = OpLoad %uint %18
         %21 = OpIMul %uint %19 %uint_8
         %25 = OpAccessChain %_ptr_Input_uint %gl_NumWorkGroups %uint_0
         %26 = OpLoad %uint %25
         %27 = OpIMul %uint %21 %26
         %28 = OpAccessChain %_ptr_Function_uint %local_offset %uint_0
         %29 = OpLoad %uint %28
         %30 = OpIAdd %uint %27 %29
               OpStore %local_index %30
         %34 = OpAccessChain %_ptr_Input_uint %gl_WorkGroupID %uint_2
         %35 = OpLoad %uint %34
               OpStore %slice %35
         %52 = OpAccessChain %_ptr_PushConstant__ptr_PhysicalStorageBuffer_PositionReferences %registers %int_0
         %53 = OpLoad %_ptr_PhysicalStorageBuffer_PositionReferences %52
         %54 = OpLoad %uint %slice
         %56 = OpAccessChain %_ptr_PhysicalStorageBuffer__ptr_PhysicalStorageBuffer_Position %53 %int_0 %54
         %57 = OpLoad %_ptr_PhysicalStorageBuffer_Position %56 Aligned 8
               OpStore %positions %57
         %63 = OpAccessChain %_ptr_PushConstant_float %registers %int_1
         %64 = OpLoad %float %63
         %65 = OpLoad %uint %slice
         %66 = OpConvertUToF %float %65
         %68 = OpFMul %float %66 %float_0_100000001
         %69 = OpFAdd %float %64 %68
         %70 = OpExtInst %float %1 Fract %69
         %71 = OpFMul %float %float_6_2831254 %70
               OpStore %offset %71
         %74 = OpLoad %v2uint %local_offset
         %75 = OpConvertUToF %v2float %74
               OpStore %pos %75
         %78 = OpAccessChain %_ptr_Function_float %pos %uint_0
         %79 = OpLoad %float %78
         %80 = OpFMul %float %float_2_20000005 %79
         %81 = OpLoad %float %offset
         %82 = OpFAdd %float %80 %81
         %83 = OpExtInst %float %1 Sin %82
         %84 = OpFMul %float %float_0_200000003 %83
         %85 = OpAccessChain %_ptr_Function_float %pos %uint_0
         %86 = OpLoad %float %85
         %87 = OpFAdd %float %86 %84
         %88 = OpAccessChain %_ptr_Function_float %pos %uint_0
               OpStore %88 %87
         %90 = OpAccessChain %_ptr_Function_float %pos %uint_1
         %91 = OpLoad %float %90
         %92 = OpFMul %float %float_2_25 %91
         %94 = OpLoad %float %offset
         %95 = OpFMul %float %float_2 %94
         %96 = OpFAdd %float %92 %95
         %97 = OpExtInst %float %1 Sin %96
         %98 = OpFMul %float %float_0_200000003 %97
         %99 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %100 = OpLoad %float %99
        %101 = OpFAdd %float %100 %98
        %102 = OpAccessChain %_ptr_Function_float %pos %uint_1
               OpStore %102 %101
        %104 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %105 = OpLoad %float %104
        %106 = OpFMul %float %float_1_79999995 %105
        %108 = OpLoad %float %offset
        %109 = OpFMul %float %float_3 %108
        %110 = OpFAdd %float %106 %109
        %111 = OpExtInst %float %1 Cos %110
        %112 = OpFMul %float %float_0_200000003 %111
        %113 = OpAccessChain %_ptr_Function_float %pos %uint_0
        %114 = OpLoad %float %113
        %115 = OpFAdd %float %114 %112
        %116 = OpAccessChain %_ptr_Function_float %pos %uint_0
               OpStore %116 %115
        %118 = OpAccessChain %_ptr_Function_float %pos %uint_0
        %119 = OpLoad %float %118
        %120 = OpFMul %float %float_2_8499999 %119
        %122 = OpLoad %float %offset
        %123 = OpFMul %float %float_4 %122
        %124 = OpFAdd %float %120 %123
        %125 = OpExtInst %float %1 Cos %124
        %126 = OpFMul %float %float_0_200000003 %125
        %127 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %128 = OpLoad %float %127
        %129 = OpFAdd %float %128 %126
        %130 = OpAccessChain %_ptr_Function_float %pos %uint_1
               OpStore %130 %129
        %132 = OpLoad %float %offset
        %133 = OpExtInst %float %1 Sin %132
        %134 = OpFMul %float %float_0_5 %133
        %135 = OpAccessChain %_ptr_Function_float %pos %uint_0
        %136 = OpLoad %float %135
        %137 = OpFAdd %float %136 %134
        %138 = OpAccessChain %_ptr_Function_float %pos %uint_0
               OpStore %138 %137
        %139 = OpLoad %float %offset
        %141 = OpFAdd %float %139 %float_0_300000012
        %142 = OpExtInst %float %1 Sin %141
        %143 = OpFMul %float %float_0_5 %142
        %144 = OpAccessChain %_ptr_Function_float %pos %uint_1
        %145 = OpLoad %float %144
        %146 = OpFAdd %float %145 %143
        %147 = OpAccessChain %_ptr_Function_float %pos %uint_1
               OpStore %147 %146
        %148 = OpLoad %_ptr_PhysicalStorageBuffer_Position %positions
        %149 = OpLoad %uint %local_index
        %150 = OpLoad %v2float %pos
        %153 = OpLoad %v3uint %gl_NumWorkGroups
        %154 = OpVectorShuffle %v2uint %153 %153 0 1
        %155 = OpConvertUToF %v2float %154
        %156 = OpFMul %v2float %152 %155
        %158 = OpCompositeConstruct %v2float %float_1 %float_1
        %159 = OpFSub %v2float %156 %158
        %160 = OpFDiv %v2float %150 %159
        %161 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
        %162 = OpFSub %v2float %160 %161
        %164 = OpAccessChain %_ptr_PhysicalStorageBuffer_v2float %148 %int_0 %149
               OpStore %164 %162 Aligned 8
               OpReturn
               OpFunctionEnd
