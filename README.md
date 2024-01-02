# UDP Socket Service

Current Version 0.5.2

```bash
 _______ _____  ______   _______              __           __   
|   |   |     \|   __ \ |     __|.-----.----.|  |--.-----.|  |_ 
|   |   |  --  |    __/ |__     ||  _  |  __||    <|  -__||   _|
|_______|_____/|___|    |_______||_____|____||__|__|_____||____|
                                                                                      
```

_A simple, fast udp socket server with pluggable handler and sample clients._

## REPL

### Tiny-KV Commands

The `udp-socket-server` supports a small number of commands that line up with the default tiny-kv handler.  The cammands are:

* get key -> value
* set key value -> ok
* del key -> ok
* keys -> ["key1", "key2", ... ]
* dbsize -> the number of elements
* loaddb [filename] -> number of elements loaded
* savedb [filename] -> number of elements saved

### Tiny-KV Data Format

Tiny-kv uses `HashMap<String, String>` for backing.  The data format for this is a `.kv` file with a key, then space then any type of string data including more spaces, json, base64, etc.  Here is an example:


```bash
100 my value as a string
101 flarb
102 first_name: john, last_name: smith, email: john.smith@gmail.com
```

### Other REPL Commands include...

* ping -> PONG ; just to ensure everything is working
* now -> the unix timestamp in seconds
* now_ns -> timestamp in nano seconds (works only on linux)
* status -> start time, up-time, error count, etc

###### dpw | 2023.12.12

