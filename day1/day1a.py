import sys

class Safe:
    def __init__(self):
        self.dial: list[int] = list(range(100))
        self.password = 0

    def _rotate(self, steps: int):
        shadow_dial = self.dial.copy()
        for _ in range(abs(steps)):
            if steps < 0:
                shadow_dial = shadow_dial[-1:] + shadow_dial[:-1]
            else:
                shadow_dial = shadow_dial[1:] + shadow_dial[:1]
        if shadow_dial[0] == 0:
            self.password += 1
        steps %= len(self.dial)
        self.dial = self.dial[steps:] + self.dial[:steps]
    
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
    safe = Safe()
    safe.rotate_left(50)
    print(f"Starting number: {safe.get_current_number()}")
    password = 0
    input_file = sys.argv[1]
    lines = read_input(input_file)
    print(f"Number of lines: {len(lines)}")
    for line in lines:
        if line.startswith("L"):
            safe.rotate_left(int(line[1:]))
        elif line.startswith("R"):
            safe.rotate_right(int(line[1:]))
        if safe.get_current_number() == 0:
            password += 1
    print(f"Password: {password}")
    print(f"Safe Password: {safe.password}")
