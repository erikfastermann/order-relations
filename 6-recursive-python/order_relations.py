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

def order_relations_recursive(l, prefix, remainder):
    # Need to copy the set, to remove items while iterating.
    for a, b in set(remainder): 
        current = set(prefix)
        current.add((a, b))
        if is_transitive(l, current):
            # assert is_antisymmetric(l, current)
            yield current
            remainder.remove((a, b))
            yield from order_relations_recursive(l, current, remainder.difference([(b, a)]))

def order_relations(l):
    remainder = set((a, b) for (a, b) in product(l, l) if a != b)
    yield set() # Only reflexiv entries
    yield from order_relations_recursive(l, set(), remainder)

l = [1, 2, 3, 4, 5, 6]
# r = list(order_relations(l))
# for rr in r:
#     print(rr)
# print(len(r))

count = sum(1 for _ in order_relations(l))
print(count)