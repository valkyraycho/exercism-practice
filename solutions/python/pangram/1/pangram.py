def is_pangram(sentence):
    import re
    sentence = re.sub('[^a-zA-Z]+', "", sentence).replace(" ", "")
    print(sentence)

    return len(set(sentence.lower())) == 26
