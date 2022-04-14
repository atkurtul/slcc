

function traverse(){
  if [[ -d $1 ]]; then
    for f in $1/*; do
      traverse "$f" &
    done
  else
    mkdir -p ./bin/$(dirname $1)
    glslc -c $1 -o ./bin/${1%.*}.spv 2> /dev/null
    if [[ $? -eq 0 ]]; then
      spirv-val ./bin/${1%.*}.spv
      mkdir -p ./asm/$(dirname $1)
      spirv-dis ./bin/${1%.*}.spv --raw-id> ./asm/${1%.*}.s
    fi
  fi
}

traverse "shaders"