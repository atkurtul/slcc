; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 24
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %4 "main" %9
               OpExecutionMode %4 OriginUpperLeft
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "outFragColor"
               OpDecorate %9 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeVector %6 3
         %11 = OpConstant %6 1
         %12 = OpConstantComposite %10 %11 %11 %11
         %13 = OpTypeInt 32 0
         %14 = OpConstant %13 0
         %15 = OpTypePointer Output %6
         %18 = OpConstant %13 1
         %21 = OpConstant %13 2
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %16 = OpAccessChain %15 %9 %14
         %17 = OpCompositeExtract %6 %12 0
               OpStore %16 %17
         %19 = OpAccessChain %15 %9 %18
         %20 = OpCompositeExtract %6 %12 1
               OpStore %19 %20
         %22 = OpAccessChain %15 %9 %21
         %23 = OpCompositeExtract %6 %12 2
               OpStore %22 %23
               OpReturn
               OpFunctionEnd
