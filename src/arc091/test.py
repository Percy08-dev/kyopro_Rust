def f(x, t):
    return (100+t)*x//100

def main():
    t = 49
    m = 100
    data = {f(i, t) for i in range(m)}

    res = [i for i in range(m) if not(i in data)]
    print(res)


if __name__ == "__main__":
    main()