// title
// url: https://leetcode.cn/problems/???/

using namespace std;

// 算法实现部分
class Solution {
  public:
};

// ==================== 测试代码 ====================
// 只有在定义了 DOCTEST_CONFIG_IMPLEMENT 时才编译测试代码
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#ifdef DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../include/doctest.h"

TEST_CASE("???") {
    Solution sol;

    SECTION("示例 1") {
        vector<int> nums1 = {2, 7, 11, 15};
        CHECK(sol.twoSum(nums1, 9) == vector<int>{0, 1});
    }
}
#endif
