#!/usr/bin/env bash

install_script() {
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local script_path="$script_dir/algotest.sh"
    local bin_dir="$HOME/.local/bin"
    local link_name="algotest"

    # 如果 ~/.local/bin 目录不在 PATH 变量中
    # bash/zsh: export PATH="$HOME/.local/bin:$PATH"
    # fish: fish_add_path ~/.local/bin # 可自动处理重复和永久化
    mkdir -p $bin_dir
    echo "link: $script_path -> ${bin_dir}/${link_name}"
    ln -sf $script_path ${bin_dir}/${link_name}
    chmod +x ${bin_dir}/${link_name}

    echo "copy: $script_dir/completions/algotest.fish -> $HOME/.config/fish/completions/algotest.fish"
    cp $script_dir/completions/algotest.fish $HOME/.config/fish/completions/algotest.fish
}
uninstall_script() {
    local bin_dir="$HOME/.local/bin"
    local link_name="algotest"

    echo "remove: ${bin_dir}/${link_name##*/}"
    rm ${bin_dir}/${link_name##*/}

    echo "$HOME/.config/fish/completions/algotest.fish"
    rm $HOME/.config/fish/completions/algotest.fish
}

case $1 in
install) install_script ;;
uninstall) uninstall_script ;;
esac
