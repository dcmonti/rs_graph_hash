use std::collections::HashMap;

use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::HandleGraph,
    hashgraph::HashGraph,
    mutablehandlegraph::MutableHandleGraph,
};

pub fn execute(
    graph: &HashGraph,
    start_node: usize,
    start_offset: usize,
    end_node: usize,
    end_offset: usize,
) -> HashGraph {
    let mut sub_graph = HashGraph::new();

    let start_node_id = NodeId::from(start_node);
    let start_handle = sub_graph.create_handle(
        &graph.get_node(&start_node_id).unwrap().sequence[start_offset..],
        start_node_id,
    );

    let end_node_id = NodeId::from(end_node);
    let last_handle = sub_graph.create_handle(
        &graph.get_node(&end_node_id).unwrap().sequence[..=end_offset],
        end_node_id,
    );

    let mut created_handles = HashMap::new();
    created_handles.insert(start_handle.id(), start_handle);
    created_handles.insert(last_handle.id(), last_handle);

    recursive_node_visit(
        graph,
        start_handle,
        last_handle,
        &mut sub_graph,
        &mut created_handles,
    );
    /*
    for (node_id, node) in sub_graph.graph.iter(){
        println!("node_id: {}, node: {:?}", node_id, node);
    }
     */

    sub_graph
}

fn recursive_node_visit(
    graph: &HashGraph,
    current_handle: Handle,
    last_handle: Handle,
    sub_graph: &mut HashGraph,
    created_handles: &mut HashMap<NodeId, Handle>,
) {
    for succ_handle in graph.handle_edges_iter(current_handle, Direction::Right) {
        if succ_handle.id() <= last_handle.id() {
            let new_handle = if !created_handles.contains_key(&succ_handle.id()) {
                sub_graph.create_handle(
                    &graph.get_node(&succ_handle.id()).unwrap().sequence,
                    succ_handle.id(),
                )
            } else {
                created_handles.get(&succ_handle.id()).unwrap().clone()
            };
            created_handles.insert(new_handle.id(), new_handle);
            let edge = Edge::edge_handle(current_handle, new_handle);
            sub_graph.create_edge(&edge);
            recursive_node_visit(graph, new_handle, last_handle, sub_graph, created_handles);
        }
    }
}
