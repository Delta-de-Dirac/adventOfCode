def proc1(z, w):
    if ((z % 26) + 12 != w):
        z = z * 26 + (w + 4)
    return z


def proc2(z, w):
    if ((z % 26) + 11 != w):
        z = z * 26 + (w + 11)
    return z


def proc14(z, w):
    if (((z % 26) - 5) != w):
        z = z // 26
        z = z * 26 + (w + 14)
        return z
    z = z // 26
    return z


for w in range(1, 10):
    for z in range(0, 26):
        print(f"{z}\t {w}\t=>\t{proc14(z,w)}")
