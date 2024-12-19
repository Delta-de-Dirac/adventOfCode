local io = require("io")

-- input
io.input("input/day02.txt")
local line = io.read("*line")
local lines = {}
while line do
    table.insert(lines, line)
    line = io.read("*line")
end
io.input()
-- end input

-- solution 1

local safe_reports = 0;

for _, line in pairs(lines) do
    local safe       = true
    local last_level = nil
    local increasing = false
    local decreasing = false
    for word in string.gmatch(line, "[^%s]+") do 
        local current_level = tonumber(word)
        if last_level then
            local difference = current_level - last_level
            if math.abs(difference) < 1 or math.abs(difference) > 3 then
                safe = false
                break
            end
            if increasing then
                if difference < 0 then
                    safe = false
                    break
                end
            elseif decresing then
                if difference > 0 then
                    safe = false
                    break
                end
            else
                if difference < 0 then
                    decreasing = true
                elseif difference > 0 then
                    increasing = true
                end
            end
        end
        last_level = current_level
    end
    if safe then
        safe_reports = safe_reports + 1
    end
end

io.write("Result part 1: " .. safe_reports .. "\n")
-- end solution 1

-- solution 2

io.write("Result part 2: " .. 'x' .. "\n")
-- end solution 2



