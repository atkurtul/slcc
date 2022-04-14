; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 54
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint GLCompute %4 "main" %12
               OpExecutionMode %4 LocalSize 1 1 1
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "index"
               OpName %12 "gl_GlobalInvocationID"
               OpName %20 "position"
               OpName %21 "Particle"
               OpMemberName %21 0 "pos"
               OpMemberName %21 1 "vel"
               OpName %23 "Pos"
               OpMemberName %23 0 "particles"
               OpName %25 ""
               OpName %31 "velocity"
               OpName %36 "UBO"
               OpMemberName %36 0 "deltaT"
               OpMemberName %36 1 "particleCount"
               OpName %38 "ubo"
               OpDecorate %12 BuiltIn GlobalInvocationId
               OpMemberDecorate %21 0 Offset 0
               OpMemberDecorate %21 1 Offset 16
               OpDecorate %22 ArrayStride 32
               OpMemberDecorate %23 0 Offset 0
               OpDecorate %23 BufferBlock
               OpDecorate %25 DescriptorSet 0
               OpDecorate %25 Binding 0
               OpMemberDecorate %36 0 Offset 0
               OpMemberDecorate %36 1 Offset 4
               OpDecorate %36 Block
               OpDecorate %38 DescriptorSet 0
               OpDecorate %38 Binding 1
               OpDecorate %51 SpecId 0
               OpDecorate %53 BuiltIn WorkgroupSize
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 1
          %7 = OpTypePointer Function %6
          %9 = OpTypeInt 32 0
         %10 = OpTypeVector %9 3
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %14 = OpTypeVector %6 3
         %17 = OpTypeFloat 32
         %18 = OpTypeVector %17 4
         %19 = OpTypePointer Function %18
         %21 = OpTypeStruct %18 %18
         %22 = OpTypeRuntimeArray %21
         %23 = OpTypeStruct %22
         %24 = OpTypePointer Uniform %23
         %25 = OpVariable %24 Uniform
         %26 = OpConstant %6 0
         %28 = OpTypePointer Uniform %18
         %33 = OpConstant %6 1
         %36 = OpTypeStruct %17 %6
         %37 = OpTypePointer Uniform %36
         %38 = OpVariable %37 Uniform
         %39 = OpTypePointer Uniform %17
         %42 = OpConstant %17 0.0500000007
         %51 = OpSpecConstant %9 1
         %52 = OpConstant %9 1
         %53 = OpSpecConstantComposite %10 %51 %52 %52
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %8 = OpVariable %7 Function
         %20 = OpVariable %19 Function
         %31 = OpVariable %19 Function
         %13 = OpLoad %10 %12
         %15 = OpBitcast %14 %13
         %16 = OpCompositeExtract %6 %15 0
               OpStore %8 %16
         %27 = OpLoad %6 %8
         %29 = OpAccessChain %28 %25 %26 %27 %26
         %30 = OpLoad %18 %29
               OpStore %20 %30
         %32 = OpLoad %6 %8
         %34 = OpAccessChain %28 %25 %26 %32 %33
         %35 = OpLoad %18 %34
               OpStore %31 %35
         %40 = OpAccessChain %39 %38 %26
         %41 = OpLoad %17 %40
         %43 = OpFMul %17 %41 %42
         %44 = OpLoad %18 %31
         %45 = OpVectorTimesScalar %18 %44 %43
         %46 = OpLoad %18 %20
         %47 = OpFAdd %18 %46 %45
               OpStore %20 %47
         %48 = OpLoad %6 %8
         %49 = OpLoad %18 %20
         %50 = OpAccessChain %28 %25 %26 %48 %26
               OpStore %50 %49
               OpReturn
               OpFunctionEnd
