// Tree example in Rust by David Atkinson
// Can be used multithreaded

pub mod tree;
use tree::TreeNodeRef;

fn print_tree(n: &TreeNodeRef::<String>, depth: usize) {
    for _ in 0..depth {
        print!("    ");
    }
    if n.has_children() {
        println!("{} has children:", n.val());
        n.children().iter().for_each(|n| print_tree(&n.clone(), depth+1));
    } else {
        println!("{} has no children.", n.val());
    }
}
fn main() {
    // Create a new root node
    let mut root_node = TreeNodeRef::<String>::new("Richard Stark".to_string());

    // Add some children to the root node
    let mut ned = root_node.push(String::from("Ned Stark"));
    root_node.push(String::from("Brandon Stark"));
    root_node.push(String::from("Benjen Stark"));
    root_node.push(String::from("Lyanna Stark"));

    // Ned had many children
    let neds_kids = vec![ "Robb Stark", "Jon Snow", "Sansa Stark", "Arya Stark", "Bran Stark", "Rickon Stark" ].iter().map(|s| s.to_string()).collect();
    ned.push_children(&neds_kids);

    // Print the family tree
    print_tree(&root_node, 0);

    // Whoops, it turns out Jon Snow was Lyanna's child!
    
    // Get a reference to Jon Snow and Lyanna
    let mut jon = match ned.find("Jon Snow".to_string()) {
        Some(n) => n,
        None => panic!("Jon Snow could not be found!"),
    };

    let mut lyanna = match root_node.find("Lyanna Stark".to_string()) {
        Some(n) => n,
        None => panic!("Lyanna Stark could not be found!"),
    };

    // Move Jon Snow from Ned Stark to Lyanna Stark
    jon.move_to(&mut lyanna);

    // Change Jon's name
    jon.set_val(String::from("Jon Targaryen"));

    // Print the family tree, again!
    print_tree(&root_node, 0); 
}
