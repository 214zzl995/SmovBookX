<template>
  <div>
    <!-- <el-button type="danger" @click="test1">cs</el-button>
        <el-button type="danger" @click="test2">csss</el-button>
        <el-button type="danger" @click="test3">文件检索测试</el-button>
        <el-button type="danger" @click="test4">即时渲染测试</el-button>
        <el-button type="danger" @click="test5">打开界面测试</el-button>
        <el-button type="danger" @click="test6">后台新建界面测试</el-button>-->

    <el-button type="danger" @click="test7">测试打开web服务器</el-button>

    <el-button type="danger" @click="test8">测试关闭web服务器</el-button>

    <el-button type="danger" @click="test10"
      >测试向后端发送shutdown事件</el-button
    >

    <el-button type="danger" @click="test9">测试获取本地ip</el-button>

    <el-button type="danger" @click="test11">爬虫序列化测试</el-button>

    <el-button type="danger" @click="test12">爬虫测试</el-button>

    <el-button type="danger" @click="test13">爬虫测试(家里)</el-button>

    <el-button type="danger" @click="test14">悬浮图标测试</el-button>

    <el-button type="danger" @click="test15">测试打开新窗口</el-button>

    <el-button type="danger" @click="test17">添加TASK</el-button>
    <el-button type="danger" @click="test17_2">获取当前所有线程</el-button>

    <el-button type="danger" @click="test18">打开TaskPool窗口</el-button>

    <!-- <el-icon v-if="show" @click="show = !show">
            <component :is="Bowl"></component>
        </el-icon>
        <el-icon v-if="!show" @click="show = !show">
            <component :is="Box"></component>
        </el-icon>-->

    <router-view></router-view>
  </div>
</template>

<script lang="ts">
import {
  defineComponent,
  reactive,
  ref,
  inject,
  h,
  render,
  createApp,
  Transition,
  withCtx,
} from "vue";
import { ThreadPool } from "../ts/ThreadPool";
import { useRouter } from "vue-router";
import { Log } from "../type/log";
import { CurentTime } from "../util/time";
import { dialog, notification } from "@tauri-apps/api";
import { readDir } from "@tauri-apps/api/fs";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";
import { request } from "../util/invoke";
import { Bowl, Box } from "@element-plus/icons-vue";
import mountContent from "../components/qrCode/qrCode";
import { emit, listen } from "@tauri-apps/api/event";

export default defineComponent({
  setup() {
    // 任务实体
    const router = useRouter();

    const show = ref(false);

    const qr = ref("");

    const test = () => {};
    const src = ref(
      convertFileSrc("E:\\ccccc\\zl\\SSNI-107\\img\\thumbs_SSNI-107.jpg")
    );

    let logs = inject("log") as any;

    const test1 = () => {
      const option: notification.Options = {
        title: "test",
        body: "test",
        icon: "test",
      };

      notification.sendNotification(option);
    };

    const test2 = () => {
      const log: Log = {
        time: CurentTime(),
        thread: "前端发消息的测试",
        msg: "前端发消息的测试",
        level: "INFO",
      };
      logs.value = log;
    };

    const test3 = () => {
      const assetUrl = convertFileSrc(
        "E:\\ccccc\\zl\\SSNI-107\\img\\thumbs_SSNI-107.jpg"
      );
      console.log(assetUrl);
    };

    const test4_1 = {
      name: "test4_1",
      setup() {
        return () => {
          return h(
            // 1. 要渲染的标签名称：第一个参数【必需】
            "button",
            // 2. 渲染标签的属性：第二个参数【可选】
            {
              style: {
                color: "#333",
                border: "1px solid #ccc",
              },
              class: "wdnmd",
              id: "",
              onClick: changeNum,
            },
            // 3. 渲染标签的子元素数组：第三个参数【可选】
            [
              // "render函数文本" 文本内容可以直接写入
              h("p", num.value), // h()创建的VNodes
            ]
          );
        };
      },
    };

    let num = ref(0); // vue3中需要使用ref或reactive声明变量，否则无法实现双向数据绑定
    const changeNum = () => {
      num.value++; // 改变ref定义的变量值需要使用.value , reactive不需要
    };

    const test4 = () => {
      createApp(test4_1).mount(document.createElement("div"));
    };

    const test7 = () => {
      request("run_hfs").then((res: any) => {});
    };

    const test9 = () => {
      request("get_local_ip").then((res: any) => {
        console.log(res);
        qr.value = res.data;

        mountContent({ title: "手机端扫描", qr: res.data + ":8000" });
      });
    };

    const test8 = () => {
      request("shutdown_signal").then((res: any) => {});
    };

    const test5 = async () => {
      // loading embedded asset:
    };

    const test6 = async () => {
      await request("create_new_window", {
        label: "IPVR-075",
        url: "SmovDetail/3",
      });
    };

    const qrcode = () => {};

    const test10 = () => {
      emit("HFS://ShutDown").then((res) => {
        console.log(res);
      });
    };

    const test11 = async () => {
      await request("generate");
    };

    const test12 = async () => {
      await request("smov_crawler", {
        retrievingSmov: {
          id: 1,
          seek_name: "ipx005",
          smov_id: 1,
          status: 2,
        },
      });
    };

    let webview: any = null;

    onMounted(() => {});

    const test13 = async () => {
      await request("smov_crawler", {
        retrievingSmov: {
          id: 1,
          seek_name: "IPVR-075",
          smov_id: 1,
          status: 3,
        },
      });
    };

    const test14 = () => {
      webview = new WebviewWindow("test", {
        url: "/test",
        title: "测试窗口",
        width: 20,
        height: 10,
        minWidth: 50,
        minHeight: 50,
        center: true,
        visible: false,
        alwaysOnTop: true,
        // skipTaskbar: true,
        resizable: false,
        decorations: false,
        transparent: true,
      });
      webview.show();
      webview.unminimize();
    };

    const test15 = () => {
      request("create_new_window", { label: "test", effect: "", path: "" });
    };

    const test16 = () => {
      emit("TASK://add_task_convert", { id: 1, name: "test" }).then((res) => {
        console.log(res);
      });
    };

    const test16_2 = () => {
      emit("TASK://get_task_len", { id: 1, name: "test" }).then((res) => {
        console.log(res);
      });
    };

    const test17 = () => {

      request("add_task_crawler", { taskAsk: { id: 1, name: "IPVR - 075" } }).then(
        (res) => {
          console.log(res);
        }
      );
      request("add_task_crawler", { taskAsk: { id: 2, name: "PPPD - 927" } }).then(
        (res) => {
          console.log(res);
        }
      );
      request("add_task_crawler", { taskAsk: { id: 3, name: "IPX - 177" } }).then(
        (res) => {
          console.log(res);
        }
      );
      request("add_task_crawler", { taskAsk: { id: 4, name: "SSNI - 780" } }).then(
        (res) => {
          console.log(res);
        }
      );
      request("add_task_crawler", { taskAsk: { id:5, name: "CAWD - 243" } }).then(
        (res) => {
          console.log(res);
        }
      );

    };

    const test17_2 = () => {
      request("get_task_pool").then(res=>{
        console.log(res)
      })
    };

    const test18 = () => {

      request("go_task_pool")
    };

    const addTaskEvent = () => {
      !(async () =>
        await listen("TASKPOOL://status_change", (event: any) => {
          console.log("消息");
          console.log(event);
        }))();
    };

    onMounted(() => {
      //addTaskEvent();
    });

    return {
      test,
      test1,
      test2,
      test3,
      test4,
      test5,
      test6,
      src,
      Bowl,
      Box,
      show,
      test7,
      test8,
      test9,
      qr,
      test10,
      test11,
      test12,
      test13,
      test14,
      test15,
      test16,
      test16_2,
      test17,
      test17_2,
      test18,
    };
  },
});
</script>

<style>
</style>