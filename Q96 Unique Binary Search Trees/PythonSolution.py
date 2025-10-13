class Solution:
    def numTrees(self, n: int) -> int:
        G = [0] * (n + 1)
        G[0] = G[1] = 1
        for nodes in range(2, n + 1):
            total = 0
            for root in range(1, nodes + 1):
                total += G[root - 1] * G[nodes - root]
            G[nodes] = total
        return G[n]
    