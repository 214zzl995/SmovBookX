export type Task = {
    id:Number,
    name:String
}

export interface TaskProps {
    uuid?: string
    task?: Task
}