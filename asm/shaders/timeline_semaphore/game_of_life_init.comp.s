; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 100
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
               OpName %index "index"
               OpName %gl_GlobalInvocationID "gl_GlobalInvocationID"
               OpName %mask "mask"
               OpName %wrapped_index "wrapped_index"
               OpName %is_alive "is_alive"
               OpName %Image "Image"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
               OpDecorate %Image DescriptorSet 0
               OpDecorate %Image Binding 0
               OpDecorate %Image NonReadable
               OpDecorate %gl_WorkGroupSize BuiltIn WorkgroupSize
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
        %int = OpTypeInt 32 1
      %v2int = OpTypeVector %int 2
%_ptr_Function_v2int = OpTypePointer Function %v2int
       %uint = OpTypeInt 32 0
     %v3uint = OpTypeVector %uint 3
%_ptr_Input_v3uint = OpTypePointer Input %v3uint
%gl_GlobalInvocationID = OpVariable %_ptr_Input_v3uint Input
     %v2uint = OpTypeVector %uint 2
      %int_3 = OpConstant %int 3
         %20 = OpConstantComposite %v2int %int_3 %int_3
      %int_7 = OpConstant %int 7
         %22 = OpConstantComposite %v2int %int_7 %int_7
     %int_16 = OpConstant %int 16
      %int_0 = OpConstant %int 0
         %28 = OpConstantComposite %v2int %int_0 %int_0
       %bool = OpTypeBool
     %v2bool = OpTypeVector %bool 2
      %int_1 = OpConstant %int 1
         %39 = OpConstantComposite %v2int %int_1 %int_0
      %int_2 = OpConstant %int 2
         %47 = OpConstantComposite %v2int %int_2 %int_1
         %55 = OpConstantComposite %v2int %int_0 %int_2
         %63 = OpConstantComposite %v2int %int_1 %int_2
         %71 = OpConstantComposite %v2int %int_2 %int_2
%_ptr_Function_bool = OpTypePointer Function %bool
       %true = OpConstantTrue %bool
      %false = OpConstantFalse %bool
      %float = OpTypeFloat 32
         %83 = OpTypeImage %float 2D 0 0 0 2 Rgba8
%_ptr_UniformConstant_83 = OpTypePointer UniformConstant %83
      %Image = OpVariable %_ptr_UniformConstant_83 UniformConstant
    %v4float = OpTypeVector %float 4
    %float_1 = OpConstant %float 1
    %float_0 = OpConstant %float 0
         %92 = OpConstantComposite %v4float %float_1 %float_1 %float_1 %float_0
         %93 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
     %v4bool = OpTypeVector %bool 4
     %uint_8 = OpConstant %uint 8
     %uint_1 = OpConstant %uint 1
%gl_WorkGroupSize = OpConstantComposite %v3uint %uint_8 %uint_8 %uint_1
       %main = OpFunction %void None %3
          %5 = OpLabel
      %index = OpVariable %_ptr_Function_v2int Function
       %mask = OpVariable %_ptr_Function_v2int Function
%wrapped_index = OpVariable %_ptr_Function_v2int Function
   %is_alive = OpVariable %_ptr_Function_bool Function
         %15 = OpLoad %v3uint %gl_GlobalInvocationID
         %16 = OpVectorShuffle %v2uint %15 %15 0 1
         %17 = OpBitcast %v2int %16
               OpStore %index %17
         %23 = OpLoad %v2int %index
         %25 = OpCompositeConstruct %v2int %int_16 %int_16
         %26 = OpBitwiseAnd %v2int %23 %25
         %31 = OpINotEqual %v2bool %26 %28
         %32 = OpSelect %v2int %31 %22 %20
               OpStore %mask %32
         %34 = OpLoad %v2int %index
         %35 = OpLoad %v2int %mask
         %36 = OpBitwiseAnd %v2int %34 %35
               OpStore %wrapped_index %36
         %37 = OpLoad %v2int %wrapped_index
         %40 = OpIEqual %v2bool %37 %39
         %41 = OpAll %bool %40
         %42 = OpLogicalNot %bool %41
               OpSelectionMerge %44 None
               OpBranchConditional %42 %43 %44
         %43 = OpLabel
         %45 = OpLoad %v2int %wrapped_index
         %48 = OpIEqual %v2bool %45 %47
         %49 = OpAll %bool %48
               OpBranch %44
         %44 = OpLabel
         %50 = OpPhi %bool %41 %5 %49 %43
         %51 = OpLogicalNot %bool %50
               OpSelectionMerge %53 None
               OpBranchConditional %51 %52 %53
         %52 = OpLabel
         %54 = OpLoad %v2int %wrapped_index
         %56 = OpIEqual %v2bool %54 %55
         %57 = OpAll %bool %56
               OpBranch %53
         %53 = OpLabel
         %58 = OpPhi %bool %50 %44 %57 %52
         %59 = OpLogicalNot %bool %58
               OpSelectionMerge %61 None
               OpBranchConditional %59 %60 %61
         %60 = OpLabel
         %62 = OpLoad %v2int %wrapped_index
         %64 = OpIEqual %v2bool %62 %63
         %65 = OpAll %bool %64
               OpBranch %61
         %61 = OpLabel
         %66 = OpPhi %bool %58 %53 %65 %60
         %67 = OpLogicalNot %bool %66
               OpSelectionMerge %69 None
               OpBranchConditional %67 %68 %69
         %68 = OpLabel
         %70 = OpLoad %v2int %wrapped_index
         %72 = OpIEqual %v2bool %70 %71
         %73 = OpAll %bool %72
               OpBranch %69
         %69 = OpLabel
         %74 = OpPhi %bool %66 %61 %73 %68
               OpSelectionMerge %76 None
               OpBranchConditional %74 %75 %80
         %75 = OpLabel
               OpStore %is_alive %true
               OpBranch %76
         %80 = OpLabel
               OpStore %is_alive %false
               OpBranch %76
         %76 = OpLabel
         %86 = OpLoad %83 %Image
         %87 = OpLoad %v2int %index
         %88 = OpLoad %bool %is_alive
         %95 = OpCompositeConstruct %v4bool %88 %88 %88 %88
         %96 = OpSelect %v4float %95 %92 %93
               OpImageWrite %86 %87 %96
               OpReturn
               OpFunctionEnd
