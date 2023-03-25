
# Rush

Rush is an esoteric shell that aims to replace reasonable scripting langauges with rust.

In a normal shell you would issue a command that looks like this:
```sh
mkdir -p ~/new_dir/thing
```

In rush it would look like this
```rush
mkdir("~/new_dir/thing").opts("-p")
```

So much better right?

## Concepts

Variable scoping. All variables can only exist at their current run level, similar to how lifetimes work in rust. This means that to declar a variable at a higher scope then a script is at it must request it of its parent.
This arguable could give is some secrity benifates over other shells as you can controll what can controll what. In addition, there might be some multi-threading stuff or some shit that rust enables IDK.

Even though I haven't written this I think its only use would be as a reliable scripting language and not a user shell.


## Goals

The main goal of rush is to be dumb. However secondary there is a goal that by knowing rust you can do everything on a computer and this is the next step in that.
I don't want this to be something that just marginally changes things like nuShell or fish and ends up luke warm. This project is dumb is proud of it.

