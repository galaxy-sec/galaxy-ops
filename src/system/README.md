# system 模块说明

`src/system/` 对应 `gops sys` 这一组能力。

## 目标

系统层负责把多个模块组织成一个可操作、可本地化、可交付的系统对象。

它解决的是：

- 系统骨架如何初始化
- 系统模型如何维护
- 模块列表如何维护
- 系统设置如何初始化
- 系统级下载、安装、启动、停止、状态、诊断如何组织

## 当前目录

```text
src/system/
├── conf.rs
├── init.rs
├── mod_list.rs
├── operator.rs
├── path.rs
├── refs.rs
├── spec.rs
├── setting/
│   ├── export.rs
│   ├── localize.rs
│   ├── mod.rs
│   ├── sys.rs
│   └── templatize.rs
└── README.md
```

## 主要文件

### `operator.rs`

系统对象入口。`gops sys new`、`update`、`localize` 以及各类系统操作命令都会落到这里。

### `spec.rs`

系统定义和初始化模板相关逻辑。

### `mod_list.rs`

系统模块列表相关逻辑。

### `path.rs`

系统路径组织，负责定位：

- `sys-prj.yml`
- `sys/sys_model.yml`
- `sys/mod_list.yml`
- `sys/setting/...`

### `setting/`

系统设置、本地化和模板化相关逻辑。

## 与 CLI 的对应

```text
gops sys new
gops sys update
gops sys localize
gops sys setting
gops sys download/install/uninstall/start/stop/status/diagnose
```

## 输出对象

当前系统层最终管理的是这种对象结构：

```text
<system-root>/
├── sys-prj.yml
├── _gal/
└── sys/
    ├── sys_model.yml
    ├── mod_list.yml
    ├── setting/
    └── workflows/
```

## 关系

系统是模块之上的组合层，也是运维项目导入的来源对象：

```text
module -> system -> ops_prj
```
