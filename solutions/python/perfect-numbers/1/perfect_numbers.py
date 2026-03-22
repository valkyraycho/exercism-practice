def classify(number):
    """ A perfect number equals the sum of its positive divisors.

    :param number: int a positive integer
    :return: str the classification of the input integer
    """
    if number < 1:
        raise ValueError("Classification is only possible for positive integers.")

    sum_factors = find_sum_factors(number)
    return "perfect" if sum_factors == number else \
            "abundant" if sum_factors > number else \
            "deficient"
    
def find_sum_factors(number):
    result = set()
    for i in range(1, int(number ** 0.5) + 1):
        if number%i == 0:
            result.add(i)
            result.add(number//i)
    return sum(result) - number