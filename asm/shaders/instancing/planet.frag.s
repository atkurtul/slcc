; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 86
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
               OpName %samplerColorMap "samplerColorMap"
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
               OpDecorate %samplerColorMap DescriptorSet 0
               OpDecorate %samplerColorMap Binding 1
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
         %10 = OpTypeImage %float 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
%_ptr_UniformConstant_11 = OpTypePointer UniformConstant %11
%samplerColorMap = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
    %v3float = OpTypeVector %float 3
%_ptr_Input_v3float = OpTypePointer Input %v3float
    %inColor = OpVariable %_ptr_Input_v3float Input
    %float_1 = OpConstant %float 1
  %float_1_5 = OpConstant %float 1.5
%_ptr_Function_v3float = OpTypePointer Function %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
    %float_4 = OpConstant %float 4
  %float_0_5 = OpConstant %float 0.5
         %66 = OpConstantComposite %v3float %float_0_5 %float_0_5 %float_0_5
       %uint = OpTypeInt 32 0
     %uint_0 = OpConstant %uint 0
%_ptr_Function_float = OpTypePointer Function %float
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
         %14 = OpLoad %11 %samplerColorMap
         %18 = OpLoad %v2float %inUV
         %19 = OpImageSampleImplicitLod %v4float %14 %18
         %23 = OpLoad %v3float %inColor
         %25 = OpCompositeExtract %float %23 0
         %26 = OpCompositeExtract %float %23 1
         %27 = OpCompositeExtract %float %23 2
         %28 = OpCompositeConstruct %v4float %25 %26 %27 %float_1
         %29 = OpFMul %v4float %19 %28
         %31 = OpVectorTimesScalar %v4float %29 %float_1_5
               OpStore %color %31
         %35 = OpLoad %v3float %inNormal
         %36 = OpExtInst %v3float %1 Normalize %35
               OpStore %N %36
         %39 = OpLoad %v3float %inLightVec
         %40 = OpExtInst %v3float %1 Normalize %39
               OpStore %L %40
         %43 = OpLoad %v3float %inViewVec
         %44 = OpExtInst %v3float %1 Normalize %43
               OpStore %V %44
         %46 = OpLoad %v3float %L
         %47 = OpFNegate %v3float %46
         %48 = OpLoad %v3float %N
         %49 = OpExtInst %v3float %1 Reflect %47 %48
               OpStore %R %49
         %51 = OpLoad %v3float %N
         %52 = OpLoad %v3float %L
         %53 = OpDot %float %51 %52
         %55 = OpExtInst %float %1 FMax %53 %float_0
         %56 = OpLoad %v3float %inColor
         %57 = OpVectorTimesScalar %v3float %56 %55
               OpStore %diffuse %57
         %59 = OpLoad %v3float %R
         %60 = OpLoad %v3float %V
         %61 = OpDot %float %59 %60
         %62 = OpExtInst %float %1 FMax %61 %float_0
         %64 = OpExtInst %float %1 Pow %62 %float_4
         %67 = OpVectorTimesScalar %v3float %66 %64
         %71 = OpAccessChain %_ptr_Function_float %color %uint_0
         %72 = OpLoad %float %71
         %73 = OpVectorTimesScalar %v3float %67 %72
               OpStore %specular %73
         %76 = OpLoad %v3float %diffuse
         %77 = OpLoad %v4float %color
         %78 = OpVectorShuffle %v3float %77 %77 0 1 2
         %79 = OpFMul %v3float %76 %78
         %80 = OpLoad %v3float %specular
         %81 = OpFAdd %v3float %79 %80
         %82 = OpCompositeExtract %float %81 0
         %83 = OpCompositeExtract %float %81 1
         %84 = OpCompositeExtract %float %81 2
         %85 = OpCompositeConstruct %v4float %82 %83 %84 %float_1
               OpStore %outFragColor %85
               OpReturn
               OpFunctionEnd
