static NIL: u32 = 0;
static INF: u32 = u32::MAX;

/*
Struct que modela un grafo bipartito. Contiene información adicional
que factilita la implementación del algoritmo de Hopcroft-Karp.
*/
pub struct BipartiteGraph {
    m: u32,
    adj: Vec<Vec<u32>>,
    pair_u: Vec<u32>,
    pair_v: Vec<u32>,
    dist: Vec<u32>
}

/*
Implementación de métodos para el struct BipartiteGraph
*/
impl BipartiteGraph {

    /*
    Constructor del struct. Recibe como argumentos las cardinalidades de ambos
    conjuntos de nodos del grafo bipartito.
    */
    pub fn new(m: u32, n: u32) -> Self {
        Self {
            m: m,
            adj: vec![vec![]; (m + 1) as usize],
            pair_u: vec![NIL; (m + 1) as usize],
            pair_v: vec![NIL; (n + 1) as usize],
            dist: vec![0; (m + 1) as usize]
        }
    }

    /*
    Agrega al grafo la arista (u, v). Se asume que los nodos ingresados existen
    dentro del grafo por lo que llamar a este procedimiento con nodos no existentes
    resultaría en pánico.
    */
    pub fn add_edge(&mut self, u: u32, v: u32) -> () {
        self.adj[u as usize].push(v);
    }

    /*
    Implementación de BFS que indica si existe un camino de aumento dentro del grafo.
    */
    fn bfs(&mut self) -> bool {
        let mut q: Vec<u32> = vec![];
        for u in 1..(self.m+1) {
            if self.pair_u[u as usize] == NIL {
                self.dist[u as usize] = 0;
                q.push(u);
            } else {
                self.dist[u as usize] = INF;
            }
        }
        self.dist[NIL as usize] = INF;

        while q.len() > 0 {
            let u = q.remove(0);
            if self.dist[u as usize] < self.dist[NIL as usize] {
                for v in self.adj[u as usize].iter() {
                    if self.dist[self.pair_v[*v as usize] as usize] == INF {
                        self.dist[self.pair_v[*v as usize] as usize] = self.dist[u as usize] + 1;
                        q.push(self.pair_v[*v as usize]);
                    }
                }
            }
        }
        self.dist[NIL as usize] != INF
    }

    /*
    Implementación de dfs que indica si existe un camino de aumento que comienza desde el 
    nodo u del grafo.
    */
    fn dfs(&mut self, u: u32) -> bool {
        if u != NIL {
            for v in self.adj[u as usize].clone().iter() {
                if self.dist[self.pair_v[*v as usize] as usize] == self.dist[u as usize] + 1 {
                    if self.dfs(self.pair_v[*v as usize]) {
                        self.pair_u[u as usize] = *v;
                        self.pair_v[*v as usize] = u;
                        return true;
                    }
                }
            }
            self.dist[u as usize] = INF;
            return false;
        }
        true
    }

    /*
    Implementación del algoritmo de Hopcroft-Karp para encontrar el máximo apareamiento dentro de
    un grafo bipartito. Retorna el número de aristas que contiene el apareamiento calculado por
    el algoritmo.
    */
    pub fn hopcroft_karp(&mut self) -> u32 {
        let mut result: u32 = 0;

        while self.bfs() {
            for u in 1..(self.m+1) {
                if self.pair_u[u as usize] == NIL && self.dfs(u) {
                    result += 1;
                }
            }
        }

        result

    }
}