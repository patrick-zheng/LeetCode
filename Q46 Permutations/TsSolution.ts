function permute(nums: number[]): number[][] {
    nums.sort((a, b) => a - b);  // Sort to start from the lowest lexicographic permutation
    const result: number[][] = [];

    do {
        result.push([...nums]);
    } while (nextPermutation(nums));

    return result;
}

function nextPermutation(nums: number[]): boolean {
    let i = nums.length - 2;
    while (i >= 0 && nums[i] >= nums[i + 1]) i--;

    if (i < 0) return false;

    let j = nums.length - 1;
    while (nums[j] <= nums[i]) j--;

    [nums[i], nums[j]] = [nums[j], nums[i]];

    reverse(nums, i + 1);
    return true;
}

function reverse(nums: number[], start: number) {
    let end = nums.length - 1;
    while (start < end) {
        [nums[start], nums[end]] = [nums[end], nums[start]];
        start++;
        end--;
    }
}
