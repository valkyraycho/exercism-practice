def equilateral(sides):
    return 0 not in sides and len(set(sides)) == 1


def isosceles(sides):
    return 0 not in sides and (len(set(sides)) == 1 or len(set(sides)) == 2) and align_inequality(sides)


def scalene(sides):
    return 0 not in sides and len(set(sides)) == 3 and align_inequality(sides)

def align_inequality(sides):
    return sides[0]+sides[1] >= sides[2] and sides[0]+sides[2] >= sides[1] and sides[2]+sides[1] >= sides[0]