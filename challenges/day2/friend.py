import csv
import re

def main():        
    part_two()

def parse():
    csvfile = open('input2.txt')
    return next(csv.reader(csvfile, delimiter = ','))

def part_two():
    id_ranges = parse()
    sum_invalid_ids = 0
    for id_range in id_ranges:
        first, last = tuple([int(s) for s in id_range.split("-")])
        for i in range(first, last+1):
            id_str = str(i)
            id_len = len(id_str)
            bisect = id_len // 2
            found = False
            for x in range(1, bisect + 1):
                if not found:
                    finds = re.findall(id_str[0:x], id_str)
                    if(len(finds)*x == id_len):
                        sum_invalid_ids += i
                        found = True
    print(sum_invalid_ids)

if __name__=="__main__":
    main()