<template>
    <div class="TaskPoolDeatil" :id="uuid.toString()"
        :class="messageDrawerStatus ? 'TaskPoolDeatilOpen' : 'TaskPoolDeatilClose'">
        <div class="taskTitle">
            <span class="taskName">{{ task.ask.name }}</span>
        </div>
        <el-progress :text-inside="true" :stroke-width="30" :percentage="20" status="exception" class="taskPoolProgress"
            @click="changeMessageDrawerStatus" />
        <div class="messageDrawer" :class="messageDrawerStatus ? 'messageDrawerOpen' : 'messageDrawerClose'">
            <div></div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { TaskEvent } from '@/ts/Task'
import { PropType } from 'vue';

const messageDrawerStatus = ref(false);

const changeMessageDrawerStatus = () => {
    messageDrawerStatus.value = !messageDrawerStatus.value
    console.log(messageDrawerStatus.value)
}


const props = defineProps({
    uuid: {
        type: [String, Number],
        default: ""
    },
    task: {
        type: Object as PropType<TaskEvent>,
        default: { id: 0, name: "" }
    },
})

</script>

<style lang="less" scoped>
.TaskPoolDeatil {
    padding: 0.7rem;
    display: flex;
    flex-direction: column;
    position: relative;
    z-index: 100;
}

.TaskPoolDeatilOpen {
    height: 11rem;
    transition: all 0.3s ease-in;
    animation: taskMessageEnter;
}

.TaskPoolDeatilClose {
    height: 3rem;
    transition: all 0.3s ease-out;
    animation: taskMessageLeave;
}


.taskPoolProgress {
    z-index: 100;
    border-radius: 15px;
    box-shadow: 0px 0px 6px rgba(0, 0, 0, 0.228);
    cursor: pointer;
}

.taskName {
    color: rgba(95, 95, 95, 1);
    font-size: 0.3rem;
    font-weight: 600;
    margin-left: 2%;
}

.taskTitle {
    display: flex;
}

.messageDrawer {
    z-index: 0;
    background-color: rgba(220, 220, 220, 0.808);
    position: absolute;
    left: 5%;
    width: 90%;
    top: 2.5rem;
    border-radius: 0.2rem;
    box-shadow: 0px 0px 6px rgba(0, 0, 0, .12);
}

.messageDrawerOpen {
    height: 80%;
    transition: all 0.2s ease-in;
    animation: taskMessageEnter;
}

//奇怪的动画 可以优化 但是不想优化 感觉怪怪的 怪喜欢的

.messageDrawerClose {
    height: 2rem;
    transition: all 0.2s ease-out;
    animation: taskMessageLeave;
}

@keyframes taskMessageEnter {
    0% {
        max-height: 3rem;
    }

    100% {
        max-height: 11rem;
    }
}

@keyframes taskMessageLeave {
    0% {
        max-height: 11rem;
    }

    100% {
        max-height: 3rem;
    }
}
</style>