<template>
    <div class="TaskPoolDeatil">
        <div class="taskTitle">
            <span class="taskName">{{ task.ask.name }}</span>
        </div>
        <el-progress :text-inside="true" :stroke-width="30" :percentage="20" status="exception"
            class="taskPoolProgress" />
    </div>


</template>

<script lang="ts" setup>
import { TaskEvent } from '@/ts/Task'
import { PropType } from 'vue';

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

</style>