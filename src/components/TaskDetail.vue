<template>
    <el-progress :text-inside="true" :stroke-width="20" :percentage="50" status="exception">
        <span>Content</span>
    </el-progress>

</template>

<script lang="ts" setup>
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

<style lang="less" scoped>

</style>