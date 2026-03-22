def value(colors):
    color_codes = {
        "black": 0,
        "brown": 1,
        "red": 2,
        "orange": 3,
        "yellow": 4,
        "green": 5,
        "blue": 6,
        "violet": 7,
        "grey": 8,
        "white": 9
    }
    val = 0

    for color in colors[:2]:
        val = val * 10 + color_codes.get(color, 0)

    return val
    
