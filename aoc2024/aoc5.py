part_one = True

def is_update_in_order(page_ordering_rules, update):
    for i in range(len(update)):
        condition_met = True

        for j in range(len(update)):
            if i < j:
                if [update[j], update[i]] in page_ordering_rules:
                    condition_met = False
                    break
            elif i > j:
                if [update[i], update[j]] in page_ordering_rules:
                    condition_met = False
                    break

        if not condition_met:
            break

    return condition_met, i, j

def order_update(page_ordering_rules, update):
    while not is_update_in_order(page_ordering_rules, update)[0]:
        i, j = is_update_in_order(page_ordering_rules, update)[1:]
        update[i], update[j] = update[j], update[i]
    return update

with open("input5.txt", "r") as file:
    input = file.read().splitlines()

page_ordering_rules = []
updates = []

start = True
for line in input:
    if line == "":
        start = False
        continue
    if start:
        page_ordering_rules.append([int(x) for x in line.split("|")])
    else:
        updates.append([int(x) for x in line.split(",")])

output = 0
for update in updates:
    is_ordered, i, j = is_update_in_order(page_ordering_rules, update)
    if part_one and is_ordered:
        output += update[len(update) // 2]
    if not part_one and not is_ordered:
        update = order_update(page_ordering_rules, update)
        output += update[len(update) // 2]

print(output)