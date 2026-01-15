# rrr

`rrr` the Remote Repl Runner. A client and a server, communicating via unix
sockets to help you manage and interact with you shells. 

Or to be more broad, rrr actually permits you to create and communicate with
a managed process.

rrr was made in sort a way it should be quick and fast to interact with it
using pipes, short commands and even from your editors.

e.g to evaluate the type of the selected text in helix, and replace it with
the type name:
`|rrr <shell> t`

## Installation

Just build with `cargo build`, and place the executable in $PATH, don't forget
that to use the default launchers, the server should be launched from the project
root directory, you could also install with the `./install` script, which
copies the `rrr` to `/usr/bin`, copies launchers to `/usr/share/rrr/launchers`
 and creates an `rrr-server` script in `/usr/bin` to start the server.

## rrr sever

The server is a simple socket server listening at `/tmp/rrr.sock`, it manages
shell processes and provides an interface to communicate with them.

You need only one server instance running, to start the server, run:
```bash
rrr
```
And that's it.

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
```

### Sending messages to the repl

```bash
rrr <replname> [cmd]
# e.g
echo "c = 5" | rrr jl-shell
echo "c"     | rrr jl-shell t
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


