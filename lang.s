; SPIR-V
; Version: 1.3
; Generator: Khronos; 0
; Bound: 50
; Schema: 0
               OpCapability Kernel
               OpCapability Addresses
               OpCapability VulkanMemoryModel
               OpCapability Shader
               OpExtension "SPV_KHR_vulkan_memory_model"
               OpMemoryModel Logical Vulkan
               OpEntryPoint Kernel %1 "blur_box"
       %void = OpTypeVoid
      %float = OpTypeFloat 32
          %4 = OpTypeImage %float 2D 0 0 0 2 Rgba8
       %uint = OpTypeInt 32 0
          %3 = OpTypeFunction %void %4 %uint %4
     %v4uint = OpTypeVector %uint 4
%_ptr_Function_v4uint = OpTypePointer Function %v4uint
%_ptr_Function_uint = OpTypePointer Function %uint
     %v2uint = OpTypeVector %uint 2
%_ptr_Function_v2uint = OpTypePointer Function %v2uint
%_ptr_Function_uint_0 = OpTypePointer Function %uint
       %bool = OpTypeBool
     %uint_0 = OpConstant %uint 0
     %uint_1 = OpConstant %uint 1
          %1 = OpFunction %void None %3
          %7 = OpFunctionParameter %4
          %8 = OpFunctionParameter %uint
          %9 = OpFunctionParameter %4
         %10 = OpLabel
         %13 = OpVariable %_ptr_Function_v4uint Function
         %20 = OpVariable %_ptr_Function_uint Function
         %27 = OpVariable %_ptr_Function_v2uint Function
         %17 = OpCompositeConstruct %v4uint %uint_0 %uint_0 %uint_0 %uint_0
               OpStore %13 %17
               OpStore %20 %uint_0
         %23 = OpSNegate %uint %8
         %24 = OpSNegate %uint %8
         %25 = OpCompositeConstruct %v2uint %23 %24
               OpStore %27 %25
               OpBranch %29
         %29 = OpLabel
               OpLoopMerge %33 %32 None
               OpBranch %30
         %30 = OpLabel
         %36 = OpInBoundsAccessChain %_ptr_Function_uint_0 %27 %uint_0
         %37 = OpLoad %uint %36
         %39 = OpSLessThanEqual %bool %37 %8
               OpBranchConditional %39 %31 %33
         %31 = OpLabel
         %40 = OpLoad %uint %20
         %45 = OpIAdd %uint %40 %uint_1
               OpStore %20 %45
               OpBranch %32
         %32 = OpLabel
         %47 = OpInBoundsAccessChain %_ptr_Function_uint_0 %27 %uint_0
         %48 = OpLoad %uint %47
         %49 = OpIAdd %uint %48 %uint_1
               OpStore %47 %49
               OpBranch %29
         %33 = OpLabel
               OpReturn
               OpFunctionEnd
