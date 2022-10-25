<template>
    <div class="task">
        <TaskStatusSup :TaskStatus="task.status" />
        <span>{{ task.ask.name }}</span>
    </div>
</template>


<script lang = 'ts' setup>
import { TaskEvent } from '@/ts/Task'
import { PropType } from 'vue';

const props = defineProps({
    uuid: {
        type: String,
        default: ""
    },
    task: {
        type: Object as PropType<TaskEvent>,
        default: { id: 0, name: "" }
    },
})


</script>

<style lang='less' scoped>
.task {
    cursor: pointer;
    padding-top: 0.3rem;
    padding-bottom: 0.3rem;
    display: flex;
    padding-left: 0.6rem;
    align-items: center;
    font-size: 0.9rem;
}

.task:hover {
    background: rgba(236, 236, 236, 0.719);
    border-radius: 5px;
}
</style>