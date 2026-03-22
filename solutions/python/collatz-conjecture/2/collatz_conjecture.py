def steps(number):
    if number < 1:
        raise ValueError("Only positive integers are allowed")
    if number == 1:
        return 0

    # Create a dictionary to store the number of steps for each number
    memo = {1: 0}

    def calculate_steps(n):
        # Use a stack to simulate the recursive calls
        stack = []
        while n not in memo:
            stack.append(n)
            if n % 2 == 0:
                n = n // 2
            else:
                n = 3 * n + 1

        # Now we know the steps to reach 1 from n
        result = memo[n]

        # Backtrack and fill the memo dictionary
        while stack:
            n = stack.pop()
            result += 1
            memo[n] = result

        return memo[number]

    return calculate_steps(number)
