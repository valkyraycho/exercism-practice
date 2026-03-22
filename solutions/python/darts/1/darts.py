def score(x, y):
    dis = distance(x, y)
    if dis > 10:
        return 0
    if dis > 5:
        return 1
    if dis > 1:
        return 5
    return 10

def distance(x, y):
    return (x**2 + y**2) ** 0.5