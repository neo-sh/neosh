-----[[-----------------------------------]]-----
----                                         ----
---        Extended NEOSH Lua stdlib          ---
---     This file is licensed under GPLv3     ---
----                                         ----
-----]]-----------------------------------[[-----

--- @class neosh
local neosh = neosh or {}

--- Return human-readable tables
--- NOTE: this field is going to be populated when requiring 'inspect.lua'
neosh.inspect = {}
-- neosh.prompt = neosh.prompt or require("neosh.prompt")

--- Pretty print the given objects
neosh.fprint = function(...)
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

--- Extract the given map-like table keys names and returns them
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

--- Search if a map-like table contains a key
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

--- Splits a string at N instances of a separator
--- @tparam string str The string to split
--- @tparam string sep The separator to be used when splitting the string
--- @tparam table kwargs Extra arguments:
---         - plain, boolean: pass literal `sep` to `string.find` call
---         - trim_empty, boolean: remove empty items from the returned table
---         - splits, number: number of instances to split the string
--- @return table
neosh.split = function(str, sep, kwargs)
    if not sep then
        sep = "%s"
    end
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

--- Filter a table using a predicate function
--- @tparam table tbl
--- @tparam function func
--- @return table
neosh.tbl_filter = function(tbl, func)
    local filtered_tbl = {}
    for _, value in pairs(tbl) do
        if func(value) then
            table.insert(filtered_tbl, value)
        end
    end
    return filtered_tbl
end

--- Apply a function to all values of a table
--- @tparam table tbl
--- @tparam function func
--- @return table
neosh.tbl_map = function(tbl, func)
    local map_tbl = {}
    for k, v in pairs(tbl) do
        map_tbl[k] = func(v)
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
    extend_table(behavior, false, ...)
end

--- Deeply merges two or more map-like tables and its sub-tables
--- @tparam string behavior Decides what to do if a key is found in more than one map
--- @vararg table
--- @return table
neosh.tbl_deep_extend = function(behavior, ...)
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

--- Check if strings starts with given pattern
--- @tparam string str
--- @tparam string pattern
--- @return boolean
neosh.starts_with = function(str, pattern)
    return str:sub(1, #pattern) == pattern
end

--- Check if strings ends with given pattern
--- @tparam string str
--- @tparam string pattern
--- @return boolean
neosh.ends_with = function(str, pattern)
    return str:sub(-#pattern) == pattern
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
