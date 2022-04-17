; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 54
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main" %gl_GlobalInvocationID
               OpExecutionMode %main LocalSize 1 1 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %index "index"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %position "position"
               OpName %Particle "Particle"
               OpMemberName %Particle 0 "pos"
               OpMemberName %Particle 1 "vel"
               OpName %Pos "Pos"
               OpMemberName %Pos 0 "particles"
               OpName %_ ""
               OpName %velocity "velocity"
               OpName %UBO "UBO"
               OpMemberName %UBO 0 "deltaT"
               OpMemberName %UBO 1 "particleCount"
               OpName %ubo "ubo"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpMemberDecorate %Particle 0 Offset 0
               OpMemberDecorate %Particle 1 Offset 16
               OpDecorate %_runtimearr_Particle ArrayStride 32
               OpMemberDecorate %Pos 0 Offset 0
               OpDecorate %Pos BufferBlock
               OpDecorate %_ DescriptorSet 0
               OpDecorate %_ Binding 0
               OpMemberDecorate %UBO 0 Offset 0
               OpMemberDecorate %UBO 1 Offset 4
               OpDecorate %UBO Block
               OpDecorate %ubo DescriptorSet 0
               OpDecorate %ubo Binding 1
               OpDecorate %51 SpecId 0
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
%_ptr_Function_int = OpTypePointer Function %int
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
      %v3int = OpTypeVector %int 3
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
   %Particle = OpTypeStruct %v4float %v4float
%_runtimearr_Particle = OpTypeRuntimeArray %Particle
        %Pos = OpTypeStruct %_runtimearr_Particle
%_ptr_Uniform_Pos = OpTypePointer Uniform %Pos
          %_ = OpVariable %_ptr_Uniform_Pos Uniform
      %int_0 = OpConstant %int 0
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
      %int_1 = OpConstant %int 1
        %UBO = OpTypeStruct %float %int
%_ptr_Uniform_UBO = OpTypePointer Uniform %UBO
        %ubo = OpVariable %_ptr_Uniform_UBO Uniform
%_ptr_Uniform_float = OpTypePointer Uniform %float
%float_0_0500000007 = OpConstant %float 0.0500000007
         %51 = OpSpecConstant %uint 1
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpSpecConstantComposite %v3uint %51 %uint_1 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %index = OpVariable %_ptr_Function_int Function
   %position = OpVariable %_ptr_Function_v4float Function
   %velocity = OpVariable %_ptr_Function_v4float Function
         %13 = OpLoad %v3uint %gl_GlobalInvocationID
         %15 = OpBitcast %v3int %13
         %16 = OpCompositeExtract %int %15 0
               OpStore %index %16
         %27 = OpLoad %int %index
         %29 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %27 %int_0
         %30 = OpLoad %v4float %29
               OpStore %position %30
         %32 = OpLoad %int %index
         %34 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %32 %int_1
         %35 = OpLoad %v4float %34
               OpStore %velocity %35
         %40 = OpAccessChain %_ptr_Uniform_float %ubo %int_0
         %41 = OpLoad %float %40
         %43 = OpFMul %float %41 %float_0_0500000007
         %44 = OpLoad %v4float %velocity
         %45 = OpVectorTimesScalar %v4float %44 %43
         %46 = OpLoad %v4float %position
         %47 = OpFAdd %v4float %46 %45
               OpStore %position %47
         %48 = OpLoad %int %index
         %49 = OpLoad %v4float %position
         %50 = OpAccessChain %_ptr_Uniform_v4float %_ %int_0 %48 %int_0
               OpStore %50 %49
               OpReturn
               OpFunctionEnd
