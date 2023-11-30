use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Dir {
        nodes: HashMap<String, *mut Node>,
        parent: *mut Node,
    },
    File {
        size: usize,
    },
}

impl Node {
    pub fn size_list(&self) -> (usize, Vec<usize>) {
        let mut size_list = Vec::new();
        let mut size = 0;

        if let Node::File { size } = self {
            (*size, size_list)
        } else if let Node::Dir {
            nodes,
            parent: _,
        } = self
        {
            unsafe {
                for node in nodes.values() {
                    let (temp_size, temp_list) = (**node).size_list();
                    size += temp_size;
                    size_list.extend(temp_list);
                }

                size_list.push(size);

                (size, size_list)
            }
        } else {
            unreachable!()
        }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let file_system = Box::into_raw(Box::new(Node::Dir {
        nodes: HashMap::new(),
        parent: std::ptr::null_mut(),
    }));

    let mut cur_dir = file_system;

    for line in lines {
        if line.starts_with("$ cd ") {
            let dir_name = &line[5..];

            if dir_name == "/" {
                cur_dir = file_system;
                continue;
            }

            if dir_name == ".." {
                unsafe {
                    if let Node::Dir {
                        nodes: _,
                        parent,
                    } = *cur_dir
                    {
                        cur_dir = parent;
                    }
                }
                continue;
            }

            unsafe {
                if let Node::Dir {
                    ref nodes,
                    parent: _,
                } = *cur_dir
                {
                    cur_dir = nodes[dir_name];
                }
            }

            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        // maybe ls results

        if line.starts_with("dir ") {
            let dir_name = &line[4..];
            let dir = Box::into_raw(Box::new(Node::Dir {
                nodes: HashMap::new(),
                parent: cur_dir,
            }));

            unsafe {
                if let Node::Dir {
                    ref mut nodes,
                    parent: _,
                } = *cur_dir
                {
                    nodes.insert(dir_name.to_string(), dir);
                }
            }

            continue;
        }

        // files

        let (size, file_name) = line.split_once(' ').unwrap();
        let size = size.parse::<usize>().unwrap();

        let file = Box::into_raw(Box::new(Node::File {
            size,
        }));

        unsafe {
            if let Node::Dir {
                ref mut nodes,
                parent: _,
            } = *cur_dir
            {
                nodes.insert(file_name.to_string(), file);
            }
        }
    }

    let (_, size_list) = unsafe { (*file_system).size_list() };

    let mut sum = 0;

    for size in size_list {
        if size <= 100000 {
            sum += size;
        }
    }

    sum.to_string()
}
