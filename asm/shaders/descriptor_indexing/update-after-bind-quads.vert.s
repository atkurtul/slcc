; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 96
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %main "main" %gl_VertexIndex %out_uv %gl_InstanceIndex %_
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %local_offset "local_offset"
               OpName %gl_VertexIndex "gl_VertexIndex"
               OpName %out_uv "out_uv"
               OpName %cos_phase "cos_phase"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "phase"
               OpName %registers "registers"
               OpName %sin_phase "sin_phase"
               OpName %instance_x "instance_x"
               OpName %gl_InstanceIndex "gl_InstanceIndex"
               OpName %instance_y "instance_y"
               OpName %instance_offset "instance_offset"
               OpName %gl_PerVertex "gl_PerVertex"
               OpMemberName %gl_PerVertex 0 "gl_Position"
               OpMemberName %gl_PerVertex 1 "gl_PointSize"
               OpMemberName %gl_PerVertex 2 "gl_ClipDistance"
               OpMemberName %gl_PerVertex 3 "gl_CullDistance"
               OpName %_ ""
               OpDecorate %gl_VertexIndex BuiltIn VertexIndex
               OpDecorate %out_uv Location 0
               OpMemberDecorate %Registers 0 Offset 0
               OpDecorate %Registers Block
               OpDecorate %gl_InstanceIndex BuiltIn InstanceIndex
               OpMemberDecorate %gl_PerVertex 0 BuiltIn Position
               OpMemberDecorate %gl_PerVertex 1 BuiltIn PointSize
               OpMemberDecorate %gl_PerVertex 2 BuiltIn ClipDistance
               OpMemberDecorate %gl_PerVertex 3 BuiltIn CullDistance
               OpDecorate %gl_PerVertex Block
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
        %int = OpTypeInt 32 1
%_ptr_Input_int = OpTypePointer Input %int
%gl_VertexIndex = OpVariable %_ptr_Input_int Input
      %int_1 = OpConstant %int 1
%_ptr_Output_v2float = OpTypePointer Output %v2float
     %out_uv = OpVariable %_ptr_Output_v2float Output
%_ptr_Function_float = OpTypePointer Function %float
  %Registers = OpTypeStruct %float
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_float = OpTypePointer PushConstant %float
%mat2v2float = OpTypeMatrix %v2float 2
    %float_1 = OpConstant %float 1
    %float_0 = OpConstant %float 0
  %float_0_5 = OpConstant %float 0.5
%_ptr_Function_int = OpTypePointer Function %int
%gl_InstanceIndex = OpVariable %_ptr_Input_int Input
      %int_8 = OpConstant %int 8
   %float_15 = OpConstant %float 15
    %float_7 = OpConstant %float 7
         %72 = OpConstantComposite %v2float %float_15 %float_7
%float_2_0999999 = OpConstant %float 2.0999999
    %v4float = OpTypeVector %float 4
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
%_arr_float_uint_1 = OpTypeArray %float %uint_1
%gl_PerVertex = OpTypeStruct %v4float %float %_arr_float_uint_1 %_arr_float_uint_1
%_ptr_Output_gl_PerVertex = OpTypePointer Output %gl_PerVertex
          %_ = OpVariable %_ptr_Output_gl_PerVertex Output
%float_0_100000001 = OpConstant %float 0.100000001
%_ptr_Output_v4float = OpTypePointer Output %v4float
       %main = OpFunction %void None %3
          %5 = OpLabel
%local_offset = OpVariable %_ptr_Function_v2float Function
  %cos_phase = OpVariable %_ptr_Function_float Function
  %sin_phase = OpVariable %_ptr_Function_float Function
 %instance_x = OpVariable %_ptr_Function_int Function
 %instance_y = OpVariable %_ptr_Function_int Function
%instance_offset = OpVariable %_ptr_Function_v2float Function
         %13 = OpLoad %int %gl_VertexIndex
         %15 = OpBitwiseAnd %int %13 %int_1
         %16 = OpConvertSToF %float %15
         %17 = OpLoad %int %gl_VertexIndex
         %18 = OpShiftRightArithmetic %int %17 %int_1
         %19 = OpConvertSToF %float %18
         %20 = OpCompositeConstruct %v2float %16 %19
               OpStore %local_offset %20
         %23 = OpLoad %v2float %local_offset
               OpStore %out_uv %23
         %31 = OpAccessChain %_ptr_PushConstant_float %registers %int_0
         %32 = OpLoad %float %31
         %33 = OpExtInst %float %1 Cos %32
               OpStore %cos_phase %33
         %35 = OpAccessChain %_ptr_PushConstant_float %registers %int_0
         %36 = OpLoad %float %35
         %37 = OpExtInst %float %1 Sin %36
               OpStore %sin_phase %37
         %38 = OpLoad %float %cos_phase
         %39 = OpLoad %float %sin_phase
         %40 = OpFNegate %float %39
         %41 = OpLoad %float %sin_phase
         %42 = OpLoad %float %cos_phase
         %46 = OpCompositeConstruct %v2float %38 %40
         %47 = OpCompositeConstruct %v2float %41 %42
         %48 = OpCompositeConstruct %mat2v2float %46 %47
         %49 = OpLoad %v2float %local_offset
         %51 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
         %52 = OpFSub %v2float %49 %51
         %53 = OpMatrixTimesVector %v2float %48 %52
               OpStore %local_offset %53
         %57 = OpLoad %int %gl_InstanceIndex
         %59 = OpSMod %int %57 %int_8
         %60 = OpIAdd %int %59 %int_8
               OpStore %instance_x %60
         %62 = OpLoad %int %gl_InstanceIndex
         %63 = OpSDiv %int %62 %int_8
               OpStore %instance_y %63
         %65 = OpLoad %int %instance_x
         %66 = OpConvertSToF %float %65
         %67 = OpLoad %int %instance_y
         %68 = OpConvertSToF %float %67
         %69 = OpCompositeConstruct %v2float %66 %68
         %73 = OpFDiv %v2float %69 %72
               OpStore %instance_offset %73
         %75 = OpLoad %v2float %instance_offset
         %76 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
         %77 = OpFSub %v2float %75 %76
         %78 = OpVectorTimesScalar %v2float %77 %float_2_0999999
               OpStore %instance_offset %78
         %87 = OpLoad %v2float %local_offset
         %88 = OpVectorTimesScalar %v2float %87 %float_0_100000001
         %89 = OpLoad %v2float %instance_offset
         %90 = OpFAdd %v2float %88 %89
         %91 = OpCompositeExtract %float %90 0
         %92 = OpCompositeExtract %float %90 1
         %93 = OpCompositeConstruct %v4float %91 %92 %float_0 %float_1
         %95 = OpAccessChain %_ptr_Output_v4float %_ %int_0
               OpStore %95 %93
               OpReturn
               OpFunctionEnd
