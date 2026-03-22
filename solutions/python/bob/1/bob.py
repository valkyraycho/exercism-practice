def response(hey_bob):
    hey_bob = hey_bob.strip()
    if not hey_bob:
        return "Fine. Be that way!"
    if check_letter(hey_bob) and hey_bob.endswith("?") and hey_bob.upper() == hey_bob:
        return "Calm down, I know what I'm doing!"
    if hey_bob.endswith("?"):
        return "Sure."
    if check_letter(hey_bob) and hey_bob.upper() == hey_bob:
        return "Whoa, chill out!" 
    return "Whatever."

def check_letter(string):
    for c in string:
        if c.isalpha():
            return True