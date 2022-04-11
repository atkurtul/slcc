
args="--cfg-cleanup --simplify-instructions --strip-debug --strip-nonsemantic --ssa-rewrite --redundancy-elimination"

cargo run -- --file lang.sl --output lang.spv; 
spirv-dis.exe ./lang.spv > lang.s
spirv-val lang.spv

spirv-opt lang.spv -Os -O -o lang.opt.spv $ags
spirv-dis lang.opt.spv  --no-header --comment > lang.opt.s

cp lang.spv      ~/mediaz/CMakeBuild/bin/Shaders/lang.spv
cp lang.opt.spv  ~/mediaz/CMakeBuild/bin/Shaders/lang.opt.spv

rm lang.spv
rm lang.opt.spv
