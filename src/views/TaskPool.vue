<template>
  <el-container class="TaskPool">
    <el-header class="header" height="2.1rem">
      <action-bar :imize="false" :minImize="false" :top="true" />
    </el-header>
    <el-container>
      <el-aside class="TaskPoolAside" width="25%">
        <!-- 当前的几种查找的方案都不合适 暂定方案为添加查询按钮 点击查询按钮 或键盘快捷键 c+f 时 悬浮一个和edge 差不多的 搜索框 这个搜索框的位置需要斟酌一下 如何符合一个人的操作 向下选择和 向上选择是需要的 需要定位到位置并滚动过去  -->
        <!-- 还有就是 对于下拉的设计 应该会做二级菜单 二级菜单的颜色选择是灰色  缩进与下面的选择器相同 缩进需要斟酌一下 -->
        <!-- 当前需要对不同的情况有默认的处理 比如打开时 树是什么状态的  -->
        <!-- 按钮的添加 需要全部缩回去的按钮 全部打开的按钮 搜索按钮 -->
        <!-- 动画 他们值得动画 虽然动画我还不知道怎么做 但是应该要有 -->
        <!-- <el-input v-model="searchContent"></el-input> -->
        <div class="operate">
          <div
            class="operate-item"
            @mousemove="operateHover.search = true"
            @mouseleave="operateHover.search = false"
          >
            <search
              theme="outline"
              size="24"
              :fill="operateHover.search ? '#909399' : '#333'"
            />
          </div>
        </div>
      </el-aside>
      <el-main class="TaskPoolMain"> Main </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
import { Search } from "@icon-park/vue-next";

const operateHover = ref({
  search: false,
  fold: false,
  expand: false,
});
const searchContent = ref("");

//搜索内容防抖 在修改搜索内容时 如果前面还在执行就先清除后继续
let SearchTimeOut: any;

watch(searchContent, (newValue, oldValue) => {
  clearTimeout(SearchTimeOut);
  SearchTimeOut = setTimeout(() => {
    console.log(newValue);
  }, 2000);
});
</script>

<style lang="less" scoped>
.header {
  background: #f0f2f5;
  padding: 0;
}
.TaskPool {
  background: #fafafa;
  width: 100vw;
  height: 100vh;
}
.TaskPoolAside {
  height: 100%;
  box-shadow: 0px 12px 32px 4px rgba(0, 0, 0, 0.04),
    0px 8px 20px rgba(0, 0, 0, 0.08);
}

.operate {
  padding-right: 0.5rem;
  padding-top: 0.5rem;
  display: flex;
  flex-direction: row-reverse;
  cursor: pointer;
}

.operate-item:hover {
  cursor: pointer;
}
</style>