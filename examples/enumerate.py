lst = [100, 30, -100, 2000, 50]


for i, v in enumerate(lst):
    print(f"{i}: {v}")


print()


for i, v in enumerate(lst, start=-3):
    print(f"{i}: {v}")
