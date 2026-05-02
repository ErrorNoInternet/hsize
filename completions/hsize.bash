_hsize() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="hsize"
                ;;
            hsize,g)
                cmd="hsize__subcmd__generate"
                ;;
            hsize,gen)
                cmd="hsize__subcmd__generate"
                ;;
            hsize,generate)
                cmd="hsize__subcmd__generate"
                ;;
            hsize,help)
                cmd="hsize__subcmd__help"
                ;;
            hsize,r)
                cmd="hsize__subcmd__replace"
                ;;
            hsize,re)
                cmd="hsize__subcmd__replace"
                ;;
            hsize,replace)
                cmd="hsize__subcmd__replace"
                ;;
            hsize__subcmd__generate,c)
                cmd="hsize__subcmd__generate__subcmd__completions"
                ;;
            hsize__subcmd__generate,comp)
                cmd="hsize__subcmd__generate__subcmd__completions"
                ;;
            hsize__subcmd__generate,completions)
                cmd="hsize__subcmd__generate__subcmd__completions"
                ;;
            hsize__subcmd__generate,help)
                cmd="hsize__subcmd__generate__subcmd__help"
                ;;
            hsize__subcmd__generate,m)
                cmd="hsize__subcmd__generate__subcmd__manpages"
                ;;
            hsize__subcmd__generate,man)
                cmd="hsize__subcmd__generate__subcmd__manpages"
                ;;
            hsize__subcmd__generate,manpages)
                cmd="hsize__subcmd__generate__subcmd__manpages"
                ;;
            hsize__subcmd__generate__subcmd__help,completions)
                cmd="hsize__subcmd__generate__subcmd__help__subcmd__completions"
                ;;
            hsize__subcmd__generate__subcmd__help,help)
                cmd="hsize__subcmd__generate__subcmd__help__subcmd__help"
                ;;
            hsize__subcmd__generate__subcmd__help,manpages)
                cmd="hsize__subcmd__generate__subcmd__help__subcmd__manpages"
                ;;
            hsize__subcmd__help,generate)
                cmd="hsize__subcmd__help__subcmd__generate"
                ;;
            hsize__subcmd__help,help)
                cmd="hsize__subcmd__help__subcmd__help"
                ;;
            hsize__subcmd__help,replace)
                cmd="hsize__subcmd__help__subcmd__replace"
                ;;
            hsize__subcmd__help__subcmd__generate,completions)
                cmd="hsize__subcmd__help__subcmd__generate__subcmd__completions"
                ;;
            hsize__subcmd__help__subcmd__generate,manpages)
                cmd="hsize__subcmd__help__subcmd__generate__subcmd__manpages"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        hsize)
            opts="-p -f -B -t -b -e -s -n -S -h -V --precision --from-scale --from-binary --to-scale --to-binary --scientific-notation --separator --no-b-suffix --skip-short-numbers --help --version [SIZES]... replace r re generate g gen help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --precision)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --from-scale)
                    COMPREPLY=($(compgen -W "b k m g t p e z y r q" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "b k m g t p e z y r q" -- "${cur}"))
                    return 0
                    ;;
                --to-scale)
                    COMPREPLY=($(compgen -W "b k m g t p e z y r q" -- "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -W "b k m g t p e z y r q" -- "${cur}"))
                    return 0
                    ;;
                --separator)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate)
            opts="-h --help completions c comp manpages m man help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__completions)
            opts="-s -h --shell --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --shell)
                    COMPREPLY=($(compgen -W "bash elvish fish powershell zsh" -- "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -W "bash elvish fish powershell zsh" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__help)
            opts="completions manpages help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__help__subcmd__completions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__help__subcmd__manpages)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__generate__subcmd__manpages)
            opts="-o -h --output-directory --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output-directory)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                -o)
                    COMPREPLY=()
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o plusdirs
                    fi
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help)
            opts="replace generate help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help__subcmd__generate)
            opts="completions manpages"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help__subcmd__generate__subcmd__completions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help__subcmd__generate__subcmd__manpages)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__help__subcmd__replace)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        hsize__subcmd__replace)
            opts="-r -U -L -i -h --regex --multi-line --left-align --in-place --help [FILES]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --regex)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _hsize -o nosort -o bashdefault -o default hsize
else
    complete -F _hsize -o bashdefault -o default hsize
fi
