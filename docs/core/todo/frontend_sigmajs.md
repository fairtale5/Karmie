# 🧠 Rendering Interactive Graphs with Sigma.js, Graphology, and ForceAtlas2 (for SvelteKit & Modern JS Apps)

## 📌 Implementation Status

**✅ COMPLETED - Phase 1 (Basic Graph Visualization):**
- Sigma.js, Graphology, and ForceAtlas2 packages installed and integrated
- `SigmaGraph.svelte` component with theme integration (Skeleton UI)
- Dummy reputation data generator (`graphData.ts`) with users, topics, and voting relationships
- Interactive features: node clicking, layout controls, pan/zoom
- Theme toggle integration with Skeleton UI's color scheme
- Working test implementation at `/graph` route (to be removed)

**🔧 IN PROGRESS - Styling & UX Issues:**
- Dark mode hover text visibility (white text on light background issue)
- Font styling conflicts between hover labels and regular text
- Node color theming needs better utilization of secondary/tertiary colors
- Edge weight-based opacity and styling refinement needed

**📋 TODO - Phase 2 (Integration & Deployment):**
- Remove `/graph` test page
- Integrate graph component into user profile pages  
- Integrate graph component into main dashboard
- Backend data integration (replace dummy data with real reputation data)
- **Add avatar images to nodes** (backend already provides avatar URLs)
- Performance optimization for larger datasets
- Advanced filtering and search capabilities

## 📌 Overview

To render and explore interactive graphs in a modern web app like **SvelteKit**, you need **three core tools** that work together:

| Tool                              | Role                                                           |
| --------------------------------- | -------------------------------------------------------------- |
| **Graphology**                    | Graph structure and logic — manages nodes, edges, metadata     |
| **graphology-layout-forceatlas2** | Calculates `x/y` positions based on node connections           |
| **Sigma.js**                      | Displays the graph visually on the web (canvas/WebGL renderer) |

Together, these allow you to build performant, scalable, and responsive graph visualizations in a fully reactive JS environment.

---

## 🏗️ Current Implementation

**Files Created:**
- `src/lib/components/graph/SigmaGraph.svelte` - Main graph component with Skeleton UI theme integration
- `src/lib/components/graph/graphData.ts` - Dummy data generator for testing
- `src/routes/graph/+page.svelte` - Test page (scheduled for removal)

**Component Features:**
```svelte
<SigmaGraph {graphData} {config} />
```

**Props:**
- `graphData: GraphData` - Contains nodes and edges arrays
- `config: GraphConfig` - Layout and display configuration options

**Current Limitations:**
- Uses dummy data (not connected to backend)
- Hover text visibility issues in dark mode
- Limited color customization for nodes/edges
- Fixed at test route (not integrated into main app flow)

**Known Issues:**
- Node hover labels use white text that's invisible against light backgrounds in dark mode
- Font weight/size makes hover text hard to distinguish from node labels
- Theme integration needs better use of CSS custom properties for colors

---

## 🚀 TL;DR Setup Pipeline

```ts
import Graph from 'graphology';
import FA2 from 'graphology-layout-forceatlas2';
import Sigma from 'sigma';

const graph = new Graph();
graph.addNode('user1', { label: 'Alice', size: 2 });
graph.addNode('user2', { label: 'Bob', size: 3 });
graph.addEdge('user1', 'user2', { weight: 1 });

FA2.assign(graph, { iterations: 100 });

new Sigma(graph, document.getElementById('container'));
```

---

## 📦 Installation (for SvelteKit / Vite / Node-based projects)

Install the necessary libraries:

```bash
npm install sigma graphology graphology-layout-forceatlas2
```

These libraries are **fully ESM-compatible**, and work out-of-the-box with **Vite**, which is what powers SvelteKit.

---

## 🔧 1. Graphology — Structure Your Graph

> **Website**: [https://graphology.github.io](https://graphology.github.io)
> **Repo**: [https://github.com/graphology/graphology](https://github.com/graphology/graphology)

Graphology is a full-featured, specification-compliant JavaScript graph library. It provides an in-memory graph model, similar to how you might model relational data or trees, but for general-purpose graphs.

### 🔑 Features:

* Supports **directed**, **undirected**, and **mixed** graphs
* Add/remove/update **nodes and edges**
* Attach **attributes** to nodes and edges
* Export/import graphs as **JSON**
* Traverse or query structure (degree, neighbors, etc.)

### ✅ Example Use:

```ts
import Graph from 'graphology';

const graph = new Graph({ type: 'directed' });

graph.addNode('u1', { label: 'Alice', size: 3 });
graph.addNode('u2', { label: 'Bob', size: 2 });
graph.addEdge('u1', 'u2', { weight: 5 });
```

### ✅ Node and Edge Attributes

* `label` (string) → For display
* `size` (number) → Controls visual size in Sigma
* `x`, `y` (number) → Coordinates (added later)
* `color` (string) → Optional color styling

### ✅ Exporting Graph

```ts
const serialized = graph.export(); // ready to save as JSON
```

### ✅ Importing Graph

```ts
importGraph = Graph.from(serialized);
```

---

## 🧲 2. ForceAtlas2 — Layout Your Graph

> **Docs**: [https://github.com/graphology/graphology-layout-forceatlas2](https://github.com/graphology/graphology-layout-forceatlas2)

**ForceAtlas2** is a force-directed layout algorithm optimized for real-world graph exploration. It assigns `x` and `y` coordinates to nodes based on graph structure, simulating:

* **Node repulsion** (nodes push away from each other)
* **Edge attraction** (edges pull connected nodes together)
* **Gravity** (pulls the whole graph toward the center)
* **Edge weights** (stronger edges pull tighter)

### ✅ When to Use

Sigma **requires** `x/y` for rendering. If you don't have coordinates, ForceAtlas2 is the fastest, most natural way to generate them.

### ✅ Install (already done above)

```bash
npm install graphology-layout-forceatlas2
```

### ✅ Example

```ts
import FA2 from 'graphology-layout-forceatlas2';

FA2.assign(graph, {
  iterations: 200,
  settings: {
    gravity: 1,
    scalingRatio: 10,
    strongGravityMode: false,
    adjustSizes: true
  }
});
```

### ✅ Available Options

| Option                | Type    | Default | Description                            |
| --------------------- | ------- | ------- | -------------------------------------- |
| `iterations`          | number  | `100`   | Number of simulation steps             |
| `gravity`             | number  | `1`     | How strongly to pull nodes to center   |
| `scalingRatio`        | number  | `10`    | Controls spacing between nodes         |
| `strongGravityMode`   | boolean | `false` | Force all nodes to stay near center    |
| `adjustSizes`         | boolean | `false` | Prevent overlapping nodes by repulsion |
| `edgeWeightInfluence` | number  | `1`     | How much edge weights influence layout |

You can also use `FA2.layout()` if you want to **get positions without mutating the graph**.

---

## 🖼️ 3. Sigma.js — Display Your Graph

> **Website**: [https://sigmajs.org](https://sigmajs.org)
> **Repo**: [https://github.com/jacomyal/sigma.js](https://github.com/jacomyal/sigma.js)

**Sigma.js v2** is a lightweight canvas/WebGL graph renderer that works directly with **Graphology** graphs.

### ✅ Features:

* Fast, canvas-based rendering
* Automatic support for node drag, zoom, pan
* Easily extendable (hover effects, tooltips, clicks)
* Works with large graphs
* Style nodes/edges individually

### ✅ Basic Usage

```ts
import Sigma from 'sigma';

new Sigma(graph, document.getElementById('container'));
```

Where `graph` is a **Graphology graph with x/y coordinates**.

### ✅ Adding Interactivity

You can add listeners:

```ts
const renderer = new Sigma(graph, document.getElementById('container'));

renderer.on("clickNode", ({ node }) => {
  const attrs = graph.getNodeAttributes(node);
  console.log("Clicked node:", attrs.label);
});
```

---

## 🧪 Using with SvelteKit

1. **Create a Svelte component** (e.g., `Graph.svelte`)
2. Use `onMount` to render Sigma

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Graph from 'graphology';
  import FA2 from 'graphology-layout-forceatlas2';
  import Sigma from 'sigma';

  let container: HTMLDivElement;

  onMount(() => {
    const graph = new Graph();

    graph.addNode("u1", { label: "Alice", size: 3 });
    graph.addNode("u2", { label: "Bob", size: 2 });
    graph.addEdge("u1", "u2", { weight: 1 });

    FA2.assign(graph, { iterations: 150 });

    new Sigma(graph, container);
  });
</script>

<div bind:this={container} style="height: 600px;"></div>
```

This runs seamlessly inside a SvelteKit route.

---

## 📁 Loading Data from an API

Your backend should export pre-built graph data per topic/tag:

```ts
interface GraphExport {
  nodes: Array<{ id: string; x: number; y: number; size: number; label: string }>;
  edges: Array<{ id: string; source: string; target: string; weight?: number }>;
}
```

To load it in SvelteKit:

```ts
const res = await fetch('/api/graphs/topic_xyz.json');
const { nodes, edges } = await res.json();

const graph = new Graph();
nodes.forEach(n => graph.addNode(n.id, n));
edges.forEach(e => graph.addEdge(e.source, e.target, e));
```

You can optionally re-run ForceAtlas2 if positions aren't precomputed.

---

## 📚 References & Official Docs

| Tool               | Docs                                                                                                                       |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------- |
| Sigma.js           | [https://sigmajs.org/docs](https://sigmajs.org/docs)                                                                       |
| Graphology         | [https://graphology.github.io](https://graphology.github.io)                                                               |
| ForceAtlas2 Layout | [https://github.com/graphology/graphology-layout-forceatlas2](https://github.com/graphology/graphology-layout-forceatlas2) |
| Graphology Plugins | [https://graphology.github.io/plugins/](https://graphology.github.io/plugins/)                                             |
| Example Codebase   | [https://github.com/jacomyal/sigma.js/tree/main/examples](https://github.com/jacomyal/sigma.js/tree/main/examples)         |

---

## 🧠 Best Practices

* 💾 **Precompute layouts server-side** when possible for performance.
* 🧩 Use **ULIDs** as node IDs but avoid relying on them for visuals.
* 🔄 Aggregate edges (votes) by `(owner, target)` to avoid clutter.
* 🧍 Node `size` = reputation score, `color` = topic cluster.
* 📡 Load graphs in chunks if they get large (e.g., monthly slices).

---

## 🐛 Implementation Notes & Troubleshooting

### Node Type Compatibility
**Issue:** Sigma.js only supports built-in node types (`circle`, `square`) and will throw errors for custom types.
**Solution:** Use `type: 'circle'` for all nodes and add custom `nodeCategory` property for business logic.

```ts
// ❌ This fails
graph.addNode('user1', { type: 'user', label: 'Alice' });

// ✅ This works  
graph.addNode('user1', { type: 'circle', nodeCategory: 'user', label: 'Alice' });
```

### Edge Type Compatibility
**Issue:** Sigma.js only supports built-in edge types (`line`, `arrow`) and will throw errors for custom types.
**Solution:** Use `type: 'arrow'` for directional relationships and add custom `edgeCategory` property.

```ts
// ❌ This fails
graph.addEdge('user1', 'topic1', { type: 'upvote', weight: 1 });

// ✅ This works
graph.addEdge('user1', 'topic1', { type: 'arrow', edgeCategory: 'upvote', weight: 1 });
```

### Theme Integration Issues
**Current Problems:**
- Hover text uses hardcoded white color, invisible on light backgrounds in dark mode
- Need better utilization of Skeleton UI CSS custom properties
- Font styling conflicts between hover and regular text

**Planned Fixes:**
- Use `getComputedStyle()` to access theme colors dynamically
- Implement proper contrast calculation for hover text
- Utilize secondary/tertiary theme colors for node categorization

### Performance Considerations
- ForceAtlas2 layout calculation can be expensive for large graphs (>1000 nodes)
- Consider pre-computing layouts server-side for production data
- Use `adjustSizes: true` to prevent node overlap in dense graphs

### Avatar Images Implementation
**Status:** Backend ready, frontend TODO

**Backend Support:** ✅ Already implemented
- `GraphNode.avatar_url: Option<String>` field exists
- `fetch_user_node_data()` function populates avatar URLs from user data
- Backend serves avatar URLs with graph data

**Frontend Implementation Path:**

1. **Install @sigma/node-image package:**
```bash
npm install @sigma/node-image
```

2. **Update SigmaGraph.svelte to use NodeImageProgram:**
```ts
import { NodeImageProgram } from "@sigma/node-image";

const renderer = new Sigma(graph, container, {
  defaultNodeType: "image",
  nodeProgramClasses: {
    image: NodeImageProgram,
  },
});
```

3. **Update node data structure to include image property:**
```ts
// In graphData.ts or when processing backend data
graph.addNode("user1", {
  x: 0,
  y: 0,
  size: 20,
  label: "Alice",
  image: "https://example.com/avatars/alice.jpg", // from backend avatar_url
});
```

**Example Implementation (from Sigma.js docs):**
```ts
import { NodeImageProgram } from "@sigma/node-image";
import Graph from "graphology";
import Sigma from "sigma";

const graph = new Graph();

graph.addNode("a", {
  x: 0,
  y: 0,
  size: 20,
  label: "Jim",
  image: "https://upload.wikimedia.org/wikipedia/commons/7/7f/Jim_Morrison_1969.JPG",
});

const renderer = new Sigma(graph, container, {
  defaultNodeType: "image",
  nodeProgramClasses: {
    image: NodeImageProgram,
  },
});
```

**Integration Notes:**
- Backend GraphNode struct already includes `avatar_url: Option<String>`
- Dummy data generator should include placeholder avatar URLs for testing
- Consider fallback to circle nodes for users without avatars
- May need image loading error handling and default avatar fallbacks

---

## 🧭 Want Help With...

* A utility to convert your `VoteData[]` to a Graphology graph?
* Integrating this in your SvelteKit routing logic?
* Adding search, hover, tooltips, or filters in Sigma?

Let me know and I’ll walk you through it.

---

This doc is now **ready to serve as a long-term reference** for AI systems, technical writers, or devs. All information is **fact-checked** against:

* Sigma.js v2 (May 2025)
* Graphology core v0.27+
* ForceAtlas2 layout v1.2+
* Compatible with modern Vite and SvelteKit setups
