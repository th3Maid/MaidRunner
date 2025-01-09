![banner](docs/documentation/media_kit/splited.png)

![banner](docs/documentation/images/lineBar.png)

[![witchcraft-cybersecurity](https://snapcraft.io/witchcraft-cybersecurity/badge.svg)](https://snapcraft.io/witchcraft-cybersecurity)
![witchcraft](https://img.shields.io/github/actions/workflow/status/cosmic-zip/witchcraft/witchcraft.yml)
![GitHub issues](https://img.shields.io/github/issues/cosmic-zip/witchcraft)
![GitHub License](https://img.shields.io/github/license/cosmic-zip/witchcraft)
![GitHub top language](https://img.shields.io/github/languages/top/cosmic-zip/witchcraft)

<p align="center">
  🎉 Your OPSEC companion. Now with tldr-pages integration! 🎉
</p>

---

### WITCHCRAFT

WITCHCRAFT is a toolkit for cybersecurity. It handles forensic research, OSINT, scanning, backups, data copying, and penetration testing for apps and APIs, serving as a flexible platform for diverse security tasks.

### Visit the wiki:

https://cosmic-zip.github.io/wiki/wiki.html

### Instalation

The project initially includes a set of default files. These files
are created using the best possible data analysis techniques, and
their final versions are merged into the main project.

The spellbook package also provide:

-   **Advanced osint search in over 1000 sources**
-   **Default Credentials Database**
-   **IP Lookup**
-   **A Set of Social Media Pages for Evil Twin Attacks**
-   **General Wordlists for Directories and Subdomains**
-   **MAC Address Vendor Database**
-   **Usernames Wordlist**
-   **XSS Wordlist**
-   **And more, UwU!**

-   **Unique Wordlists**:

    -   _Moth_
    -   _Ladybug_

### Install using snap

<p class=center>
  <a href="https://snapcraft.io/witchcraft-cybersecurity">
    <img alt="Get it from the Snap Store" src="https://snapcraft.io/static/images/badges/en/snap-store-black.svg" />
  </a>
</p>

#### After installed add a snap alias:

```console
sudo snap alias witchcraft-cybersecurity.witchcraft witchcraft
```

### Build from source (recommended for parrot/kali linux)

To build the project, follow these instructions:

**Step 1: Clone the Repository**

```bash
git clone https://github.com/cosmic-zip/witchcraft.git --branch=main --depth 1
cd witchcraft
```

**Step 2: Run the Build Script**

Execute the provided build script build.sh:

```bash
chmod +x build-devel.sh
./build-devel.sh
```

The script will prompt you to enter the root password, create a folder called 'release,' and place the built executables inside it. Additionally, it will prompt you to run the commands for downloading the archive files for OSINT and wordlists. You can choose to download one, both, or neither, but keep in mind that the archives are required for ip lookup operations.

**Running Witch_Craft**

Execute the following command to run the witchcraft application:

```bash
witchcraft
```

**Note**: The first argument will always not have a "-" (minus) before it.

#### This script will run like:

```bash
witchcraft nuke.hd --device /dev/sdx1
```

```bash
witchcraft search.meta --keywork "Anonymous"
```

```bash
witchcraft search.ipscore --ip 127.0.0.1
```

### Plugins

The witchcraft project is extensible through static files and Rust code. Moreover, it is possible to extend its functionalities using `db.json`. This file contains a list of small shell scripts, which means you can integrate anything that interacts with the terminal using ARGS (argsv, readargs(), sys.args(), etc).

**Note**: There is a Python script called sort.py inside /var/witchcraft/spellbook/dataset. It is used to create a sorted version of the dataset.

```json
{
    "description": "Securely deletes and overwrites the contents of a device seven times",
    "name": "nuke.hd",
    "command": "shred -vzn 7 @@device"
}
```

### Evilpages

Put your cloned pages in /var/witchcraft/spellbook/evilpages. Use the SingleFile extension or a similar tool to clone webpages:

https://addons.mozilla.org/en-US/firefox/addon/single-file/

### RC File

To record logs of all your interactions, create a file named `.witchrc` in your home folder and add the following line to it:

```txt
path_log_file=~/my_frog.jsonl
```

You can use `~/` to represent your home directory or specify the complete path, such as:

```txt
path_log_file=/path/to/my/file.jsonl
```

### License

<p align="center">
  🎉 This project is licensed under the GNU General Public License v3.0.
</p>

**WITCHCRAFT includes IP2Proxy LITE data available from https://www.ip2location.com/proxy-database**
**WITCHCRAFT includes cinsscore data available from https://www.cinsscore.com**
