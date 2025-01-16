#!/bin/bash
set -e
echo && echo "Installer"
sudo cp -r --update=none spellbook/ /var/
sudo chown -R $(whoami):$(whoami) /var/spellbook
sudo apt install -y chromium-browser
echo 'export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/' >> ~/.bash_profile
export WITCH_SPELLS_ROOT_DIR=/var/spellbook/archive/
sudo cp -r witchcraft /bin/witchcraft
