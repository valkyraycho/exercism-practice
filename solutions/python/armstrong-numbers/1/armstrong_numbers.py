def is_armstrong_number(number):
    power = len(str(number))
    number_ = number
    count = 0

    while number:
        count += (number % 10) ** power
        number = number // 10
    return count == number_
