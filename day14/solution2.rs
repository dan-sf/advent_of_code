
fn main() {
    let input = [1,5,7,9,0,1];
    let mut elf_one = 0;
    let mut elf_two = 1;
    let mut recipes = vec![3, 7];

    loop {
        if recipes.len() >= input.len() && &recipes[recipes.len()-input.len()..recipes.len()] == input {
            println!("{}", recipes.len()-input.len());
            break;
        }
        if recipes.len() >= input.len()+1 && &recipes[recipes.len()-input.len()-1..recipes.len()-1] == input {
            println!("{}", recipes.len()-input.len()-1);
            break;
        }

        let new_recipe = recipes[elf_one] + recipes[elf_two];
        if new_recipe >= 10 {
            recipes.push(1);
        }
        recipes.push(new_recipe % 10);
        elf_one += recipes[elf_one] + 1;
        elf_one %= recipes.len();
        elf_two += recipes[elf_two] + 1;
        elf_two %= recipes.len();
    }
}

