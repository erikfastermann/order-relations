from itertools import product

def is_antisymmetric(l, s):
    for a in l:
        for b in l:
            if a == b:
                continue
            if (a, b) in s and (b, a) in s:
                return False
    return True

def is_transitive(l, s):
    for a in l:
        for b in l:
            for c in l:
                if (a, b) in s and (b, c) in s and (a, c) not in s:
                    return False
    return True

def order_relations(l):
    assert len(set(l)) == len(l)

    reflexiv = [(a, a) for a in l]
    non_reflexiv = [(a, b) for (a, b) in list(product(l, l)) if a != b]

    yield set(reflexiv)
    found = list([set()])
    current = list()
    while len(found) > 0:
        for prefix in found:
            for (a, b) in non_reflexiv:
                if (a, b) in prefix:
                    continue

                r = prefix.union([(a, b)])
                if r in current:
                    continue

                if is_antisymmetric(l, r) and is_transitive(l, r):
                    yield r.union(reflexiv)
                    current.append(r)
        found = current
        current = []

l = [1, 2, 3, 4]
# r = list(order_relations(l))
# for rr in r:
#     print(rr)
# print(len(r))

count = sum(1 for _ in order_relations(l))
print(count)