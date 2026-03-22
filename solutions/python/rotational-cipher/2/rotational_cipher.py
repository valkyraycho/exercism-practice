def rotate(text, key):
    def shift_char(c):
        if c.isalpha():
            start = ord("A") if c.isupper() else ord("a")
            return chr(start + (ord(c)+key-start) % 26)
        return c
    return "".join([shift_char(c) for c in text])
            
