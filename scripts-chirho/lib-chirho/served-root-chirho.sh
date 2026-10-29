#!/usr/bin/env bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
#
# A served gate must talk to ITS OWN server. Measured 2026-09-21: a gate started its engine through
# a shell function in the background, so `$!` was the subshell and the engine outlived the run; the
# next run could not bind the port, and - because the readiness loop only asks "does anything
# answer?" - it quietly ran every arm against the PREVIOUS run's server and root. These two
# functions close both halves: refuse a port somebody already holds, and after start require that
# the process WE started is alive and is the one listening.
#
# Sourced by the rehearsal gates. Start the engine by invoking the binary directly with `&`, never
# through a shell function, so `$!` is the engine's own pid.

# require_port_free_chirho <port>: exit 2 when anything is listening on the loopback port.
require_port_free_chirho() {
  local holder_chirho
  holder_chirho="$(lsof -nP -t -iTCP:"$1" -sTCP:LISTEN 2>/dev/null | head -1)"
  if [ -n "$holder_chirho" ]; then
    printf 'refusing: port %s is already held by pid %s (%s); a gate must not talk to a server it did not start\n' \
      "$1" "$holder_chirho" "$(ps -o comm= -p "$holder_chirho" 2>/dev/null | tr -d '\n')" >&2
    exit 2
  fi
}

# own_server_verdict_chirho <pid> <port>: prints "own" when <pid> is alive and is the listener on
# <port>; otherwise "dead", or "foreign:<pid>" when another process holds the port.
own_server_verdict_chirho() {
  local listener_chirho
  kill -0 "$1" 2>/dev/null || { echo dead; return; }
  listener_chirho="$(lsof -nP -t -iTCP:"$2" -sTCP:LISTEN 2>/dev/null | head -1)"
  if [ "$listener_chirho" = "$1" ]; then echo own; else echo "foreign:${listener_chirho:-none}"; fi
}

# local_curl_chirho <curl args...>: a request to a loopback server the gate started, made DIRECTLY and with a
# deadline. -q ignores any .curlrc, --noproxy '*' ignores inherited proxy variables, and a hung server costs at most
# CURL_DEADLINE_CHIRHO seconds (default 30), never the run. -q must stay curl's first argument.
local_curl_chirho() {
  curl -q --noproxy '*' --connect-timeout 5 --max-time "${CURL_DEADLINE_CHIRHO:-30}" "$@"
}

# stop_own_child_chirho <pid> <seconds>: end a process this run started, within a bound, and set STOP_HOW_CHIRHO
# to how it ended: "exited:<rc>" when it was gone or left within <seconds>, "terminated:<rc>" after TERM, or
# "killed:<rc>" when it outlived TERM by 5 s. Only that pid is signalled, never a name or a group. Call it directly,
# not inside $( ): a subshell cannot wait for the gate's own child.
stop_own_child_chirho() {
  local pid_chirho="$1" ticks_chirho=0
  STOP_HOW_CHIRHO=exited
  while kill -0 "$pid_chirho" 2>/dev/null && [ "$ticks_chirho" -lt $(( $2 * 2 )) ]; do sleep 0.5; ticks_chirho=$((ticks_chirho + 1)); done
  if kill -0 "$pid_chirho" 2>/dev/null; then
    STOP_HOW_CHIRHO=terminated; kill -TERM "$pid_chirho" 2>/dev/null; ticks_chirho=0
    while kill -0 "$pid_chirho" 2>/dev/null && [ "$ticks_chirho" -lt 10 ]; do sleep 0.5; ticks_chirho=$((ticks_chirho + 1)); done
    if kill -0 "$pid_chirho" 2>/dev/null; then STOP_HOW_CHIRHO=killed; kill -KILL "$pid_chirho" 2>/dev/null; fi
  fi
  wait "$pid_chirho" 2>/dev/null
  STOP_HOW_CHIRHO="$STOP_HOW_CHIRHO:$?"
}
