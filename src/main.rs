mod phaser;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[command(subcommand)]
        shell: Shell,
    },

    Version,
    Phaser {
        a1: String,
        a2: String,
        a3: String,
        a4: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum Shell {
    Zsh,
    Bash,
    Fish,
}

impl Shell {
    fn init(self) -> String {
        match self {
            Shell::Zsh => ZSH.to_string(),
            Shell::Bash => BASH.to_string(),
            Shell::Fish => FISH.to_string(),
        }
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { shell } => {
            println!("{}", shell.init());
            Ok(())
        }

        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }

        Command::Phaser { a1, a2, a3, a4 } => {
            crate::phaser::print(a1, a2, a3, a4)?;
            Ok(())
        }
    }
}

static ZSH: &str = r#"
setopt PROMPT_SUBST
[[ -n ${ETHR_BG+x} ]] || echo "Please set environment variable ETHR_BG to your terminal background color."
[[ -n ${ETHR_SUCCESS+x} ]] || export ETHR_SUCCESS="467159"
[[ -n ${ETHR_FAIL+x} ]] || export ETHR_FAIL="A04C62"
BG="${ETHR_BG}"
PROMPT_COLOR="${ETHR_SUCCESS}"
_prompt_phaser() {
	ethr phaser "$@" | perl -pe 's/\e\[[0-9;]*m/%{$&%}/g'
}
precmd() {
	local exit_code=$?
	(( exit_code == 0 )) && PROMPT_COLOR="${ETHR_SUCCESS}" || PROMPT_COLOR="${ETHR_FAIL}"
	local pth=$(pwd)
	local sliced="${pth##*/}"
	local len=${#sliced}
	if [[ $pth == $HOME ]]; then
		local len=1
	fi
	if [[ $len -ge 13 ]]; then
		local len=11
	fi
	local prompt_prefix=$'%{\e[48;2;20;25;31m%}'
	PROMPT="${prompt_prefix}%1~ ✨ $(_prompt_phaser "$BG" "$PROMPT_COLOR" $( expr 13 - $len ) ' ')$(_prompt_phaser "$PROMPT_COLOR" "$BG" 1 ' ') "
}
_prompt() {
	local emoji="✅"
	local string="$BASH_COMMAND"
	local cmd=${1%% *}
	case $cmd in
		cd | ls | pwd | mkdir | mv) emoji="📂" ;;
		vi | nano | vim | emacs | hx) emoji="✏️" ;;
		clang | make | 'clang++') emoji="🛠️";;
		python | py | python3) emoji="🐍";;
		brew) emoji="🍺";;
		rm | trash) emoji="🔥";;
		git) emoji="🔶";;
		sudo) emoji="🔑";;
		awk | sed | grep | egrep) emoji="🔍";;
		ftp | sftp | ssh | ping | nc) emoji="🌐";;
		cat | more | less) emoji="📚";;
		touch) emoji="✋";;
		rustc | cargo | rustfmt) emoji="🦀";;
		lldb) emoji="🔧";;
		echo) emoji="📢";;
		bash | sh | ksh | csh | tcsh | zsh | fish) emoji="🐚";;
		*) emoji="✅";;
	esac
	local pth=$(pwd)
	local sliced="${pth##*/}"
	local len=${#sliced}
	if [[ $pth == $HOME ]]; then
		local len=1
	fi
	printf "\e[?25l\e[s\e[1A\e[$(( len + 2 ))G$emoji \e[u\e[?25h"
}
autoload -Uz add-zsh-hook
add-zsh-hook preexec _prompt
"#;

static BASH: &str = r#"
[[ -n ${ETHR_BG+x} ]] || echo "Please set environment variable ETHR_BG to your terminal background color."
[[ -n ${ETHR_SUCCESS+x} ]] || export ETHR_SUCCESS="467159"
[[ -n ${ETHR_FAIL+x} ]] || export ETHR_FAIL="A04C62"
BG="${ETHR_BG}"
PROMPT_COLOR="${ETHR_SUCCESS}"

_prompt_phaser() {
	ethr phaser "$@" | perl -pe 's/\e\[[0-9;]*m/\001$&\002/g'
}

_prompt_command() {
	local exit_code=$?
	_ETHR_IN_PROMPT=1
	(( exit_code == 0 )) && PROMPT_COLOR="${ETHR_SUCCESS}" || PROMPT_COLOR="${ETHR_FAIL}"
	local pth=$(pwd)
	local sliced="${pth##*/}"
	local len=${#sliced}
	if [[ $pth == $HOME ]]; then
		local len=1
	fi
	if [[ $len -ge 13 ]]; then
		local len=11
	fi
	local prompt_prefix='\[\e[48;2;20;25;31m\]'
	PS1="${prompt_prefix}\W ✨ $(_prompt_phaser "$BG" "$PROMPT_COLOR" $(( 13 - len )) ' ')$(_prompt_phaser "$PROMPT_COLOR" "$BG" 1 ' ') "
	_ETHR_AT_PROMPT=1
	_ETHR_IN_PROMPT=0
	return $exit_code
}

_prompt() {
	local command=$1
	local emoji="✅"
	command="${command#"${command%%[![:space:]]*}"}"
	local cmd=${command%%[[:space:]]*}
	case $cmd in
		cd | ls | pwd | mkdir | mv) emoji="📂" ;;
		vi | nano | vim | emacs | hx) emoji="✏️" ;;
		clang | make | 'clang++') emoji="🛠️";;
		python | py | python3) emoji="🐍";;
		brew) emoji="🍺";;
		rm | trash) emoji="🔥";;
		git) emoji="🔶";;
		sudo) emoji="🔑";;
		awk | sed | grep | egrep) emoji="🔍";;
		ftp | sftp | ssh | ping | nc) emoji="🌐";;
		cat | more | less) emoji="📚";;
		touch) emoji="✋";;
		rustc | cargo | rustfmt) emoji="🦀";;
		lldb) emoji="🔧";;
		echo) emoji="📢";;
		bash | sh | ksh | csh | tcsh | zsh | fish) emoji="🐚";;
		*) emoji="✅";;
	esac
	local pth=$(pwd)
	local sliced="${pth##*/}"
	local len=${#sliced}
	if [[ $pth == $HOME ]]; then
		local len=1
	fi
	printf "\e[?25l\e[s\e[1A\e[$(( len + 2 ))G$emoji \e[u\e[?25h"
}

_prompt_preexec() {
	[[ ${_ETHR_IN_PROMPT:-0} == 1 ]] && return
	[[ ${_ETHR_AT_PROMPT:-0} == 1 ]] || return
	local command=$BASH_COMMAND
	[[ -n ${command//[[:space:]]/} ]] || return
	case $command in
		_prompt_command | _prompt_preexec | _prompt_preexec\ * | _prompt | _prompt\ *) return ;;
	esac
	_ETHR_AT_PROMPT=0
	_prompt "$command"
}

PROMPT_COMMAND=_prompt_command
trap _prompt_preexec DEBUG
"#;

static FISH: &str = r#"
set -q ETHR_BG; or echo "Please set environment variable ETHR_BG to your terminal background color."
set -q ETHR_SUCCESS; or set -gx ETHR_SUCCESS "467159"
set -q ETHR_FAIL; or set -gx ETHR_FAIL "A04C62"

function _prompt_phaser
	ethr phaser $argv
end

function fish_prompt
	set -l exit_code $status
	set -l prompt_color $ETHR_SUCCESS
	if test $exit_code -ne 0
		set prompt_color $ETHR_FAIL
	end
	set -l pth (pwd)
	set -l sliced (basename "$pth")
	set -l len (string length -- "$sliced")
	set -l display $sliced
	if test "$pth" = "$HOME"
		set len 1
		set display "~"
	end
	if test $len -ge 13
		set len 11
	end
	printf "\e[48;2;20;25;31m%s ✨ " "$display"
	_prompt_phaser "$ETHR_BG" "$prompt_color" (math 13 - $len) " "
	_prompt_phaser "$prompt_color" "$ETHR_BG" 1 " "
	printf " "
end

function _prompt --on-event fish_preexec
	set -l emoji "✅"
	set -l cmd (string split -m1 " " -- "$argv[1]")[1]
	switch "$cmd"
		case cd ls pwd mkdir mv
			set emoji "📂"
		case vi nano vim emacs hx
			set emoji "✏️"
		case clang make clang++
			set emoji "🛠️"
		case python py python3
			set emoji "🐍"
		case brew
			set emoji "🍺"
		case rm trash
			set emoji "🔥"
		case git
			set emoji "🔶"
		case sudo
			set emoji "🔑"
		case awk sed grep egrep
			set emoji "🔍"
		case ftp sftp ssh ping nc
			set emoji "🌐"
		case cat more less
			set emoji "📚"
		case touch
			set emoji "✋"
		case rustc cargo rustfmt
			set emoji "🦀"
		case lldb
			set emoji "🔧"
		case echo
			set emoji "📢"
		case bash sh ksh csh tcsh zsh fish
			set emoji "🐚"
	end
	set -l pth (pwd)
	set -l sliced (basename "$pth")
	set -l len (string length -- "$sliced")
	if test "$pth" = "$HOME"
		set len 1
	end
	printf "\e[?25l\e[s\e[1A\e[%sG%s \e[u\e[?25h" (math $len + 2) "$emoji"
end
"#;
