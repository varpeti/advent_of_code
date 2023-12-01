import os
import datetime

STR_FUNCTIONS = '''
pub fn day{day}_a(input: &String) -> String {{
    drop(input.to_owned());
    format!("a")
}}

pub fn day{day}_b(input: &String) -> String {{
    drop(input.to_owned());
    format!("b")
}}
'''

def write_file(file_name: str, data: str):
    if os.path.exists(file_name):
        return
    with open(file_name, "w") as f:
        f.write(data)

if __name__ == "__main__":
    year = datetime.date.today().year
    for day in range(1, 25):
        write_file(f"{year}/src/d{day}.rs", STR_FUNCTIONS.format(day=day))
        write_file(f"{year}/tests/t{day}", "TODO\nTODO")
        write_file(f"{year}/inputs/i{day}", "")
