# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_hsize_global_optspecs
	string join \n p/precision= f/from-scale= B/from-binary t/to-scale= b/to-binary e/scientific-notation s/separator= n/no-b-suffix S/skip-short-numbers h/help V/version
end

function __fish_hsize_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_hsize_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_hsize_using_subcommand
	set -l cmd (__fish_hsize_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c hsize -n "__fish_hsize_needs_command" -s p -l precision -d 'Number of decimal places to display in the converted number' -r
complete -c hsize -n "__fish_hsize_needs_command" -s f -l from-scale -d 'Size scale of the given numbers' -r -f -a "b\t''
k\t''
m\t''
g\t''
t\t''
p\t''
e\t''
z\t''
y\t''
r\t''
q\t''"
complete -c hsize -n "__fish_hsize_needs_command" -s t -l to-scale -d 'Size scale of the converted numbers' -r -f -a "b\t''
k\t''
m\t''
g\t''
t\t''
p\t''
e\t''
z\t''
y\t''
r\t''
q\t''"
complete -c hsize -n "__fish_hsize_needs_command" -s s -l separator -d 'Character(s) to put between the number and unit' -r
complete -c hsize -n "__fish_hsize_needs_command" -s B -l from-binary -d 'Given numbers are powers of 2 (1K = 1024)'
complete -c hsize -n "__fish_hsize_needs_command" -s b -l to-binary -d 'Converted numbers should be powers of 2 (1K = 1024)'
complete -c hsize -n "__fish_hsize_needs_command" -s e -l scientific-notation -d 'Displayed numbers should be in scientific notation'
complete -c hsize -n "__fish_hsize_needs_command" -s n -l no-b-suffix -d 'Remove the \'B\' at the end of the unit (MB -> M)'
complete -c hsize -n "__fish_hsize_needs_command" -s S -l skip-short-numbers -d 'Skip converting numbers if they\'ll end up being longer than the original'
complete -c hsize -n "__fish_hsize_needs_command" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_needs_command" -s V -l version -d 'Print version'
complete -c hsize -n "__fish_hsize_needs_command" -a "replace" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_hsize_needs_command" -a "r" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_hsize_needs_command" -a "re" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_hsize_needs_command" -a "generate" -d 'Generate various shell command files'
complete -c hsize -n "__fish_hsize_needs_command" -a "g" -d 'Generate various shell command files'
complete -c hsize -n "__fish_hsize_needs_command" -a "gen" -d 'Generate various shell command files'
complete -c hsize -n "__fish_hsize_needs_command" -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand replace" -s r -l regex -d 'Regex to use for matching numbers' -r
complete -c hsize -n "__fish_hsize_using_subcommand replace" -s U -l multi-line -d 'Enable multi-line regex searching'
complete -c hsize -n "__fish_hsize_using_subcommand replace" -s L -l left-align -d 'Don\'t align converted sizes to the right'
complete -c hsize -n "__fish_hsize_using_subcommand replace" -s i -l in-place -d 'Modify (search and replace) files in-place'
complete -c hsize -n "__fish_hsize_using_subcommand replace" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand r" -s r -l regex -d 'Regex to use for matching numbers' -r
complete -c hsize -n "__fish_hsize_using_subcommand r" -s U -l multi-line -d 'Enable multi-line regex searching'
complete -c hsize -n "__fish_hsize_using_subcommand r" -s L -l left-align -d 'Don\'t align converted sizes to the right'
complete -c hsize -n "__fish_hsize_using_subcommand r" -s i -l in-place -d 'Modify (search and replace) files in-place'
complete -c hsize -n "__fish_hsize_using_subcommand r" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand re" -s r -l regex -d 'Regex to use for matching numbers' -r
complete -c hsize -n "__fish_hsize_using_subcommand re" -s U -l multi-line -d 'Enable multi-line regex searching'
complete -c hsize -n "__fish_hsize_using_subcommand re" -s L -l left-align -d 'Don\'t align converted sizes to the right'
complete -c hsize -n "__fish_hsize_using_subcommand re" -s i -l in-place -d 'Modify (search and replace) files in-place'
complete -c hsize -n "__fish_hsize_using_subcommand re" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "c" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "comp" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "m" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "man" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from completions" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from completions" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from c" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from c" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from comp" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from comp" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from manpages" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from manpages" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from m" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from m" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from man" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from man" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand generate; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "c" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "comp" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "m" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "man" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand g; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from completions" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from completions" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from c" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from c" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from comp" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from comp" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from manpages" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from manpages" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from m" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from m" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from man" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from man" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand g; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "c" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "comp" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "m" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "man" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and not __fish_seen_subcommand_from completions c comp manpages m man help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from completions" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from completions" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from c" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from c" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from comp" -s s -l shell -d 'Output completion files for the given shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from comp" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from manpages" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from manpages" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from m" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from m" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from man" -s o -l output-directory -d 'Directory to save generated manpages' -r -f -a "(__fish_complete_directories)"
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from man" -s h -l help -d 'Print help'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from help" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from help" -f -a "manpages" -d 'Roff manpages'
complete -c hsize -n "__fish_hsize_using_subcommand gen; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand help; and not __fish_seen_subcommand_from replace generate help" -f -a "replace" -d 'Use regex to search and replace numbers'
complete -c hsize -n "__fish_hsize_using_subcommand help; and not __fish_seen_subcommand_from replace generate help" -f -a "generate" -d 'Generate various shell command files'
complete -c hsize -n "__fish_hsize_using_subcommand help; and not __fish_seen_subcommand_from replace generate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c hsize -n "__fish_hsize_using_subcommand help; and __fish_seen_subcommand_from generate" -f -a "completions" -d 'Shell completions'
complete -c hsize -n "__fish_hsize_using_subcommand help; and __fish_seen_subcommand_from generate" -f -a "manpages" -d 'Roff manpages'
