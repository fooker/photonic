let alert = {
    type = "alert",
    name = "alert",
    hue = 0.0,
    block = 1,
    speed = 1.0
}

in {
    size = 100,

    root = alert,

    output = {
        type = "terminal",
        waterfall = True
    }
}
