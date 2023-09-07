_liquidator_autocomplete() {
  local cur prev opts base
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD - 1]}"

  # Base command options
  opts="help version query store execute instantiate"

  if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]]; then
    COMPREPLY=($(compgen -W "${opts}" -- ${cur}))
    return 0
  fi

  # Handle subcommands
  case "${prev}" in
    query)
      local subopts="config state owner route all_routes"
      COMPREPLY=($(compgen -W "${subopts}" -- ${cur}))
      ;;
    execute)
      local subopts="set_route remove_route config owner liquidate ibc_transfer"
      COMPREPLY=($(compgen -W "${subopts}" -- ${cur}))
      ;;
    *) ;;
  esac

  return 0
}

complete -F _liquidator_autocomplete liquidator
