
print("start")
_vector = []
_counter = {}
_hashinside = {}

from random import randint

total = 100_000_000
for i in range(total):
    secret_number = randint(1, 101)
    modifier = _hashinside.get(secret_number, None)
    if modifier:
        modifier = {
            "freq": modifier["freq"] + 1,
            "prob": (modifier["freq"] + 1) / total
        }
    else:
        modifier = {
            "freq": 0,
            "prob": 0.0
        }

    _hashinside[secret_number] = modifier

    _counter[secret_number] = _counter.get(secret_number, 0) + 1

    _vector.append(secret_number)


for k, v in _hashinside.items():
    print(k, v)
print("done")




