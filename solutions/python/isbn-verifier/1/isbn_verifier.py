def is_valid(isbn):
    count = 0
    isbn = isbn.replace("-", "")
    if len(isbn) != 10: return False
    for i, c in enumerate(isbn):
        if c.isdigit():
            count += int(c) * (10 - i)
        elif i == 9 and c == "X":
            count += 10
        else:
            return False
        print(count)
            
    return not count % 11