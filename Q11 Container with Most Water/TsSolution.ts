function maxArea(height: number[]): number {
    let left = 0;
    let right = height.length - 1;
    let maxVolume = 0;

    while (left < right) {
        const width = right - left;
        if (height[left] < height[right]) {
            maxVolume = Math.max(maxVolume, height[left] * width);
            left++;
        } else {
            maxVolume = Math.max(maxVolume, height[right] * width);
            right--;
        }
    }

    return maxVolume;
};