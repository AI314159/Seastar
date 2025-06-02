use std::{collections::{HashMap, HashSet}, path::Path};

use crate::io::{Config, DepSpec};

#[derive(Debug, Clone)]
pub enum DepSource {
    Git { repo: String, tag: Option<String> },
    Path(String),
}

#[derive(Debug, Clone)]
pub struct Dep {
    pub name: String,
    pub source: DepSource,
}

#[derive(Debug)]
pub struct DepGraph {
    pub nodes: HashMap<String, DepNode>,
}

#[derive(Debug)]
pub struct DepNode {
    pub dep: Dep,
    pub dependencies: Vec<String>, // I might want to use DepNode instead, this was easier
}

pub fn parse_deps(config: &Config) -> Vec<Dep> {
    let mut deps: Vec<Dep> = Vec::new();
    for (dep_name, dep_spec) in &config.dependencies {
        match dep_spec {
            DepSpec::Simple(url) => deps.push(Dep {
                name: dep_name.to_string(),
                source: DepSource::Git {
                    repo: url.to_string(),
                    tag: None,
                },
            }),
            DepSpec::Detailed { git, tag, path } => {
                if let Some(git) = git {
                    deps.push(Dep {
                        name: dep_name.to_string(),
                        source: DepSource::Git {
                            repo: git.to_string(),
                            tag: tag.clone(),
                        },
                    });
                }
                if let Some(path) = path {
                    deps.push(Dep {
                        name: dep_name.to_string(),
                        source: DepSource::Path(path.to_string()),
                    });
                }
            }
        }
    }

    for dep in &deps {
        println!("Dep: {:?}", dep);
    }

    deps
}

pub fn resolve_and_fetch(deps: &[Dep], dep_dir: &str) -> DepGraph {
    let mut graph = DepGraph { nodes: HashMap::new() };
    let mut visited = HashSet::new();

    for dep in deps {
        resolve_dep_recursive(dep, dep_dir, &mut graph, &mut visited);
    }

    graph
}

fn resolve_dep_recursive(
    dep: &Dep,
    dep_dir: &str,
    graph: &mut DepGraph,
    visited: &mut HashSet<String>,
) {
    if visited.contains(&dep.name) {
        return;
    }
    visited.insert(dep.name.clone());

    fetch(dep, dep_dir);

    let dep_config = load_dep_config(dep, dep_dir); 
    println!("{:?}", dep_config);
    let child_deps = parse_deps(&dep_config);

    let mut dependencies = Vec::new();
    for child in &child_deps {
        dependencies.push(child.name.clone());
        resolve_dep_recursive(child, dep_dir, graph, visited);
    }

    graph.nodes.insert(dep.name.clone(), DepNode {
        dep: dep.clone(),
        dependencies,
    });
}

fn load_dep_config(dep: &Dep, dep_dir: &str) -> Config {
    let path = format!("{}/{}/Seastar.toml", dep_dir, dep.name);
    crate::io::load_config(path)
}

fn fetch(dep: &Dep, dep_dir: &str) {
    match &dep.source {
        DepSource::Git { repo, tag } => {
            todo!("Not implemented");
            // TODO: Working
        }
        DepSource::Path(path) => {
            println!("Copying local dependency from: {}", path);
            let src = Path::new(path);
            let dst = Path::new(dep_dir).join(&dep.name);
            match super::fs_copy::copy_dir_recursive(src, &dst) {
                Ok(_) => println!("Copied {} to {:?}", path, dst),
                Err(e) => eprintln!("Failed to copy {}: {}", path, e),
            }
        }
    }
}

impl DepGraph {
    pub fn topological_order(&self) -> Vec<&DepNode> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();

        fn visit<'a>(
            name: &str,
            graph: &'a DepGraph,
            visited: &mut HashSet<String>,
            order: &mut Vec<&'a DepNode>,
        ) {
            if visited.contains(name) {
                return;
            }
            visited.insert(name.to_string());
            if let Some(node) = graph.nodes.get(name) {
                for dep in &node.dependencies {
                    visit(dep, graph, visited, order);
                }
                order.push(node);
            }
        }

        for name in self.nodes.keys() {
            visit(name, self, &mut visited, &mut order);
        }
        order
    }
}
