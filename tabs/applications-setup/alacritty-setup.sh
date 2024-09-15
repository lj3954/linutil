#!/bin/sh -e

. ../common-script.sh

setupAlacritty() {
    echo "Install Alacritty if not already installed..."
    if ! command_exists alacritty; then
        case ${PACKAGER} in
            pacman)
                $ESCALATION_TOOL ${PACKAGER} -S --needed --noconfirm alacritty
                ;;
            *)
                $ESCALATION_TOOL ${PACKAGER} install -y alacritty
                ;;
        esac
    else
        echo "alacritty is already installed."
    fi
    echo "Copy alacritty config files"
    if [ -d "${HOME}/.config/alacritty" ]; then
        cp -r "${HOME}/.config/alacritty" "${HOME}/.config/alacritty-bak"
    fi
    mkdir -p "${HOME}/.config/alacritty/"
    wget -O "${HOME}/.config/alacritty/alacritty.toml" "https://github.com/ChrisTitusTech/dwm-titus/raw/main/config/alacritty/alacritty.toml"
    wget -O "${HOME}/.config/alacritty/nordic.toml" "https://github.com/ChrisTitusTech/dwm-titus/raw/main/config/alacritty/nordic.toml"
}

revertAlacritty() {
    echo "Reverting Alacritty setup..."
    if [ -d "${HOME}/.config/alacritty-bak" ]; then
        rm -rf "${HOME}/.config/alacritty"
        mv "${HOME}/.config/alacritty-bak" "${HOME}/.config/alacritty"
        echo "Alacritty setup reverted"

        if command_exists alacritty; then
            printf "Do you want to uninstall Alacritty as well? (y/N): "
            read uninstall_choice
            if [ "$uninstall_choice" = "y" ] || [ "$uninstall_choice" = "Y" ]; then
                case ${PACKAGER} in
                    pacman)
                        $ESCALATION_TOOL ${PACKAGER} -R --noconfirm alacritty
                        ;;
                    *)
                        $ESCALATION_TOOL ${PACKAGER} remove -y alacritty
                        ;;
                esac
                echo "Alacritty uninstalled."
            fi
        fi
    else
        echo "No Alacritty configuration found. Nothing to revert."
    fi
}

run() {
    checkEnv
    checkEscalationTool
    setupAlacritty
}

revert() {
    checkEnv
    checkEscalationTool
    revertAlacritty
}