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
        <div class="operate searchDiv">
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
            <div class="operate-title">
              <div class="operate-item" :class="groupShow.crawler ? 'operate-item-open' : ''"
                @click="groupShowFnCrawler">

                <instruction theme="multi-color" size="18" :fill="['#333', '#2F88FF', '#FFF', '#43CCF8']"
                  :strokeWidth="3" strokeLinecap="square" />
                <span class="operateTitle">
                  爬虫
                </span>
                <div class="dropDown" :class="groupShow.crawler ? 'dropDownShow' : 'dropDownHide'">
                </div>
              </div>
            </div>

            <Transition name="task-group">
              <div class="groupItem animate__slideInDown" v-show="groupShow.crawler">
                <TaskAbbreviation v-for="(item, key) in showData.Crawler" key="key" :uuid="key" :task="item" />
              </div>
            </Transition>
          </div>
          <div class="taskGroup operate">
            <div class="operate-title">
              <div class="operate-item" :class="groupShow.convert ? 'operate-item-open' : ''"
                @click="groupShowFnConvert">
                <instruction theme="multi-color" size="18" :fill="['#333', '#2F88FF', '#FFF', '#43CCF8']"
                  :strokeWidth="3" strokeLinecap="square" />
                <span class="operateTitle">
                  转码
                </span>

                <div class="dropDown" :class="groupShow.convert ? 'dropDownShow' : 'dropDownHide'">
                </div>
              </div>
            </div>
            <Transition name="task-group">
              <div class="groupItem" v-show="groupShow.convert">
                <TaskAbbreviation v-for="(item, key) in showData.Convert" key="key" :uuid="key" :task="item" />
              </div>
            </Transition>
          </div>
        </div>

      </el-aside>
      <el-main class="TaskPoolMain"> Main </el-main>
    </el-container>
  </el-container>
</template>

<script lang="ts" setup>
import { Search, Close, Instruction } from "@icon-park/vue-next";
import { TaskEvent, TaskType, TaskAsk, TaskStatus } from '@/ts/Task'
import { request } from "@/util/invoke";

const operateHover = ref({
  search: false,
  fold: false,
  expand: false,
});

const groupShow = ref({
  crawler: false,
  convert: false
})

let data = ref({
  Convert:{},
  Crawler:{}
});

const showData = computed({
  get() {
    // 如果读取计算属性的值，默认调用get方法
    return data.value
  },
  set(v) {
    // v是计算属性下传递的实参
    // 如果要想修改计算属性的值，默认调用set方法
  }
});

const testDataCrawler = ref(
  {
    "1": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "2": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "3": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "4": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "5": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "6": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "7": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "8": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "9": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "10": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "11": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "12": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "13": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "14": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "15": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "16": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "17": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "18": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "19": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "20": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "21": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "22": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "23": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "24": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "25": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "26": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "27": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
  }
);

const testDataConvert = ref(
  {
    "1": { event_type: TaskType.Convert, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "2": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "3": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "4": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "5": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "6": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "7": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "8": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "9": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "10": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "11": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "12": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "13": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "14": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "15": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "16": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "17": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "18": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "19": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "20": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "21": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "22": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "23": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "24": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "25": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "26": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
    "27": { event_type: TaskType.Crawler, ask: { id: 1, name: "1" } as TaskAsk, status: TaskStatus.Wait } as TaskEvent,
  }
);

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

let SearchTimeOut: any;

const groupShowFnCrawler = () => {
  groupShow.value.convert = false;
  groupShow.value.crawler = !groupShow.value.crawler;

}

const groupShowFnConvert = () => {
  groupShow.value.crawler = false;
  groupShow.value.convert = !groupShow.value.convert;

}

watch(searchContent, (newValue, oldValue) => {
  clearTimeout(SearchTimeOut);
  SearchTimeOut = setTimeout(() => {
    console.log(newValue);
  }, 2000);
});

const getTaskPool = () => {
  request("get_task_pool").then((res:any) =>{
   if (res.code == 200){
      data.value = res.data
      console.log(data.value)
   }
  })
}

//先试试能不能正常获取数据
onMounted(() => {
   getTaskPool();
})

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
  display: flex;
  flex-direction: column;
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

.operate-item-open {
  //box-shadow: 0px 17px 10px 0px rgb(0 0 0 / 4%);
  border-bottom: rgba(0, 0, 0, 0.13) solid 0px;
}

.operate-item {
  width: 100%;
  display: flex;
  padding-top: 0.3rem;
  padding-bottom: 0.3rem;
  color: #333;
  align-items: center;
  font-size: 0.9rem;
  justify-content: start;
}

.operate-item:hover {
  cursor: pointer;
}

.operate-item:active {
  cursor: pointer;
  color: #333;
}

.operateTitle {
  font-weight: 600;
  margin-left: 0.5rem;
}

.operate-title {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
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

.searchDiv {
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

.dropDownShow {
  animation-name: dropDownTurnWise;
  animation-duration: .3s;
  animation-iteration-count: 1;
  transform: rotate(180deg);
}

.dropDownHide {
  animation-name: dropDownTurnAnti;
  animation-duration: .3s;
  animation-iteration-count: 1;
  transform: rotate(90deg);
}

@keyframes dropDownTurnWise {
  0% {
    transform: rotate(90deg);
  }

  100% {
    transform: rotate(180deg);
  }
}

@keyframes dropDownTurnAnti {
  0% {
    transform: rotate(180deg);
  }

  100% {
    transform: rotate(90deg);
  }
}

.taskTree {
  flex: 1;
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
  max-height: 70vh;
  overflow-y: scroll;
  cursor: auto;
}

.task-group-enter-active {
  transition: all 0.3s ease-out;
  animation: 0.3s taskGroupEnter;

}

.task-group-leave-active {
  transition: all 0.3s cubic-bezier(1, 0.5, 0.8, 1);
  animation: 0.3s taskGroupLeave;
}

.task-group-enter-from,
.task-group-leave-to {
  opacity: 0;
}

@keyframes taskGroupEnter {
  0% {
    max-height: 0;
  }

  100% {
    max-height:70vh;
  }
}

@keyframes taskGroupLeave {
  0% {
    max-height: 70vh;
  }

  100% {
    max-height: 0;
  }
}
</style>