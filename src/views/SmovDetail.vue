<template>
  <el-container class="smovDetail">
    <el-header class="editBar" data-tauri-drag-region>
      <action-bar :minImize="false" :top="true" :once="true">
        <div class="detailIco" @click="detailShow = !detailShow">
          <el-icon :size="17">
            <expand />
          </el-icon>
          <p>详情</p>
        </div>

        <div class="detailIco" @click="toOpen">
          <el-icon :size="17">
            <video-camera-filled />
          </el-icon>
          <p>打开</p>
        </div>

        <div class="detailIco" @click="convertSmov2Hls">
          <el-icon :size="17">
            <expand />
          </el-icon>
          <p>2HLS</p>
        </div>
      </action-bar>
    </el-header>
    <el-main class="ImgShow" id="ImgShow">
      <!-- :style="img.x === 0 ? img.y === 0 ? '' : 'left' + img.x + 'px;top:' + img.y + 'px;' : 'left' + img.x + 'px;top:' + img.y + 'px;'" -->
      <div
        id="ShowImg"
        class="ShowImg"
        :style="
          'transform:translateX(' + img.x + 'px) translateY(' + img.y + 'px);'
        "
        @dblclick="resetImg"
      >
        <img
          :class="ctrl ? 'ShowImgCtrl' : ''"
          :style="
            'transform: scale(' +
            img.scale +
            ') translateX(' +
            0 +
            'px) translateY(' +
            0 +
            'px);'
          "
          :src="
            focus === -1
              ? data.main_img === undefined
                ? nonePic
                : data.main_img === ''
                ? nonePic
                : convertFileSrc(data.main_img)
              : convertFileSrc(data.detail_img[focus])
          "
        />
      </div>
    </el-main>

    <div class="ImgTray" id="ImgTray">
      <div
        class="imgLabel"
        :key="-1"
        :class="focus == -1 ? 'imgLabelFocus' : ''"
        @click="focus = -1"
      >
        <img
          :src="
            data.main_img === undefined
              ? nonePic
              : data.main_img === ''
              ? nonePic
              : convertFileSrc(data.main_img)
          "
        />
      </div>
      <div
        class="imgLabel"
        v-for="(item, index) in data.detail_img"
        :key="index"
        :class="focus == index ? 'imgLabelFocus' : ''"
        @click="changeMainImg(index)"
      >
        <img :src="convertFileSrc(item)" />
      </div>
    </div>

    <el-drawer
      v-model="detailShow"
      :with-header="false"
      :modal="true"
      modal-class="detailModal"
    >
      <div class="detail">
        <p>
          <span class="key">名称：</span>
          <span class="value">{{ data.name }}</span>
        </p>
        <p>
          <span class="key">发行日期：</span>
          <span class="value">{{ data.release_time }}</span>
        </p>
        <p>
          <span class="key">文件大小：</span>
          <span class="value">{{ ~~(data.len / 1024 / 1024) }}MB</span>
        </p>
        <p>
          <span class="key">制作：</span>
          <span class="value">{{ data.maker }}</span>
        </p>
        <p v-if="data.serie != '无系列'">
          <span class="key">系列：</span>
          <span class="value">{{ data.serie }}</span>
        </p>
        <p>
          <span class="key">文件类型：</span>
          <span class="value">{{ data.extension }}</span>
        </p>
        <p>
          <span class="key">导演：</span>
          <span class="value">{{ data.director }}</span>
        </p>
        <p>
          <span class="key">是否中文：</span>
          <span class="value">{{ data.is_ch ? "是" : "否" }}</span>
        </p>
        <p>
          <span class="key">演员：</span>
          <span
            v-for="(item, index) in data.actors"
            class="value"
            :key="index"
            style="margin-right: 2px"
          >
            <a href="#">{{ item.name }}</a>
          </span>
        </p>

        <div class="tags">
          <span class="key">标签：</span>
          <span class="tag" v-for="tag in data.tags" :key="tag">
            <a href="#">{{ tag.name }}</a>
          </span>
        </div>
      </div>
    </el-drawer>
  </el-container>
</template>

<script lang='ts' >
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { defineComponent, onMounted } from "vue";
import { useRoute } from "vue-router";
import nonePic from "../assets/NoneImages.png";
import { Menu, VideoCameraFilled } from "@element-plus/icons-vue";
import { getCurrent } from "@tauri-apps/api/window";
import { shell } from "@tauri-apps/api";
import { join } from "@tauri-apps/api/path";
import { request } from "@/util/invoke";

export default defineComponent({
  props: {},
  components: {
    Expand: Menu,
    VideoCameraFilled: VideoCameraFilled,
  },
  setup(props) {
    const { $alert } = getCurrentInstance()!.appContext.config.globalProperties;

    const msg = ref(false);

    const data = ref({} as any);

    const route = useRoute();

    const detailShow = ref(false);

    const id = ref(route.params.Id);

    const focus = ref(-1);

    const ctrl = ref(false);

    const current = getCurrent();

    let img = ref({
      temp: {},
      imgTemp: {},
      scale: 1,
      step: 0.15,
      min_scale: 0.3,
      max_scale: 3,
      x: 0,
      y: 0,
    } as any);

    const SizeEvent = async (event: any) => {
      event.preventDefault();
      if (ctrl.value) {
        const detail = event.wheelDelta || event.detail;
        if (detail > 0) {
          const size =
            img.value.max_scale - img.value.scale > img.value.step
              ? img.value.step
              : img.value.max_scale - img.value.scale;
          img.value.scale = img.value.scale + size;
        } else {
          const size =
            img.value.scale - img.value.min_scale > img.value.step
              ? img.value.step
              : img.value.min_scale - img.value.scale;
          img.value.scale = img.value.scale - size;
        }
      }
    };

    const ctrlDownEvent = async (event: KeyboardEvent) => {
      if (event.code == "F12" || event.code == "F5") {
      } else {
        event.preventDefault();
      }
      ctrl.value = event.ctrlKey;
    };

    const resetImg = () => {
      img.value.x = 0;
      img.value.y = 0;
      img.value.scale = 1;
    };

    onMounted(() => {
      getData();
      const mainImg = document.getElementById("ShowImg")!;
      const imgDiv = document.getElementById("ImgShow")!;
      img.value.temp = imgDiv;
      img.value.imgTemp = mainImg;
      img.value.temp.addEventListener("mousewheel", SizeEvent, {
        passive: false,
      });

      mainImg.onmousedown = function (ev) {
        var oEvent = ev;
        // 阻止默认事件
        oEvent.preventDefault();
        const disX = oEvent.clientX - img.value.x;
        const disY = oEvent.clientY - img.value.y;
        //取消窗口调整

        imgDiv.onmousemove = function (ev) {
          if (ctrl.value) {
            oEvent = ev;
            oEvent.preventDefault();
            // var x = oEvent.clientX - disX;
            // var y = oEvent.clientY - disY;

            // //图形移动的边界判断
            // x = x <= 0 ? 0 : x;
            // x = x >= imgDiv.offsetWidth - mainImg.offsetWidth ? mainImg.offsetWidth - mainImg.offsetWidth : x;
            // y = y <= 0 ? 0 : y;
            // y = y >= imgDiv.offsetHeight - mainImg.offsetHeight ? imgDiv.offsetHeight - mainImg.offsetHeight : y;

            img.value.x = oEvent.clientX - disX;
            img.value.y = oEvent.clientY - disY;
          }
        };
        // 图形移出父盒子取消移动事件,防止移动过快触发鼠标移出事件,导致鼠标弹起事件失效
        imgDiv.onmouseleave = function () {
          imgDiv.onmousemove = null;
          imgDiv.onmouseup = null;
        };
        // 鼠标弹起后停止移动 窗口可以调整
        imgDiv.onmouseup = function () {
          imgDiv.onmousemove = null;
          imgDiv.onmouseup = null;
          current.setResizable(true);
        };
      };

      document.onkeydown = function (event) {
        ctrlDownEvent(event);
      };

      document.onkeyup = function (event) {
        ctrlDownEvent(event);
      };
    });

    const getData = () => {
      invoke("get_smov_by_id", { id: Number(id.value) })
        .then((res: any) => {
          if (res.code == 200) {
            console.log(res.data);
            data.value = res.data;
          }
        })
        .finally(() => {});
    };

    const toOpen = async () => {
      try {
        shell.open(
          await join(
            data.value.path,
            data.value.realname + "." + data.value.extension
          )
        );
      } catch (e) {
        console.log(e);
        $alert.error("打开出现错误，可能没有设置文件类型的默认打开程序");
      }
    };

    const changeMainImg = (index: number) => {
      img.value.scale = 1;
      img.value.x = 0;
      img.value.y = 0;
      focus.value = index;
    };

    const convertSmov2Hls = () => {
      request("convert_smov2hls", { id: Number(id.value) });
    };

    return {
      convertFileSrc,
      toOpen,
      dialogVisible: true,
      msg,
      data,
      detailShow,
      focus,
      nonePic,
      changeMainImg,
      img,
      ctrl,
      resetImg,
      convertSmov2Hls,
    };
  },
});
</script>

<style lang='less' scoped>
.smovDetail {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  z-index: 500;
  overflow: hidden;
  border-radius: 8px;
  box-shadow: 10px 10px 5px #888888;
  background-color: #f9f9f9;
}

.detail {
  height: 100%;
  display: flex;
  flex-direction: column;
  p {
    text-align: justify;
    text-justify: inter-ideograph;
    margin-top: 4px;
    margin-bottom: 4px;
  }
}

.key {
  font-size: 1rem;
  font-weight: 600;
}
.value {
  font-size: 1rem;
  font-weight: 500;
  a {
    color: brown;
    font-weight: 600;
  }
}

.tags {
  display: flex;
  flex-wrap: wrap;
  .tag {
    font-size: 1rem;
    margin-right: 5px;
    a {
      color: brown;
      font-weight: 600;
    }
  }
}

.ImgTray {
  background-color: rgba(202, 202, 202, 0.411);
  max-height: 80vw;
  border-radius: 12px;
  position: absolute;
  left: 50%;
  flex-direction: column;
  display: flex;
  padding: 6px;
  overflow: auto;
  transform-origin: left center;
  transform: rotate(270deg) translateX(-45vh);
}

.ImgTray::-webkit-scrollbar {
  display: none; /* Chrome Safari */
}

.imgLabel {
  width: 50px;
  height: 48px;
  padding: 4px;
  background-color: rgba(153, 153, 153, 0.2);
  border: 2px solid rgba(153, 153, 153, 0);
  margin-top: 3px;
  margin-bottom: 3px;
  border-radius: 6px;
  flex-shrink: 0;
  transform: rotate(90deg);
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 6px;
  }
}

.imgLabelFocus {
  border: 2px solid rgba(207, 49, 49, 0.712);
}

.ImgShow {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

.ShowImgCtrl {
  cursor: move;
}
.ShowImg {
  position: absolute;
  img {
    max-width: 100%;
    max-height: 100%;
    transition: all 120ms;
  }
}

.editBar {
  height: 30px;
  padding: 0;
}

.detailIco {
  width: 4rem;
  height: 100%;
  display: flex;
  margin-left: 3%;
  margin-right: 3%;
  align-items: center;
  cursor: pointer;
  justify-content: center;
  background-color: #4983aa9f;
  border-radius: 0 0 5px 5px;
  box-shadow: var(--el-box-shadow-light);
  p {
    font-size: 0.9rem;
    font-weight: 600;
    font-family: "Times New Roman", Times, serif;
    font-style: normal;
  }
}

.detailIco:hover {
  background-color: #4983aa5b;
}
</style>

<style>
.detailModal {
  background-color: rgba(165, 42, 42, 0);
}
</style>
