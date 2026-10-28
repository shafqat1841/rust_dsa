enum Kind {
    Byte(u8),
    Sequence(Vec<Node>),
    Repeat {
        count: usize,
        child: Box<Node>,
    },
}

struct Node {
    kind: Kind,
    len: usize,
}

// Parse one sequence. Stop before its closing ']'.
fn parse_sequence(bytes: &[u8], i: &mut usize) -> Node {
    let mut nodes = Vec::new();
    let mut len = 0usize;

    while *i < bytes.len() && bytes[*i] != b']' {
        if bytes[*i].is_ascii_digit() {
            // Parse the entire repetition count.
            let mut count = 0usize;

            while *i < bytes.len() && bytes[*i].is_ascii_digit() {
                count = count * 10
                    + usize::from(bytes[*i] - b'0');
                *i += 1;
            }

            *i += 1; // Skip '['.

            let child = parse_sequence(bytes, i);

            *i += 1; // Skip ']'.

            let node_len = count * child.len;
            len += node_len;

            nodes.push(Node {
                kind: Kind::Repeat {
                    count,
                    child: Box::new(child),
                },
                len: node_len,
            });
        } else {
            let byte = bytes[*i];
            *i += 1;
            len += 1;

            nodes.push(Node {
                kind: Kind::Byte(byte),
                len: 1,
            });
        }
    }

    Node {
        kind: Kind::Sequence(nodes),
        len,
    }
}

// Write directly into the final output buffer.
fn write_node(node: &Node, output: &mut [u8], pos: &mut usize) {
    match &node.kind {
        Kind::Byte(byte) => {
            output[*pos] = *byte;
            *pos += 1;
        }

        Kind::Sequence(children) => {
            for child in children {
                write_node(child, output, pos);
            }
        }

        Kind::Repeat { count, child } => {
            let start = *pos;

            // Write the child's decoded output once.
            write_node(child, output, pos);

            let child_len = child.len;

            // Fill the remaining copies directly into the output.
            if child_len > 0 {
                for repetition in 1..*count {
                    let destination = start + repetition * child_len;

                    output.copy_within(
                        start..start + child_len,
                        destination,
                    );
                }
            }

            *pos = start + node.len;
        }
    }
}

pub fn sol3(s: String) -> String {
    let bytes = s.as_bytes();
    let mut index = 0;

    let root = parse_sequence(bytes, &mut index);

    // Allocate the final output exactly once.
    let mut output = vec![0u8; root.len];
    let mut pos = 0;

    write_node(&root, &mut output, &mut pos);

    debug_assert_eq!(pos, output.len());

    String::from_utf8(output).unwrap()
}