mod phaser;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[command(subcommand)]
        shell: Shell
    },

    Version,
    Phaser {a1: String, a2: String, a3: String, a4: String},
}

#[derive(Subcommand, Debug, Clone)]
enum Shell {
    Zsh,
    Bash,
    Fish
}

impl Shell {
    fn init(self) -> String {
        match self {
            Shell::Zsh => ZSH.to_string(),
            _ => "".to_string(),
        }
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init {shell} => {
            match shell {
                Shell::Bash => return Err("Unsupported shell as of current version.".to_string()),
                Shell::Fish => return Err("Unsupported shell as of current version.".to_string()),
                _ => println!("{}", shell.init()),
            }; Ok(())
        },

        Command::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        },

        Command::Phaser {a1, a2, a3, a4} => {
            crate::phaser::print(a1, a2, a3, a4)?;
            Ok(())
        }
    }
}

static ZSH: &str = r#"
setopt PROMPT_SUBST
[[ -v ETHEREAL_BG ]] || echo "Please set environment variable ETHEREAL_BG to your terminal background color."
[[ -v ETHEREAL_SUCCESS ]] || export ETHEREAL_SUCCESS="467159"
[[ -v ETHEREAL_FAIL ]] || export ETHEREAL_FAIL="A04C62"
BG="${ETHEREAL_BG}"
PROMPT_COLOR="${ETHEREAL_SUCCESS}"
_prompt_phaser() {
	ethr phaser "$@" | perl -pe 's/\e\[[0-9;]*m/%{$&%}/g'
}
precmd() {
	local exit_code=$?
	(( exit_code == 0 )) && PROMPT_COLOR="${ETHEREAL_SUCCESS}" || PROMPT_COLOR="${ETHEREAL_FAIL}"
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
		bash | sh | ksh | csh | tcsh | zsh) emoji="🐚";;
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

