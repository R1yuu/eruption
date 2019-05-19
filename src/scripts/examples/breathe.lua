-- This file is part of Eruption.

-- Eruption is free software: you can redistribute it and/or modify
-- it under the terms of the GNU General Public License as published by
-- the Free Software Foundation, either version 3 of the License, or
-- (at your option) any later version.

-- Eruption is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU General Public License for more details.

-- You should have received a copy of the GNU General Public License
-- along with Eruption.  If not, see <http://www.gnu.org/licenses/>.

-- global script configuration --
config["script_name"] = "breathe"
config["script_description"] = "A breathing effect for your keyboard"
config["script_version"] = "0.0.1"
config["script_author"] = "The Eruption development team"
config["min_supported_version"] = "0.0.1"

-- global constants --
color_off = 0x00000000
color_bright = 0x00ffffff
color_background = 0x00111111
color_step = 0x00111111

breathe_multiplier = 0.25
breathe_upper_lim = -10
breathe_lower_lim = 10

color_afterglow = rgb_to_color(255, 0, 0)
color_step_afterglow = rgb_to_color(10, 0, 0)
afterglow_step = 2

-- global state variables --
breathe_step = get_runnable_tasks() * breathe_multiplier
color_map = {}
color_map_pressed = {}

ticks = 0
percentage = 0

-- event handler functions --
function on_startup(config)
    init_state()
end

function on_quit(exit_code)
end

function on_key_down(key_index)
    color_map_pressed[key_index] = color_afterglow
end

function on_tick(delta)
    ticks = ticks + delta + 1

    -- update system load indicator approximately every 5 seconds
    if ticks % 250 == 0 then
        breathe_step = max(min(get_runnable_tasks() * breathe_multiplier, 3.25), 0.25)
        trace("Runqueue: " .. get_runnable_tasks() .. " Step: " .. breathe_step)
    end
    
    local num_keys = get_num_keys()

    -- calculate 'fill' percentage for breathing effect
    percentage = percentage + ((breathe_step * max(delta, 1)) + (easing(percentage) * breathe_step))
    if percentage >= (100 - breathe_upper_lim) then
        percentage = 100 - breathe_upper_lim
        breathe_step = breathe_step * -1
    elseif percentage <= (0 + breathe_lower_lim) then
        percentage = 0 + breathe_lower_lim
        breathe_step = breathe_step * -1
    end
    
    -- generate breathing color map values
    local upper_bound = num_keys * (min(percentage, 100) / 100)
    for i = 0, num_keys do
        if i <= upper_bound then
            color_map[i] = color_map[i] + color_step

            if color_map[i] >= 0x00ffffff then
                color_map[i] = 0x00ffffff
            elseif color_map[i] <= 0x00000000 then
                color_map[i] = 0x00000000
            end
        else
            color_map[i] = color_background
        end
    end

    -- calculate afterglow effect for pressed keys
    if ticks % afterglow_step == 0 then
        for i = 0, num_keys do        
            if color_map_pressed[i] >= 0x00000000 then
                color_map_pressed[i] = color_map_pressed[i] - color_step_afterglow

                if color_map_pressed[i] >= 0x00ffffff then
                    color_map_pressed[i] = 0x00ffffff
                elseif color_map_pressed[i] <= 0x00000000 then
                    color_map_pressed[i] = 0x00000000
                end
            end
        end
    end

    -- now combine all the color maps to a final map
    local color_map_combined = {}
    for i = 0, num_keys do
        color_map_combined[i] = color_map[i] + color_map_pressed[i]

        -- let the afterglow effect override all other effects
        if color_map_pressed[i] > 0x00000000 then
            color_map_combined[i] = color_map_pressed[i]
        end

        if color_map_combined[i] >= 0x00ffffff then
            color_map_combined[i] = 0x00ffffff
        elseif color_map_combined[i] <= 0x00000000 then
            color_map_combined[i] = 0x00000000
        end
    end

    set_color_map(color_map_combined)
end

-- a simple easing function that mimics breathing
function easing(x)    
    return pow(sin(5 * x / 3.14159), 2)
end

-- init global state
function init_state()
    local num_keys = get_num_keys()
    for i = 0, num_keys do
        color_map[i] = color_background
        color_map_pressed[i] = color_off
    end
end
