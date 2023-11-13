def sum_as_string1(a, b):
    return str(a + b)


def sum_as_string2(a: int, b: int) -> str:
    return str(a + b)


sum_as_string1(1, "2")
sum_as_string2(1, "2")
