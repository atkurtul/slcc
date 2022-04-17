; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 108
; Schema: 0
               OpCapability Shader
               OpCapability ImageQuery
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %outColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %ar "ar"
               OpName %dir "dir"
               OpName %ts "ts"
               OpName %samplerColor1 "samplerColor1"
               OpName %P "P"
               OpName %inUV "inUV"
               OpName %color "color"
               OpName %i "i"
               OpName %dv "dv"
               OpName %indexable "indexable"
               OpName %outColor "outColor"
               OpName %samplerColor0 "samplerColor0"
               OpDecorate %dir SpecId 0
               OpDecorate %samplerColor1 DescriptorSet 0
               OpDecorate %samplerColor1 Binding 1
               OpDecorate %inUV Location 0
               OpDecorate %outColor Location 0
               OpDecorate %samplerColor0 DescriptorSet 0
               OpDecorate %samplerColor0 Binding 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
%_ptr_Function_float = OpTypePointer Function %float
    %float_1 = OpConstant %float 1
        %int = OpTypeInt 32 1
        %dir = OpSpecConstant %int 0
      %int_1 = OpConstant %int 1
       %bool = OpTypeBool
         %14 = OpSpecConstantOp %bool IEqual %dir %int_1
    %v2float = OpTypeVector %float 2
%_ptr_Function_v2float = OpTypePointer Function %v2float
         %20 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %21 = OpTypeSampledImage %20
%_ptr_UniformConstant_21 = OpTypePointer UniformConstant %21
%samplerColor1 = OpVariable %_ptr_UniformConstant_21 UniformConstant
      %int_0 = OpConstant %int 0
      %v2int = OpTypeVector %int 2
       %uint = OpTypeInt 32 0
     %uint_1 = OpConstant %uint 1
     %uint_0 = OpConstant %uint 0
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %float_0 = OpConstant %float 0
   %float_12 = OpConstant %float 12
%float_0_00300000003 = OpConstant %float 0.00300000003
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %54 = OpConstantComposite %v4float %float_0 %float_0 %float_0 %float_0
%_ptr_Function_int = OpTypePointer Function %int
     %int_25 = OpConstant %int 25
    %uint_25 = OpConstant %uint 25
%_arr_float_uint_25 = OpTypeArray %float %uint_25
%float_0_00244992995 = OpConstant %float 0.00244992995
%float_0_00435384549 = OpConstant %float 0.00435384549
%float_0_00735999644 = OpConstant %float 0.00735999644
%float_0_0118349791 = OpConstant %float 0.0118349791
%float_0_0181026701 = OpConstant %float 0.0181026701
%float_0_0263392292 = OpConstant %float 0.0263392292
%float_0_0364543013 = OpConstant %float 0.0364543013
%float_0_0479932055 = OpConstant %float 0.0479932055
%float_0_0601029806 = OpConstant %float 0.0601029806
%float_0_0715974495 = OpConstant %float 0.0715974495
%float_0_0811305419 = OpConstant %float 0.0811305419
%float_0_0874493197 = OpConstant %float 0.0874493197
%float_0_0896631107 = OpConstant %float 0.0896631107
         %92 = OpConstantComposite %_arr_float_uint_25 %float_0_00244992995 %float_0_00435384549 %float_0_00735999644 %float_0_0118349791 %float_0_0181026701 %float_0_0263392292 %float_0_0364543013 %float_0_0479932055 %float_0_0601029806 %float_0_0715974495 %float_0_0811305419 %float_0_0874493197 %float_0_0896631107 %float_0_0874493197 %float_0_0811305419 %float_0_0715974495 %float_0_0601029806 %float_0_0479932055 %float_0_0364543013 %float_0_0263392292 %float_0_0181026701 %float_0_0118349791 %float_0_00735999644 %float_0_00435384549 %float_0_00244992995
%_ptr_Function__arr_float_uint_25 = OpTypePointer Function %_arr_float_uint_25
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %outColor = OpVariable %_ptr_Output_v4float Output
%samplerColor0 = OpVariable %_ptr_UniformConstant_21 UniformConstant
       %main = OpFunction %void None %3
          %5 = OpLabel
         %ar = OpVariable %_ptr_Function_float Function
         %ts = OpVariable %_ptr_Function_v2float Function
          %P = OpVariable %_ptr_Function_v2float Function
      %color = OpVariable %_ptr_Function_v4float Function
          %i = OpVariable %_ptr_Function_int Function
         %dv = OpVariable %_ptr_Function_v2float Function
  %indexable = OpVariable %_ptr_Function__arr_float_uint_25 Function
               OpStore %ar %float_1
               OpSelectionMerge %16 None
               OpBranchConditional %14 %15 %16
         %15 = OpLabel
         %24 = OpLoad %21 %samplerColor1
         %26 = OpImage %20 %24
         %28 = OpImageQuerySizeLod %v2int %26 %int_0
         %29 = OpConvertSToF %v2float %28
               OpStore %ts %29
         %32 = OpAccessChain %_ptr_Function_float %ts %uint_1
         %33 = OpLoad %float %32
         %35 = OpAccessChain %_ptr_Function_float %ts %uint_0
         %36 = OpLoad %float %35
         %37 = OpFDiv %float %33 %36
               OpStore %ar %37
               OpBranch %16
         %16 = OpLabel
         %41 = OpLoad %v2float %inUV
         %42 = OpVectorShuffle %v2float %41 %41 1 0
         %45 = OpLoad %float %ar
         %46 = OpFMul %float %float_12 %45
         %48 = OpFMul %float %46 %float_0_00300000003
         %49 = OpCompositeConstruct %v2float %float_0 %48
         %50 = OpFSub %v2float %42 %49
               OpStore %P %50
               OpStore %color %54
               OpStore %i %int_0
               OpBranch %57
         %57 = OpLabel
               OpLoopMerge %59 %60 None
               OpBranch %61
         %61 = OpLabel
         %62 = OpLoad %int %i
         %64 = OpSLessThan %bool %62 %int_25
               OpBranchConditional %64 %58 %59
         %58 = OpLabel
         %66 = OpLoad %int %i
         %67 = OpConvertSToF %float %66
         %68 = OpFMul %float %67 %float_0_00300000003
         %69 = OpCompositeConstruct %v2float %float_0 %68
         %70 = OpLoad %float %ar
         %71 = OpVectorTimesScalar %v2float %69 %70
               OpStore %dv %71
         %72 = OpLoad %21 %samplerColor1
         %73 = OpLoad %v2float %P
         %74 = OpLoad %v2float %dv
         %75 = OpFAdd %v2float %73 %74
         %76 = OpImageSampleImplicitLod %v4float %72 %75
         %93 = OpLoad %int %i
               OpStore %indexable %92
         %96 = OpAccessChain %_ptr_Function_float %indexable %93
         %97 = OpLoad %float %96
         %98 = OpVectorTimesScalar %v4float %76 %97
         %99 = OpVectorTimesScalar %v4float %98 %float_1
        %100 = OpLoad %v4float %color
        %101 = OpFAdd %v4float %100 %99
               OpStore %color %101
               OpBranch %60
         %60 = OpLabel
        %102 = OpLoad %int %i
        %103 = OpIAdd %int %102 %int_1
               OpStore %i %103
               OpBranch %57
         %59 = OpLabel
        %106 = OpLoad %v4float %color
               OpStore %outColor %106
               OpReturn
               OpFunctionEnd
