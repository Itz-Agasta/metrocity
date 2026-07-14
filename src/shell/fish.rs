pub const SNIPPET: &str = r#"# metrocity - terminal screensaver (fish integration)
# https://github.com/Itz-Agasta/metrocity

# Cancel any pending activation timer. Runs before every command (fish_preexec)
# so metrocity never fires while you are actually using the shell.
function _metrocity_cancel --on-event fish_preexec
    if set -q _METROCITY_PID
        kill $_METROCITY_PID 2>/dev/null
        set -e _METROCITY_PID
    end
end

# Arm a fresh timer every time the prompt is shown (the shell goes idle).
# fish_preexec cancels it the moment a command runs, so metrocity only
# activates during genuine idle at the prompt.
function _metrocity_schedule --on-event fish_prompt
    _metrocity_cancel
    type -q metrocity; or return

    set -l timeout 120
    if set -q METROCITY_TIMEOUT
        set timeout $METROCITY_TIMEOUT
    end

    # Pick scene from METROCITY_SCENE if set, otherwise metrocity uses its
    # configured default (cafe).
    set -l launch metrocity
    if set -q METROCITY_SCENE; and test -n "$METROCITY_SCENE"
        set launch metrocity --scene $METROCITY_SCENE
    end

    # fish cannot background a function, so run the wait-then-launch in a child
    # fish process. The begin/end block and disown keep job start/end messages
    # off the prompt.
    begin
        fish -c "sleep $timeout; and $launch" &
    end
    set -g _METROCITY_PID $last_pid
    disown $_METROCITY_PID 2>/dev/null
end
"#;
