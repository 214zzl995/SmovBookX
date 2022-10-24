<template>
    <div>
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

</style>