class Solution:
    def merge(self, intervals: list[list[int]]) -> list[list[int]]:
        if not intervals:
            return []
        
        # Sort intervals by start time
        intervals.sort(key=lambda x: x[0])
        
        merged = []
        start, end = intervals[0]
        
        for i in range(1, len(intervals)):
            curr_start, curr_end = intervals[i]
            
            if curr_start <= end:  
                # Overlap → extend the interval
                end = max(end, curr_end)
            else:
                # No overlap → push finished interval
                merged.append([start, end])
                start, end = curr_start, curr_end
        
        # Append last interval
        merged.append([start, end])
        
        return merged
