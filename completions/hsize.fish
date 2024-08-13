complete -c hsize -n "__fish_use_subcommand" -s p -l precision -d 'Number of decimal places to display in the converted number' -r
complete -c hsize -n "__fish_use_subcommand" -s f -l from-scale -d 'Size scale of the given numbers' -r -f -a "{b	'',k	'',m	'',g	'',t	'',p	'',e	'',z	'',y	'',r	'',q	''}"
complete -c hsize -n "__fish_use_subcommand" -s t -l to-scale -d 'Size scale of the converted numbers' -r -f -a "{b	'',k	'',m	'',g	'',t	'',p	'',e	'',z	'',y	'',r	'',q	''}"
complete -c hsize -n "__fish_use_subcommand" -s s -l separator -d 'Character(s) to put between the number and unit' -r
complete -c hsize -n "__fish_use_subcommand" -s B -l from-binary -d 'Given numbers are powers of 2 (1K = 1024)'
complete -c hsize -n "__fish_use_subcommand" -s b -l to-binary -d 'Converted numbers should be powers of 2 (1K = 1024)'
complete -c hsize -n "__fish_use_subcommand" -s e -l scientific-notation -d 'Displayed numbers should be in scientific notation'
complete -c hsize -n "__fish_use_subcommand" -s n -l no-b-suffix -d 'Remove the \'B\' at the end of the unit (MB -> M)'
complete -c hsize -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c hsize -n "__fish_use_subcommand" -f -a "replace" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_use_subcommand" -f -a "generate" -d 'Generate various shell command files'
complete -c hsize -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_seen_subcommand_from replace" -s r -l regex -d 'Regex to use for matching numbers' -r
complete -c hsize -n "__fish_seen_subcommand_from replace" -s U -l multi-line -d 'Enable multi-line regex searching'
complete -c hsize -n "__fish_seen_subcommand_from replace" -s L -l left-align -d 'Don\'t align converted sizes to the right'
complete -c hsize -n "__fish_seen_subcommand_from replace" -s i -l in-place -d 'Modify (search and replace) files in-place'
complete -c hsize -n "__fish_seen_subcommand_from replace" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from completions" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "{bash	'',elvish	'',fish	'',powershell	'',zsh	''}"
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from completions" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from manpages" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from manpages" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from replace; and not __fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from help" -f -a "replace" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from replace; and not __fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from help" -f -a "generate" -d 'Generate various shell command files'
complete -c hsize -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from replace; and not __fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from completions; and not __fish_seen_subcommand_from manpages" -f -a "manpages" -d 'Roff manpages'
