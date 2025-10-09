# mini-blog

## Create DB

```sh
➜  ~ createdb mini_blog --template=template0 
➜  ~ psql -d mini_blog
psql (14.19 (Homebrew))
Type "help" for help.

mini_blog=# \dt
Did not find any relations.
mini_blog=#
```

## Cross build for Linux

```sh
~/D/C/G/mini-blog (main) [1]> mkdir .cargo
~/D/C/G/mini-blog (main)> touch .cargo/config.toml
~/D/C/G/mini-blog (main)> cargo build --release --target x86_64-unknown-linux-gnu
```