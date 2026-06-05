#!/usr/bin/env bash

# Commands to run after the Container is created

mkdir -p /home/vscode/.local/bin

# install latest neovim
[ $(uname -m) = x86_64 ] && curl -L https://github.com/neovim/neovim/releases/latest/download/nvim-linux-x86_64.tar.gz | tar -xz --strip-components=1 -C /home/vscode/.local/
[ $(uname -m) = aarch64 ] && curl -L https://github.com/neovim/neovim/releases/latest/download/nvim-linux-arm64.tar.gz | tar -xz --strip-components=1 -C /home/vscode/.local/

# install fnm a fast node version manager
curl -fsSL https://fnm.vercel.app/install | bash -s -- --install-dir /home/vscode/.local/bin

sh -c "$(curl -fsLS get.chezmoi.io)" -- -b $HOME/.local/bin init --apply developerSid

direnv allow /workspace

cat <<'EOT' >> /home/vscode/./.omzLocalPlugins
plugins+=(rust)
EOT

cat <<'EOT' > /home/vscode/.localConfig
if command -v fnm &> /dev/null; then
   source <(fnm completions 2> /dev/null)
   eval "$(fnm env --use-on-cd --shell zsh)"
fi

eval $(ssh-agent -s) &> /dev/null

if command -v lsd &> /dev/null; then
  alias ls=lsd
  alias ll='lsd -lh'
  alias la='lsd -lah'
  alias tree='lsd --tree'
fi

if command -v bat &> /dev/null; then
  alias cat=bat
fi

if command -v batcat &> /dev/null; then
  alias cat=batcat
fi

export PATH=/home/vscode/.opencode/bin:$PATH
EOT

sudo chown -R vscode:vscode /workspace/.build