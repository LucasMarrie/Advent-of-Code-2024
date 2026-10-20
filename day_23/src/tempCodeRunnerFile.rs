            if graph.entry(*node1).or_default().contains(node2) || graph.entry(*node2).or_default().contains(node1){
                panic!("freaky");
            }