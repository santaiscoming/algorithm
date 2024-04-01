from collections import deque


class TopologicalSort:
    def __init__(self, vetex_cnt):
        # 1부터 넣을거임
        self.vertex_cnt = vetex_cnt + 1
        self.graph = [[] for _ in range(self.vertex_cnt)]
        self.indegree = [0 for _ in range(self.vertex_cnt)]

    def create_graph(self, _from, to):
        self.graph[_from].append(to)
        self.indegree[to] += 1

    def sort(self):
        result = []
        queue = deque()

        # 1. 진입 차수가 0인 정점을 q에 넣는다
        for vertex, indegree in enumerate(self.indegree):
            if vertex == 0:
                continue

            if indegree == 0:
                queue.append(vertex)

        # outdegree가 0인 노드들의 간선을 지워준다
        # deque한것은 방문한것
        while queue:
            # queue에 들어간것은 indegree가 0인것들이니 해당 adj를 돌면서 간선을 삭제한다
            # 간선삭제 : indegree -= 1
            zero_indegree_vertex = queue.popleft()
            result.append(zero_indegree_vertex)

            for next_vertex in self.graph[zero_indegree_vertex]:
                self.indegree[next_vertex] -= 1

                if self.indegree[next_vertex] == 0:
                    queue.append(next_vertex)

        return result


topological_sort = TopologicalSort(8)

topological_sort.create_graph(1, 2)
topological_sort.create_graph(1, 3)
topological_sort.create_graph(1, 4)
topological_sort.create_graph(2, 5)
topological_sort.create_graph(3, 7)
topological_sort.create_graph(4, 6)
topological_sort.create_graph(4, 7)
topological_sort.create_graph(5, 7)
topological_sort.create_graph(7, 8)

print(topological_sort.graph)
print(topological_sort.indegree)
print(topological_sort.sort())
