import csv

def is_invalid_range(range_to_check: str) -> bool:
    """
    Check if a range is valid. Range is invalid if it has any sequence that repeats twice, which makes up the entirety of the id.
    """
    midpoint = len(range_to_check) // 2
    first_half = range_to_check[:midpoint]
    second_half = range_to_check[midpoint:]
    if first_half == second_half:
        return True
    return False

if __name__ == "__main__":
    total = 0
    with open("day2/input.csv", "r") as f:
        # read csv
        reader = csv.reader(f)
        for row in reader:
            for col in row:
                ranges = col.split("-")
                for i in range(int(ranges[0]), int(ranges[1]) + 1):
                    if is_invalid_range(str(i)):
                        total += i
    print(total)