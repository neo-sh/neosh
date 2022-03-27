-----[[-----------------------------------]]-----
----                                         ----
---        Extended NEOSH Lua stdlib          ---
---     This file is licensed under GPLv3     ---
----                                         ----
-----]]-----------------------------------[[-----

--- @class neosh
local neosh = neosh or {}

----- UTILS ---------------------------
---------------------------------------

--- Fail-safe unpack declaration, it is not the same in all Lua 5.x versions
local unpack = unpack or table.unpack

--- Wrapper for built-in Lua assert that allows checking against types and functions
--- @tparam table args_tbl
neosh.assert = function(args_tbl)
    assert(type(args_tbl) == "table", "args_tbl: expected table, got " .. type(args_tbl))

    for arg, body in pairs(args_tbl) do
        assert(type(body) == "table", "args_tbl: expected table for body, got " .. type(body))

        -- Valid arguments for body table (array):
        --  arg_value: variable content (any, 1st arg)
        --  type: checks if variable has a certain data type (string, optional, 2nd arg)
        --        type also accepts a table of valid types
        --  cond: checks if variable meets a condition (function, optional, 2nd arg)
        --  err_msg: error message to be sent, type argument has a default one (string, 3rd arg)
        local arg_value, type_or_cond, err_msg = unpack(body)
        if type(type_or_cond) == "string" then
            if err_msg then
                assert(type(arg_value) == type_or_cond, err_msg)
            else
                assert(
                    type(arg_value) == type_or_cond,
                    arg .. ": expected " .. type_or_cond .. ", got " .. type(arg_value)
                )
            end
        elseif type(type_or_cond) == "table" then
            if err_msg then
                assert(neosh.tbl_has_value(type_or_cond, type(arg_value)), err_msg)
            else
                assert(
                    neosh.tbl_has_value(type_or_cond, type(arg_value)),
                    arg
                        .. ": expected "
                        .. table.concat(type_or_cond, "|")
                        .. ", got "
                        .. type(arg_value)
                )
            end
        elseif type(type_or_cond) == "function" then
            assert(type_or_cond(arg_value), arg .. ": " .. err_msg)
        end
    end
end

--- Return human-readable tables
--- NOTE: this field is going to be populated when requiring 'inspect.lua'
neosh.inspect = {}

--- Pretty print the given objects
--- @vararg any
neosh.fprint = function(...)
    local args = { ... }
    for _, arg in ipairs(args) do
        print(neosh.inspect(arg))
    end
end

--- Print formatted string in a C-style
--- @tparam string str
--- @vararg any
neosh.printf = function(str, ...)
    neosh.assert({ str = { str, "string" } })
    print(str:format(...))
end

----- STRINGS -------------------------
---------------------------------------

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
    neosh.assert({ str = { str, "string" } })
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

--- Check if string starts with given pattern
--- @tparam string str
--- @tparam string pattern
--- @return boolean
neosh.starts_with = function(str, pattern)
    neosh.assert({
        str = { str, "string" },
        pattern = { pattern, 'string' },
    })
    return str:sub(1, #pattern) == pattern
end

--- Check if string ends with given pattern
--- @tparam string str
--- @tparam string pattern
--- @return boolean
neosh.ends_with = function(str, pattern)
    neosh.assert({
        str = { str, "string" },
        pattern = { pattern, 'string' },
    })
   return str:sub(-#pattern) == pattern
end

--- Check if string contains given pattern
--- @tparam string str
--- @tparam string pattern
--- @return boolean
neosh.str_contains = function(str, pattern)
    neosh.assert({
        str = { str, "string" },
        pattern = { pattern, 'string' },
    })
    return str:match(pattern) ~= nil
end

--- Splits a string at N instances of a separator
--- @tparam string str The string to split
--- @tparam string sep The separator to be used when splitting the string (optional, default is '%s')
--- @tparam table kwargs Extra arguments:
---         - plain, boolean: pass literal `sep` to `string.find` call
---         - trim_empty, boolean: remove empty items from the returned table
---         - splits, number: number of instances to split the string
--- @return table
neosh.split = function(str, sep, kwargs)
    neosh.assert({ str = { str, "string" } })

    sep = sep or "%s"
    kwargs = kwargs or {}
    local plain = kwargs.plain
    local trim_empty = kwargs.trim_empty
    local splits = kwargs.splits or -1

    local str_tbl = {}
    local nField, nStart = 1, 1
    local nFirst, nLast
    if plain then
        nFirst, nLast = str:find(sep, nStart, plain)
    else
        nFirst, nLast = str:find(sep, nStart)
    end

    while nFirst and splits ~= 0 do
        str_tbl[nField] = str:sub(nStart, nFirst - 1)
        nField = nField + 1
        nStart = nLast + 1
        nFirst, nLast = str:find(sep, nStart)
        splits = splits - 1
    end
    str_tbl[nField] = str:sub(nStart)

    if trim_empty then
        for i = #str_tbl, 1, -1 do
            if str_tbl[i] == "" then
                table.remove(str_tbl, i)
            end
        end
    end

    return str_tbl
end

----- TABLES --------------------------
---------------------------------------

--- Check if a table is an array
---@tparam table tbl
---@return boolean
neosh.tbl_is_array = function(tbl)
    neosh.assert({ tbl = { tbl, "table" } })
    return #tbl > 0 or next(tbl) == nil
end

--- Check if a table is a map (key: value)
---@tparam table tbl
---@return boolean
neosh.tbl_is_map = function(tbl)
    neosh.assert({ tbl = { tbl, "table" } })
    return #tbl == 0
end

--- Extract the given map-like table keys names and returns them
--- @tparam table tbl The table to extract its keys
--- @return table
neosh.tbl_keys = function(tbl)
    neosh.assert({ tbl = { tbl, neosh.tbl_is_map, "expected a map-like table" } })

    local keys = {}
    for key, _ in pairs(tbl) do
        table.insert(keys, key)
    end

    return keys
end

--- Extract the given table values and returns them
---@tparam table tbl The table to extract its keys
---@return table
neosh.tbl_values = function(tbl)
    neosh.assert({ tbl = { tbl, "table" } })

    local values = {}
    for _, value in pairs(tbl) do
        table.insert(values, value)
    end

    return values
end

--- Search if a table contains a value
--- @tparam table tbl The table to look for the given value
--- @tparam any val The value to be looked for
--- @return boolean
neosh.tbl_has_value = function(tbl, val)
    neosh.assert({ tbl = { tbl, "table" } })

    for _, value in ipairs(neosh.tbl_values(tbl)) do
        if value == val then
            return true
        end
    end

    return false
end

--- Search if a map-like table contains a key
--- @tparam table tbl The table to look for the given key
--- @tparam string key The key to be looked for
--- @return boolean
neosh.tbl_has_key = function(tbl, key)
    neosh.assert({ tbl = { tbl, "table" } })

    for _, k in ipairs(neosh.tbl_keys(tbl)) do
        if k == key then
            return true
        end
    end

    return false
end

--- Filter a table using a predicate function
--- @tparam table tbl
--- @tparam function fn
--- @return table
neosh.tbl_filter = function(tbl, fn)
    neosh.assert({
        tbl = { tbl, "table" },
        fn = { fn, "function" },
    })

    local filtered_tbl = {}
    for _, value in pairs(tbl) do
        if fn(value) then
            table.insert(filtered_tbl, value)
        end
    end
    return filtered_tbl
end

--- Apply a function to all values of a table
--- @tparam table tbl
--- @tparam function fn
--- @return table
neosh.tbl_map = function(tbl, fn)
    neosh.assert({
        tbl = { tbl, "table" },
        fn = { fn, "function" },
    })

    local map_tbl = {}
    for k, v in pairs(tbl) do
        map_tbl[k] = fn(v)
    end
    return map_tbl
end

--- Merges two or more map-like tables
--- @tparam string behavior Decides what to do if a key is found in more than one map
--- @tparam boolean deep_extend Decides if subtables should be also merged
--- @vararg table
--- @return table
--- @private
local extend_table = function(behavior, deep_extend, ...)
    if behavior ~= "keep" and behavior ~= "error" and behavior ~= "force" then
        error(string.format("Invalid tbl_extend behavior: '%s'", behavior))
    end

    local extended_tbl = {}
    for i = 1, select("#", ...) do
        local tbl = select(i, ...)
        if tbl then
            for k, v in pairs(tbl) do
                if deep_extend and type(tbl[k]) == "table" and type(v) == "table" then
                    extended_tbl[k] = neosh.tbl_extend(behavior, deep_extend, tbl[k], v)
                elseif behavior ~= "force" and extended_tbl[k] ~= nil then
                    if behavior == "error" then
                        error(string.format("Key '%s' found in more than one map", k))
                    end
                else
                    extended_tbl[k] = v
                end
            end
        end
    end
    return extended_tbl
end

--- Merges two or more map-like tables
--- @tparam string behavior Decides what to do if a key is found in more than one map
--- @vararg table
--- @return table
neosh.tbl_extend = function(behavior, ...)
    neosh.assert({ behavior = { behavior, "string" } })

    extend_table(behavior, false, ...)
end

--- Deeply merges two or more map-like tables and its sub-tables
--- @tparam string behavior Decides what to do if a key is found in more than one map
--- @vararg table
--- @return table
neosh.tbl_deep_extend = function(behavior, ...)
    neosh.assert({ behavior = { behavior, "string" } })
    extend_table(behavior, true, ...)
end

--- Returns a deep copy of the given object
--- @tparam any orig
--- @return any
neosh.deep_copy = function(orig)
    local copy
    local orig_type = type(orig)
    if orig_type == "table" then
        copy = {}
        for orig_key, orig_value in next, orig, nil do
            copy[neosh.deep_copy(orig_key)] = neosh.deep_copy(orig_value)
        end
        setmetatable(copy, neosh.deep_copy(getmetatable(orig)))
    else
        copy = orig
    end

    return copy
end

neosh = setmetatable(neosh, {
    __index = function(_, key)
        return function(...)
            local args = { ... }
            local cmd = key
            for _, arg in ipairs(args) do
                cmd = cmd .. " " .. arg
            end
            os.execute(cmd)
        end
    end,
})

return neosh

-- vim: sw=4:ts=4:sts=4:tw=100:
