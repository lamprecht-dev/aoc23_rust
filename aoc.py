import sys
import requests
import os
from datetime import date
import browser_cookie3
import shutil

### WRITEN BY antooro - https://github.com/antooro/advent-of-code-2019/blob/master/startDay.py

# Get cookies from the browser
cj = browser_cookie3.firefox()
if not (".adventofcode.com" in str(cj)):
    cj = browser_cookie3.chrome()

# Get today number of day
day_today = date.today().strftime("%d").lstrip("0")

# If we provide an argument, use it as the desired day. Ex: ./startDay.py 5. Otherwise use day_today
if len(sys.argv) > 1:
    day = int(sys.argv[1])
    if day < 0 or day > 31:
        exit("Day is not valid")
else:
    day = day_today

print(f"Initializing day {day}")

# Copy the template file and rename it
shutil.copyfile(os.path.join("src", "bin", "template.rs"), f"src/bin/day{day}.rs")

# Create the data directory if it doesn't exist
os.makedirs("data", exist_ok=True)

# Download the input data and write it to the data/X.input file
r = requests.get(f"https://adventofcode.com/2023/day/{day}/input", cookies=cj)
with open(os.path.join("data", f"{day}.input"), "w") as f:
    f.write(r.text)

# Create the empty "X.1.test" file in the data directory
with open(os.path.join("data", f"{day}.1.test"), "w"):
    pass  # Creating an empty file