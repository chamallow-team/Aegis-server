# CSP example
This directory contains an example of a [csp parser](./csp.lua) written in lua.

It currently use [luvit](https://luvit.io) as a lua runtime. *To adapt it to other runtimes, modify the bit library to use bit3é (lua <= 5.2) or bitwise operators (lua > 5.2)* (search for "---FIXME")

```lua
local csp = require"./csp"
local bind = require"utils".bind
local client = require"net".connect("8080", "127.0.0.1")

client:once("connect", function()
  local packet = csp.Packet("connect", {version = "0.0.1", csp = "1.0"}, nil, bind(client.write, client))
  packet:send()
end)

client:on("data", function(chunk)
  
  while #chunk > 0 do
    local packet, err = csp.parse(chunk)
    if err then 
      csp.Packet("disconnect", {}, "corrupted packet", bind(client.write, client)):send()
      client:shutdown()
      break
    end

    log_csp(packet.buffer, packet)
    chunk = err
  end
  
end)
```

More examples may come in the future..