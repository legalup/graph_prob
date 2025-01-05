use std::env;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, prelude::*, stdin, stdout, BufReader, BufWriter, Result, Write};
use std::num::NonZeroUsize;
use std::ops::{Bound, RangeBounds};
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::{DefaultHasher, RandomState},
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque,
    },
};

use ::contest_llamas::graph::{
    connectivity::{ConnectivityDirectedGraph, ConnectivityUndirectedGraph},
    graph::{DirectedGraph, UndirectedGraph},
};

const INF: i64 = 0x3f3f3f3f;
const MOD: usize = 1_000_000_007;
static USE_FILE_INPUT: bool = true;
// love this line:
// let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

pub fn doit(nums: &mut Vec<(i32, i32)>) {
    println!("here is nums {:?}", nums);
}

struct GraphProblem {
    m: usize,
    generator: usize,
    mid: usize,
    H: HashSet<usize>,
    iHm1: HashSet<usize>,
    graph: DirectedGraph,
}

impl GraphProblem {
    /// The values of `b` and `c` are always 0 in the provided inputs.
    fn new(m: usize, generator: usize) -> GraphProblem {
        GraphProblem {
            m,
            generator,
            mid: (m - 1) / 2,
            H: HashSet::<usize>::new(),
            iHm1: HashSet::<usize>::new(),
            graph : DirectedGraph::new(m, (m+1)),
        }
    }

    pub fn findnextvert(&mut self) -> Option<usize> {
        if self.H.is_empty() {
            return Some(1usize);
        }

        if self.H.len() == self.mid {
            return None;
        }

        for ii in 1..=self.mid {
            if !self.H.contains(&ii) {
                return Some(ii);
            }
        }

        None
    }

    pub fn add_forward_cycle_directed(&mut self, x: usize) -> usize {
        let y = (x * self.generator) % self.m;
        let z = (y * self.generator) % self.m;
        let w = (z * self.generator) % self.m;

        self.graph.add_edge(x, y);
        self.graph.add_edge(y, x);

        self.graph.add_edge(y - 1, z - 1);
        self.graph.add_edge(z - 1, y - 1);

        self.graph.add_edge(z, w);
        self.graph.add_edge(w, z);

        self.graph.add_edge(w - 1, x - 1);
        self.graph.add_edge(x - 1, w - 1);

        //augment zm
        if x <= self.mid {
            self.H.insert(x);
        };
        if y <= self.mid {
            self.H.insert(y);
        };
        if z <= self.mid {
            self.H.insert(z);
        };
        if w <= self.mid {
            self.H.insert(w);
        };

        if x <= self.mid {
            self.iHm1.insert(x);
        };
        if y <= self.mid {
            self.iHm1.insert(y);
        };
        if z <= self.mid {
            self.iHm1.insert(z);
        };
        if w <= self.mid {
            self.iHm1.insert(w);
        };

        self.H.len()
    }


    pub fn print_conn_comps(&self) {
        let cg = ConnectivityDirectedGraph::new(&self.graph);
        //println!("the number of connected components is {} ", cg.num_cc);
        //println!("the coloring is {:?}", cg.cc);
        let mut freq = HashMap::<usize, Vec<usize>>::new();
        for (idx, ii) in cg.cc.iter().enumerate() {
            freq.entry(*ii).and_modify(|v| v.push(idx)).or_default();
        }

        for key in freq.keys() {
            if freq[&key].len() > 1 {
                println!("component: {:?}", freq[&key]);
            }
        }
    }

    pub fn print_adj_list(&self) {
        let mut domod = |x: usize| -> i32 {
            let mut ix = x as i32;
            if ix > (self.mid as i32) {
                ix -= (self.m as i32)
            };
            ix
        };

        (0..self.m).for_each(|x| {
            print!("for vertex {}:", domod(x));
            for y in self.graph.adj_list(x) {
                print!("{},", domod(y.1))
            }
            //{ let z = y.1; let mut iz = z as i32; if iz>14 {iz -= (moddy as i32); print!("{},",iz)};}
            println!("---")
        });
    }

    pub fn solve(&mut self)
    {
        while let Some(v) = self.findnextvert() {
            self.add_forward_cycle_directed(v);
            println!("here is v {} and H {:?}", v, self.H);
            self.print_conn_comps();
            println!("---------------");
        }
    }
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let m = 29usize;
    let i = 12usize;
    let mut testy  = GraphProblem::new(m,i);

    testy.solve();
    

    Ok(())
}
