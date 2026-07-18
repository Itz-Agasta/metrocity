pub const SNIPPET: &str = r#"# metrocity - terminal screensaver (zsh integration)
# https://github.com/Itz-Agasta/metrocity
#
# Usage (add to ~/.zshrc):
#   export METROCITY_TIMEOUT=120  # seconds of idle before activation (default: 120)
#   export METROCITY_SCENE=cafe   # optional: pin a scene - cafe, city or meadow (default: random)
#   eval "$(metrocity shell-init zsh)"

_metrocity_activate() {
  # Guard: only fire when enabled and not already running.
  [[ -z "${METROCITY_ENABLED}" ]] && return
  [[ -n "${METROCITY_RUNNING}" ]] && return

  export METROCITY_RUNNING=1
  # Pin the scene from METROCITY_SCENE if set, otherwise metrocity picks
  # a random scene each launch.
  if command -v metrocity >/dev/null 2>&1; then
    if [[ -n "${METROCITY_SCENE}" ]]; then
      metrocity --scene "${METROCITY_SCENE}"
    else
      metrocity
    fi
  fi
  unset METROCITY_RUNNING
}

# Seconds of inactivity before metrocity activates.
# Override by setting METROCITY_TIMEOUT in your shell config before sourcing this.
TMOUT="${METROCITY_TIMEOUT:-120}"

# TRAPALRM is called by zsh when the TMOUT alarm fires.
# Defining it prevents zsh from exiting on timeout.
TRAPALRM() {
  _metrocity_activate
}

export METROCITY_ENABLED=1
"#;
