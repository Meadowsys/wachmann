# wachmann

[Galacon] [Discord server] moderation bot (not in production at the moment)

## download

Why would you?

Automated compiled builds are available on [Github Actions].

## dependencies

- [node.js] (v16.13.2)
- [pnpm] (v6.29.1)
- [arangodb] (v3.8.5-1)

## development/build dependencies

- [rust] (v1.58.1)

## how I setup everything

The kind of useful beginners guide to installing everything needed to develop on this project, tested on macOS Monterey aarch64 (Apple Silicon), written with Debian-based linux distros in mind as well with a 99.99999% chance of working on these distros, as well as a ~0% chance of working on Windows. Oh, it also doesn't cover how to configure the database, because h.

### node

To install [node.js], I prefer to use a node version manager, specifically [nvm.fish], which helps with installing/updating specific versions of node and using multiple versions when I need to. nvm.fish depends on [fisher], both of which depend on [fish shell].

To install fish on macOS, I went to their [website][fish shell], then downloaded and ran the installer (supports both Intel and Apple Silicon Macs).

To install fish on ubuntu (and debian) you can run:

```sh
sudo add-apt-repository ppa:fish-shell/release-3
sudo apt-get update
sudo apt-get install fish
```

I will not cover changing fish to your default shell because that is not needed, and I am not sure if you want that. Run fish by typing:

```sh
fish
```

Exit fish by typing:

```sh
exit
```

Install fisher:

```sh
curl -sSL https://git.io/fisher | source && fisher install jorgebucaran/fisher
```

Install nvm.fish:

```sh
fisher install jorgebucaran/nvm.fish
```

Install and use node:

```sh
nvm install 16.13.2
```

Set this version as default node version, so that it automatically uses this version when you open fish:

```sh
set -U nvm_default_version 16.13.2
```

### pnpm

[pnpm] offers two variants: standalone version (is essentially a single binary that packages pnpm and node together), or an npm package. Since we have node installed we should use the npm package (as it is smaller), and install it like so:

```sh
curl -f https://get.pnpm.io/v6.16.js | node - i -g pnpm@6.29.1
```

### arangodb

[arangodb] is not required if you just wish to compile the program.

I did not install [arangodb] on my Mac. I installed it on another computer running Ubuntu server, and connected to it over the local network. However, arangodb does offer macOS packages, and it is probably easier and nicer to have the database local on your machine.

arangodb also offers a debian package, which you can fetch and install (and remove the installer package afterwards) like so:

```sh
curl -L https://download.arangodb.com/arangodb38/Community/Linux/arangodb3_3.8.5.1-1_amd64.deb > arangodb.deb
sudo dpkg -i arangodb.deb
rm arangodb.deb
```

### rust

[rust] offers rustup, a rust version manager, and the primary and preferred installation method for rust. install:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The latest stable version of rust is also installed by default.

The default options of the rustup installer are perfectly acceptable and are what I use, with one exception: because we are using fish shell, the path modifications likely do nothing, as they affect files that (to the best of my knowledge) fish doesn't read or understand. To do this, I choose (2) to customise the installation, accept the defaults for the first 3 options (press enter 3 times), then typed `n` to decline modifying the path variable. It then asks for confirmation, and choose (1) to go ahead.

To modify the path variable, I needed to append the path to the end of `fish_user_paths`. This is a fish universal variable that is prepended to the end of `PATH` so you don't need to tinker with `PATH` yourself. You can set this by running:

```sh
set -U fish_user_paths $fish_user_paths ~/.cargo/bin
```

You can install and use the specific version of the compiler needed by running:

```sh
rustup update 1.58.1
rustup default 1.58.1
```

## building for development

- start arangodb (if you didn't touch anything on install, it should be running as a system service so nothing needs to be done. if you did something else you would probably know what you need to do)
- `pnpm run dev` starts a file watcher to compile the typescript code on change
- `node target/debug/db-server.mjs` to start the database server connector thing
- `cargo r` builds and runs the rust code, which connects to the typescript code (start that one first)

## building for production

- `pnpm run build` builds the typescript database connector code
- `cargo b --release` builds the rust code
- outputs: `target/release/db-server.mjs` and `target/release/wachmann`, you can move them out all into the same folder next to each other if you wish

## required env variables

you can set them as usual, or alternatively, you can put them in a file called `.env` in the same directory as the programs, and both wachmann and the db will read variables from it

- `TOKEN`: discord bot token, ex: `eeeeeeeeeeeeeeeeeeeeeeee.eeeeee.eeeeeeeeeeeeeeeeeeeeeeeeeee`
- `ARANGO_URL`: url to connect to the arangodb url, ex: `http://127.0.0.1:8529`
- `ARANGO_USER`: arangodb user to authenticate as, ex: `wachmann-dev` (you can create this user using arangodb [web interface])
- `ARANGO_PASSWORD`: password of the arangodb user, ex: `very-secure-development-password-that-i-totally-didn't-just-commit-into-the-readme-of-this-repo`
- `ARANGO_DATABASE`: database to use, ex: `wachmann-dev-db` (you can create this database using arangodb [web interface], make sure to set your user as the owner of the database)

## running wachmann

note: you may need to run `chmod +x wachmann` and `chmod +x db-server.mjs` the first time if the files are not marked as executable

- `./db.mjs` first to start database connector thing
- `./wachmann` to start the bot

[Galacon]: https://www.galacon.eu
[Discord server]: https://discord.gg/galacon

[Github Actions]: https://github.com/autumnblazey/wachmann/actions

[node.js]: https://nodejs.org/
[pnpm]: https://pnpm.io
[arangodb]: https://www.arangodb.com
[rust]: https://www.rust-lang.org

[nvm.fish]: https://github.com/jorgebucaran/nvm.fish
[fisher]: https://github.com/jorgebucaran/fisher
[fish shell]: https://fishshell.com

[web interface]: https://www.arangodb.com/docs/3.8/getting-started-web-interface.html
