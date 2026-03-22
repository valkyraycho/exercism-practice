def format_ohms(val):
    units = [
        (1_000_000_000, "gigaohms"),
        (1_000_000, "megaohms"),
        (1_000, "kiloohms"),
        (1, "ohms")
    ]
    
    for factor, unit in units:
        if val >= factor:
            return f"{val // factor} {unit}"

    return f"{val} ohms"
            
def label(colors):
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


    return format_ohms(val * 10 ** color_codes.get(colors[2], 0))
    
