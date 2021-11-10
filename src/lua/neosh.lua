-----[[-----------------------------------]]-----
----                                         ----
---        Extended NEOSH Lua stdlib          ---
---     This file is licensed under GPLv3     ---
----                                         ----
-----]]-----------------------------------[[-----

--- @class neosh
local neosh = neosh or {}

--- Return human-readable tables
neosh.inspect = require("inspect")
neosh.prompt = require("neosh.prompt")

--- Pretty print the given objects
neosh.pp = function(...)
  local args = { ... }
  for _, arg in ipairs(args) do
    print(neosh.inspect(arg))
  end
end

--- Check if string is empty or if it is nil
--- @tparam str string The string to be checked
--- @return boolean
neosh.is_empty = function(str)
  return str == "" or str == nil
end

--- Escape special characters in a string
--- @tparam string str The string to be escaped
--- @return string
neosh.escape_str = function(str)
  local escape_patterns = {
    "%^",
    "%$",
    "%(",
    "%)",
    "%[",
    "%]",
    "%%",
    "%.",
    "%-",
    "%*",
    "%+",
    "%?",
  }

  return str:gsub(("([%s])"):format(table.concat(escape_patterns)), "%%%1")
end

--- Extract the given table keys names and returns them
--- @tparam table tbl The table to extract its keys
--- @return table
neosh.tbl_keys = function(tbl)
  local keys = {}

  for key, _ in pairs(tbl) do
    table.insert(keys, key)
  end

  return keys
end

--- Search if a table contains a value
--- @tparam table tbl The table to look for the given value
--- @tparam any val The value to be looked for
--- @return boolean
neosh.has_value = function(tbl, val)
  for _, value in ipairs(tbl) do
    if value == val then
      return true
    end
  end

  return false
end

--- Search if a table contains a key
--- @tparam table tbl The table to look for the given key
--- @tparam string key The key to be looked for
--- @return boolean
neosh.has_key = function(tbl, key)
  for _, k in ipairs(neosh.tbl_keys(tbl)) do
    if k == key then
      return true
    end
  end

  return false
end

--- TODO: Splits a string at N instances of a separator
--[[ neostd.split = function(str, sep, kwargs)
--
-- NOTE: kwargs will cover the Neovim 'vim.split' arguments and also a "python-like times to split argument"
--
end ]]

return neosh

-- vim: sw=2:ts=2:sts=2:tw=100:
