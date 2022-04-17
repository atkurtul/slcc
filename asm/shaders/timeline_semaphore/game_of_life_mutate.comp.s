; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 64
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
               OpName %v "v"
               OpName %ImageInput "ImageInput"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %Registers "Registers"
               OpMemberName %Registers 0 "counter"
               OpName %registers "registers"
               OpName %ImageOutput "ImageOutput"
               OpDecorate %ImageInput DescriptorSet 1
               OpDecorate %ImageInput Binding 0
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %Registers 0 Offset 0
               OpDecorate %Registers Block
               OpDecorate %ImageOutput DescriptorSet 0
               OpDecorate %ImageOutput Binding 0
               OpDecorate %ImageOutput NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
 %ImageInput = OpVariable %_ptr_UniformConstant_11 UniformConstant
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %v2uint = OpTypeVector %uint 2
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
      %int_0 = OpConstant %int 0
    %v3float = OpTypeVector %float 3
    %float_0 = OpConstant %float 0
         %32 = OpConstantComposite %v3float %float_0 %float_0 %float_0
       %bool = OpTypeBool
     %v3bool = OpTypeVector %bool 3
     %uint_3 = OpConstant %uint 3
%_ptr_Function_float = OpTypePointer Function %float
  %Registers = OpTypeStruct %float
%_ptr_PushConstant_Registers = OpTypePointer PushConstant %Registers
  %registers = OpVariable %_ptr_PushConstant_Registers PushConstant
%_ptr_PushConstant_float = OpTypePointer PushConstant %float
         %53 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_53 = OpTypePointer UniformConstant %53
%ImageOutput = OpVariable %_ptr_UniformConstant_53 UniformConstant
     %uint_8 = OpConstant %uint 8
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
          %v = OpVariable %_ptr_Function_v4float Function
         %14 = OpLoad %11 %ImageInput
         %20 = OpLoad %v3uint %gl_GlobalInvocationID
         %21 = OpVectorShuffle %v2uint %20 %20 0 1
         %24 = OpBitcast %v2int %21
         %26 = OpImage %10 %14
         %27 = OpImageFetch %v4float %26 %24 Lod %int_0
               OpStore %v %27
         %29 = OpLoad %v4float %v
         %30 = OpVectorShuffle %v3float %29 %29 0 1 2
         %35 = OpFUnordNotEqual %v3bool %30 %32
         %36 = OpAny %bool %35
               OpSelectionMerge %38 None
               OpBranchConditional %36 %37 %51
         %37 = OpLabel
         %41 = OpAccessChain %_ptr_Function_float %v %uint_3
         %42 = OpLoad %float %41
         %47 = OpAccessChain %_ptr_PushConstant_float %registers %int_0
         %48 = OpLoad %float %47
         %49 = OpExtInst %float %1 FMax %42 %48
         %50 = OpAccessChain %_ptr_Function_float %v %uint_3
               OpStore %50 %49
               OpBranch %38
         %51 = OpLabel
         %52 = OpAccessChain %_ptr_Function_float %v %uint_3
               OpStore %52 %float_0
               OpBranch %38
         %38 = OpLabel
         %56 = OpLoad %53 %ImageOutput
         %57 = OpLoad %v3uint %gl_GlobalInvocationID
         %58 = OpVectorShuffle %v2uint %57 %57 0 1
         %59 = OpBitcast %v2int %58
         %60 = OpLoad %v4float %v
               OpImageWrite %56 %59 %60
               OpReturn
               OpFunctionEnd
