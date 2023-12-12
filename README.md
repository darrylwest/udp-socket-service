# UDP Socket Service


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

### Other REPL Commands include...

* ping -> PONG ; just to ensure everything is working

###### dpw | 2023.12.12

