function searchRange(nums: number[], target: number): number[] {
    const findLeftIndex = (nums: number[], target: number): number => {
        let left = 0, right = nums.length - 1;
        while (left <= right) {
            const mid = Math.floor((left + right) / 2);
            if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return left;
    };

    const findRightIndex = (nums: number[], target: number): number => {
        let left = 0, right = nums.length - 1;
        while (left <= right) {
            const mid = Math.floor((left + right) / 2);
            if (nums[mid] <= target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return right;
    };

    const leftIndex = findLeftIndex(nums, target);
    const rightIndex = findRightIndex(nums, target);

    if (leftIndex <= rightIndex && nums[leftIndex] === target && nums[rightIndex] === target) {
        return [leftIndex, rightIndex];
    }

    return [-1, -1];
};