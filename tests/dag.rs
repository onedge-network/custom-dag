use std::collections::{
    HashSet,
    HashMap,
};
use custom_dag::{
    Node, 
    Dag,
    collitions::CollidingNode,
};
// randomness is added to the integration tests to assert the validity of nodes that could contain any value within the range of the id type.
// This means that the tests using the random generator are not deterministic. This is why we include them as integration tests.
// It is necessary to ackownledge, then, that a complete formal verification should include all the values of the id type set.
// In favor of performance, probabilistic testing for large number of nodes is prioritized first.
use rand::Rng;

#[test]
fn create_1_000_000_random_nodes_unconnected_DAG() {
    let mut dag = Dag::new();
    for i in 0..1_000_000 {
       dag.insert(Node::new(i, None, None));
    }
    for i in 0..1_000_000 {
        assert!(dag.contains_id(&i));
    }
}

#[test]
fn insert_existing_node_id_does_not_update() {
    type TestType = u32;
    let id: TestType = 0;
    let nodeA = Node::new(id,Some(3),Some(5));
    let nodeB = Node::new(id,Some(42),Some(43));
    let mut dag = Dag::new();
    // Nodes are considered equal by id under the Node<T> type, but they can contain different field values (other than id).
    assert_eq!(nodeA, nodeB);
    assert_ne!(nodeA.left, nodeB.left);
    // The new node was not present in the dag when inserted.
    assert_eq!(dag.insert(nodeA), None);
    // When trying to insert a new Node<T> with the same id, it will return the present node and will deflect the insertion to the collition collection.
    // The returned value of this action is the node already prensent, with the same id.
    let insert_result = dag.insert(nodeB).expect("Wrond type assumption.");
    assert_eq!(insert_result, &nodeA);
    assert_eq!(insert_result.left, Some(3));
    assert_eq!(insert_result.right, Some(5));
    let node_in_dag = dag.get(&id);
    assert_eq!(
        node_in_dag, 
        Some(&Node {
            id,
            left: nodeA.left,
            right: nodeA.right,
        })
    );
    assert_ne!(
        node_in_dag.expect("Wrong value assumption.").left, 
        nodeB.left,
    );
    assert_ne!(
        node_in_dag.expect("Wrong value assumption.").right, 
        nodeB.right,
    );
    // Examine the DAG's collition collection.
    let collitions: &HashSet<CollidingNode<TestType>> = dag.get_collitions(&id).expect("Wrong value assumption.");
    let colliding_node = collitions.get(&CollidingNode::from(nodeB)).expect("Wrong value assumption.");
    // The colliding node corresponds to nodeB.
    assert_eq!(colliding_node, &CollidingNode::from(nodeB));
    // The collitions set contains a CollidingNode that corresponds to nodeB.
    assert!(collitions.contains(&nodeB.into()));
    // The collitions set does not contains nodeA, even if it has the same id, neither a CollitionNode that corresponds to nodeA.
    assert!(!collitions.contains(&nodeA.into()));
    assert!(!collitions.contains(&CollidingNode::from(nodeA)));
    // The fields in the colliding node corresponds to nodeB.
    assert_eq!(colliding_node.id, nodeB.id);
    assert_eq!(colliding_node.left, nodeB.left);
    assert_eq!(colliding_node.right, nodeB.right);
}