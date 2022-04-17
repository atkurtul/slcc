; SPIR-V
; Version: 1.0
; Generator: Google Shaderc over Glslang; 10
; Bound: 50
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %in_normal %o_normal %o_albedo %in_pos %in_uv
               OpExecutionMode %main OriginUpperLeft
               OpSource ESSL 320
               OpSourceExtension "GL_GOOGLE_cpp_style_line_directive"
               OpSourceExtension "GL_GOOGLE_include_directive"
               OpName %main "main"
               OpName %normal "normal"
               OpName %in_normal "in_normal"
               OpName %o_normal "o_normal"
               OpName %base_color "base_color"
               OpName %PBRMaterialUniform "PBRMaterialUniform"
               OpMemberName %PBRMaterialUniform 0 "base_color_factor"
               OpMemberName %PBRMaterialUniform 1 "metallic_factor"
               OpMemberName %PBRMaterialUniform 2 "roughness_factor"
               OpName %pbr_material_uniform "pbr_material_uniform"
               OpName %o_albedo "o_albedo"
               OpName %in_pos "in_pos"
               OpName %in_uv "in_uv"
               OpName %GlobalUniform "GlobalUniform"
               OpMemberName %GlobalUniform 0 "model"
               OpMemberName %GlobalUniform 1 "view_proj"
               OpMemberName %GlobalUniform 2 "camera_position"
               OpName %global_uniform "global_uniform"
               OpDecorate %in_normal Location 2
               OpDecorate %o_normal Location 1
               OpMemberDecorate %PBRMaterialUniform 0 Offset 0
               OpMemberDecorate %PBRMaterialUniform 1 Offset 16
               OpMemberDecorate %PBRMaterialUniform 2 Offset 20
               OpDecorate %PBRMaterialUniform Block
               OpDecorate %o_albedo Location 0
               OpDecorate %in_pos Location 0
               OpDecorate %in_uv Location 1
               OpMemberDecorate %GlobalUniform 0 ColMajor
               OpMemberDecorate %GlobalUniform 0 Offset 0
               OpMemberDecorate %GlobalUniform 0 MatrixStride 16
               OpMemberDecorate %GlobalUniform 1 ColMajor
               OpMemberDecorate %GlobalUniform 1 Offset 64
               OpMemberDecorate %GlobalUniform 1 MatrixStride 16
               OpMemberDecorate %GlobalUniform 2 Offset 128
               OpDecorate %GlobalUniform Block
               OpDecorate %global_uniform DescriptorSet 0
               OpDecorate %global_uniform Binding 1
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
%_ptr_Input_v3float = OpTypePointer Input %v3float
  %in_normal = OpVariable %_ptr_Input_v3float Input
    %v4float = OpTypeVector %float 4
%_ptr_Output_v4float = OpTypePointer Output %v4float
   %o_normal = OpVariable %_ptr_Output_v4float Output
  %float_0_5 = OpConstant %float 0.5
    %float_1 = OpConstant %float 1
%_ptr_Function_v4float = OpTypePointer Function %v4float
    %float_0 = OpConstant %float 0
         %30 = OpConstantComposite %v4float %float_1 %float_0 %float_0 %float_1
%PBRMaterialUniform = OpTypeStruct %v4float %float %float
%_ptr_PushConstant_PBRMaterialUniform = OpTypePointer PushConstant %PBRMaterialUniform
%pbr_material_uniform = OpVariable %_ptr_PushConstant_PBRMaterialUniform PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_v4float = OpTypePointer PushConstant %v4float
   %o_albedo = OpVariable %_ptr_Output_v4float Output
%_ptr_Input_v4float = OpTypePointer Input %v4float
     %in_pos = OpVariable %_ptr_Input_v4float Input
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
      %in_uv = OpVariable %_ptr_Input_v2float Input
%mat4v4float = OpTypeMatrix %v4float 4
%GlobalUniform = OpTypeStruct %mat4v4float %mat4v4float %v3float
%_ptr_Uniform_GlobalUniform = OpTypePointer Uniform %GlobalUniform
%global_uniform = OpVariable %_ptr_Uniform_GlobalUniform Uniform
       %main = OpFunction %void None %3
          %5 = OpLabel
     %normal = OpVariable %_ptr_Function_v3float Function
 %base_color = OpVariable %_ptr_Function_v4float Function
         %12 = OpLoad %v3float %in_normal
         %13 = OpExtInst %v3float %1 Normalize %12
               OpStore %normal %13
         %18 = OpLoad %v3float %normal
         %19 = OpVectorTimesScalar %v3float %18 %float_0_5
         %20 = OpCompositeConstruct %v3float %float_0_5 %float_0_5 %float_0_5
         %21 = OpFAdd %v3float %19 %20
         %23 = OpCompositeExtract %float %21 0
         %24 = OpCompositeExtract %float %21 1
         %25 = OpCompositeExtract %float %21 2
         %26 = OpCompositeConstruct %v4float %23 %24 %25 %float_1
               OpStore %o_normal %26
               OpStore %base_color %30
         %37 = OpAccessChain %_ptr_PushConstant_v4float %pbr_material_uniform %int_0
         %38 = OpLoad %v4float %37
               OpStore %base_color %38
         %40 = OpLoad %v4float %base_color
               OpStore %o_albedo %40
               OpReturn
               OpFunctionEnd
