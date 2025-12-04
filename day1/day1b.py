import sys

class CountingSafe:
    def __init__(self):
        self.dial: list[int] = list(range(100))
        self.password = 0

    def _rotate(self, steps: int):
        # every time the dial goes through the number 0, the password is incremented
        for _ in range(abs(steps)):
            if steps < 0:
                self.dial = self.dial[-1:] + self.dial[:-1]
            else:
                self.dial = self.dial[1:] + self.dial[:1]
            if self.dial[0] == 0:
                self.password += 1
    
    def rotate_left(self, steps: int):
        self._rotate(-steps)
    
    def rotate_right(self, steps: int):
        self._rotate(steps)

    def get_current_number(self) -> int:
        return self.dial[0]

def read_input(input_file: str) -> list[str]:
    with open(input_file, "r") as f:
        return f.read().splitlines()

if __name__ == "__main__":
    safe = CountingSafe()
    safe.rotate_left(50)
    print(f"Starting number: {safe.get_current_number()}")
    input_file = sys.argv[1]
    lines = read_input(input_file)
    print(f"Number of lines: {len(lines)}")
    for line in lines:
        if line.startswith("L"):
            safe.rotate_left(int(line[1:]))
        elif line.startswith("R"):
            safe.rotate_right(int(line[1:]))
    print(f"Password: {safe.password}")