class Solution:
    def canCompleteCircuit(self, gas: list[int], cost: list[int]) -> int:
        total = 0      # net gas over all stations
        tank = 0       # current gas in tank
        start = 0      # candidate starting index
        
        for i in range(len(gas)):
            diff = gas[i] - cost[i]
            total += diff
            tank += diff
            
            # If we can't reach the next station from current start
            if tank < 0:
                # Next station is the new candidate start
                start = i + 1
                tank = 0
        
        # Check if overall gas is enough
        return start if total >= 0 else -1
        