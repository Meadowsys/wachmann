# wachmann

[Galacon] [Discord server] moderation bot (not in production at the moment)

## download

Why would you?

Automated compiled builds are available on [Github Actions].

## supported platforms

- developed on macOS Monterey aarch64
- developed for debian-based distros (Ubuntu)
- likely works on macOS, Linux, or any Unix operating system
- not Windows (written using Unix specific features, like Unix sockets)

## dependencies

- [rust] (v1.60.0) (development/build)
- [node.js] (v16.14.2) (development/build/**production**)
- [pnpm] (v6.32.11) (development/build)
- [arangodb] (v3.9.1) (development/**production**)

## building for development

- start arangodb (if you didn't touch anything on install, it should be running as a system service so nothing needs to be done. if you did something else you would probably know what you need to do)
- `pnpm run dev` starts a file watcher to compile the typescript code on change
- `cargo r` builds and runs the rust code, which runs the typescript code and connects to it

## building for production

- `pnpm run build` builds the typescript database connector code
- `cargo b --release` builds the rust code
- outputs: `target/release/wachmann` (the typescript code is embedded in this binary), only this single file is required in production

## required env variables

set them however you would like. using `.env` files is supported

- `TOKEN`: discord bot token, ex: `eeeeeeeeeeeeeeeeeeeeeeee.eeeeee.eeeeeeeeeeeeeeeeeeeeeeeeeee`
- `ARANGO_URL`: url to connect to the arangodb url, ex: `http://127.0.0.1:8529`
- `ARANGO_USER`: arangodb user to authenticate as, ex: `wachmann-dev` (you can create this user using arangodb [web interface])
- `ARANGO_PASSWORD`: password of the arangodb user, ex: `very-secure-development-password-that-i-totally-didn't-just-commit-into-the-readme-of-this-repo`
- `ARANGO_DATABASE`: database to use, ex: `wachmann-dev-db` (you can create this database using arangodb [web interface], make sure to set your user as the owner of the database)

## running wachmann

note: you may need to run `chmod +x wachmann` the first time if the files are not marked as executable

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
