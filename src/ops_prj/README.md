# ops_prj 模块说明

`src/ops_prj/` 对应 `gops prj` 这一组能力。

## 目标

项目层负责把一个 `System` 导入到具体客户或环境项目中，形成可持续维护的交付对象。

它解决的是：

- 运维项目如何初始化
- 系统如何导入到项目
- 项目如何更新本地引用

## 当前目录

```text
src/ops_prj/
├── conf.rs
├── import.rs
├── init.rs
├── install.rs
├── path.rs
├── project.rs
├── system.rs
└── README.md
```

## 主要文件

### `project.rs`

运维项目对象入口，围绕：

- `ops-prj.yml`
- `ops-systems.yml`

等项目级文件工作。

### `import.rs`

系统导入逻辑，对应 `gops prj import`。

### `init.rs`

项目初始化模板，对应 `gops prj new`。

### `path.rs`

项目路径组织，负责定位：

- `ops-prj.yml`
- `ops-systems.yml`

### `system.rs`

项目内系统引用相关逻辑。

## 与 CLI 的对应

```text
gops prj new
gops prj import
gops prj update
```

## 输出对象

当前项目层默认生成的对象结构类似：

```text
<ops-project-root>/
├── ops-prj.yml
├── ops-systems.yml
├── version.txt
└── _gal/
```

## 关系

项目层是交付对象层：

```text
module -> system -> ops_prj
```

其中：

- `module` 提供可复用能力单元
- `system` 提供共享系统定义
- `ops_prj` 提供客户或环境级落地对象
