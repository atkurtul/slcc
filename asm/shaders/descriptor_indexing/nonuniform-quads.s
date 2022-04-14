; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 98
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %12 %22 %56 %84 %96
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %9 "local_offset"
               OpName %12 "gl_VertexIndex"
               OpName %22 "out_uv"
               OpName %25 "cos_phase"
               OpName %26 "Registers"
               OpMemberName %26 0 "phase"
               OpName %28 "registers"
               OpName %34 "sin_phase"
               OpName %55 "instance_x"
               OpName %56 "gl_InstanceIndex"
               OpName %60 "instance_y"
               OpName %63 "instance_offset"
               OpName %82 "gl_PerVertex"
               OpMemberName %82 0 "gl_Position"
               OpMemberName %82 1 "gl_PointSize"
               OpMemberName %82 2 "gl_ClipDistance"
               OpMemberName %82 3 "gl_CullDistance"
               OpName %84 ""
               OpName %96 "out_texture_index"
               OpDecorate %12 BuiltIn VertexIndex
               OpDecorate %22 Location 0
               OpMemberDecorate %26 0 Offset 0
               OpDecorate %26 Block
               OpDecorate %56 BuiltIn InstanceIndex
               OpMemberDecorate %82 0 BuiltIn Position
               OpMemberDecorate %82 1 BuiltIn PointSize
               OpMemberDecorate %82 2 BuiltIn ClipDistance
               OpMemberDecorate %82 3 BuiltIn CullDistance
               OpDecorate %82 Block
               OpDecorate %96 Flat
               OpDecorate %96 Location 1
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeFloat 32
          %7 = OpTypeVector %6 2
          %8 = OpTypePointer Function %7
         %10 = OpTypeInt 32 1
         %11 = OpTypePointer Input %10
         %12 = OpVariable %11 Input
         %14 = OpConstant %10 1
         %21 = OpTypePointer Output %7
         %22 = OpVariable %21 Output
         %24 = OpTypePointer Function %6
         %26 = OpTypeStruct %6
         %27 = OpTypePointer PushConstant %26
         %28 = OpVariable %27 PushConstant
         %29 = OpConstant %10 0
         %30 = OpTypePointer PushConstant %6
         %43 = OpTypeMatrix %7 2
         %44 = OpConstant %6 1
         %45 = OpConstant %6 0
         %50 = OpConstant %6 0.5
         %54 = OpTypePointer Function %10
         %56 = OpVariable %11 Input
         %58 = OpConstant %10 8
         %69 = OpConstant %6 15
         %70 = OpConstant %6 7
         %71 = OpConstantComposite %7 %69 %70
         %73 = OpConstant %6 2.0999999
         %78 = OpTypeVector %6 4
         %79 = OpTypeInt 32 0
         %80 = OpConstant %79 1
         %81 = OpTypeArray %6 %80
         %82 = OpTypeStruct %78 %6 %81 %81
         %83 = OpTypePointer Output %82
         %84 = OpVariable %83 Output
         %85 = OpConstant %6 0.100000001
         %93 = OpTypePointer Output %78
         %95 = OpTypePointer Output %10
         %96 = OpVariable %95 Output
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %25 = OpVariable %24 Function
         %34 = OpVariable %24 Function
         %55 = OpVariable %54 Function
         %60 = OpVariable %54 Function
         %63 = OpVariable %8 Function
         %13 = OpLoad %10 %12
         %15 = OpBitwiseAnd %10 %13 %14
         %16 = OpConvertSToF %6 %15
         %17 = OpLoad %10 %12
         %18 = OpShiftRightArithmetic %10 %17 %14
         %19 = OpConvertSToF %6 %18
         %20 = OpCompositeConstruct %7 %16 %19
               OpStore %9 %20
         %23 = OpLoad %7 %9
               OpStore %22 %23
         %31 = OpAccessChain %30 %28 %29
         %32 = OpLoad %6 %31
         %33 = OpExtInst %6 %1 Cos %32
               OpStore %25 %33
         %35 = OpAccessChain %30 %28 %29
         %36 = OpLoad %6 %35
         %37 = OpExtInst %6 %1 Sin %36
               OpStore %34 %37
         %38 = OpLoad %6 %25
         %39 = OpLoad %6 %34
         %40 = OpFNegate %6 %39
         %41 = OpLoad %6 %34
         %42 = OpLoad %6 %25
         %46 = OpCompositeConstruct %7 %38 %40
         %47 = OpCompositeConstruct %7 %41 %42
         %48 = OpCompositeConstruct %43 %46 %47
         %49 = OpLoad %7 %9
         %51 = OpCompositeConstruct %7 %50 %50
         %52 = OpFSub %7 %49 %51
         %53 = OpMatrixTimesVector %7 %48 %52
               OpStore %9 %53
         %57 = OpLoad %10 %56
         %59 = OpSMod %10 %57 %58
               OpStore %55 %59
         %61 = OpLoad %10 %56
         %62 = OpSDiv %10 %61 %58
               OpStore %60 %62
         %64 = OpLoad %10 %55
         %65 = OpConvertSToF %6 %64
         %66 = OpLoad %10 %60
         %67 = OpConvertSToF %6 %66
         %68 = OpCompositeConstruct %7 %65 %67
         %72 = OpFDiv %7 %68 %71
               OpStore %63 %72
         %74 = OpLoad %7 %63
         %75 = OpCompositeConstruct %7 %50 %50
         %76 = OpFSub %7 %74 %75
         %77 = OpVectorTimesScalar %7 %76 %73
               OpStore %63 %77
         %86 = OpLoad %7 %9
         %87 = OpVectorTimesScalar %7 %86 %85
         %88 = OpLoad %7 %63
         %89 = OpFAdd %7 %87 %88
         %90 = OpCompositeExtract %6 %89 0
         %91 = OpCompositeExtract %6 %89 1
         %92 = OpCompositeConstruct %78 %90 %91 %45 %44
         %94 = OpAccessChain %93 %84 %29
               OpStore %94 %92
         %97 = OpLoad %10 %56
               OpStore %96 %97
               OpReturn
               OpFunctionEnd
