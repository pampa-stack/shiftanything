# shellcheck shell=bash disable=SC2154
# Sourced by run.sh inside the isolated QA VM, with a private XDG profile.
mkdir -p "$XDG_CONFIG_HOME/pipewire/pipewire.conf.d"
cat > "$XDG_CONFIG_HOME/pipewire/pipewire.conf.d/90-shift-qa.conf" <<'PW'
context.objects = [
  { factory = adapter
    args = {
      factory.name = support.null-audio-sink
      node.name = shift-qa-output
      node.description = "Shift QA Output"
      media.class = Audio/Sink
      audio.position = [ FL FR ]
    }
  }
  { factory = adapter
    args = {
      factory.name = support.null-audio-sink
      node.name = shift-qa-input
      node.description = "Shift QA Input"
      media.class = Audio/Source/Virtual
      audio.position = [ FL FR ]
    }
  }
]
PW
# Use one named socket and never the guest desktop's default audio daemon.
export PIPEWIRE_CORE=pipewire-shift-$slug-$$
export PIPEWIRE_REMOTE=$PIPEWIRE_CORE
test ! -e "$XDG_RUNTIME_DIR/$PIPEWIRE_REMOTE"
pipewire --version
pipewire > "$out/pipewire.log" 2>&1 &
services+=($!)
for ((attempt=0;attempt<50;attempt++)); do
    if [[ -S "$XDG_RUNTIME_DIR/$PIPEWIRE_REMOTE" ]]; then break; fi
    sleep .2
done
test -S "$XDG_RUNTIME_DIR/$PIPEWIRE_REMOTE"
wireplumber > "$out/wireplumber.log" 2>&1 &
services+=($!)
pw-cli ls Node | tee "$out/virtual-nodes.txt" | awk '
    /node.name = "shift-qa-output"/ {output=1}
    /node.name = "shift-qa-input"/ {input=1}
    END {exit !(input && output)}
'
grep -F 'node.name = "shift-qa-' "$out/virtual-nodes.txt"
