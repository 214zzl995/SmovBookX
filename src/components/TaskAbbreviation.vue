<template>
    <div class="task">
        <span>{{ task.name }}</span>
    </div>
</template>


<script lang = 'ts' setup>
import { Task } from '@/ts/Task'
import { PropType } from 'vue';

const props = defineProps({
    uuid: {
        type: String,
        default: ""
    },
    task: {
        type: Object as PropType<Task>,
        default: { id: 0, name: "" }
    },
})


</script>

<style lang='less' scoped>
.task{
    cursor: pointer;
    padding-top: 0.3rem;
    padding-bottom: 0.3rem;
    display: flex;
    padding-left: 0.6rem
}
.task:hover {
    background: rgba(236, 236, 236, 0.719);
    border-radius: 5px;
}
</style>