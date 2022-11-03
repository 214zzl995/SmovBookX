export type TaskAsk = {
    id: Number,
    name: String
}

export type TaskEvent = {
    event_type: TaskType,
    ask: TaskAsk,
    status: TaskStatus,
    
}

export enum TaskType {
    Crawler ="Crawler",
    Convert = "Convert",
}

export enum TaskStatus {
    Wait="Wait",
    Running ="Running",
    Fail ="Fail",
    Success ="Success",
}

export interface TaskProps {
    uuid?: string
    task?: TaskAsk
}