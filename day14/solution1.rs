
fn main() {
    let input = 157901;
    let mut elf_one = 0;
    let mut elf_two = 1;
    let mut recipes = vec![3, 7];

    while recipes.len() < input + 10 {
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

    for num in &recipes[input..input+10] {
        print!("{}", num);
    }
    println!("");
}

