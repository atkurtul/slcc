; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 94
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %inColor %inNormal %inLightVec %inViewVec %outFragColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %color "color"
               OpName %samplerArray "samplerArray"
               OpName %inUV "inUV"
               OpName %inColor "inColor"
               OpName %N "N"
               OpName %inNormal "inNormal"
               OpName %L "L"
               OpName %inLightVec "inLightVec"
               OpName %V "V"
               OpName %inViewVec "inViewVec"
               OpName %R "R"
               OpName %diffuse "diffuse"
               OpName %specular "specular"
               OpName %outFragColor "outFragColor"
               OpDecorate %samplerArray DescriptorSet 0
               OpDecorate %samplerArray Binding 1
               OpDecorate %inUV Location 2
               OpDecorate %inColor Location 1
               OpDecorate %inNormal Location 0
               OpDecorate %inLightVec Location 4
               OpDecorate %inViewVec Location 3
               OpDecorate %outFragColor Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
         %10 = OpTypeImage %float 2D 0 1 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
%samplerArray = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
       %inUV = OpVariable %_ptr_Input_v3float Input
    %inColor = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
%_ptr_Function_v3float = OpTypePointer Function %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
%float_0_100000001 = OpConstant %float 0.100000001
    %float_0 = OpConstant %float 0
       %bool = OpTypeBool
   %float_16 = OpConstant %float 16
 %float_0_75 = OpConstant %float 0.75
         %71 = OpConstantComposite %v3float %float_0_75 %float_0_75 %float_0_75
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Function_float = OpTypePointer Function %float
         %80 = OpConstantComposite %v3float %float_0 %float_0 %float_0
%_ptr_Output_v4float = OpTypePointer Output %v4float
%outFragColor = OpVariable %_ptr_Output_v4float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
      %color = OpVariable %_ptr_Function_v4float Function
          %N = OpVariable %_ptr_Function_v3float Function
          %L = OpVariable %_ptr_Function_v3float Function
          %V = OpVariable %_ptr_Function_v3float Function
          %R = OpVariable %_ptr_Function_v3float Function
    %diffuse = OpVariable %_ptr_Function_v3float Function
   %specular = OpVariable %_ptr_Function_v3float Function
         %61 = OpVariable %_ptr_Function_v3float Function
         %14 = OpLoad %11 %samplerArray
         %18 = OpLoad %v3float %inUV
         %19 = OpImageSampleImplicitLod %v4float %14 %18
         %21 = OpLoad %v3float %inColor
         %23 = OpCompositeExtract %float %21 0
         %24 = OpCompositeExtract %float %21 1
         %25 = OpCompositeExtract %float %21 2
         %26 = OpCompositeConstruct %v4float %23 %24 %25 %float_1
         %27 = OpFMul %v4float %19 %26
               OpStore %color %27
         %31 = OpLoad %v3float %inNormal
         %32 = OpExtInst %v3float %1 Normalize %31
               OpStore %N %32
         %35 = OpLoad %v3float %inLightVec
         %36 = OpExtInst %v3float %1 Normalize %35
               OpStore %L %36
         %39 = OpLoad %v3float %inViewVec
         %40 = OpExtInst %v3float %1 Normalize %39
               OpStore %V %40
         %42 = OpLoad %v3float %L
         %43 = OpFNegate %v3float %42
         %44 = OpLoad %v3float %N
         %45 = OpExtInst %v3float %1 Reflect %43 %44
               OpStore %R %45
         %47 = OpLoad %v3float %N
         %48 = OpLoad %v3float %L
         %49 = OpDot %float %47 %48
         %51 = OpExtInst %float %1 FMax %49 %float_0_100000001
         %52 = OpLoad %v3float %inColor
         %53 = OpVectorTimesScalar %v3float %52 %51
               OpStore %diffuse %53
         %55 = OpLoad %v3float %N
         %56 = OpLoad %v3float %L
         %57 = OpDot %float %55 %56
         %60 = OpFOrdGreaterThan %bool %57 %float_0
               OpSelectionMerge %63 None
               OpBranchConditional %60 %62 %79
         %62 = OpLabel
         %64 = OpLoad %v3float %R
         %65 = OpLoad %v3float %V
         %66 = OpDot %float %64 %65
         %67 = OpExtInst %float %1 FMax %66 %float_0
         %69 = OpExtInst %float %1 Pow %67 %float_16
         %72 = OpVectorTimesScalar %v3float %71 %69
         %76 = OpAccessChain %_ptr_Function_float %color %uint_0
         %77 = OpLoad %float %76
         %78 = OpVectorTimesScalar %v3float %72 %77
               OpStore %61 %78
               OpBranch %63
         %79 = OpLabel
               OpStore %61 %80
               OpBranch %63
         %63 = OpLabel
         %81 = OpLoad %v3float %61
               OpStore %specular %81
         %84 = OpLoad %v3float %diffuse
         %85 = OpLoad %v4float %color
         %86 = OpVectorShuffle %v3float %85 %85 0 1 2
         %87 = OpFMul %v3float %84 %86
         %88 = OpLoad %v3float %specular
         %89 = OpFAdd %v3float %87 %88
         %90 = OpCompositeExtract %float %89 0
         %91 = OpCompositeExtract %float %89 1
         %92 = OpCompositeExtract %float %89 2
         %93 = OpCompositeConstruct %v4float %90 %91 %92 %float_1
               OpStore %outFragColor %93
               OpReturn
               OpFunctionEnd
