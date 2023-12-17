# input = open("../data/examples/15-1.txt").read()
input = open("../data/inputs/15.txt").read()


def hash(s):
    acc = 0
    for x in s:
        acc = (acc + ord(x)) * 17 % 256
    return acc


# total = sum((hash(s) for s in input.strip().split(",")))
# print(total)
boxes = [{} for i in range(256)]

for op in input.strip().split(","):
    if "-" in op:
        label = op[:-1]
        index = hash(label)
        box = boxes[index]
        try:
            del box[label]
        except KeyError:
            pass
    else:
        label, focal = op.split("=")
        index = hash(label)
        box = boxes[index]
        box[label] = int(focal)

total = 0
for i, box in enumerate(boxes):
    for slot, focal in enumerate(box.values()):
        total += (i + 1) * (slot + 1) * focal
print(total)
