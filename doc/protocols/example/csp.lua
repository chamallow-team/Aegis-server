---FIXME luajit2, luvit
local bit = require("bit")
---FIXME lua 5.1, 5.2, luajit
-- local bit = require("bit32")

local stringchar, stringbyte, stringformat = string.char, string.byte, string.format

-- type declaration

local parser_parse, parser_stringify

---@class CspPacket
---@field headers table
---@field method CspMethods
---@field data string|nil
---@field buffer string|nil
---@field err string|nil
---@field _write fun(buffer: string, callback?: function)|nil

---@class CspError
---@field message CspErrorMessage
---@field id CspErrorId
---@field pos number

-- ============== CSP integration ==============


-- errors
---@enum CspErrorId
local err_id = {
  duplicate_header = "DUP_HEADER",
  unknown_header = "UKWN_HEADER",
  unknown_method = "UKWN_METHOD",
  unknown_control = "UKWN_CTRL",
  missing_control = "MISS_CTRL",
  unexpected_control = "UNXPT_CTRL",
  invalid_number = "INV_NUM",
  invalid_data_length = "INV_DATA_LEN",

  unknown = "UNKNOWN",
}

---@enum CspErrorMessage
local err_msg = {
  [err_id.duplicate_header] = "Duplicated header: %s.",
  [err_id.unknown_header] = "Unknown header: %02d.",
  [err_id.unknown_method] = "Unknown method: %02d.",
  [err_id.unknown_control] = "Unknown control: %02d.",
  [err_id.missing_control] = "Missing control: %s.",
  [err_id.unexpected_control] = "Unexpected control: %s.",
  [err_id.invalid_number] = "Invalid number: expected %d bytes, found %d.",
  [err_id.invalid_data_length] = "Invalid length header, data length mismatch it."
}


-- see /doc/protocols/CSP.md#control-characters
---@enum (key) CspControls
local controls = {
  header_end = 1,
  data_start = 2,
  string_start = 3,
  string_end = 4,
}

-- see /doc/protocols/CSP.md#header-keys
---@enum (key) CspHeaders
local headers = {
  method = 32,
  server = 33,
  length = 34,
  identity = 35,
  version = 36,
  update = 37,
  id = 38,
  reconnect = 39,
}

-- see /doc/protocols/headers/method.md
---@enum (key) CspMethods
local methods = {
  connect = 32,
  auth = 33,
  disconnect = 34,
  admin = 35,
  update = 36,
  action = 37,
  error = 38,
  state = 39
}

---return the name of the method given their byte representation
---@param id number
---@return string method
local function get_method(id)
  for k, v in pairs(methods) do
    if v == id then return k:lower() end
  end
  return ""
end

---guess the number of bytes required to store any given number
---@param n number
---@return integer bytes
local function guess_bytes(n)
  local pow = 1
  -- iterate until we found the perfect size
  while n > 2 ^ (8 * pow) - 1 do
    pow = pow + 1
  end
  return pow
end

---convert a number to little endian
---@param n number
---@return table bytes
local function to_bytes(n,size)
  local bytes = {}
  -- real size needed for the number
  local real_size = guess_bytes(n)

  for pow = 0, size - 1 do
    if pow >= real_size then
      table.insert(bytes, 0)
    else
    -- little endian
      table.insert(
        bytes,
        ---FIXME lua 5.2, 5.1, luvit, luajit
        bit.band(bit.rshift(n, 8*pow), 0xff)
        ---FIXME lua 5.4, 5.3
        -- (n >> 8*pow) & 0xff

      )
    end
  end
  return bytes
end

---convert a little endian bytes to number
---@param t table bytes
---@return number
local function to_num(t)
  local n = 0
  for i, byte in ipairs(t) do
    n = n + byte * 2 ^ (8 * (i - 1))
  end
  return n
end

--- a switch/case function
local function switch(t)
  t.case = function(x, ...)
    local f = t[x] or t.default
    if type(f) == "function" then
      return f(x, ...)
    else
      return f
    end
  end

  return t
end


-- ================== Packet ==================
-- bad practice but fuck it
local packet_id = 0

---set header
---@param self CspPacket
---@param header CspHeaders
---@param value any
---@return string? error
local function packet_set_header(self, header, value)
  header = string.lower(header)

  if not headers[header] then
    return stringformat("Invalid header %s", header)
  end

  if header == "id" and value == true then
    self:generate_id()
  else
    self.headers[header] = value
  end
end

---set headers using a table
---@param self CspPacket
---@param headers table<CspHeaders, any>
---@return nil
local function packet_set_headers(self, headers)
  for k,v in pairs(headers) do
    local err = self:set_header(k,v)
    if err then
      return err
    end
  end

end

---get header
---@param self CspPacket
---@param header CspHeaders
---@return any value
local function packet_get_header(self, header)
  return self.headers[string.lower(header)]
end

---set data and length header
---@param self CspPacket
---@param data string
---@return string? error
local function packet_set_data(self, data)
  if type(data) ~= "string" then
    return stringformat("expected a string, got %s", type(data))
  end

  self:set_header("length", #data)
  self.data = data
end

---get headers table
---@param self CspPacket
---@return table<CspHeaders, any> headers
local function packet_get_headers(self)
  local h = {}

  for k, v in pairs(self.headers) do
    h[k] = v
  end
  
  return h
end

---generate an id using intenal counter. yeah that's bad practice but it's only a example, not aim to provide a safe implementation
---@param self CspPacket
local function packet_generate_id(self)
  self:set_header("id", packet_id)
  packet_id = packet_id + 1
end

---parse the packet and prepare the intenal buffer
---@param self CspPacket
---@return string? error
local function packet_prepare(self)
  if not self.method then
    return "missing method"
  end

  local buffer, err = parser_stringify(self)

  if err then
    return err
  end

  self.buffer = buffer
end

---write the buffer the provided writter
---@param self CspPacket
---@param cb? function
---@return any|nil result
---@return string? error
local function packet_send(self, cb)
  if not self._write then
    return nil, "no writter found on the packet"
  end

  if not self.buffer then
    local err = self:prepare()

    if not self.buffer then
      return nil, "failed to prepare the packet: " .. err
    end
  end


  return self._write(self.buffer, cb)
end

---reply to the current packet, set the packet's id and try to send it
---@param self CspPacket
---@param packet CspPacket
---@return any|nil result
---@return string? error
local function packet_reply(self, packet)
  if not self.headers.id then
    return nil, "cannot reply to a packet without id"
  end

  packet._write = self._write

  if not packet then
    return nil, "invalid packet packet"
  end

  packet:set_header("id", self.headers.id)
  return packet:send()
end

---create a Packet ready to use
---@param method CspMethods method used
---@param headers? table<CspHeaders, any> table of headers
---@param data? string the data associated, will set the legth header
---@param write? fun(buffer: string, callback?: function) writter fucntion that will write internal buffer when using send
---@return CspPacket
local function Packet(method, headers, data, write)

  ---@class CspPacket
  local packet = {
    method = method,
    headers = {},
    data = nil,

    _write = write,

    set_header = packet_set_header,
    set_headers = packet_set_headers,
    get_header = packet_get_header,
    get_headers = packet_get_headers,
    set_data = packet_set_data,
    generate_id = packet_generate_id,
    
    prepare = packet_prepare,
    send = packet_send,
    reply = packet_reply,
  }
  
  headers = headers or {}

  local err = packet:set_headers(headers)
  if err then
    packet.err = err
    return packet
  end

  if data then
    err = packet:set_data(data)
    if err then
      packet.err = err
      return packet
    end
  end

  return packet
end


-- ============== Parser Utility ==============
-- this parser is adapted from https://github.com/lil-evil/toml.lua
-- it allow safe iteration over the buffer and some simplification of code

--- get the byte at current step or at custom position (not relative to cursor!)
---@param self table Parser
---@param at number|nil current step or at custom position (not relative to cursor!)
---@param n number|nil number of bytes to retreive
---@return number|table byte the byte(s) at the requested place. empty string if out of bound
local function parser_get_byte(self, at, n)
  n = type(n) == "number" and n or 1
  at = at or self.cursor
  if not self.buffer[at] then
    return n>1 and {} or -1
  end

  if n > 1 then
    local t = {}
    for i = 0, n - 1 do
      table.insert(t, self.buffer[at + i])
    end
    return t
  end

  return self.buffer[at]
end

--- step the cursor from 1 step or custom steps (can be negative)
---@param self table Parser
---@param step number|nil next step (1) or custom steps (can be negative)
local function parser_step(self, step)
  self.cursor = self.cursor + (step or 1)
  self.pos = self.pos + (step or 1)

  return self:get_byte(self.cursor)
end

---act like step, but do not move the cursor. basically an handy fucntion to not type get_char(cursor+step)
---@param self table parser
---@param step number|nil next step (1) or custom steps (can be negative)
---@return string char the char at the effective step
local function parser_poke(self, step)
  local at = self.cursor + (step or 1)
  return self:get_byte(at)
end

--- checks if not out of bound of the buffer
---@param self table Parser
---@param step number|nil next step (1) or custom steps (can be negative)
---@return boolean in_of_bounds
local function parser_bound(self, step)
  local at = self.cursor + (step or 1)
  return at <= #self.buffer and at > 0
end

---throw an error catched by the "decode" function
---@param self? table parser
---@param id CspErrorId
local function parser_error(self, id, ...)
  local message = err_msg[id]

  local msg = string.format(message or id, ...)

  if not message then id = "UNKNOWN" end

  ---@class CspError
  return {
    pos = self and self.pos or -1,
    id = id,
    message = msg
  }
end

local function Parser(buffer)
  -- look mom, oop!
  local parser = {
    cursor = 1,
    buffer = { stringbyte(buffer, 1, #buffer) },

    pos = 1,

    ---@diagnostic disable-next-line: missing-parameter
    packet = Packet(),

    is_header = true,
    is_data = false,

    -- members
    get_byte = parser_get_byte,
    step = parser_step,
    poke = parser_poke,
    bound = parser_bound,

    error = parser_error,
  }

  return parser
end

-- ============== parse ==============

---parse a number given the size of this number
---@param parser table
---@param n number the size of the number (usually 4 or 8)
---@return nil|number number the number or nil in case of error
---@return table|nil error nil or error in case of error
local function parse_number(parser, n)
  parser:step() -- skip the current byte
  local bytes = parser:get_byte(nil, n)

  -- if the packet ends too soon
  if #bytes ~= n then
    return nil, {err_id.invalid_number, n, #bytes}
  end
  parser:step(n - 1) -- consumes the bytes retreived

  local number = to_num(bytes)

  -- arctifact of old implementation, may be usefull in the future
  --if not number then
  --  return nil, stringformat("invalid number: %s", table.concat(bytes, " "))
  --end
  return number
end

---parse the string using string_start and string_end
---@param parser table
---@return nil|string string the number or nil in case of error
---@return table|nil error nil or error in case of error
local function parse_string(parser)
  parser:step() -- skip the current byte
  if parser:get_byte() ~= controls.string_start then
    return nil, {err_id.missing_control, "string_start"}
  end
  parser:step() -- consumes string_start

  local buffer, valid = {}, false
  while parser:bound(0) do
    local byte = parser:get_byte()

    if byte == controls.string_end then
      valid = true
      break
    end

    table.insert(buffer, stringchar(byte))

    parser:step()
  end

  -- if for some reason the buffer ends before the strings end
  if not valid then
    return nil, {err_id.missing_control, "string_end"}
  end

  return table.concat(buffer)
end

-- to have a cleaner parse function, and maybe some perfomance improvement ..?
local parser_match = switch({
  [controls.data_start] = function(byte, parser)
    local headers = parser.packet.headers
    if not headers.length or parser.is_header then
      return err_id.unexpected_control, "data_start"
    end

    parser.is_data = true
  end,
  [controls.header_end] = function(byte, parser)
    if not parser.is_header then
      return {err_id.unexpected_control, "header_end"}
    end
    parser.is_header = false
  end,
  [controls.string_start] = function(byte, parser)
    return {err_id.unexpected_control, "string_start"}
  end,
  [controls.string_end] = function(byte, parser)
    return {err_id.unexpected_control, "string_end"}
  end,

  [headers.method] = function(byte, parser)
    local packet = parser.packet
    if packet.method then
      return err_id.duplicate_header, "method"
    end

    packet.method = get_method(parser:step())
    if #packet.method < 1 then
      return err_id.unknown_method, parser:get_byte()
    end
  end,
  [headers.server] = function(byte, parser)
    local headers, err = parser.packet.headers, nil

    if headers.server then
      return err_id.duplicate_header, "server"
    end

    headers.server, err = parse_number(parser, 4)
    if err then
      return table.unpack(err)
    end
  end,
  [headers.length] = function(byte, parser)
    local headers, err = parser.packet.headers, nil

    if headers.length then
      return err_id.duplicate_header, "length"
    end

    headers.length, err = parse_number(parser, 8)
    if err then
      return table.unpack(err)
    end
  end,
  [headers.identity] = function(byte, parser)
    local headers, err = parser.packet.headers, nil

    if headers.identity then
      return err_id.duplicate_header, "identity"
    end

    headers.identity, err = parse_string(parser)
    if err then
      return table.unpack(err)
    end
  end,
  [headers.version] = function(byte, parser)
    local headers, err = parser.packet.headers, nil

    if headers.version then
      return err_id.duplicate_header, "version"
    end

    headers.version, err = parse_string(parser)
    if err then
      return table.unpack(err)
    end
  end,
  [headers.update] = function(byte, parser)
    local headers = parser.packet.headers
    if headers.update then
      return err_id.duplicate_header, "update"
    end

    headers.update = true
  end,
  [headers.id] = function(byte, parser)
    local headers, err = parser.packet.headers, nil

    if headers.id then
      return err_id.duplicate_header, "id"
    end

    headers.id, err = parse_number(parser, 8)
    if err then
      return table.unpack(err)
    end
  end,
  [headers.reconnect] = function(byte, parser)
    local headers = parser.packet.headers
    if headers.reconnect then
      return err_id.duplicate_header, "reconnect"
    end

    headers.reconnect = true
  end,

  default = function(byte, parser)
    if byte <= 31 then
      return err_id.unknown_control, byte
    else
      return err_id.unknown_header, byte
    end
  end
})

---parse a buffer until supposedly end of packet
---@param buffer string
---@return CspPacket|nil 
---@return string|CspError
function parser_parse(buffer)
  local parser = Parser(buffer)
  local err = {}

  -- loop
  while parser:bound(0) do
    if parser.is_data then
      -- consumes the buffer until end of file, data will be parsed accordingly of the software needs
      local buffer = {}
      for i=1, parser.packet.headers.length do
        if not parser:bound(0) then
          err = {err_id.invalid_data_length}
          break
        end

        ---@diagnostic disable-next-line: param-type-mismatch
        table.insert(buffer, stringchar(parser:get_byte()))

        parser:step()
      end
      parser.packet.data = table.concat(buffer)
      break
    end

    err = {parser_match.case(parser:get_byte(), parser)}
    if #err > 0 then
      break
    end

    parser:step()

    if not parser.packet.headers.length and not parser.is_header then
      break
    end
  end -- loop end

  if parser.is_header and #err == 0 then
    err = {err_id.missing_control, "header_end"}
  end

  parser.packet.buffer = string.sub(buffer, 1, parser.pos-1)

  if #err > 0 then
    return nil, parser:error(table.unpack(err))
  else
    return parser.packet, string.sub(buffer, parser.pos)
  end
end

-- ============== stringify ==============

---parse a number to little endian
---@param number number
---@param size number number of bytes to use to store this number
---@return table|nil
---@return nil|table
local function stringify_number(number, size)
  if number > 2 ^ (8 * size) - 1 then
    return nil, {"number %d is bigger than maximum allowed by a %d bytes number", number, size}
  end

  local bytes = to_bytes(number, size)

  return bytes
end

---parse a string to a csp string
---@param str string
---@return table
local function stringify_string(str)
  local buffer = { controls.string_start, stringbyte(str, 1, #str) }
  table.insert(buffer, controls.string_end)
  return buffer
end

local stringify_match = switch({
  [headers.method] = function(byte, packet, value)
    if type(value) ~= "string" then
      return {"method should be a string, got %s", type(value)}
    end

    local method = methods[value:lower()]

    if not method then
      return "unknown method: %s", value
    end
    table.insert(packet, method)
  end,
  [headers.server] = function(byte, packet, value)
    if type(value) ~= "number" then
      return {"server should be a number, got %s", type(value)}
    end

    local bytes, err = stringify_number(value, 4)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
  [headers.length] = function(byte, packet, value)
    if type(value) ~= "number" then
      return {"length should be a number, got %s", type(value)}
    end

    local bytes, err = stringify_number(value, 8)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
  [headers.identity] = function(byte, packet, value)
    if type(value) ~= "string" then
      return {"identity should be a string, got %s", type(value)}
    end

    local bytes, err = stringify_string(value)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
  [headers.version] = function(byte, packet, value)
    if type(value) ~= "string" then
      return {"version should be a string, got %s", type(value)}
    end

    local bytes, err = stringify_string(value)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
  [headers.update] = function(byte, packet, value)
    if type(value) ~= "boolean" then
      return {"update should be a boolean, got %s", type(value)}
    end

    if not value then
      packet[#packet] = nil
    end
  end,
  [headers.id] = function(byte, packet, value)
    if type(value) ~= "number" then
      return {"length should be a number, got %s", type(value)}
    end

    local bytes, err = stringify_number(value, 8)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
})

---parse a packet to string
---@param packet CspPacket
---@return string|nil 
---@return string? error
function parser_stringify(packet)
  local buffer = {}
  local err = {}

  table.insert(buffer, headers.method)
  err = stringify_match[headers.method](headers.method, buffer, string.lower(packet.method)) or {}

  if #err > 0 then
    return nil, string.format(err[1], table.unpack(err, 2))
  end

  for k, v in pairs(packet.headers) do
    local key = headers[k]
    if not key then
      err = {"unknown header: %s", k}
      break
    end
    table.insert(buffer, key)

    err = stringify_match.case(key, buffer, v) or {}
    if #err > 0 then
      break
    end
  end

  table.insert(buffer, controls.header_end)

  local buff = stringchar(table.unpack(buffer))

  if packet.data then
    local size = #packet.data
    local len = packet.headers.length

    if size ~= len then
      err = {"length header and data size mismatch (%d != %d)", len, size}
    else
      buff = buff .. stringchar(controls.data_start) .. packet.data
    end
  end


  if #err > 0 then
    return nil, string.format(err[1], table.unpack(err, 2))
  else
    return buff, nil
  end
end

-- ================== LOG ==================


---debug function to show data information
---@param raw string
---@param packet? CspPacket
local function log_csp(raw, packet)
  -- readability
  for i=0, math.floor(#raw/16) do
    for _ = 1, 16 do
      local pos = (_ +i*15)
      local char = string.sub(raw, pos, pos)
      if #char > 0 then
        io.stdout:write(stringformat("%03d ", string.byte(char)))
      else
        io.stdout:write("... ")
      end
    end

    io.stdout:write("| ")


    for _ = 1, 16 do
      local pos = (_ +i*15)
      local char = string.sub(raw, pos, pos)

      if #char > 0 then
        local byte = string.byte(char)
        
        if byte and byte >= 32 and byte <= 126 then
          io.stdout:write(char)
        else
          io.stdout:write(".")
        end        
      end

    end

    io.stdout:write("\n")
  end

  if type(packet) == "table" then
    io.stdout:write(stringformat("\x1b[32m%s\x1b[0m:\n", packet.method or "nil"))
    for k, v in pairs(packet.headers) do
      io.stdout:write(stringformat("  | %s%s: %s\n", k, string.rep(" ", 12-#k), v))
    end
  end
  
  io.stdout:flush()
end


return {
  Packet = Packet,
  log_csp = log_csp,

  methods = methods,
  headers = headers,
  error_id = err_id,
  error_message = err_msg,

  switch = switch,
  num_guess_bytes = guess_bytes,
  num_to_bytes = to_bytes,
  bytes_to_num = to_num,

  parse = parser_parse,
  stringify = parser_stringify
}
