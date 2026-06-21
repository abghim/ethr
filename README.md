<h1 align="center">
  Ethr
</h1>

<p align="center">
  <img src="https://img.shields.io/badge/dynamic/json.svg?url=https%3A%2F%2Fapi.github.com%2Frepos%2Fabghim%2Fethr%2Factions%2Fworkflows%2Frust.yml%2Fruns%3Fbranch%3Dmain%26event%3Dpush%26per_page%3D1&query=%24.workflow_runs%5B0%5D.conclusion&label=actions&color=94DFF7&labelColor=1c2228&style=for-the-badge&logo=github">
  <img src="https://img.shields.io/badge/dynamic/json.svg?url=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fethr&query=%24.crate.newest_version&label=crates.io&color=%23F7A967&labelColor=1c2228&style=for-the-badge&logo=rust">
  <img src="https://img.shields.io/badge/license-wtfpl-F7B5C7?labelColor=1c2228&style=for-the-badge&logo=unlicense&logoColor=white">
</p>

<p align="center">
  Ethereal shell prompts for zsh, bash, and fish. Practical meets stunning.
</p>

<img width="1229" height="8" alt="image" src="https://github.com/user-attachments/assets/3bec774d-e55d-4e10-84e3-347dbf08693c" />


## Install
### Download binary
Install via cargo:
```zsh
cargo install ethr
```

### Setup
For zsh (extensively tested):
```zsh
# add to .zshrc
eval "$(ethr init zsh)"
```

For bash:
```bash
eval "$(ethr init bash)"
```

For fish:
```fish
eval "$(ethr init fish)"
```

### Configure (Important)
```
export ETHR_BG="14191f" # terminal bg color, required
export ETHR_SUCCESS="467159" # command success color
export ETHR_FAIL="A04C62" # command success color
```

## Features
### Gradient prompt padding.

### Command status.

### Consistent prompt width.

