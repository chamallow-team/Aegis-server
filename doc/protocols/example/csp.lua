local bit = require "bit"

local stringchar, stringbyte, stringformat = string.char, string.byte, string.format


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
    return -1
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
---@param self table parser
---@param message any
local function parser_error(self, message)
  return {
    pos = self.pos,
    message = message
  }
end

local function Parser(buffer)
  -- look mom, oop!
  local parser = {
    cursor = 1,
    buffer = { stringbyte(buffer, 1, #buffer) },

    pos = 1,

    parsed = {},

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

-- ============== CSP integration ==============

-- see /doc/protocols/CSP.md#control-characters
local controls = {
  header_end = 1,
  data_start = 2,
  string_start = 3,
  string_end = 4,
}

-- see /doc/protocols/CSP.md#header-keys
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

---return the name of the method givent they byte representation
---@param id number
---@return string method
local function get_method(id)
  for k, v in pairs(methods) do
    if v == id then return k:upper() end
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
local function to_bytes(n)
  local bytes = {}
  local size = guess_bytes(n)

  for pow = 0, size do
    -- little endian
    table.insert(
      bytes,
      bit.band(bit.rshift(n, 8 * pow), 0xff)
    -- (n >> 8*pow) & 0xff
    )
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


-- ============== parse ==============

---parse a number given the size of this number
---@param parser table
---@param n number the size of the number (usually 4 or 8)
---@return nil|number number the number or nil in case of error
---@return string|nil error nil or error in case of error
local function parse_number(parser, n)
  parser:step() -- skip the current byte
  local bytes = parser:get_byte(nil, n)

  -- if the packet ends too soon
  if #bytes ~= n then
    return nil, stringformat("invalid number: expected %d bytes, found %d", n, #bytes)
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
---@return string|nil error nil or error in case of error
local function parse_string(parser)
  parser:step() -- skip the current byte
  if parser:get_byte() ~= controls.string_start then
    return nil, "missing string_start"
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
    return nil, "missing string_end"
  end

  return table.concat(buffer)
end

-- to have a cleaner parse function, and maybe some perfomance improvement ..?
local match = switch({
  [controls.data_start] = function(byte, parser)
    local parsed = parser.parsed
    if not parsed.length then
      return "missing length header, but data_start is found."
    end
    if parser.is_header then
      return "found data_start without header_end"
    end

    parser.is_data = true
  end,
  [controls.header_end] = function(byte, parser)
    if not parser.is_header then
      return "header_end has already been set"
    end
    parser.is_header = false
  end,
  [controls.string_start] = function(byte, parser)
    return "no reason to find a string_start here"
  end,
  [controls.string_end] = function(byte, parser)
    return "no reason to find a string_end here"
  end,

  [headers.method] = function(byte, parser)
    local parsed = parser.parsed
    if parsed.method then
      return "method already set"
    end

    parsed.method = get_method(parser:step())
    if #parsed.method < 1 then
      return "unknown method " .. parser:get_byte()
    end
  end,
  [headers.server] = function(byte, parser)
    local parsed, err = parser.parsed, nil
    if parsed.server then
      return "server already set"
    end

    parsed.server, err = parse_number(parser, 4)
    if err then
      return err
    end
  end,
  [headers.length] = function(byte, parser)
    local parsed, err = parser.parsed, nil
    if parsed.length then
      return "length already set"
    end

    parsed.length, err = parse_number(parser, 8)
    if err then
      return err
    end
  end,
  [headers.identity] = function(byte, parser)
    local parsed, err = parser.parsed, nil
    if parsed.identity then
      return "identity already set"
    end

    parsed.identity, err = parse_string(parser)
    if err then
      return err
    end
  end,
  [headers.version] = function(byte, parser)
    local parsed, err = parser.parsed, nil
    if parsed.version then
      return "version already set"
    end

    parsed.version, err = parse_string(parser)
    if err then
      return err
    end
  end,
  [headers.update] = function(byte, parser)
    local parsed = parser.parsed
    if parsed.update then
      return "update already set"
    end

    parsed.update = true
  end,
  [headers.id] = function(byte, parser)
    local parsed, err = parser.parsed, nil
    if parsed.id then
      return "id already set"
    end

    parsed.id, err = parse_number(parser, 8)
    if err then
      return err
    end
  end,
  [headers.reconnect] = function(byte, parser)
    local parsed = parser.parsed
    if parsed.reconnect then
      return "reconnect already set"
    end

    parsed.reconnect = true
  end,

  default = function(byte, parser)
    return stringformat("unknown header/control: %d", byte)
  end
})

local function parser_parse(buffer)
  local parser = Parser(buffer)
  local err_msg = nil

  -- states

  -- loop
  while parser:bound(0) do
    if parser.is_data then
      -- consumes the buffer until end of file, data will be parsed accordingly of the software need
      local buffer = {}
      while parser:bound(0) do
        ---@diagnostic disable-next-line: param-type-mismatch
        table.insert(buffer, stringchar(parser:get_byte()))
        parser:step()
      end
      parser.parsed.data = table.concat(buffer)
      break
    end

    local err = match.case(parser:get_byte(), parser)
    if err then
      err_msg = err
      break
    end

    parser:step()
  end -- loop end

  if parser.is_header and not err_msg then
    err_msg = "no header_end found"
  end


  if err_msg then
    return nil, parser:error(err_msg)
  else
    return parser.parsed, nil
  end
end

-- ============== stringify ==============
local function stringify_number(number, size)
  if number > 2 ^ (8 * size) - 1 then
    return stringformat("number %d is bigger than maximum allowed by a %d bytes number", number, size)
  end

  local bytes = to_bytes(number)

  return bytes
end

local function stringify_string(str)
  local buffer = { controls.string_start, stringbyte(str, 1, #str) }
  table.insert(buffer, controls.string_end)
  return buffer
end

local stringify = switch({
  [headers.method] = function(byte, packet, value)
    if type(value) ~= "string" then
      return stringformat("method should be a string, got %s", type(value))
    end

    local method = methods[value:lower()]

    if not method then
      return stringformat("unknown method: %s", value)
    end
    table.insert(packet, method)
  end,
  [headers.server] = function(byte, packet, value)
    if type(value) ~= "number" then
      return stringformat("server should be a number, got %s", type(value))
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
      return stringformat("length should be a number, got %s", type(value))
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
      return stringformat("identity should be a string, got %s", type(value))
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
      return stringformat("version should be a string, got %s", type(value))
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
      return stringformat("update should be a boolean, got %s", type(value))
    end

    if not value then
      packet[#packet] = nil
    end
  end,
  [headers.id] = function(byte, packet, value)
    if type(value) ~= "number" then
      return stringformat("length should be a number, got %s", type(value))
    end

    local bytes, err = stringify_number(value, 8)
    if err then return err end

    ---@diagnostic disable-next-line: param-type-mismatch
    for i, v in ipairs(bytes) do
      table.insert(packet, v)
    end
  end,
})

local function parser_stringify(tbl)
  local packet = {}
  local err_msg = nil

  local has_data = false

  for k, v in pairs(tbl) do
    if k == "data" then
      has_data = true
    else
      local key = headers[k]
      if not key then
        err_msg = stringformat("unknown header %s", k)
        break
      end
      table.insert(packet, key)

      local err = stringify.case(key, packet, v)
      if err then
        err_msg = err
        break
      end
    end
  end

  table.insert(packet, controls.header_end)

  if has_data then
    table.insert(packet, controls.data_start)

    for i = 1, #tbl.data do
      table.insert(packet, stringbyte(string.sub(tbl.data, i, i)))
    end
  end


  if err_msg then
    return nil, err_msg
  else
    return stringchar(table.unpack(packet)), nil
  end
end


return {
  parse = parser_parse,
  stringify = parser_stringify
}
