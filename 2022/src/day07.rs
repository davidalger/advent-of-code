use sscanf::sscanf;
use utils::prelude::*;

parse!(|i| -> HashMap<String, u32> {
    {
        let mut cwd = Vec::new();
        let mut sizes = HashMap::new();
        for l in i.lines() {
            if let Ok(dir) = sscanf!(l, "$ cd {String}") {
                if dir == ".." {
                    cwd.pop();
                } else {
                    cwd.push(dir);
                }
            }
            if let Ok((size, _)) = sscanf!(l, "{u32} {String}") {
                let mut cwd = cwd.clone();
                loop {
                    let dir = cwd.join("-");
                    if let Some(cur) = sizes.get(&dir) {
                        sizes.insert(dir, size + cur);
                    } else {
                        sizes.insert(dir, size);
                    }
                    cwd.pop();
                    if cwd.is_empty() {
                        break;
                    }
                }
            }
        }
        sizes
    }
} as Sizes);

pub fn part1(sizes: Sizes) -> u32 {
    sizes.iter().map(|(_, sz)| *sz).filter(|sz| *sz <= 100000).sum()
}

pub fn part2(sizes: Sizes) -> u32 {
    let need = 30000000 - (70000000 - sizes.get("/").unwrap());
    sizes.iter().map(|(_, size)| *size).filter(|size| need <= *size).sorted().next().unwrap()
}

tests! {
    (part1, "sample", 95437)
    (part1, "puzzle", 1513699)
    (part2, "sample", 24933642)
    (part2, "puzzle", 7991939)
}
