; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 96
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %12 %22 %56 %85
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
               OpName %61 "instance_y"
               OpName %64 "instance_offset"
               OpName %83 "gl_PerVertex"
               OpMemberName %83 0 "gl_Position"
               OpMemberName %83 1 "gl_PointSize"
               OpMemberName %83 2 "gl_ClipDistance"
               OpMemberName %83 3 "gl_CullDistance"
               OpName %85 ""
               OpDecorate %12 BuiltIn VertexIndex
               OpDecorate %22 Location 0
               OpMemberDecorate %26 0 Offset 0
               OpDecorate %26 Block
               OpDecorate %56 BuiltIn InstanceIndex
               OpMemberDecorate %83 0 BuiltIn Position
               OpMemberDecorate %83 1 BuiltIn PointSize
               OpMemberDecorate %83 2 BuiltIn ClipDistance
               OpMemberDecorate %83 3 BuiltIn CullDistance
               OpDecorate %83 Block
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
         %70 = OpConstant %6 15
         %71 = OpConstant %6 7
         %72 = OpConstantComposite %7 %70 %71
         %74 = OpConstant %6 2.0999999
         %79 = OpTypeVector %6 4
         %80 = OpTypeInt 32 0
         %81 = OpConstant %80 1
         %82 = OpTypeArray %6 %81
         %83 = OpTypeStruct %79 %6 %82 %82
         %84 = OpTypePointer Output %83
         %85 = OpVariable %84 Output
         %86 = OpConstant %6 0.100000001
         %94 = OpTypePointer Output %79
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpVariable %8 Function
         %25 = OpVariable %24 Function
         %34 = OpVariable %24 Function
         %55 = OpVariable %54 Function
         %61 = OpVariable %54 Function
         %64 = OpVariable %8 Function
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
         %60 = OpIAdd %10 %59 %58
               OpStore %55 %60
         %62 = OpLoad %10 %56
         %63 = OpSDiv %10 %62 %58
               OpStore %61 %63
         %65 = OpLoad %10 %55
         %66 = OpConvertSToF %6 %65
         %67 = OpLoad %10 %61
         %68 = OpConvertSToF %6 %67
         %69 = OpCompositeConstruct %7 %66 %68
         %73 = OpFDiv %7 %69 %72
               OpStore %64 %73
         %75 = OpLoad %7 %64
         %76 = OpCompositeConstruct %7 %50 %50
         %77 = OpFSub %7 %75 %76
         %78 = OpVectorTimesScalar %7 %77 %74
               OpStore %64 %78
         %87 = OpLoad %7 %9
         %88 = OpVectorTimesScalar %7 %87 %86
         %89 = OpLoad %7 %64
         %90 = OpFAdd %7 %88 %89
         %91 = OpCompositeExtract %6 %90 0
         %92 = OpCompositeExtract %6 %90 1
         %93 = OpCompositeConstruct %79 %91 %92 %45 %44
         %95 = OpAccessChain %94 %85 %29
               OpStore %95 %93
               OpReturn
               OpFunctionEnd
