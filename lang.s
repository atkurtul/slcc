; SPIR-V
; Version: 1.3
; Generator: Khronos; 0
; Bound: 152
; Schema: 0
               OpCapability Kernel
               OpCapability Addresses
               OpMemoryModel Physical32 OpenCL
               OpEntryPoint Kernel %1 "blur_box"
               OpDecorate %gl_GlobalInvocationID BuiltIn GlobalInvocationId
       %void = OpTypeVoid
          %4 = OpTypeImage %void 2D 0 0 0 2 Unknown
       %uint = OpTypeInt 32 0
          %3 = OpTypeFunction %void %4 %uint %4
     %v2uint = OpTypeVector %uint 2
%_ptr_Function_v2uint = OpTypePointer Function %v2uint
%_ptr_Function_uint = OpTypePointer Function %uint
     %v3uint = OpTypeVector %uint 3
%_ptr_Function_v3uint = OpTypePointer Function %v3uint
%_ptr_Function_v2uint_0 = OpTypePointer Function %v2uint
     %v4uint = OpTypeVector %uint 4
%_ptr_Function_v4uint = OpTypePointer Function %v4uint
%_ptr_Function_uint_0 = OpTypePointer Function %uint
       %bool = OpTypeBool
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
     %uint_2 = OpConstant %uint 2
     %uint_0 = OpConstant %uint 0
     %uint_1 = OpConstant %uint 1
          %1 = OpFunction %void None %3
          %6 = OpFunctionParameter %4
          %7 = OpFunctionParameter %uint
          %8 = OpFunctionParameter %4
          %9 = OpLabel
         %14 = OpVariable %_ptr_Function_v2uint Function
         %21 = OpVariable %_ptr_Function_uint Function
         %26 = OpVariable %_ptr_Function_uint Function
%gl_GlobalInvocationID = OpVariable %_ptr_Function_v3uint Function
         %43 = OpVariable %_ptr_Function_v2uint_0 Function
         %47 = OpVariable %_ptr_Function_v4uint Function
         %54 = OpVariable %_ptr_Function_uint_0 Function
         %60 = OpVariable %_ptr_Function_v2uint Function
         %82 = OpVariable %_ptr_Function_v2uint Function
               OpBranch %10
         %10 = OpLabel
         %11 = OpImageQuerySize %v2uint %6
               OpStore %14 %11
         %19 = OpInBoundsAccessChain %_ptr_Function_uint %14 %uint_0
         %20 = OpLoad %uint %19
               OpStore %21 %20
         %24 = OpInBoundsAccessChain %_ptr_Function_uint %14 %uint_1
         %25 = OpLoad %uint %24
               OpStore %26 %25
         %32 = OpLoad %v3uint %gl_GlobalInvocationID
         %35 = OpVectorExtractDynamic %uint %32 %uint_0
         %37 = OpLoad %v3uint %gl_GlobalInvocationID
         %40 = OpVectorExtractDynamic %uint %37 %uint_1
         %41 = OpCompositeConstruct %v2uint %35 %40
               OpStore %43 %41
         %51 = OpCompositeConstruct %v4uint %uint_0 %uint_0 %uint_0 %uint_0
               OpStore %47 %51
               OpStore %54 %uint_0
         %57 = OpSNegate %uint %7
         %58 = OpSNegate %uint %7
         %59 = OpCompositeConstruct %v2uint %57 %58
               OpStore %60 %59
               OpBranch %61
         %61 = OpLabel
         %66 = OpInBoundsAccessChain %_ptr_Function_uint %60 %uint_0
         %67 = OpLoad %uint %66
         %69 = OpSLessThanEqual %bool %67 %7
               OpBranchConditional %69 %63 %64
         %63 = OpLabel
               OpBranch %70
         %70 = OpLabel
         %75 = OpInBoundsAccessChain %_ptr_Function_uint %60 %uint_1
         %76 = OpLoad %uint %75
         %77 = OpSLessThanEqual %bool %76 %7
               OpBranchConditional %77 %72 %73
         %72 = OpLabel
         %78 = OpLoad %v2uint %43
         %80 = OpLoad %v2uint %60
         %81 = OpIAdd %v2uint %78 %80
               OpStore %82 %81
         %88 = OpInBoundsAccessChain %_ptr_Function_uint %82 %uint_0
         %89 = OpLoad %uint %88
         %90 = OpSLessThanEqual %bool %uint_0 %89
         %92 = OpInBoundsAccessChain %_ptr_Function_uint %82 %uint_0
         %93 = OpLoad %uint %92
         %94 = OpLoad %uint %21
         %95 = OpSLessThan %bool %93 %94
         %96 = OpLogicalAnd %bool %90 %95
         %99 = OpInBoundsAccessChain %_ptr_Function_uint %82 %uint_1
        %100 = OpLoad %uint %99
        %101 = OpSLessThanEqual %bool %uint_0 %100
        %102 = OpLogicalAnd %bool %96 %101
        %104 = OpInBoundsAccessChain %_ptr_Function_uint %82 %uint_1
        %105 = OpLoad %uint %104
        %106 = OpLoad %uint %26
        %107 = OpSLessThan %bool %105 %106
        %108 = OpLogicalAnd %bool %102 %107
               OpBranchConditional %108 %83 %85
         %83 = OpLabel
        %109 = OpLoad %uint %54
        %113 = OpIAdd %uint %109 %uint_1
               OpStore %54 %113
        %114 = OpLoad %v2uint %82
        %115 = OpImageRead %v4float %6 %114
        %118 = OpLoad %v4uint %47
        %120 = OpConvertFToU %v4uint %115
        %123 = OpIAdd %v4uint %118 %120
               OpStore %47 %123
               OpBranch %85
         %85 = OpLabel
               OpBranch %71
         %71 = OpLabel
        %125 = OpInBoundsAccessChain %_ptr_Function_uint %60 %uint_1
        %126 = OpLoad %uint %125
        %127 = OpIAdd %uint %126 %uint_1
               OpStore %125 %127
               OpBranch %70
         %73 = OpLabel
               OpBranch %62
         %62 = OpLabel
        %129 = OpInBoundsAccessChain %_ptr_Function_uint %60 %uint_0
        %130 = OpLoad %uint %129
        %131 = OpIAdd %uint %130 %uint_1
               OpStore %129 %131
               OpBranch %61
         %64 = OpLabel
        %134 = OpLoad %uint %54
        %137 = OpUDiv %uint %134 %uint_2
        %138 = OpLoad %v4uint %47
        %140 = OpCompositeConstruct %v4uint %137 %137 %137 %137
        %143 = OpIAdd %v4uint %138 %140
        %144 = OpLoad %uint %54
        %146 = OpCompositeConstruct %v4uint %144 %144 %144 %144
        %149 = OpUDiv %v4uint %143 %146
        %150 = OpLoad %v2uint %43
               OpImageWrite %8 %150 %149
               OpReturn
               OpFunctionEnd
