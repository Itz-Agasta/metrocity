pub const SNIPPET: &str = r#"# metrocity - terminal screensaver (bash integration)
# https://github.com/Itz-Agasta/metrocity

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
    command -v metrocity >/dev/null 2>&1 && metrocity
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
