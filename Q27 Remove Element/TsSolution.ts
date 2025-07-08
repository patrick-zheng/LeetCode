function removeElement(nums: number[], val: number): number {
    let n: number = nums.length;
    let i: number = 0;

    while (i < n) {
        if (nums[i] == val) {
            nums[i] = nums[n - 1];
            n -= 1;
        } else { i += 1; }
    }
    return n;
};
