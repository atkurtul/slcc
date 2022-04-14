; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 20
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %9 %17
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outFragColor"
               OpName %13 "samplerColor"
               OpName %17 "inUV"
               OpDecorate %9 Location 0
               OpDecorate %13 DescriptorSet 0
               OpDecorate %13 Binding 1
               OpDecorate %17 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeImage %6 2D 0 0 0 1 Unknown
         %11 = OpTypeSampledImage %10
         %12 = OpTypePointer UniformConstant %11
         %13 = OpVariable %12 UniformConstant
         %15 = OpTypeVector %6 2
         %16 = OpTypePointer Input %15
         %17 = OpVariable %16 Input
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %14 = OpLoad %11 %13
         %18 = OpLoad %15 %17
         %19 = OpImageSampleImplicitLod %7 %14 %18
               OpStore %9 %19
               OpReturn
               OpFunctionEnd
