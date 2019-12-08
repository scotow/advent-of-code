use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<'a> {
    source: Option<&'a Node<'a>>,
    around: Vec<Node<'a>>
}

impl<'a> Node<'a> {
    pub fn build_tree(s: &str) -> Node {
        let mut global: HashMap<u32, &Node> = HashMap::new();
        let root = Node{ source: None, around: Vec::new() };
        global.insert(0x434F4D, &root);

        let mut orphans: HashMap<u32, Node> = HashMap::new();

        s.lines()
            .map(|l| l.as_bytes())
            .map(|e| ((e[0] as u32) << 16 | (e[1] as u32) << 8 | (e[2] as u32), (e[4] as u32) << 16 | (e[5] as u32) << 8 | (e[6] as u32)))
            .for_each(|(src, obj)| {
                // let obj_n = Node { around: Vec::new() };
                // if global.contains_key(&src) {
                //     global.get(&src).unwrap().around.push(obj_n);
                // } else {
                //     let src_n = Node { around: vec!(obj_n) };
                //     global.insert(src, &src_n);
                // }
                // global.insert(obj, &obj_n);

                if global.contains_key(&src) {
                    let src_node = *global.get_mut(&src).unwrap();
                    let obj_node = Node{ source: Some(src_node), around: Vec::new() };
                    src_node.around.push(obj_node);
                    global.insert(obj, &obj_node);
                } else if orphans.contains_key(&src) {
                    let obj_node = Node{ source: None, around: Vec::new() };
                    let src_node = Node{ source: None, around: vec!(obj_node) };
                    obj_node.source = Some(&src_node);
                    orphans.insert(src, src_node);
                }
            });

        root
    }
}