use rustc_hash::FxHashMap;

struct Edge {
    id: Id,
    dist: u32,
}

type Id = (usize, usize);
type Graph = FxHashMap<Id, Vec<Edge>>;

utils::parse_grid!(u8);

pub fn part1(grid: Grid) -> u32 {
    longest_path(
        &parse_graph(&grid, false),
        &mut FxHashMap::default(),
        (1, 0),                                           // second tile of top row
        (grid[grid.len() - 1].len() - 2, grid.len() - 1), // second to last tile of bottom row
    )
}

pub fn part2(grid: Grid) -> u32 {
    longest_path(
        &parse_graph(&grid, true),
        &mut FxHashMap::default(),
        (1, 0),                                           // second tile of top row
        (grid[grid.len() - 1].len() - 2, grid.len() - 1), // second to last tile of bottom row
    )
}

fn parse_graph(grid: &Grid, undirected: bool) -> Graph {
    let mut graph = FxHashMap::default();
    let mut keys = Vec::new();

    // parse nodes
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if grid[y][x] != b'#' {
                keys.push((x, y));
                graph.insert((x, y), {
                    // adjacent edges
                    let mut edges = vec![];
                    if x > 0 && grid[y][x - 1] != b'#' && (undirected || grid[y][x - 1] != b'>') {
                        edges.push(Edge { id: (x - 1, y), dist: 1 });
                    }
                    if x + 1 < grid[0].len()
                        && grid[y][x + 1] != b'#'
                        && (undirected || grid[y][x + 1] != b'<')
                    {
                        edges.push(Edge { id: (x + 1, y), dist: 1 });
                    }
                    if y > 0 && grid[y - 1][x] != b'#' && (undirected || grid[y - 1][x] != b'v') {
                        edges.push(Edge { id: (x, y - 1), dist: 1 });
                    }
                    if y + 1 < grid.len()
                        && grid[y + 1][x] != b'#'
                        && (undirected || grid[y + 1][x] != b'^')
                    {
                        edges.push(Edge { id: (x, y + 1), dist: 1 });
                    }
                    edges
                });
            }
        }
    }

    // contract paths (optimize cyclic graph found in part2)
    if undirected {
        for key in keys {
            if graph[&key].len() == 2 {
                let id0 = graph[&key][0].id;
                let id1 = graph[&key][1].id;
                let dist = graph[&key][0].dist + graph[&key][1].dist;

                for (a, b) in [(id0, id1), (id1, id0)] {
                    let a = graph.get_mut(&a).unwrap();
                    a.retain(|x| x.id != key);
                    a.push(Edge { id: b, dist });
                }

                graph.remove(&key);
            }
        }
    }

    graph
}

// modified dfs
fn longest_path(graph: &Graph, visited: &mut FxHashMap<Id, u32>, point: Id, dest: Id) -> u32 {
    if point == dest {
        visited.values().sum()
    } else {
        let mut dist = 0;
        for next in graph.get(&point).unwrap() {
            if !visited.contains_key(&next.id) && next.id != point {
                visited.insert(point, next.dist);

                let value = longest_path(graph, visited, next.id, dest);
                if value > dist {
                    dist = value;
                }

                visited.remove(&next.id);
            }
        }
        dist
    }
}

utils::tests! {
    (part1, "sample", 94)
    (part1, "puzzle", 2362)
    (part2, "sample", 154)
    (part2, "puzzle", 6538)
}
