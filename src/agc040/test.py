x = "<>>><<><<<<<>>><"
x = list(x)
# y = "0, 3, 2, 1, 0, 2, 1, 0, 7, 6, 5, 4, 3, 2, 1, 0".split(", ")
y = input()[1:-1].split(", ")

for i in range(len(x + y)):
    if i % 2 == 0 :
        print(y[i//2], end = "")
    else:
        print(x[i//2], end = "")
    
    """
    if len(x) <= (i//2) + 1:
        print(y[(i+1)//2:])
        break
    elif len(y) <= (i//2) + 1:
        print(x[(i+1)//2:])
        break
    """
