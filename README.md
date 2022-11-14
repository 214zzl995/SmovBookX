# SmovBook

文件整理流程 -> 获取文件元数据 -> 判断是否为单文件 判断标准为目录下是否存在其他视频文件 -> 更新文件名为name -> 更新目录下可能存在的文件和字幕文件为name -> 迁移文件夹位置 

当为单文件时 新建一个名为name的文件夹

在网络检索时 当取到这个视频确实有检索到的数据 就要对文件位置进行处理了

需要增加代理池功能，通过代理访问

FFmpeg concat 分离器

这种方法成功率很高，也是最好的，但是需要 FFmpeg 1.1 以上版本。先创建一个文本文件 filelist.txt：

file 'input1.mkv'

file 'input2.mkv'

file 'input3.mkv'

然后：

ffmpeg -f concat -i filelist.txt -c copy output.mkv

注意：使用 FFmpeg concat 分离器时，如果文件名有奇怪的字符，要在 filelist.txt 中转义

看来需要对大量错误的option写法做返回判断了。。。。

下阶段计划：

1.SmovItem 

2.SmovView

3.增加不存在的文件自动删除功能

4.增加回收站删除功能

5.合并视频文件提示

6.自动更新功能 codeing提供 下载软件 服务器提供返回文本

晚上更新一个切换关闭列表 切换为可用的 功能 0.1.6版本

打包方式更改为签名 

全局变量修改为 env-cmd

npm install -g env-cmd 安装

删除数据时需要同时删除检索队列的数据

周末写 设置界面

发现一个问题 就是 检索文件夹的时候 因为 整理文件夹不在检索范围内 所以会导致把已经检索的文件 判断为未检索 所以当我在新增检索文件夹时应该把 默认的整理文件夹给加进去

hfs功能 已完成

1. 配置文件可自定义 路径为Roaming\smovbook
   1) app运行时 直接缓存一个config 这个config模板可以直接从 rocket 做一个默认的 当文件不存在时 新增一个默认的config 然后存储到这个目录下 ？是否可以直接使用config 这个类 build 还是只能用 配置文件 是否会造成性能的损耗 
2. 状态返回
   1) 在运行时需要对文件服务器做一个多次返回状态 当成功时 应该直接给一个启动成功的状态返回
   2) 在运行失败时 直接返回错误 （已完成）
3. 对hfs文件服务器状态缓存
   1) 缓存到哪里 应该缓存到app 因为当用户使用时hfs文件服务器不一定已经启动了 ？当文件服务器未启动 我是否能得到一个config 如何能得到 我能否做到自由修改 和重启
   2) 服务器启动状态的缓存 ？如何获得hfs的启动状态 是用lazy去缓存一个状态还是说 我能直接得到这个线程的运行状态 或是是否存在 ？
4. hfs文件服务器重启
   1) rocket的关闭在默认有一个 Shutdown 的方法 或许可以通过http的方式关闭？ 是否可以通过 commond的方式调用
5. hfs文件服务器的性能问题
   1) 文件服务器是暂停比较好 还是每次新增一个线程比较好 ？如何删除这个线程 删除线程是否会导致当前正在使用的资源出现问题？ 

  当我请求后台启动服务器时 这个任务貌似会一直处于一个阻塞的状态 应为他应当返回一个结果

  文件服务器的返回 应该都是要做多次返回的类型！！ 不能用单次返回 会有问题 可能会有侦测不到的问题

  是否需要两个文件？ 好像只设置一个文件就可以了 反正可以自定义

桌面检索悬浮球功能
1）当seek界面切换为悬浮球时 需要有动画显示  完成
2）悬浮球需要尽量不影响到当前正在进行的事务   完成
3）悬浮球应该要有右键菜单   想要一个 右键菜单只能新建一个窗口来实现 暂时先不做 因为很麻烦
4）loading动画存在时 另一个翻转的动画不会出现 完成

检索后自动刷新界面功能
1）如何保证keep_alive存在又尽量能做到无感刷新

详情超链接调整
1）点击详情超链接会回到主页，并自动设置过滤条件 或是重新开一个页面也可以

主页过滤
1）还没想好怎么设计这个东西

seek界面优化
1）修改返回为消息 不应该一个线程返回一次 虽然没啥问题 但是我不爽
2）错误显示设置 
3）检索过滤，成功或者失败
4）单独检索
5）错误重新检索 自定义重试次数

文件检索界面优化
1）增加数据增减的接口 无需刷新来重新获取数据

视频文件在线播放
1）未想好怎么实现 是内嵌一个播放器 还是按照当前的方法 其实这样挺不错来着

本地文件标记 自定义标签等
1）还有好多东西 根本做不完 我离职在家一直做差不多 烦死了

SmovBookM优化
1）添加近期扫描快速切换
2）优化列表的加载 防止卡顿
3）请求数据时的错误处理
4）扫码界面优化
5）软件内播放器需要给一个窗口

界面优化 各种fontsize 修改为rem 或em单位

推送测试测试

BUG
当出现两个相同名称 但是不同类型的文件时 这条数据会因为名字相同不插入

Item对象显示优化 根据图片大小设置宽度 高度固定

数据库版本升级功能 升级版本检测sqlite版本

 环境变量需要配置 PKG_CONFIG_PATH
 gst = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer" } 
 gst-app = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-app"} 
 gst-rtp = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-rtp"} 
 gst-sdp = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-sdp" } 
 gst-utils = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-utils"} 
 gst-video = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-video" } 
 gst-webrtc = {git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs", package = "gstreamer-webrtc" }
 gst = { package = "gstreamer", git = "https://gitlab.freedesktop.org/gstreamer/gstreamer-rs" }

# 任务系统！！！ 重要 要做

  负责所有任务的进度 包括检索 转码 
  状态 未开始 开始 进度 完成 失败 暂停

待定使用的框架 async-task  tokio(当前最佳实现 参考runtime task tokio::task::JoinSet)

rust闭包 https://blog.csdn.net/quicmous/article/details/123292918 
全局状态(不推荐) https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

#任务队列需要实现的功能

1. 将需要执行的数据 实体化
2. 固定池，能够做到一种类型只生成 配置的数量
3. 能做到暂停
4. 所有回调在任务中实现 
5. 要在任务中 传入池子的appheader，否则无法回传 进度以及状态
6. 不固定线程数 便于扩展线程数量
7. 任务中需要能对pool中 当前不同类型运行数量做处理 
8. 传入任务使用commond实现
9. 需要有vec格式的传入 减少io

当前真正需要解决的问题是 5 7
commond传入任务 将数据插入 task队列 
将每一个 任务封装成一个 async 然后通过 channel 或 tokio的unbounded_channel 来发送 开始消息和结束消息 
AppHandle 在app外初始化 然后在setup 装载 最后 塞入manage

# pool参考

https://blog.csdn.net/m0_68007835/article/details/123956420

# type impl 参考

https://stackoverflow.com/questions/35568871/is-it-possible-to-implement-methods-on-type-aliases 

# taskpool重构

- [x] taskpool 多类型树
- [x] 后台线程池实现
- [x] 获取后台任务
- [x] 线程池优雅停机
- [x] 状态回传
- [x] 进度回传
- [x] 前台进度显示
- [ ] 工作任务缓存
- [ ] 任务日志缓存
- [ ] 开始界面初始化
- [ ] 检索系统 检索的性能需求较高 
- [ ] 任务删除
- [ ] 悬浮球 悬浮球将会作为一个单独的界面显示
- [ ] 当关机时 需要将所有正在运行的任务修改为失败


# taskpool界面
# 问题
1. 如何处理 不同类型在界面上的显示呢 是全部放到一个界面上还是 
2. 假设分两个界面显示 如何在两个界面滚动 


# 程序更新 及 初始化
- [x] 数据库更新模块 
- [ ] 文件系统更新模块

# 程序更新 及 初始化 更新执行程序的问题
1. 在不同版本 更新到这个版本时 如何处理中间部分的逻辑 是否可以考虑sql的方式 不断迭代当前文件库的版本 每次执行完导出一个 当前的版本 当当前版本还没有到目标版本时继续进行下一步执行 那么问题来了 如何动态调用这个方法 可以使用hook？
https://stackoverflow.com/questions/32885446/dynamically-select-a-function-to-call-without-intermediate-variables




