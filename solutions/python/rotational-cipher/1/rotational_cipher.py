def rotate(text, key):
    result = ''
    for c in text:
        if c.isalpha():
            num = ord(c) + key % 26
            if c.isupper() and num > ord("Z"):
                num -= 26
            elif c.islower() and num > ord("z"):
                num -= 26
            result += chr(num)
        else:
            result += c
    return result
            
