#!/bin/bash

HAS_SUDO = hash pacman 2>/dev/null

if ! [ HAS_SUDO ]; then
  echo "please install sudo"
  exit 1
fi

IS_ARCH = hash pacman 2>/dev/null
IS_DEB  = hash apt 2>/dev/null
IS_FED  = hash dnf 2>/dev/null
IS_SUS  = hash zypper 2>/dev/null
IS_GEN  = hash portage 2>/dev/null

function install_rustup() {
  hash rustup 2>/dev/null
  if [ $? == 1 ]; then
    echo "rustup is not installed"
    read -r -p "install rustup [Y/N]" input
    case $input in
      [yY][eE][sS]|[yY])
        if [ IS_ARCH ]; then
          sudo pacman -S rustup --noconfirm
        elif [ IS_DEB ]; then
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        elif [ IS_FED ]; then
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        elif [ IS_SUS ]; then
          sudo zypper install rustup
        elif [ IS_GEN ]; then
          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        fi
        ;;
      [nN][oO]|[nN])
        echo "rustup is required tho, exiting..."
        exit 1
        ;;
      *)
        install_rustup
        ;;
    esac
  fi
}

# install rustup if not already done
install_rustup

# rustup targets
echo "adding rust targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu

function install_mingw() {
  read -r -p "do you want to install mingw, which is needed for crosscompilation to windows? [Y/N]" input
  case $input in
    [yY][eE][sS]|[yY])
      if [ IS_ARCH ]; then
        sudo pacman -S mingw-w64 --noconfirm
      elif [ IS_DEB ]; then
        sudo apt install mingw-w64 --noconfirm
      elif [ IS_FED ]; then
        sudo dnf install mingw-w64-tools -y
      elif [ IS_SUS ]; then
        sudo zypper install rustup
      elif [ IS_GEN ]; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      fi
      ;;
    [nN][oO]|[nN])
      echo "rustup is required tho, exiting..."
      exit 1
      ;;
    *)
      install_rustup
      ;;
  esac
}
