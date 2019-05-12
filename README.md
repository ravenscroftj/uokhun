# U OK Hun? Status monitoring application

Very lightweight status monitoring app written in rust.

![screenshot](assets/screenshot.png)

## Requirements

Requires rust nigtly build and cargo (use [rustup](https://rustup.rs/)).


## Installation

Clone the git repo

```shell
git clone https://github.com/ravenscroftj/uokhun.git
```

Build the application

```shell
cargo build --release
```

Run the application

```shell
./target/release/uokcli
```

## Configuration

uokhun uses a JSON config file that tells it which HTTP endpoints it should call, what parameters it should use, how regularly and who to notify when something isn't working.

Create a file called config.json

```json
{
    "endpoints":[
        {
            "url": "https://www.google.com/",
            "method": "GET",
            "period": 30
        }
    ]
}
```

This will make uokhun request `https://www.google.com/` every 30 minutes and produce a warning if it goes down.