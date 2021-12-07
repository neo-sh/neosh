--[[
-- Allows for the user and for Neosh to forge instructions.
--]]

local command = {
    command = "",
    flags = {},
    arguments = {},
    -- TODO: Add a function to apply these options
    -- We'll probably have to wait for tbl_deep_extend
    options = {
        flags = {
            --- Whether or not to use `--` in flags instead of just `-`
            --- Defaults to false
            use_double_hyphens = false,
            -- Can be e.g. `=`
            separator = " ",
        },
    }
}

-- Functions for describing the command
function command.arg(self, ...)
    table.insert(self.arguments, table.concat({ ... }, " "))

    return self
end

function command.flag(self, flag, value)
    table.insert(self.flags, value and { flag, value } or flag)

    return self
end

-- Utility functions

--- Converts a lua representation of a command
--- back into a stringified version
function command.build(self, order)
    order = order or {
        "command",
        "flags",
        "arguments",
    }

    local result = {}

    for _, key in ipairs(order) do
        local builder = command["build_" .. key]

        if not command[key] or not builder then
            return
        end

        local output = builder(self)

        if output:len() > 0 then
            table.insert(result, output)
        end
    end

    return table.concat(result, " ")
end

function command.build_command(self)
    return self.command
end

function command.build_flags(self)
    local result = ""

    for _, flag in ipairs(self.flags) do
        if type(flag) == "table" then
            result = result
                .. (self.options.flags.use_double_hyphens and "--" or "-")
                .. flag[1]
                .. self.options.flags.separator
                .. '"'
                .. flag[2]
                .. '"'
        else
            result = result
                .. (self.options.flags.use_double_hyphens and "--" or "-")
                .. flag
        end
    end

    return result
end

function command.build_arguments(self)
    return '"' .. table.concat(neosh.tbl_values(self.arguments), '" "') .. '"'
end

function command.exec(self)
    -- We should ideally bridge to rust here
    print("Executed '" .. self.command .. "' with args:", neosh.inspect(self.arguments))
end

return function(cmdname)
    local new_instruction = {}

    for k, v in pairs(command) do
        new_instruction[k] = v
    end

    new_instruction.command = cmdname

    return new_instruction
end
