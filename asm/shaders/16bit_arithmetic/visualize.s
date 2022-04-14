; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 51
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Vertex %4 "main" %8 %22 %43
               OpSource GLSL 450
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %4 "main"
               OpName %8 "gl_VertexIndex"
               OpName %20 "gl_PerVertex"
               OpMemberName %20 0 "gl_Position"
               OpMemberName %20 1 "gl_PointSize"
               OpMemberName %20 2 "gl_ClipDistance"
               OpMemberName %20 3 "gl_CullDistance"
               OpName %22 ""
               OpName %43 "o_uv"
               OpDecorate %8 BuiltIn VertexIndex
               OpMemberDecorate %20 0 BuiltIn Position
               OpMemberDecorate %20 1 BuiltIn PointSize
               OpMemberDecorate %20 2 BuiltIn ClipDistance
               OpMemberDecorate %20 3 BuiltIn CullDistance
               OpDecorate %20 Block
               OpDecorate %43 Location 0
          %2 = OpTypeVoid
          %3 = OpTypeFunction %2
          %6 = OpTypeInt 32 1
          %7 = OpTypePointer Input %6
          %8 = OpVariable %7 Input
         %10 = OpConstant %6 0
         %11 = OpTypeBool
         %15 = OpTypeFloat 32
         %16 = OpTypeVector %15 4
         %17 = OpTypeInt 32 0
         %18 = OpConstant %17 1
         %19 = OpTypeArray %15 %18
         %20 = OpTypeStruct %16 %15 %19 %19
         %21 = OpTypePointer Output %20
         %22 = OpVariable %21 Output
         %23 = OpConstant %15 -1
         %24 = OpConstant %15 0
         %25 = OpConstant %15 1
         %26 = OpConstantComposite %16 %23 %23 %24 %25
         %27 = OpTypePointer Output %16
         %31 = OpConstant %6 1
         %35 = OpConstant %15 3
         %36 = OpConstantComposite %16 %23 %35 %24 %25
         %39 = OpConstantComposite %16 %35 %23 %24 %25
         %41 = OpTypeVector %15 2
         %42 = OpTypePointer Output %41
         %43 = OpVariable %42 Output
         %47 = OpConstant %15 0.5
          %4 = OpFunction %2 None %3
          %5 = OpLabel
          %9 = OpLoad %6 %8
         %12 = OpIEqual %11 %9 %10
               OpSelectionMerge %14 None
               OpBranchConditional %12 %13 %29
         %13 = OpLabel
         %28 = OpAccessChain %27 %22 %10
               OpStore %28 %26
               OpBranch %14
         %29 = OpLabel
         %30 = OpLoad %6 %8
         %32 = OpIEqual %11 %30 %31
               OpSelectionMerge %34 None
               OpBranchConditional %32 %33 %38
         %33 = OpLabel
         %37 = OpAccessChain %27 %22 %10
               OpStore %37 %36
               OpBranch %34
         %38 = OpLabel
         %40 = OpAccessChain %27 %22 %10
               OpStore %40 %39
               OpBranch %34
         %34 = OpLabel
               OpBranch %14
         %14 = OpLabel
         %44 = OpAccessChain %27 %22 %10
         %45 = OpLoad %16 %44
         %46 = OpVectorShuffle %41 %45 %45 0 1
         %48 = OpVectorTimesScalar %41 %46 %47
         %49 = OpCompositeConstruct %41 %47 %47
         %50 = OpFAdd %41 %48 %49
               OpStore %43 %50
               OpReturn
               OpFunctionEnd
