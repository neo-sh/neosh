-- This is just here as a proof of concept

local prompt = {}

prompt.elements = {
    "[",
    prompt.user,
    " ",
    prompt.cwd,
    "] ",
}

return prompt
