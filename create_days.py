import os

STR_FUNCTIONS = '''
pub fn day{day}_a(input: &String) -> String {{
    drop(input);
    format!("a")
}}

pub fn day{day}_b(input: &String) -> String {{
    drop(input);
    format!("b")
}}
'''

def write_file(file_name: str, data: str):
    if os.path.exists(file_name):
        return
    with open(file_name, "w") as f:
        f.write(data)

if __name__ == "__main__":
    for day in range(1, 26):
        write_file(f"src/day{day}.rs", STR_FUNCTIONS.format(day=day))
        write_file(f"tests/test{day}", "TODO\nTODO")
        write_file(f"inputs/day{day}", "")
