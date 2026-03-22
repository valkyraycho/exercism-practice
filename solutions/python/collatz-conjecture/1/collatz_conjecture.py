def steps(number, memo=None):
    if number < 1:
        raise ValueError("Only positive integers are allowed")
    if number == 1:
        return 0

    memo = {}

    if number in memo:
        return memo[number]

    if number % 2 == 0:
        result = 1 + steps(number // 2, memo)
    else:
        result = 1 + steps(number * 3 + 1, memo)

    memo[number] = result
    return result
    