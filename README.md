# Graph util

Repo containing a bunch of methods and algorithms to work with graphs and solve
graph problems.
Should support both directed and undirected graphs. Should also support weighted
edges.

# Graph visualization

Build a graph visualisation tool, that renders graphs as objects in the
$`\mathbb{R}^3`$. For that, assign each vertex a $``3`$-dimensional coordinate,
the edges are the most direct connection between two vertices.

Of course one goal is to make the whole thing look good. You cannot just put
the vertices anywhere in the space, that would just create a huge mess.
Connected vertices should somewhat close. However, Vertices with a high degree
should be somewhat distant to make the graph appear less crowded.

To achieve these goals we utilize the laws of physics. We assign each vertex
an electrical charge $`c`$ based on its degree. The edges are simulated via connecting
rods. The length $`len`$ of the rod is based on the degree of the two connected vertices
$`a`$ and $`b`$. For now let's assume that $`len = min\{deg(a), deg(b)\}`$.
To be a little more flexible we introduce two more functions
$`\phi: \mathbb{R} \rightarrow \mathbb{R}`$ and $`\psi: \mathbb{R} \rightarrow \mathbb{R}`$.
These are our value-correcting functions which adjust the electrical charge $`c`$
and the length of the rod $`len`$ for a better visualization. But for the MVP
let's just assume $`\phi = \psi = id`$.

This should space out the nodes fairly evenly while still maintaining connections
between connected vertices.
The system can be rendered, once it's reached an equillibrium, that is if

$`\forall v \in V: 0 = F_v = \sum_{u \in V, u \neq v} F_e^{vu} + F_m^{vu}`$

where
$`F_e^{vu}`$ is the electrical force between $`v`$ and $`u`$ and $`F_m^{vu}`$ is the
mechanical one.
Let $`k_e`$ be the $`Coulomb`$-Constant and $`k_s`$ the rigidity constant of the rod,
$`q_v`$ the charge and $`r_v`$ the position of $`v`$.
Let $`\hat{r}_{vu} = \frac{r_u - r_v}{|r_u - r_v|}`$ the unit vector of $`r_{vu}`$
and $`d_{vu} = ||r_{vu}||`$ the distance between $`v`$ and $`u`$.

Then
$`F_e^{vu} = k_e\frac{q_vq_u}{||r_u - r_v||^2}\hat{r}_{vu}`$
and
$`F_m^{vu} = -k_s(d'_{vu} - d_{vu})\hat{r}_{vu}`$
where $`d_{vu}`$ is the old distance between $`v`$ and $`u`$, while $`d'_{vu}`$ is the
new distance between them.
With that one can calculate $`F_v`$ and once $`\forall v \in V: F_v = 0`$ the system
has reached an equillibrium and can be rendered.

To actually render the whole thing project it down into the $`\mathbb{R}^2`$.
One could also transform it's basis by a tiny amount every 10ms before projecting
it down into $`\mathbb{R}^2`$ to create a spinning image.

## TODOs

- What's the most optimal way to calculate $`F_v = 0`$?
- Can the calculation be simplified, if we get rid of $`k_s`$ and make the rods
fixed-length?
- For now this only supports connected graphs -> What about disconnected ones?
- Test different values for $`\phi`$ and $`\psi`$, e.g. $`exp, log, ...`$
- What's the fastest way to create a base-transform?
- Is there a more optimal way to create a spinning image?
- How to minimize data loss on 2d-projection?
- Render it in 2d from the start? E.g. use laws of physics for $`\mathbb{R}^2`$?
What about planar (sub-)graphs?

# Algorithms to implement

- Dijkstra
- Kruskal
- Prim
- TSP (Brute-Force + Approximation methods)
- Clique via Bron-KerboschA
- Cycle-detection algorithm
- Steiner-trees and Steiner-forests!
