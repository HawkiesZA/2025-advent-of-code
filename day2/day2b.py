import csv


def is_invalid_range(range_to_check: str) -> bool:
    """
    Check if a range is valid. Range is invalid if it has any sequence that repeats at least twice, which makes up the entirety of the id.
    """
    for i in range(1,len(range_to_check)):
        starting_chars = range_to_check[:i]
        if range_to_check.count(starting_chars) >= 2:
            other_chars = [x for x in range_to_check.split(starting_chars) if x]
            if not other_chars:
                print(f"{range_to_check} is invalid becase {starting_chars} repeats.")
                return True
    return False

def example_data() -> str:
    return "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"

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
    # ----- uncomment to test -----
    # data = example_data().split(",")
    # for i in data:
    #     ranges = i.split("-")
    #     for j in range(int(ranges[0]), int(ranges[1]) + 1):
    #         if is_invalid_range(str(j)):
    #             total += j
    
    print(total)