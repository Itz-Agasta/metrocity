pub const SNIPPET: &str = r#"# metrocity - terminal screensaver (zsh integration)
# https://github.com/Itz-Agasta/metrocity

_metrocity_activate() {
  # Guard: only fire when enabled and not already running.
  [[ -z "${METROCITY_ENABLED}" ]] && return
  [[ -n "${METROCITY_RUNNING}" ]] && return

  export METROCITY_RUNNING=1
  command -v metrocity >/dev/null 2>&1 && metrocity
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
