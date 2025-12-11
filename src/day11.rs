use std::collections::{HashMap, HashSet};

pub fn a(input: &str) -> u64 {
    let mut nodes = HashMap::new();
    for line in input.trim().lines() {
        let (node, others) = line.split_once(": ").unwrap();
        let others: Vec<_> = others.split(' ').collect();
        nodes.insert(node, others);
    }

    let mut worklist = vec![];
    worklist.push(nodes.get("you").unwrap().as_slice());

    let mut num_connections = 0;
    while let Some(list) = worklist.last_mut() {
        let Some(elem) = list.split_off_last() else {
            worklist.pop();
            continue;
        };
        if *elem == "out" {
            num_connections += 1;
        } else {
            worklist.push(nodes.get(elem).unwrap());
        }
    }
    num_connections
}

pub fn b(input: &str) -> u64 {
    #[derive(Debug, Default)]
    struct NodeInfo {
        num_parents: u64,

        num_paths: u64,
        num_fft_paths: u64,
        num_dac_paths: u64,
        num_both_paths: u64,
    }

    // pretty much <https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm>

    let mut map = HashMap::new();
    let mut nodes: HashMap<&str, NodeInfo> = HashMap::new();
    for line in input.trim().lines() {
        let (node, others) = line.split_once(": ").unwrap();
        let children: Vec<_> = others.split(' ').collect();

        for child in &children {
            nodes.entry(child).or_default().num_parents += 1;
        }
        map.insert(node, children);
        nodes.entry(node).or_default();
    }

    let mut queue: Vec<_> = nodes
        .iter_mut()
        .filter_map(|(name, node)| {
            if node.num_parents == 0 {
                node.num_paths = 1;
                Some(*name)
            } else {
                None
            }
        })
        .collect();

    while let Some(name) = queue.pop() {
        let node = nodes.get_mut(name).unwrap();

        if name == "dac" {
            node.num_dac_paths += node.num_paths;
            node.num_both_paths += node.num_fft_paths;
        } else if name == "fft" {
            node.num_fft_paths += node.num_paths;
            node.num_both_paths += node.num_dac_paths;
        }
        let num_paths = node.num_paths;
        let num_dac_paths = node.num_dac_paths;
        let num_fft_paths = node.num_fft_paths;
        let num_both_paths = node.num_both_paths;

        let Some(children) = map.get(name) else {
            continue;
        };
        for child in children {
            let childnode = nodes.get_mut(child).unwrap();

            childnode.num_paths += num_paths;
            childnode.num_dac_paths += num_dac_paths;
            childnode.num_fft_paths += num_fft_paths;
            childnode.num_both_paths += num_both_paths;

            childnode.num_parents -= 1;
            if childnode.num_parents == 0 {
                queue.push(child);
            }
        }
    }

    nodes.get("out").unwrap().num_both_paths
}

#[test]
fn test() {
    assert_eq!(
        a(r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#),
        5
    );
    assert_eq!(
        b(r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#),
        2
    );
}
