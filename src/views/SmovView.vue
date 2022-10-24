<template>
    <div class="smovView">
        <div class="smovList">
            <!-- <div class="loadingView" element-loading-text="等待数据渲染"  v-loading="loading"></div> -->
            <!-- 有性能问题需要解决 -->
            <SmovItem v-for="(item, index) in smovList" :key="index" :data="item" />
        </div>
        <div style="width:100%;height:10px"></div>
    </div>
</template>

<script lang='ts'>
import { invoke } from '@tauri-apps/api';
import { defineComponent, onMounted, ref } from 'vue';
import { Loading } from '@element-plus/icons-vue';
export default defineComponent({
    name: 'SmovView',
    props: [],
    setup(props, { emit }) {

        onMounted(() => {
            console.log(Loading.toString)
            getSmov();
        })

        const loading = ref(true);

        const smovList = ref([] as any[]);

        const getSmov = () => {
            console.log(Date.now())
            invoke("get_all_smov").then((res: any) => {
                if (res.code == 200) {
                    console.log(res)
                    smovList.value = res.data;
                }
            }).finally(() => {
                console.log(Date.now())
            })
        }
        return {
            smovList,
            loading
        };
    }
})

</script>
<style lang='less' scoped>
.smovView {
    width: 100%;

}
.smovList {
    display: flex;
    width: 100%;
    height: 100%;
    flex-wrap: wrap;
    justify-content: space-evenly;
}

.loadingView {
    width: 100%;
    height: 100%;
    background: rgb(255, 255, 255);

    .el-loading-mask {
        z-index: 800;
    }
}
</style>

<style lang="less">
.el-loading-spinner {
    color: rgb(0, 0, 0);
}
</style>