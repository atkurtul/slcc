; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 58
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %main "main"
               OpExecutionMode %main LocalSize 8 8 1
               OpSource GLSL 450
               OpSourceExtension "GL_EXT_buffer_reference"
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %Block2 "Block2"
               OpMemberName %Block2 0 "out_positions"
               OpName %_ ""
               OpName %Block "Block"
               OpMemberName %Block 0 "positions"
               OpName %__0 ""
               OpName %Block3 "Block3"
               OpMemberName %Block3 0 "rw_positions"
               OpName %__1 ""
               OpName %PushConstants "PushConstants"
               OpMemberName %PushConstants 0 "pc1"
               OpMemberName %PushConstants 1 "pc2"
               OpMemberName %PushConstants 2 "pc3"
               OpName %__2 ""
               OpName %Block4 "Block4"
               OpMemberName %Block4 0 "rw_positions2"
               OpName %__3 ""
               OpName %src "src"
               OpDecorate %_runtimearr_v2float ArrayStride 8
               OpMemberDecorate %Block2 0 NonReadable
               OpMemberDecorate %Block2 0 Offset 0
               OpDecorate %Block2 BufferBlock
               OpDecorate %_ DescriptorSet 0
               OpDecorate %_ Binding 1
               OpDecorate %_runtimearr_v2float_0 ArrayStride 8
               OpMemberDecorate %Block 0 NonWritable
               OpMemberDecorate %Block 0 Offset 0
               OpDecorate %Block BufferBlock
               OpDecorate %__0 DescriptorSet 0
               OpDecorate %__0 Binding 0
               OpDecorate %_runtimearr_v2float_1 ArrayStride 8
               OpMemberDecorate %Block3 0 Offset 0
               OpDecorate %Block3 BufferBlock
               OpDecorate %__1 DescriptorSet 0
               OpDecorate %__1 Binding 2
               OpMemberDecorate %PushConstants 0 Offset 0
               OpMemberDecorate %PushConstants 1 Offset 8
               OpMemberDecorate %PushConstants 2 Offset 16
               OpDecorate %PushConstants Block
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
               OpDecorate %_arr_v2float_uint_1 ArrayStride 16
               OpMemberDecorate %Block4 0 Offset 0
               OpDecorate %Block4 Block
               OpDecorate %__3 DescriptorSet 0
               OpDecorate %__3 Binding 4
               OpDecorate %src DescriptorSet 0
               OpDecorate %src Binding 3
               OpDecorate %src NonReadable
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v2float = OpTypeVector %float 2
%_runtimearr_v2float = OpTypeRuntimeArray %v2float
     %Block2 = OpTypeStruct %_runtimearr_v2float
%_ptr_Uniform_Block2 = OpTypePointer Uniform %Block2
          %_ = OpVariable %_ptr_Uniform_Block2 Uniform
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_runtimearr_v2float_0 = OpTypeRuntimeArray %v2float
      %Block = OpTypeStruct %_runtimearr_v2float_0
%_ptr_Uniform_Block = OpTypePointer Uniform %Block
        %__0 = OpVariable %_ptr_Uniform_Block Uniform
%_ptr_Uniform_v2float = OpTypePointer Uniform %v2float
%_runtimearr_v2float_1 = OpTypeRuntimeArray %v2float
     %Block3 = OpTypeStruct %_runtimearr_v2float_1
%_ptr_Uniform_Block3 = OpTypePointer Uniform %Block3
        %__1 = OpVariable %_ptr_Uniform_Block3 Uniform
%PushConstants = OpTypeStruct %v2float %v2float %v2float
%_ptr_PushConstant_PushConstants = OpTypePointer PushConstant %PushConstants
        %__2 = OpVariable %_ptr_PushConstant_PushConstants PushConstant
%_ptr_PushConstant_v2float = OpTypePointer PushConstant %v2float
      %int_1 = OpConstant %int 1
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
     %uint_8 = OpConstant %uint 8
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
%_arr_v2float_uint_1 = OpTypeArray %v2float %uint_1
     %Block4 = OpTypeStruct %_arr_v2float_uint_1
%_ptr_Uniform_Block4 = OpTypePointer Uniform %Block4
        %__3 = OpVariable %_ptr_Uniform_Block4 Uniform
         %55 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_55 = OpTypePointer UniformConstant %55
        %src = OpVariable %_ptr_UniformConstant_55 UniformConstant
       %main = OpFunction %void None %3
          %5 = OpLabel
         %19 = OpAccessChain %_ptr_Uniform_v2float %__0 %int_0 %int_0
         %20 = OpLoad %v2float %19
         %25 = OpAccessChain %_ptr_Uniform_v2float %__1 %int_0 %int_0
         %26 = OpLoad %v2float %25
         %27 = OpFAdd %v2float %20 %26
         %32 = OpAccessChain %_ptr_PushConstant_v2float %__2 %int_0
         %33 = OpLoad %v2float %32
         %34 = OpFAdd %v2float %27 %33
         %35 = OpAccessChain %_ptr_Uniform_v2float %_ %int_0 %int_0
               OpStore %35 %34
         %36 = OpAccessChain %_ptr_Uniform_v2float %__0 %int_0 %int_0
         %37 = OpLoad %v2float %36
         %39 = OpAccessChain %_ptr_Uniform_v2float %__0 %int_0 %int_1
         %40 = OpLoad %v2float %39
         %41 = OpFAdd %v2float %37 %40
         %42 = OpAccessChain %_ptr_PushConstant_v2float %__2 %int_1
         %43 = OpLoad %v2float %42
         %44 = OpFAdd %v2float %41 %43
         %45 = OpAccessChain %_ptr_Uniform_v2float %__1 %int_0 %int_0
               OpStore %45 %44
               OpReturn
               OpFunctionEnd
