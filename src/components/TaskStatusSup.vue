<template>
    <div class="TaskStatus" :style="'background-color:' + color + ';'">
    </div>
</template>

<script lang="ts" setup>
import { PropType } from 'vue';
import { TaskStatus } from '@/ts/Task'

const props = defineProps({
    TaskStatus: {
        type: String as PropType<TaskStatus>,
        default: "Wait"
    },
})

const color = computed(() => {
    if (props.TaskStatus == TaskStatus.Fail) {
        return "#c45656"
    } else if (props.TaskStatus == TaskStatus.Running) {
        return "#337ecc"
    } else if (props.TaskStatus == TaskStatus.Success) {
        return "#529b2e"
    } else if (props.TaskStatus == TaskStatus.Wait) {
        return "#73767a"
    }
})

</script>

<style lang="less" scoped>
.TaskStatus {
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    margin-right: 0.7rem;
}
</style>