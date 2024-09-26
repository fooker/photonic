let raindrops = {
    type = "raindrops",
    name = "raindrops",
    rate = 0.3,
    decay = [0.96, 0.98],
    color = {
        type = "fader",
        easing = {
            func = "linear",
            speed = "4s"
        },
        input = {
            type = "sequence",
            next = {
                input = "preset_next"
            },
            prev = {
                input = "preset_prev"
            },
            values = [
                ["hsl(245.3, 0.50, 0.5)", "hsl(333.6, 0.70, 0.5)"],
                ["hsl(000.0, 0.45, 0.5)", "hsl(017.5, 0.55, 0.5)"],
                ["hsl(187.5, 0.25, 0.5)", "hsl(223.9, 0.50, 0.5)"]
            ]
        }
    }
}

let noise = {
    type = "noise",
    name = "noise",
    speed = 0.005,
    stretch = 5.0,
    noise = "simplex"
}

let base = noise

let brightness = {
    type = "brightness",
    name = "brightness",
    source = base,
--    value = 1
    value = {
        type = "fader",
        easing = {
            func = "linear",
            speed = "3s"
        },
        input = {
            input = "brightness",
            initial = 1.0
        }
    }
}

let alert = {
    type = "overlay",
    name = "alert_overlay",

    base = brightness,
    pave = {
        type = "alert",
        name = "alert",
        hue = 0.0,
        block = 1,
        speed = 1.0
    },

    blend = {
        type = "fader",
        input = {
            type = "button",
            value_release = 0.0,
            value_pressed = 1.0,
            hold_time = "15s",
            trigger = {
                input = "alert"
            }
        },
        easing = {
            func = { quartic = "in_out" },
            speed = "3s"
        }
    }
    -- blend = 0.5
}

let kitchen = {
    type = "blackout",
    name = "kitchen",

    source = "alert",

    active = {
        input = "kitchen"
    },

    value = "rgb(1.0, 1.0, 1.0)",
    range = [0, 1]
}

in {
    size = 100,

    root = alert,

    output = {
        type = "terminal",
        size = 100,
        waterfall = True
    }
}
