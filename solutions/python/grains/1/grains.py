def square(number):
    try:
        if number < 1 or number > 64:
            raise ValueError("square must be between 1 and 64")
        return 2 ** (number - 1)
    except ValueError as e:
        raise ValueError("square must be between 1 and 64") from e


def total():
    count = 0
    for i in range(1, 65):
        count += square(i)
    return count
