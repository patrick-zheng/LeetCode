function removeDuplicates(nums: number[]): number {
    let write = 0;
    for (const read of nums) {
        if (write === 0 || read !== nums[write - 1]) {
            nums[write] = read;
            write++;
        }
    }
    return write;
};
