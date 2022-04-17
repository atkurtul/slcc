; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 79
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "resolution"
               OpMemberName %Registers 1 "inv_resolution"
               OpMemberName %Registers 2 "inv_input_resolution"
               OpName %registers "registers"
               OpName %uv "uv"
               OpName %rgb "rgb"
               OpName %in_tex "in_tex"
               OpName %out_tex "out_tex"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %Registers 0 Offset 0
               OpMemberDecorate %Registers 1 Offset 8
               OpMemberDecorate %Registers 2 Offset 16
               OpDecorate %Registers Block
               OpDecorate %in_tex DescriptorSet 0
               OpDecorate %in_tex Binding 0
               OpDecorate %out_tex DescriptorSet 0
               OpDecorate %out_tex Binding 1
               OpDecorate %out_tex NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %v2uint = OpTypeVector %uint 2
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
  %Registers = OpTypeStruct %v2uint %v2float %v2float
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_v2uint = OpTypePointer PushConstant %v2uint
       %bool = OpTypeBool
     %v2bool = OpTypeVector %bool 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
  %float_0_5 = OpConstant %float 0.5
      %int_1 = OpConstant %int 1
%_ptr_PushConstant_v2float = OpTypePointer PushConstant %v2float
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
         %45 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %46 = OpTypeSampledImage %45
%_ptr_UniformConstant_46 = OpTypePointer UniformConstant %46
     %in_tex = OpVariable %_ptr_UniformConstant_46 UniformConstant
    %float_0 = OpConstant %float 0
    %v4float = OpTypeVector %float 4
%float_0_200000003 = OpConstant %float 0.200000003
    %float_1 = OpConstant %float 1
         %60 = OpConstantComposite %v3float %float_0 %float_0 %float_0
         %63 = OpTypeImage %float 2D 0 0 0 2 Rgba16f
%_ptr_UniformConstant_63 = OpTypePointer UniformConstant %63
    %out_tex = OpVariable %_ptr_UniformConstant_63 UniformConstant
      %v2int = OpTypeVector %int 2
     %uint_8 = OpConstant %uint 8
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
         %uv = OpVariable %_ptr_Function_v2float Function
        %rgb = OpVariable %_ptr_Function_v3float Function
         %11 = OpLoad %v3uint %gl_GlobalInvocationID
         %12 = OpVectorShuffle %v2uint %11 %11 0 1
         %21 = OpAccessChain %_ptr_PushConstant_v2uint %registers %int_0
         %22 = OpLoad %v2uint %21
         %25 = OpULessThan %v2bool %12 %22
         %26 = OpAll %bool %25
               OpSelectionMerge %28 None
               OpBranchConditional %26 %27 %28
         %27 = OpLabel
         %31 = OpLoad %v3uint %gl_GlobalInvocationID
         %32 = OpVectorShuffle %v2uint %31 %31 0 1
         %33 = OpConvertUToF %v2float %32
         %35 = OpCompositeConstruct %v2float %float_0_5 %float_0_5
         %36 = OpFAdd %v2float %33 %35
         %39 = OpAccessChain %_ptr_PushConstant_v2float %registers %int_1
         %40 = OpLoad %v2float %39
         %41 = OpFMul %v2float %36 %40
               OpStore %uv %41
         %49 = OpLoad %46 %in_tex
         %50 = OpLoad %v2float %uv
         %53 = OpImageSampleExplicitLod %v4float %49 %50 Lod %float_0
         %54 = OpVectorShuffle %v3float %53 %53 0 1 2
               OpStore %rgb %54
         %56 = OpLoad %v3float %rgb
         %58 = OpCompositeConstruct %v3float %float_1 %float_1 %float_1
         %59 = OpFSub %v3float %56 %58
         %61 = OpExtInst %v3float %1 FMax %59 %60
         %62 = OpVectorTimesScalar %v3float %61 %float_0_200000003
               OpStore %rgb %62
         %66 = OpLoad %63 %out_tex
         %67 = OpLoad %v3uint %gl_GlobalInvocationID
         %68 = OpVectorShuffle %v2uint %67 %67 0 1
         %70 = OpBitcast %v2int %68
         %71 = OpLoad %v3float %rgb
         %72 = OpCompositeExtract %float %71 0
         %73 = OpCompositeExtract %float %71 1
         %74 = OpCompositeExtract %float %71 2
         %75 = OpCompositeConstruct %v4float %72 %73 %74 %float_1
               OpImageWrite %66 %70 %75
               OpBranch %28
         %28 = OpLabel
               OpReturn
               OpFunctionEnd
