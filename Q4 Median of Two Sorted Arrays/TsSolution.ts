function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
  if (nums1.length > nums2.length) {
    return findMedianSortedArrays(nums2, nums1);
  }

  const m = nums1.length;
  const n = nums2.length;
  let left = 0, right = m;

  while (left <= right) {
    const partitionA = Math.floor((left + right) / 2);
    const partitionB = Math.floor((m + n + 1) / 2) - partitionA;

    const maxLeftA = partitionA === 0 ? -Infinity : nums1[partitionA - 1];
    const minRightA = partitionA === m ? Infinity : nums1[partitionA];
    const maxLeftB = partitionB === 0 ? -Infinity : nums2[partitionB - 1];
    const minRightB = partitionB === n ? Infinity : nums2[partitionB];

    if (maxLeftA <= minRightB && maxLeftB <= minRightA) {
      if ((m + n) % 2 === 0) {
        return (Math.max(maxLeftA, maxLeftB) + Math.min(minRightA, minRightB)) / 2;
      } else {
        return Math.max(maxLeftA, maxLeftB);
      }
    } else if (maxLeftA > minRightB) {
      right = partitionA - 1;
    } else {
      left = partitionA + 1;
    }
  }

  throw new Error("Input arrays are not sorted or have invalid lengths.");
};
