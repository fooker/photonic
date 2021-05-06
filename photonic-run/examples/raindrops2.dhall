
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
                ["hsl(245.31, 0.5, 0.5)", "hsl(333.47, 0.7, 0.5)"],
                ["hsl(0.0, 0.45, 0.5)", "hsl(17.5, 0.55, 0.5)"],
                ["hsl(187.5, 0.25, 0.5)", "hsl(223.92, 0.5, 0.5)"]
            ]
        }
    }
}

let brightness = {
    type = "brightness",
    name = "brightness",
    brightness = {
      input = "brightness",
      initial = 1.0
    },
    source = raindrops
}

let blackout = {
    name = "blackout",
    type = "blackout",
    active = {
        input = "blackout",
        initial = False
    },
    source = brightness
}

in {
    size = 120,
    root = blackout,
    output = {
        type = "console",
        waterfall = True
    }
}