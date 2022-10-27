<template>
  <div class="smovItem" @click="openDetail">
    <img
      :src="data.thumbs_img == '' ? nonePic : convertFileSrc(data.thumbs_img)"
      class="smovImg"
    />
    <p class="name">{{ data.name }}</p>

    <el-popover
      placement="bottom"
      :width="200"
      trigger="click"
      :content="data.title"
      :visible="false"
    >
      <template #reference>
        <div class="titleDiv">
          <p class="title">
            <span>{{ data.title }}</span>
          </p>
        </div>
      </template>
    </el-popover>
  </div>
</template>

<script lang='ts'>
import { defineComponent, ref } from "vue";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import nonePic from "@/assets/none_pic.png";
import mountDetail from "./SmovDetail/SmovDetail";
import XEUtils from "xe-utils";
import {
  getAll,
  WebviewWindow,
  availableMonitors,
} from "@tauri-apps/api/window";

import { title } from "process";
import { join } from "path/posix";
import { ElSteps } from "element-plus";
import { request } from "../util/invoke";
export default defineComponent({
  name: "SmovItem",
  props: ["data"],
  setup(props, { emit }) {
    const openDetail = async () => {
      // 仅记录一下单次监听的写法
      // webview.once('tauri://created', function () {
      //     invoke("set_style", { effect: "", label: props.data.name });
      // })
      // webview.once('tauri://close-requested', function (e) {
      //     console.log(e.windowLabel)
      // })
      request("go_detail", {
        label: props.data.name,
        url: "/SmovDetail/" + props.data.id,
      });
    };
    return {
      convertFileSrc,
      nonePic,
      openDetail,
    };
  },
});
</script>
<style lang='less' scoped>
.smovImg {
  display: inline-block;
  height: 140px;
  object-fit: cover;
  border-radius: 6px;
}
.smovItem {
  height: 200px;
  border-radius: 6px;
  box-shadow: var(--el-box-shadow-light);
  margin: 0.5rem;
  cursor: pointer;
  transition: all 0.1s;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.smovItem:hover {
  transform: scale(1.05);
}

.name {
  font-size: 1.1rem;
  font-weight: 700;
  margin-top: 0.4rem;
  margin-bottom: 0.2rem;
  color: #303133;
}

.titleDiv {
  width: 100%;
  display: flex;
  justify-content: center;
  position:relative
}

.title {
  width: 80%;
  display: -webkit-box;
  word-break: break-all;
  text-overflow: ellipsis;
  font-size: 0.8rem;
  overflow: hidden;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 1;
  margin-top: 0.2rem;
  margin-bottom: 0.2rem;
  font-weight: 500;
  color:  #A8ABB2;
  position:absolute
}
</style>