import string

def extra_time_map():
    return { k: i+1 for i, k in enumerate(string.ascii_uppercase) }

def create_graph():
    graph = {}
    total_nodes = set()
    with open("input.txt") as fh:
        for line in fh:
            node = line.split(' ')[7]
            total_nodes.add(node)
            dependancy = line.split(' ')[1]
            total_nodes.add(dependancy)
            if node in graph:
                graph[node].add(dependancy)
            else:
                graph[node] = {dependancy}
    return graph, total_nodes

def get_runnable_nodes(graph, total_nodes):
    can_run = []
    for n in total_nodes:
        if n not in graph:
            can_run.append(n)
        else:
            if n in graph and graph[n] == set():
                can_run.append(n)
    return can_run

# Remove the nodes once visited
def visit_node(graph, node):
    if node in graph:
        del graph[node]

    for n in graph:
        if node in graph[n]:
            graph[n].remove(node)

def get_instructions():
    output = []
    graph, total_nodes = create_graph()
    while graph:
        runnable_nodes = get_runnable_nodes(graph, total_nodes)
        runnable_nodes.sort()
        output.append(runnable_nodes[0])
        total_nodes.remove(runnable_nodes[0])
        visit_node(graph, runnable_nodes[0])
    return output

if __name__ == '__main__':
    print("Instruction order: {}".format(''.join(get_instructions())))

