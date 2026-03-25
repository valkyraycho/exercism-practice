pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Default::default(),
                edges: Default::default(),
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.name() == name)
        }
    }

    pub mod graph_items {
        macro_rules! impl_attrs {
            ($t: ty) => {
                impl $t {
                    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                        self.attrs
                            .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                        self
                    }

                    pub fn attr(&self, key: &str) -> Option<&str> {
                        self.attrs.get(key).map(|v| v.as_str())
                    }
                }
            };
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn name(&self) -> &str {
                    &self.name
                }
            }
            impl_attrs!(Node);
        }

        pub mod edge {

            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: Default::default(),
                    }
                }
            }
            impl_attrs!(Edge);
        }
    }
}
