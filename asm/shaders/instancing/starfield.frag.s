; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 112
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %outFragColor %inUVW
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %hash33_vf3_ "hash33(vf3;"
               OpName %p3 "p3"
               OpName %starField_vf3_ "starField(vf3;"
               OpName %pos "pos"
               OpName %color "color"
               OpName %threshhold "threshhold"
               OpName %rnd "rnd"
               OpName %param "param"
               OpName %starCol "starCol"
               OpName %outFragColor "outFragColor"
               OpName %inUVW "inUVW"
               OpName %param_0 "param"
               OpDecorate %outFragColor Location 0
               OpDecorate %inUVW Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
          %9 = OpTypeFunction %float %_ptr_Function_v3float
         %13 = OpTypeFunction %v3float %_ptr_Function_v3float
%float_443_897003 = OpConstant %float 443.897003
%float_441_423004 = OpConstant %float 441.423004
%float_437_195007 = OpConstant %float 437.195007
         %21 = OpConstantComposite %v3float %float_443_897003 %float_441_423004 %float_437_195007
%float_19_1900005 = OpConstant %float 19.1900005
         %28 = OpConstantComposite %v3float %float_19_1900005 %float_19_1900005 %float_19_1900005
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Function_float = OpTypePointer Function %float
     %uint_1 = OpConstant %uint 1
     %uint_2 = OpConstant %uint 2
    %float_0 = OpConstant %float 0
         %70 = OpConstantComposite %v3float %float_0 %float_0 %float_0
%float_0_99000001 = OpConstant %float 0.99000001
       %bool = OpTypeBool
    %float_1 = OpConstant %float 1
   %float_16 = OpConstant %float 16
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
%_ptr_Input_v3float = OpTypePointer Input %v3float
      %inUVW = OpVariable %_ptr_Input_v3float Input
       %main = OpFunction %void None %3
          %5 = OpLabel
    %param_0 = OpVariable %_ptr_Function_v3float Function
        %106 = OpLoad %v3float %inUVW
               OpStore %param_0 %106
        %107 = OpFunctionCall %v3float %starField_vf3_ %param_0
        %108 = OpCompositeExtract %float %107 0
        %109 = OpCompositeExtract %float %107 1
        %110 = OpCompositeExtract %float %107 2
        %111 = OpCompositeConstruct %v4float %108 %109 %110 %float_1
               OpStore %outFragColor %111
               OpReturn
               OpFunctionEnd
%hash33_vf3_ = OpFunction %float None %9
         %p3 = OpFunctionParameter %_ptr_Function_v3float
         %12 = OpLabel
         %17 = OpLoad %v3float %p3
         %22 = OpFMul %v3float %17 %21
         %23 = OpExtInst %v3float %1 Fract %22
               OpStore %p3 %23
         %24 = OpLoad %v3float %p3
         %25 = OpLoad %v3float %p3
         %26 = OpVectorShuffle %v3float %25 %25 1 0 2
         %29 = OpFAdd %v3float %26 %28
         %30 = OpDot %float %24 %29
         %31 = OpLoad %v3float %p3
         %32 = OpCompositeConstruct %v3float %30 %30 %30
         %33 = OpFAdd %v3float %31 %32
               OpStore %p3 %33
         %37 = OpAccessChain %_ptr_Function_float %p3 %uint_0
         %38 = OpLoad %float %37
         %40 = OpAccessChain %_ptr_Function_float %p3 %uint_1
         %41 = OpLoad %float %40
         %42 = OpFAdd %float %38 %41
         %44 = OpAccessChain %_ptr_Function_float %p3 %uint_2
         %45 = OpLoad %float %44
         %46 = OpFMul %float %42 %45
         %47 = OpAccessChain %_ptr_Function_float %p3 %uint_0
         %48 = OpLoad %float %47
         %49 = OpAccessChain %_ptr_Function_float %p3 %uint_2
         %50 = OpLoad %float %49
         %51 = OpFAdd %float %48 %50
         %52 = OpAccessChain %_ptr_Function_float %p3 %uint_1
         %53 = OpLoad %float %52
         %54 = OpFMul %float %51 %53
         %55 = OpFAdd %float %46 %54
         %56 = OpAccessChain %_ptr_Function_float %p3 %uint_1
         %57 = OpLoad %float %56
         %58 = OpAccessChain %_ptr_Function_float %p3 %uint_2
         %59 = OpLoad %float %58
         %60 = OpFAdd %float %57 %59
         %61 = OpAccessChain %_ptr_Function_float %p3 %uint_0
         %62 = OpLoad %float %61
         %63 = OpFMul %float %60 %62
         %64 = OpFAdd %float %55 %63
         %65 = OpExtInst %float %1 Fract %64
               OpReturnValue %65
               OpFunctionEnd
%starField_vf3_ = OpFunction %v3float None %13
        %pos = OpFunctionParameter %_ptr_Function_v3float
         %16 = OpLabel
      %color = OpVariable %_ptr_Function_v3float Function
 %threshhold = OpVariable %_ptr_Function_float Function
        %rnd = OpVariable %_ptr_Function_float Function
      %param = OpVariable %_ptr_Function_v3float Function
    %starCol = OpVariable %_ptr_Function_float Function
               OpStore %color %70
               OpStore %threshhold %float_0_99000001
         %75 = OpLoad %v3float %pos
               OpStore %param %75
         %76 = OpFunctionCall %float %hash33_vf3_ %param
               OpStore %rnd %76
         %77 = OpLoad %float %rnd
         %78 = OpLoad %float %threshhold
         %80 = OpFOrdGreaterThanEqual %bool %77 %78
               OpSelectionMerge %82 None
               OpBranchConditional %80 %81 %82
         %81 = OpLabel
         %84 = OpLoad %float %rnd
         %85 = OpLoad %float %threshhold
         %86 = OpFSub %float %84 %85
         %88 = OpLoad %float %threshhold
         %89 = OpFSub %float %float_1 %88
         %90 = OpFDiv %float %86 %89
         %92 = OpExtInst %float %1 Pow %90 %float_16
               OpStore %starCol %92
         %93 = OpLoad %float %starCol
         %94 = OpCompositeConstruct %v3float %93 %93 %93
         %95 = OpLoad %v3float %color
         %96 = OpFAdd %v3float %95 %94
               OpStore %color %96
               OpBranch %82
         %82 = OpLabel
         %97 = OpLoad %v3float %color
               OpReturnValue %97
               OpFunctionEnd
