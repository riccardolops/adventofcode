import numpy as np

conditional_statements = True

with open("input3.txt", "r") as file:
    input = file.read()

if conditional_statements:
    preprocess = input.split("don't()")
    pre_processed = preprocess[0]
    for i in range(1, len(preprocess)):
        temp = preprocess[i].split("do()")
        for j in range(1, len(temp)):
            pre_processed += temp[j]
    sections = pre_processed.split("mul(")
else:
    sections = input.split("mul(")


output = 0
for section in sections:
    first_number = ""
    second_number = ""
    done_first = False
    for char in section:
        if char.isdigit() and not done_first:
            first_number += char
        elif char == "," and first_number != "" and not done_first:
            done_first = True
        elif char.isdigit() and done_first:
            second_number += char
        elif char == ")" and second_number != "":
            #print("first_number: " + first_number)
            #print("second_number: " + second_number)
            output += int(first_number) * int(second_number)
            break
        else:
            #print("Wrong")
            break
    #print(section)
print(output)
