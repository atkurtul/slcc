; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 59
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %21 %31 %33 %36 %47 %52
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "o_pos"
               OpName %12 "GlobalUniform"
               OpMemberName %12 0 "model"
               OpMemberName %12 1 "view_proj"
               OpMemberName %12 2 "camera_position"
               OpName %14 "global_uniform"
               OpName %21 "position"
               OpName %31 "o_uv"
               OpName %33 "texcoord_0"
               OpName %36 "o_normal"
               OpName %47 "normal"
               OpName %50 "gl_PerVertex"
               OpMemberName %50 0 "gl_Position"
               OpMemberName %50 1 "gl_PointSize"
               OpName %52 ""
               OpDecorate %9 Location 0
               OpMemberDecorate %12 0 ColMajor
               OpMemberDecorate %12 0 Offset 0
               OpMemberDecorate %12 0 MatrixStride 16
               OpMemberDecorate %12 1 ColMajor
               OpMemberDecorate %12 1 Offset 64
               OpMemberDecorate %12 1 MatrixStride 16
               OpMemberDecorate %12 2 Offset 128
               OpDecorate %12 Block
               OpDecorate %14 DescriptorSet 0
               OpDecorate %14 Binding 1
               OpDecorate %21 Location 0
               OpDecorate %31 Location 1
               OpDecorate %33 Location 1
               OpDecorate %36 Location 2
               OpDecorate %47 Location 2
               OpMemberDecorate %50 0 BuiltIn Position
               OpMemberDecorate %50 1 BuiltIn PointSize
               OpDecorate %50 Block
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 4
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeMatrix %7 4
         %11 = OpTypeVector %6 3
         %12 = OpTypeStruct %10 %10 %11
         %13 = OpTypePointer Uniform %12
         %14 = OpVariable %13 Uniform
         %15 = OpTypeInt 32 1
         %16 = OpConstant %15 0
         %17 = OpTypePointer Uniform %10
         %20 = OpTypePointer Input %11
         %21 = OpVariable %20 Input
         %23 = OpConstant %6 1
         %29 = OpTypeVector %6 2
         %30 = OpTypePointer Output %29
         %31 = OpVariable %30 Output
         %32 = OpTypePointer Input %29
         %33 = OpVariable %32 Input
         %35 = OpTypePointer Output %11
         %36 = OpVariable %35 Output
         %39 = OpTypeMatrix %11 3
         %47 = OpVariable %20 Input
         %50 = OpTypeStruct %7 %6
         %51 = OpTypePointer Output %50
         %52 = OpVariable %51 Output
         %53 = OpConstant %15 1
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %18 = OpAccessChain %17 %14 %16
         %19 = OpLoad %10 %18
         %22 = OpLoad %11 %21
         %24 = OpCompositeExtract %6 %22 0
         %25 = OpCompositeExtract %6 %22 1
         %26 = OpCompositeExtract %6 %22 2
         %27 = OpCompositeConstruct %7 %24 %25 %26 %23
         %28 = OpMatrixTimesVector %7 %19 %27
               OpStore %9 %28
         %34 = OpLoad %29 %33
               OpStore %31 %34
         %37 = OpAccessChain %17 %14 %16
         %38 = OpLoad %10 %37
         %40 = OpCompositeExtract %7 %38 0
         %41 = OpVectorShuffle %11 %40 %40 0 1 2
         %42 = OpCompositeExtract %7 %38 1
         %43 = OpVectorShuffle %11 %42 %42 0 1 2
         %44 = OpCompositeExtract %7 %38 2
         %45 = OpVectorShuffle %11 %44 %44 0 1 2
         %46 = OpCompositeConstruct %39 %41 %43 %45
         %48 = OpLoad %11 %47
         %49 = OpMatrixTimesVector %11 %46 %48
               OpStore %36 %49
         %54 = OpAccessChain %17 %14 %53
         %55 = OpLoad %10 %54
         %56 = OpLoad %7 %9
         %57 = OpMatrixTimesVector %7 %55 %56
         %58 = OpAccessChain %8 %52 %16
               OpStore %58 %57
               OpReturn
               OpFunctionEnd
