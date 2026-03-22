def is_isogram(string):
    import re
    string = re.sub("[^a-zA-Z]+", "", string).lower()
    print(string)
    return len(string) == len(set(string))
