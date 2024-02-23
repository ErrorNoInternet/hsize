#compdef hsize

autoload -U is-at-least

_hsize() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-p+[Number of decimal places to display in the converted number]:PRECISION: ' \
'--precision=[Number of decimal places to display in the converted number]:PRECISION: ' \
'-f+[Size scale of the given numbers]:SCALE:(b k m g t p e z y)' \
'--from-scale=[Size scale of the given numbers]:SCALE:(b k m g t p e z y)' \
'-t+[Size scale of the converted numbers]:SCALE:(b k m g t p e z y)' \
'--to-scale=[Size scale of the converted numbers]:SCALE:(b k m g t p e z y)' \
'-s+[Character(s) to put between the number and unit]:SEPARATOR: ' \
'--separator=[Character(s) to put between the number and unit]:SEPARATOR: ' \
'-B[Given numbers are powers of 2 (1K = 1024)]' \
'--from-binary[Given numbers are powers of 2 (1K = 1024)]' \
'-b[Converted numbers should be powers of 2 (1K = 1024)]' \
'--to-binary[Converted numbers should be powers of 2 (1K = 1024)]' \
'-e[Displayed numbers should be in scientific notation]' \
'--scientific-notation[Displayed numbers should be in scientific notation]' \
'-n[Remove the '\''B'\'' at the end of the unit (MB -> M)]' \
'--no-b-suffix[Remove the '\''B'\'' at the end of the unit (MB -> M)]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'::sizes:' \
":: :_hsize_commands" \
"*::: :->hsize" \
&& ret=0
    case $state in
    (hsize)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:hsize-command-$line[2]:"
        case $line[2] in
            (replace)
_arguments "${_arguments_options[@]}" \
'-r+[Regex to use for matching numbers]:REGEX: ' \
'--regex=[Regex to use for matching numbers]:REGEX: ' \
'-U[Enable multi-line regex searching]' \
'--multi-line[Enable multi-line regex searching]' \
'-i[Modify (search and replace) files in-place]' \
'--in-place[Modify (search and replace) files in-place]' \
'-h[Print help]' \
'--help[Print help]' \
'*::files:_files' \
&& ret=0
;;
(generate)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
":: :_hsize__generate_commands" \
"*::: :->generate" \
&& ret=0

    case $state in
    (generate)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:hsize-generate-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" \
'-s+[Output completion files for the given shell]:SHELL:(bash elvish fish powershell zsh)' \
'--shell=[Output completion files for the given shell]:SHELL:(bash elvish fish powershell zsh)' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(manpages)
_arguments "${_arguments_options[@]}" \
'-o+[Directory to save generated manpages]:OUTPUT_DIRECTORY:_files -/' \
'--output-directory=[Directory to save generated manpages]:OUTPUT_DIRECTORY:_files -/' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_hsize__generate__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:hsize-generate-help-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(manpages)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_hsize__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:hsize-help-command-$line[1]:"
        case $line[1] in
            (replace)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(generate)
_arguments "${_arguments_options[@]}" \
":: :_hsize__help__generate_commands" \
"*::: :->generate" \
&& ret=0

    case $state in
    (generate)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:hsize-help-generate-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(manpages)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_hsize_commands] )) ||
_hsize_commands() {
    local commands; commands=(
'replace:Use regex to search and replace numbers' \
'r:Use regex to search and replace numbers' \
're:Use regex to search and replace numbers' \
'generate:Generate various shell command files' \
'g:Generate various shell command files' \
'gen:Generate various shell command files' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'hsize commands' commands "$@"
}
(( $+functions[_hsize__generate__completions_commands] )) ||
_hsize__generate__completions_commands() {
    local commands; commands=()
    _describe -t commands 'hsize generate completions commands' commands "$@"
}
(( $+functions[_hsize__generate__help__completions_commands] )) ||
_hsize__generate__help__completions_commands() {
    local commands; commands=()
    _describe -t commands 'hsize generate help completions commands' commands "$@"
}
(( $+functions[_hsize__help__generate__completions_commands] )) ||
_hsize__help__generate__completions_commands() {
    local commands; commands=()
    _describe -t commands 'hsize help generate completions commands' commands "$@"
}
(( $+functions[_hsize__generate_commands] )) ||
_hsize__generate_commands() {
    local commands; commands=(
'completions:Shell completions' \
'c:Shell completions' \
'comp:Shell completions' \
'manpages:Roff manpages' \
'm:Roff manpages' \
'man:Roff manpages' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'hsize generate commands' commands "$@"
}
(( $+functions[_hsize__help__generate_commands] )) ||
_hsize__help__generate_commands() {
    local commands; commands=(
'completions:Shell completions' \
'manpages:Roff manpages' \
    )
    _describe -t commands 'hsize help generate commands' commands "$@"
}
(( $+functions[_hsize__generate__help_commands] )) ||
_hsize__generate__help_commands() {
    local commands; commands=(
'completions:Shell completions' \
'manpages:Roff manpages' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'hsize generate help commands' commands "$@"
}
(( $+functions[_hsize__generate__help__help_commands] )) ||
_hsize__generate__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'hsize generate help help commands' commands "$@"
}
(( $+functions[_hsize__help_commands] )) ||
_hsize__help_commands() {
    local commands; commands=(
'replace:Use regex to search and replace numbers' \
'generate:Generate various shell command files' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'hsize help commands' commands "$@"
}
(( $+functions[_hsize__help__help_commands] )) ||
_hsize__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'hsize help help commands' commands "$@"
}
(( $+functions[_hsize__generate__help__manpages_commands] )) ||
_hsize__generate__help__manpages_commands() {
    local commands; commands=()
    _describe -t commands 'hsize generate help manpages commands' commands "$@"
}
(( $+functions[_hsize__generate__manpages_commands] )) ||
_hsize__generate__manpages_commands() {
    local commands; commands=()
    _describe -t commands 'hsize generate manpages commands' commands "$@"
}
(( $+functions[_hsize__help__generate__manpages_commands] )) ||
_hsize__help__generate__manpages_commands() {
    local commands; commands=()
    _describe -t commands 'hsize help generate manpages commands' commands "$@"
}
(( $+functions[_hsize__help__replace_commands] )) ||
_hsize__help__replace_commands() {
    local commands; commands=()
    _describe -t commands 'hsize help replace commands' commands "$@"
}
(( $+functions[_hsize__replace_commands] )) ||
_hsize__replace_commands() {
    local commands; commands=()
    _describe -t commands 'hsize replace commands' commands "$@"
}

if [ "$funcstack[1]" = "_hsize" ]; then
    _hsize "$@"
else
    compdef _hsize hsize
fi
