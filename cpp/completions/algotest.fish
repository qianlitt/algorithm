# algotest.fish - algotest 命令的 Fish 补全
#
# 安装后，输入 `algotest ` 按 Tab 即可触发补全

### 主命令补全
complete -c algotest -f

# 帮助选项
complete -c algotest -s h -l help -d "显示帮助信息"

# 408 命令补全
complete -c algotest -n "not __fish_seen_subcommand_from 408 leet new clean" \
    -a "408" -d "运行408题目的测试"

# leet 命令补全
complete -c algotest -n "not __fish_seen_subcommand_from 408 leet new clean" \
    -a "leet" -d "运行LeetCode题目的测试"

# new 命令补全
complete -c algotest -n "not __fish_seen_subcommand_from 408 leet new clean" \
    -a "new" -d "创建新题目文件（交互式）"

# clean 命令补全
complete -c algotest -n "not __fish_seen_subcommand_from 408 leet new clean" \
    -a "clean" -d "清理构建文件"

### 408 子命令的参数补全
# 检测目录中的 408 题目文件
function __fish_complete_408_questions
    # 查找所有章节目录
    for chapter_dir in src/408/chapter*/
        if test -d $chapter_dir
            # 提取章节号
            set chapter (string replace -r '.*chapter(\d+)/$' '$1' $chapter_dir)
            # 查找该章节的所有题目文件
            for question_file in $chapter_dir/*.cpp
                set question (string replace -r '.*/(\d+)\.cpp$' '$1' $question_file)
                echo "$chapter/$question"
            end
        end
    end
end

# 当输入 algotest 408 后，补全题目
complete -c algotest -n "__fish_seen_subcommand_from 408" \
    -a "(__fish_complete_408_questions)"

### LeetCode 子命令的参数补全
# 检测目录中的 LeetCode 题目文件
function __fish_complete_leetcode_questions
    for file in src/leetcode/*.cpp
        # 提取题目编号 (如从 "1.two-sum.cpp" 中提取 "1")
        set match (string match -r '^(\d+)\.' (basename $file))
        if test -n "$match[2]"
            echo $match[2]
        end
    end
end

# 当输入 algotest leet 后，补全题目编号
complete -c algotest -n "__fish_seen_subcommand_from leet" \
    -a "(__fish_complete_leetcode_questions)"
