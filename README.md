# rrr

`rrr` the Remote Repl Runner. A client and a server, communicating via tcp
sockets to help you manage and interact with shells, or other kind of repl.

Or to be more broad, rrr actually permits you to create and communicate with managed
a process via pipes.

rrr was made in sort a way it should be simple interact with it
using pipes, short commands and from your editors.

e.g to evaluate the type of the selected text in helix(julia template),
and replace it with the type name:
`|rrr <shell> t`

## Installation

Just build with `cargo build`, and place the executable in $PATH, don't forget
that to use the default launchers, the server should be launched from the project
root directory, you could also install with the `./install` script, which
copies the `rrr` to `/usr/bin`, copies launchers to `/usr/share/rrr/launchers`
 and creates an `rrr-server` script in `/usr/bin` to start the server.
When installing rrr, the install script will ask request elevation and will ask
you for rrr password, if you set a password make sure your client also uses it.

## rrr sever

The server is a simple socket server listening at 0.0.0.0:2967 by default,
though you can configure the port and address, it manages
shell processes and provides an interface to communicate with them.

You need only one server instance running, to start the server, run:
```bash
rrr server
```
You may pass additional information

```bash
rrr p=1234 ip=127.0.0.1 l=/usr/share/rrr/launchers server
```

### security

Both the server and client take the `k=` configuration or `RRR_K` environment variable
which specifies a passcode. The passcode will be used to encrypt the query content
with `simple_encrypt`, and the server will attempt to use it's own passcode
to decrypt the data.

## rrr client

The rrr client provides an interface to communicate with the server.
You can call `rrr help` to show usage.

### Creating a shell template, or launcher

During runtime, rrr looks for launchers in a relative `launchers/` directory,
the repository contains a few launchers for interacting with fish/python/julia
shells.
To create a new launcher, you could just inspire yourself of one of them and
place an executable file in `launchers/`.

The fish runner.
```fish
#!/usr/bin/env fish
while true
    read -l sentinel
    if test "$sentinel" = kill
        exit 0
    end

    read -l runtype

    # Read lines until the sentinel is found again
    set code_to_run ""
    while read -l line
        if test "$line" = "$sentinel"
            break
        end
        if ! test "$code_to_run" = ""
            set code_to_run (echo "$code_to_run")
        end
        set code_to_run "$code_to_run$line"
    end

    if test "$runtype" = r
        eval "$code_to_run"
    else
        echo "fish laauncher does not implement msg: '$runtype'"
    end

    # Print the sentinel to signal completion
    echo "$sentinel"
end
```

### Creating a new repl from template

```bash
rrr +<replname> <templatename>
# e.g
rrr +jl-shell jl
rrr +jl-shell jl ~/projects/julia-project
```

### Sending messages to the repl

```bash
rrr <replname> [cmd]
# e.g
echo "c = 5" | rrr jl-shell
echo "c"     | rrr jl-shell t
rrr jl-shell t -- c = 7
```

The command receives the code from stdin, and takes an optional second argument
default to "r" which is simply an additional hint sent to the launcher, to
request for some thing specific, e.g `t` to get the type of the passed element.


### Shutting down a repl

```bash
rrr -<replname>
# e.g
rrr -jl-shell
```

### listing repls

```bash
rrr ls
```

### Running a repl

If you want to interface your repl with a repl interface you can use:
```bash
rrr %<raplname>
# e.g
rrr %jl
```

This will start a client which will query everytime you hit enter and will
query the shell for the prompt with a `.p` type query. You can use escapes like:

- `\k`: to kill the repl and exit
- `\q`: to quit the repl
- `\s <replid>`: To switch repl
- `!<cmd>`: to run a shell command(use this to create the repl for example).

### Client options

the client offers you the `p` and `ip` options to determine the address
of the server, you can also use it to communicate with another machine.

```bash
rrr p=1234 ip=192.168.1.191 k=passcode +jl jl
```

## Creating aliasses

The aim of rrr is to be able to use it with quick and short commands with your
editor, reason why instead of using a persistent configuration, you can create
aliases for your favorite repls and configurations.

```bash
alias r='rrr'
alias r1 = 'rrr 1'
alias rra = 'rrr k=password121 ip=192.168.1.1'
```
Or surely will you prefer to create scripts which you can place in your path
in case your editor does not pass through your shell:
```bash
#!/usr/bin/env -S /usr/bin/rrr ip=192.168.1.1
```

## Environment variables

You can pass configuration using environment variables too, and that should
be prefered than using aliasses for things like passcode, directly passed
arguments have precedence over environment variables though. It's actually
quite simple, use the parameter's name, preceded by `RRR_`, e.g:
```bash
RRR_IP = 192.168.1.1
RRR_K = My Secret passcode
RRR_P = 12345
RRR_L = /usr/share/rrr/launchers
```
RRR will simply isolate and lower what's after the `RRR_` and pake as though
you passed that as configuration.

## My config

> ~/.config/fish/conf.d/rrr.fish
```fish
set -gx RRR_K  'Something here..., but what?'

alias r rrr
alias r1 `rrr 1`
alias r2 `rrr 2`
alias r3 `rrr 3`
```
