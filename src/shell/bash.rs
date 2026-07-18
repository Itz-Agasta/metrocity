pub const SNIPPET: &str = r#"# metrocity - terminal screensaver (bash integration)
# https://github.com/Itz-Agasta/metrocity
#
# Usage (add to ~/.bashrc):
#   export METROCITY_TIMEOUT=120  # seconds of idle before activation (default: 120)
#   export METROCITY_SCENE=cafe   # optional: pin a scene - cafe, city or meadow (default: random)
#   eval "$(metrocity shell-init bash)"

_metrocity_cancel() {
  if [[ -n "${_METROCITY_PID}" ]]; then
    kill "${_METROCITY_PID}" 2>/dev/null
    unset _METROCITY_PID
  fi
}

_metrocity_schedule() {
  local timeout="${METROCITY_TIMEOUT:-120}"
  set +m
  (
    sleep "${timeout}"
    [[ "$(ps -o stat= -p $$)" == *"+"* ]] || exit 0
    # Pin the scene from METROCITY_SCENE if set, otherwise metrocity picks
    # a random scene each launch.
    command -v metrocity >/dev/null 2>&1 || exit 0
    if [[ -n "${METROCITY_SCENE}" ]]; then
      metrocity --scene "${METROCITY_SCENE}"
    else
      metrocity
    fi
  ) &
  _METROCITY_PID=$!
  set -m
  disown "${_METROCITY_PID}" 2>/dev/null
}

_metrocity_prompt() {
  _metrocity_cancel
  _metrocity_schedule
}

# Prepend our hook so it runs before any existing PROMPT_COMMAND.
if [[ -z "${PROMPT_COMMAND}" ]]; then
  PROMPT_COMMAND="_metrocity_prompt"
else
  PROMPT_COMMAND="_metrocity_prompt; ${PROMPT_COMMAND}"
fi
"#;
