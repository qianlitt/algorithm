

// ================ 测试部分 ================
// 注释掉下一行，提交时不会包含测试代码
#define TEST_MODE

#ifdef TEST_MODE
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../../../include/doctest.h"

TEST_CASE("Test Case") {
    SUBCASE("Example") {
        CHECK(foo(2, 10) == 1024);
    }
}
#endif

// ================ 主函数 =================
#ifndef TEST_MODE
int main() {
    // 主函数
    return 0;
}
#endif
