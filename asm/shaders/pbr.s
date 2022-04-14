; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 77
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %9 %21 %35 %37 %39 %50 %55
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
               OpName %35 "o_uv"
               OpName %37 "texcoord_0"
               OpName %39 "o_normal"
               OpName %50 "normal"
               OpName %53 "gl_PerVertex"
               OpMemberName %53 0 "gl_Position"
               OpMemberName %53 1 "gl_PointSize"
               OpName %55 ""
               OpName %71 "Light"
               OpMemberName %71 0 "position"
               OpMemberName %71 1 "color"
               OpName %74 "LightsInfo"
               OpMemberName %74 0 "count"
               OpMemberName %74 1 "lights"
               OpName %76 "lights"
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
               OpDecorate %35 Location 1
               OpDecorate %37 Location 1
               OpDecorate %39 Location 2
               OpDecorate %50 Location 2
               OpMemberDecorate %53 0 BuiltIn Position
               OpMemberDecorate %53 1 BuiltIn PointSize
               OpDecorate %53 Block
               OpMemberDecorate %71 0 Offset 0
               OpMemberDecorate %71 1 Offset 16
               OpDecorate %73 ArrayStride 32
               OpMemberDecorate %74 0 Offset 0
               OpMemberDecorate %74 1 Offset 16
               OpDecorate %74 Block
               OpDecorate %76 DescriptorSet 0
               OpDecorate %76 Binding 4
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 3
          %8 = OpTypePointer Output %7
          %9 = OpVariable %8 Output
         %10 = OpTypeVector %6 4
         %11 = OpTypeMatrix %10 4
         %12 = OpTypeStruct %11 %11 %7
         %13 = OpTypePointer Uniform %12
         %14 = OpVariable %13 Uniform
         %15 = OpTypeInt 32 1
         %16 = OpConstant %15 0
         %17 = OpTypePointer Uniform %11
         %20 = OpTypePointer Input %7
         %21 = OpVariable %20 Input
         %23 = OpConstant %6 1
         %33 = OpTypeVector %6 2
         %34 = OpTypePointer Output %33
         %35 = OpVariable %34 Output
         %36 = OpTypePointer Input %33
         %37 = OpVariable %36 Input
         %39 = OpVariable %8 Output
         %42 = OpTypeMatrix %7 3
         %50 = OpVariable %20 Input
         %53 = OpTypeStruct %10 %6
         %54 = OpTypePointer Output %53
         %55 = OpVariable %54 Output
         %56 = OpConstant %15 1
         %68 = OpTypePointer Output %10
         %70 = OpTypeInt 32 0
         %71 = OpTypeStruct %10 %10
         %72 = OpConstant %70 16
         %73 = OpTypeArray %71 %72
         %74 = OpTypeStruct %70 %73
         %75 = OpTypePointer Uniform %74
         %76 = OpVariable %75 Uniform
          %4 = OpFunction %2 None %3
          %5 = OpLabel
         %18 = OpAccessChain %17 %14 %16
         %19 = OpLoad %11 %18
         %22 = OpLoad %7 %21
         %24 = OpCompositeExtract %6 %22 0
         %25 = OpCompositeExtract %6 %22 1
         %26 = OpCompositeExtract %6 %22 2
         %27 = OpCompositeConstruct %10 %24 %25 %26 %23
         %28 = OpMatrixTimesVector %10 %19 %27
         %29 = OpCompositeExtract %6 %28 0
         %30 = OpCompositeExtract %6 %28 1
         %31 = OpCompositeExtract %6 %28 2
         %32 = OpCompositeConstruct %7 %29 %30 %31
               OpStore %9 %32
         %38 = OpLoad %33 %37
               OpStore %35 %38
         %40 = OpAccessChain %17 %14 %16
         %41 = OpLoad %11 %40
         %43 = OpCompositeExtract %10 %41 0
         %44 = OpVectorShuffle %7 %43 %43 0 1 2
         %45 = OpCompositeExtract %10 %41 1
         %46 = OpVectorShuffle %7 %45 %45 0 1 2
         %47 = OpCompositeExtract %10 %41 2
         %48 = OpVectorShuffle %7 %47 %47 0 1 2
         %49 = OpCompositeConstruct %42 %44 %46 %48
         %51 = OpLoad %7 %50
         %52 = OpMatrixTimesVector %7 %49 %51
               OpStore %39 %52
         %57 = OpAccessChain %17 %14 %56
         %58 = OpLoad %11 %57
         %59 = OpAccessChain %17 %14 %16
         %60 = OpLoad %11 %59
         %61 = OpMatrixTimesMatrix %11 %58 %60
         %62 = OpLoad %7 %21
         %63 = OpCompositeExtract %6 %62 0
         %64 = OpCompositeExtract %6 %62 1
         %65 = OpCompositeExtract %6 %62 2
         %66 = OpCompositeConstruct %10 %63 %64 %65 %23
         %67 = OpMatrixTimesVector %10 %61 %66
         %69 = OpAccessChain %68 %55 %16
               OpStore %69 %67
               OpReturn
               OpFunctionEnd
