; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 125
; Schema: 0
               OpCapability Shader
               OpCapability PhysicalStorageBufferAddresses
               OpExtension "SPV_KHR_physical_storage_buffer"
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel PhysicalStorageBuffer64 GLSL450
               OpEntryPoint Vertex %main "main" %gl_InstanceIndex %gl_VertexIndex %_ %out_color
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %slice "slice"
               OpName %gl_InstanceIndex "gl_InstanceIndex"
               OpName %Position "Position"
               OpMemberName %Position 0 "positions"
               OpName %positions "positions"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "view_projection"
               OpMemberName %Registers 1 "references"
               OpName %PositionReferences "PositionReferences"
               OpMemberName %PositionReferences 0 "buffers"
               OpName %registers "registers"
               OpName %pos "pos"
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpName %index_x "index_x"
               OpName %index_y "index_y"
               OpName %r "r"
               OpName %g "g"
               OpName %checkerboard "checkerboard"
               OpName %out_color "out_color"
               OpDecorate %gl_InstanceIndex BuiltIn InstanceIndex
               OpDecorate %_runtimearr_v2float ArrayStride 8
               OpMemberDecorate %Position 0 NonWritable
               OpMemberDecorate %Position 0 Offset 0
               OpDecorate %Position Block
               OpDecorate %positions RestrictPointer
               OpMemberDecorate %Registers 0 ColMajor
               OpMemberDecorate %Registers 0 Offset 0
               OpMemberDecorate %Registers 0 MatrixStride 16
               OpMemberDecorate %Registers 1 Offset 64
               OpDecorate %Registers Block
               OpDecorate %_runtimearr__ptr_PhysicalStorageBuffer_Position ArrayStride 8
               OpMemberDecorate %PositionReferences 0 NonWritable
               OpMemberDecorate %PositionReferences 0 Offset 0
               OpDecorate %PositionReferences Block
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
               OpDecorate %out_color Flat
               OpDecorate %out_color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
%_ptr_Function_int = OpTypePointer Function %int
%_ptr_Input_int = OpTypePointer Input %int
%gl_InstanceIndex = OpVariable %_ptr_Input_int Input
               OpTypeForwardPointer %_ptr_PhysicalStorageBuffer_Position PhysicalStorageBuffer
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_runtimearr_v2float = OpTypeRuntimeArray %v2float
   %Position = OpTypeStruct %_runtimearr_v2float
%_ptr_PhysicalStorageBuffer_Position = OpTypePointer PhysicalStorageBuffer %Position
%_ptr_Function__ptr_PhysicalStorageBuffer_Position = OpTypePointer Function %_ptr_PhysicalStorageBuffer_Position
    %v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
               OpTypeForwardPointer %_ptr_PhysicalStorageBuffer_PositionReferences PhysicalStorageBuffer
  %Registers = OpTypeStruct %mat4v4float %_ptr_PhysicalStorageBuffer_PositionReferences
%_runtimearr__ptr_PhysicalStorageBuffer_Position = OpTypeRuntimeArray %_ptr_PhysicalStorageBuffer_Position
%PositionReferences = OpTypeStruct %_runtimearr__ptr_PhysicalStorageBuffer_Position
%_ptr_PhysicalStorageBuffer_PositionReferences = OpTypePointer PhysicalStorageBuffer %PositionReferences
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
      %int_1 = OpConstant %int 1
%_ptr_PushConstant__ptr_PhysicalStorageBuffer_PositionReferences = OpTypePointer PushConstant %_ptr_PhysicalStorageBuffer_PositionReferences
      %int_0 = OpConstant %int 0
%_ptr_PhysicalStorageBuffer__ptr_PhysicalStorageBuffer_Position = OpTypePointer PhysicalStorageBuffer %_ptr_PhysicalStorageBuffer_Position
%_ptr_Function_v2float = OpTypePointer Function %v2float
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
%_ptr_PhysicalStorageBuffer_v2float = OpTypePointer PhysicalStorageBuffer %v2float
  %float_2_5 = OpConstant %float 2.5
    %float_3 = OpConstant %float 3
      %int_8 = OpConstant %int 8
  %float_3_5 = OpConstant %float 3.5
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
%_ptr_PushConstant_mat4v4float = OpTypePointer PushConstant %mat4v4float
    %float_0 = OpConstant %float 0
    %float_1 = OpConstant %float 1
%_ptr_Output_v4float = OpTypePointer Output %v4float
     %int_16 = OpConstant %int 16
%_ptr_Function_float = OpTypePointer Function %float
  %float_0_5 = OpConstant %float 0.5
%float_0_300000012 = OpConstant %float 0.300000012
%float_0_800000012 = OpConstant %float 0.800000012
%float_0_200000003 = OpConstant %float 0.200000003
  %out_color = OpVariable %_ptr_Output_v4float Output
%float_0_150000006 = OpConstant %float 0.150000006
       %main = OpFunction %void None %3
          %5 = OpLabel
      %slice = OpVariable %_ptr_Function_int Function
  %positions = OpVariable %_ptr_Function__ptr_PhysicalStorageBuffer_Position Function
        %pos = OpVariable %_ptr_Function_v2float Function
    %index_x = OpVariable %_ptr_Function_int Function
    %index_y = OpVariable %_ptr_Function_int Function
          %r = OpVariable %_ptr_Function_float Function
          %g = OpVariable %_ptr_Function_float Function
%checkerboard = OpVariable %_ptr_Function_int Function
         %11 = OpLoad %int %gl_InstanceIndex
               OpStore %slice %11
         %29 = OpAccessChain %_ptr_PushConstant__ptr_PhysicalStorageBuffer_PositionReferences %registers %int_1
         %30 = OpLoad %_ptr_PhysicalStorageBuffer_PositionReferences %29
         %32 = OpLoad %int %slice
         %34 = OpAccessChain %_ptr_PhysicalStorageBuffer__ptr_PhysicalStorageBuffer_Position %30 %int_0 %32
         %35 = OpLoad %_ptr_PhysicalStorageBuffer_Position %34 Aligned 8
               OpStore %positions %35
         %38 = OpLoad %_ptr_PhysicalStorageBuffer_Position %positions
         %40 = OpLoad %int %gl_VertexIndex
         %42 = OpAccessChain %_ptr_PhysicalStorageBuffer_v2float %38 %int_0 %40
         %43 = OpLoad %v2float %42 Aligned 8
         %45 = OpVectorTimesScalar %v2float %43 %float_2_5
               OpStore %pos %45
         %47 = OpLoad %int %slice
         %49 = OpSMod %int %47 %int_8
         %50 = OpConvertSToF %float %49
         %51 = OpLoad %int %slice
         %52 = OpSDiv %int %51 %int_8
         %53 = OpConvertSToF %float %52
         %54 = OpCompositeConstruct %v2float %50 %53
         %56 = OpCompositeConstruct %v2float %float_3_5 %float_3_5
         %57 = OpFSub %v2float %54 %56
         %58 = OpVectorTimesScalar %v2float %57 %float_3
         %59 = OpLoad %v2float %pos
         %60 = OpFAdd %v2float %59 %58
               OpStore %pos %60
         %68 = OpAccessChain %_ptr_PushConstant_mat4v4float %registers %int_0
         %69 = OpLoad %mat4v4float %68
         %70 = OpLoad %v2float %pos
         %73 = OpCompositeExtract %float %70 0
         %74 = OpCompositeExtract %float %70 1
         %75 = OpCompositeConstruct %v4float %73 %74 %float_0 %float_1
         %76 = OpMatrixTimesVector %v4float %69 %75
         %78 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %78 %76
         %80 = OpLoad %int %gl_VertexIndex
         %82 = OpSMod %int %80 %int_16
               OpStore %index_x %82
         %84 = OpLoad %int %gl_VertexIndex
         %85 = OpSDiv %int %84 %int_16
               OpStore %index_y %85
         %90 = OpLoad %int %index_x
         %91 = OpConvertSToF %float %90
         %92 = OpExtInst %float %1 Sin %91
         %93 = OpFMul %float %float_0_300000012 %92
         %94 = OpFAdd %float %float_0_5 %93
               OpStore %r %94
         %96 = OpLoad %int %index_y
         %97 = OpConvertSToF %float %96
         %98 = OpExtInst %float %1 Sin %97
         %99 = OpFMul %float %float_0_300000012 %98
        %100 = OpFAdd %float %float_0_5 %99
               OpStore %g %100
        %102 = OpLoad %int %index_x
        %103 = OpLoad %int %index_y
        %104 = OpBitwiseXor %int %102 %103
        %105 = OpBitwiseAnd %int %104 %int_1
               OpStore %checkerboard %105
        %106 = OpLoad %int %checkerboard
        %107 = OpConvertSToF %float %106
        %109 = OpFMul %float %107 %float_0_800000012
        %111 = OpFAdd %float %109 %float_0_200000003
        %112 = OpLoad %float %r
        %113 = OpFMul %float %112 %111
               OpStore %r %113
        %114 = OpLoad %int %checkerboard
        %115 = OpConvertSToF %float %114
        %116 = OpFMul %float %115 %float_0_800000012
        %117 = OpFAdd %float %116 %float_0_200000003
        %118 = OpLoad %float %g
        %119 = OpFMul %float %118 %117
               OpStore %g %119
        %121 = OpLoad %float %r
        %122 = OpLoad %float %g
        %124 = OpCompositeConstruct %v4float %121 %122 %float_0_150000006 %float_1
               OpStore %out_color %124
               OpReturn
               OpFunctionEnd
