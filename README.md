# shellmoji -- Prettify Your Bash Shell With Dynamic Emojis
`shellmoji` is a bash extension script that adds a small but impactful beauty in your prompt. The shell responds to each of your commands with a lively emoji, all while fitting seamlessly into your command line workflow. Append the attached script to your .bashrc file in your home directory to unlock an interactive pleasure you never knew you always wanted.

**Note:** To ensure .bashrc runs only on interactive shell (and not on shell scripts), please add the following snippet to the head of your .bashrc. Most distributions of Linux does this by default. For Mac users or those without a default config file, adding this will be necessary to prevent malfunction.

```
# If not running interactively, don't do anything
case $- in
   *i*) ;;
    *) return;;
esac
```
