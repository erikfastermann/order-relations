# - finde alle relationen von M x M
# - startmenge reflexiv
# - jeweils hinzufügen aller elemente der potenzmenge der nicht reflexiven elemente
# - prüfe antisymmetrisch
# - prüfe transitiv

from itertools import chain, combinations, product

def powerset(l):
    return chain.from_iterable(combinations(l, r) for r in range(len(l)+1))

def is_antisymmetric(l, s):
    for a in l:
        for b in l:
            if (a, b) in s and (b, a) in s and a != b:
                return False
    return True

def is_transitiv(l, s):
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

    for s in powerset(non_reflexiv):
        current = set(reflexiv)
        current.update(s)
        if is_antisymmetric(l, current) and is_transitiv(l, current):
            yield current

l = [1, 2, 3, 4, 5]
r = list(order_relations(l))
# for rr in r:
#     print(rr)

print(max(len(rr) for rr in r))
print(len(r))

# count = sum(1 for _ in order_relations(l))
# print(count)