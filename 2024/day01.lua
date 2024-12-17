local io = require("io")

-- input
io.input("input/day01.txt")
local line = io.read("*line")
local lines = {}
while line do
    table.insert(lines, line)
    line = io.read("*line")
end
io.input()
-- end input

-- solution 1
left_list  = {}
right_list = {}

for k, v in pairs(lines) do
    local _, _, l, r = string.find(v, "(%w+)%s+(%w+)")
    l = tonumber(l)
    r = tonumber(r)
    table.insert(left_list,  l)
    table.insert(right_list, r)
end

table.sort(left_list)
table.sort(right_list)

local total = 0

for k,v in pairs(left_list) do
    total = total + math.abs(left_list[k] - right_list[k])
end

io.write("Result part 1: " .. total .. "\n")
-- end solution 1

-- solution 2
total = 0

local l = 1
local r = 1

while l <= #left_list do
    local rcount = 0
    local lcount = 1
    local break_outer_loop = false
    
    while l < #left_list and left_list[l] == left_list[l+1] do
        lcount = lcount + 1
        l = l + 1
    end
    
    while left_list[l] > right_list[r] do
        r = r + 1
        if r > #right_list then
            break_outer_loop = true
            break
        end
    end
    if break_outer_loop then 
        total = total + rcount * lcount * left_list[l]
        break 
    end
    while left_list[l] == right_list[r] do
        rcount = rcount + 1
        r = r + 1
        if r > #right_list then
            break_outer_loop = true
            break
        end
    end
    if break_outer_loop then 
        total = total + rcount * lcount * left_list[l]
        break 
    end
    total = total + rcount * lcount * left_list[l]
    l = l + 1
end

io.write("Result part 2: " .. total .. "\n")
-- end solution 2



