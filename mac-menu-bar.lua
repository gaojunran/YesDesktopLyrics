-- Install hammerspoon and copy this as `lyrics.lua` to ~/.hammerspoon/.
-- Include this script in your `init.lua`: `require("lyrics")`.
-- Then `Reload Config` in Hammerspoon.

local menubar = hs.menubar.new() 


menubar:setTitle("🎵")


function updateLyric()
    local output, status = hs.execute("lyrics line")
    if status and output then
        local lyric = output:gsub("[\r\n]", "")
        menubar:setTitle("🎵 " .. lyric)
    else
        menubar:setTitle("")
    end
end

-- https://github.com/Hammerspoon/hammerspoon/issues/1942
-- You can change the interval as you want.
myTimer = hs.timer.doEvery(0.5, updateLyric)

updateLyric()
