<template>
  <el-container class="TaskPool">
    <transition name="el-zoom-in-top">
      <div v-show="searchShow" class="searchInputDiv">
        <input ref="searchInput" class="searchInput" v-model="searchContent" type="text">
        <div class="searchClose" @click="searchShowFn">
          <Close />
        </div>

      </div>
    </transition>

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
        <div class="operate search">
          <div class="operate-item" @mousemove="operateHover.search = true" @mouseleave="operateHover.search = false"
            @click="searchShowFn">
            <search theme="multi-color" size="18" :fill="['#909399', '#2F88FF', '#FFF', '#43CCF8']" :strokeWidth="3"
              strokeLinecap="square" />
            <span class="operateTitle">Search</span>
          </div>
        </div>
        <div class="divider">
          <el-divider />
        </div>

        <div class="taskTree">
          <div class="taskGroup operate">
            <div class="operate-item" @click="groupShow.crawler = !groupShow.crawler">
              <instruction theme="multi-color" size="18" :fill="['#333', '#2F88FF', '#FFF', '#43CCF8']" :strokeWidth="3"
                strokeLinecap="square" />
              <span class="operateTitle">
                爬虫
              </span>
              <div class="dropDown">
              </div>
            </div>


            <div class="groupItem animate__slideInDown" v-show="groupShow.crawler">
              <TaskAbbreviation v-for="(item, index) in testData" key="index" />
            </div>

          </div>
          <div class="taskGroup operate">
            <div class="operate-item" @click="groupShow.convert = !groupShow.convert">
              <instruction theme="multi-color" size="18" :fill="['#333', '#2F88FF', '#FFF', '#43CCF8']" :strokeWidth="3"
                strokeLinecap="square" />
              <span class="operateTitle">
                转码
              </span>
              <div class="dropDown">

              </div>
            </div>


            <div class="groupItem" v-show="groupShow.convert">
              <TaskAbbreviation v-for="(item, index) in testData" key="index" />
            </div>
          </div>
        </div>

      </el-aside>
      <el-main class="TaskPoolMain"> Main </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
import { Search, Close, Instruction } from "@icon-park/vue-next";
import { Task } from '@/ts/Task'

const operateHover = ref({
  search: false,
  fold: false,
  expand: false,
});

const groupShow = ref({
  crawler: false,
  convert: false
})

const testData = ref([
  { "1": { id: 1, name: "1" } as Task },
  { "2": { id: 2, name: "2" } as Task },
  { "3": { id: 3, name: "3" } as Task },
  { "4": { id: 4, name: "4" } as Task },
  { "5": { id: 5, name: "5" } as Task },
  { "6": { id: 6, name: "6" } as Task },
  { "7": { id: 7, name: "7" } as Task },
  { "8": { id: 8, name: "8" } as Task },
  { "9": { id: 9, name: "9" } as Task },
  { "10": { id: 10, name: "10" } as Task },
  { "11": { id: 11, name: "11" } as Task },
  { "12": { id: 12, name: "12" } as Task },
  { "13": { id: 13, name: "13" } as Task },
  { "14": { id: 14, name: "14" } as Task },
  { "15": { id: 15, name: "15" } as Task }
]);

const searchContent = ref("");

const searchShow = ref(false);

const searchInput = ref<any>(null);

const searchShowFn = () => {
  searchShow.value = !searchShow.value
  if (searchShow.value) {
    nextTick(() => {
      searchInput.value.focus();
    })
  } else {
    searchContent.value = "";
  }
}

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
  display: flex;
  cursor: pointer;
  align-items: center;
  padding-right: 0.7rem;
  padding-left: 0.7rem;
  flex-direction: column;
}

.operate-item {
  width: 100%;
  display: flex;
  padding: 0.3rem;
  padding-left: 0.6rem;
  color: #333;
  align-items: center;
  font-size: 0.9rem;
  justify-content: start;
}

.operate-item:hover {
  cursor: pointer;
  color: #909399;
}

.operate-item:active {
  cursor: pointer;
  color: #333;
}

.operateTitle {
  font-weight: 600;
  margin-left: 0.5rem;
}

.searchInputDiv {
  border: rgba(0, 0, 0, .08) solid 1px;
  position: fixed;
  top: 2.5rem;
  right: 2rem;
  z-index: 999;
  padding: 0.2rem;
  border-radius: 9px;
  box-shadow: 0px 8px 40px 4px rgba(0, 0, 0, .08),
    0px 6px 16px rgba(0, 0, 0, .12),
    0px 2px 2px -2px rgba(0, 0, 0, .16);
}

.searchInput {
  border: none;
  width: 12rem;
  height: 2rem;
  outline: none;
  font-size: 0.95rem;
  margin-left: 0.5rem;
  margin-right: 0.5rem;
  background: none;
}

.searchClose {
  padding: 0.2rem;
  cursor: pointer;
}

.searchClose:hover {
  padding: 0.2rem;
  cursor: pointer;
  color: #909399;
}

.search {
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
}

.taskTree {
  margin-top: 0rem;
}

.divider {
  padding-bottom: 0.5rem;
  padding-right: 0.3rem;
  padding-left: 0.3rem;
  padding-top: 0;

  div {
    margin: 0;
  }
}

.dropDown {
  border-left: 0.4rem solid transparent;
  border-right: 0.4rem solid transparent;
  border-bottom: 0.4rem solid #333;
  width: 0;
  height: 0;
  transform: rotate(90deg);
  margin-left: auto;
}

.taskGroup:hover {
  .dropDown {
    border-left: 0.4rem solid transparent;
    border-right: 0.4rem solid transparent;
    border-bottom: 0.4rem solid #909399;
  }
}

.taskGroup:active {
  .dropDown {
    border-left: 0.4rem solid transparent;
    border-right: 0.4rem solid transparent;
    border-bottom: 0.4rem solid #333;
  }
}

.groupItem {
  width: 100%;
  min-height: 200px;
  max-height: max-content;
}
</style>