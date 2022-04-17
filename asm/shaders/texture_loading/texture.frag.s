; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 78
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %inUV %inLodBias %inNormal %inLightVec %inViewVec %outFragColor
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %color "color"
               OpName %samplerColor "samplerColor"
               OpName %inUV "inUV"
               OpName %inLodBias "inLodBias"
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
               OpDecorate %samplerColor DescriptorSet 0
               OpDecorate %samplerColor Binding 1
               OpDecorate %inUV Location 0
               OpDecorate %inLodBias Location 1
               OpDecorate %inNormal Location 2
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
%samplerColor = OpVariable %_ptr_UniformConstant_11 UniformConstant
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
       %inUV = OpVariable %_ptr_Input_v2float Input
%_ptr_Input_float = OpTypePointer Input %float
  %inLodBias = OpVariable %_ptr_Input_float Input
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_Input_v3float = OpTypePointer Input %v3float
   %inNormal = OpVariable %_ptr_Input_v3float Input
 %inLightVec = OpVariable %_ptr_Input_v3float Input
  %inViewVec = OpVariable %_ptr_Input_v3float Input
    %float_0 = OpConstant %float 0
    %float_1 = OpConstant %float 1
         %50 = OpConstantComposite %v3float %float_1 %float_1 %float_1
%_ptr_Function_float = OpTypePointer Function %float
   %float_16 = OpConstant %float 16
       %uint = OpTypeInt 32 0
     %uint_3 = OpConstant %uint 3
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
   %specular = OpVariable %_ptr_Function_float Function
         %14 = OpLoad %11 %samplerColor
         %18 = OpLoad %v2float %inUV
         %21 = OpLoad %float %inLodBias
         %22 = OpImageSampleImplicitLod %v4float %14 %18 Bias %21
               OpStore %color %22
         %28 = OpLoad %v3float %inNormal
         %29 = OpExtInst %v3float %1 Normalize %28
               OpStore %N %29
         %32 = OpLoad %v3float %inLightVec
         %33 = OpExtInst %v3float %1 Normalize %32
               OpStore %L %33
         %36 = OpLoad %v3float %inViewVec
         %37 = OpExtInst %v3float %1 Normalize %36
               OpStore %V %37
         %39 = OpLoad %v3float %L
         %40 = OpFNegate %v3float %39
         %41 = OpLoad %v3float %N
         %42 = OpExtInst %v3float %1 Reflect %40 %41
               OpStore %R %42
         %44 = OpLoad %v3float %N
         %45 = OpLoad %v3float %L
         %46 = OpDot %float %44 %45
         %48 = OpExtInst %float %1 FMax %46 %float_0
         %51 = OpVectorTimesScalar %v3float %50 %48
               OpStore %diffuse %51
         %54 = OpLoad %v3float %R
         %55 = OpLoad %v3float %V
         %56 = OpDot %float %54 %55
         %57 = OpExtInst %float %1 FMax %56 %float_0
         %59 = OpExtInst %float %1 Pow %57 %float_16
         %62 = OpAccessChain %_ptr_Function_float %color %uint_3
         %63 = OpLoad %float %62
         %64 = OpFMul %float %59 %63
               OpStore %specular %64
         %67 = OpLoad %v3float %diffuse
         %68 = OpLoad %v4float %color
         %69 = OpVectorShuffle %v3float %68 %68 0 1 2
         %70 = OpFMul %v3float %67 %69
         %71 = OpLoad %float %specular
         %72 = OpCompositeConstruct %v3float %71 %71 %71
         %73 = OpFAdd %v3float %70 %72
         %74 = OpCompositeExtract %float %73 0
         %75 = OpCompositeExtract %float %73 1
         %76 = OpCompositeExtract %float %73 2
         %77 = OpCompositeConstruct %v4float %74 %75 %76 %float_1
               OpStore %outFragColor %77
               OpReturn
               OpFunctionEnd
