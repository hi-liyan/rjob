# rjob - A Simple Job Scheduler By Rust

rjob 是一个简单的定时任务调度程序，它支持执行 HTTP 请求任务。你可以使用它来根据指定的计划定期发送 HTTP 请求，执行各种后台任务。

## 快速开始

### 1. 添加任务定义文件

在可执行程序所在的目录中添加任务定义文件。任务定义文件可以是 JSON 或 YAML 格式。请确保在该目录中只有一个任务定义文件存在。

**jobs.json**:
```json
{
   "timezone": "Asia/Tokyo",
   "http_jobs": [
      {
         "enable": true,
         "name": "users2",
         "cron": "*/5 * * * * ?",
         "timeout": 3000,
         "max_retry": 5,
         "request": {
            "url": "https://reqres.in/api/users/2",
            "method": "GET"
         }
      },
      {
         "enable": false,
         "name": "login",
         "cron": "*/10 * * * * ?",
         "request": {
            "url": "https://reqres.in/api/login",
            "method": "POST",
            "body": {
               "email": "eve.holt@reqres.in",
               "password": "cityslicka"
            }
         }
      }
   ]
}
```
**jobs.yaml**:  
```yaml
http_jobs:
  - enable: true
    name: users2
    cron: '*/5 * * * * ?'
    request:
      url: https://reqres.in/api/users/2
      method: GET
  - enable: true
    name: login
    cron: '*/10 * * * * ?'
    request:
      url: https://reqres.in/api/login
      method: POST
      body:
        email: eve.holt@reqres.in
        password: cityslicka
```

#### 配置文件说明

该配置文件用于定义基于指定计划的定时执行的 HTTP 请求任务。配置文件包含以下部分：

1. timezone：指定任务计划所使用的时区。值应为有效的时区标识符，例如 "Asia/Tokyo"。如果未指定时区，则将使用默认值 UTC。
2. http_jobs：表示要执行的 HTTP 任务的数组。每个任务由一组属性定义：
    - enable：指定任务是否启用。如果未指定，则默认为 true。
    - name：（必须）指定任务名称。该名称将用于在日志中标识任务。
    - cron：（必须）指定任务执行的计划。值应为有效的 cron 表达式。cron 表达式的格式为：`秒 分 时 日 月 周 年`。例如：`0 0 12 * * ?` 表示每天中午 12 点执行任务。
    - timeout：指定任务执行的超时时间，单位为：毫秒。如果未指定，则默认值为 5000。
    - max_retry：指定当HTTP请求失败时的最大重试次数。如果未指定，则默认值为 3。
    - request：（必须）指定 HTTP 请求的相关属性：
        - url：（必须）指定 HTTP 请求的 URL。
        - method：指定 HTTP 请求的方法。有效值为 GET、POST、PUT、DELETE、HEAD、OPTIONS、PATCH。如果未指定，则默认值为 GET。
        - headers：指定 HTTP 请求的头部信息。值应为 JSON 格式。例如：`{"Content-Type": "application/json"}`。如果是 YAML 格式的配置文件，则应使用以下格式：
        ```yaml
        headers:
          Content-Type: application/json
        ```
        - body：指定 HTTP 请求的请求体。值应为 JSON 格式。例如：`{"name": "rjob", "version": "1.0.0"}`。如果是 YAML 格式的配置文件，则应使用以下格式：
        ```yaml
        body:
          name: rjob
          version: 1.0.0
        ```


### 2. Linux 环境执行程序

```bash
./rjob
```

### 3. Linux nohup 命令执行程序

```bash
# 在后台运行程序，程序日志会被输出到nohup.out文件中
nohup ./rjob &

# 查看进程
ps -aux | grep rjob 

# 结束进程
kill -9  进程号PID
```
### 4. 日志输出

任务执行时会打印日志，可以通过每条日志开头的UUID跟踪任务执行情况。日志输出示例如下：

```bash
4cd4a467890646c9ac96cc15d3ad3ab9 2023-05-27 12:01:20.003 Http job start, job name: login
4cd4a467890646c9ac96cc15d3ad3ab9 2023-05-27 12:01:20.003 Job: [name: login, enable: true, cron: */10 * * * * ?, request: [url: https://reqres.in/api/login, method: POST, headers: None, body: {"email":"eve.holt@reqres.in","password":"cityslicka"}]]
5f00ef4403324c2692caddb51315f6ee 2023-05-27 12:01:20.003 Http job start, job name: users2
5f00ef4403324c2692caddb51315f6ee 2023-05-27 12:01:20.003 Job: [name: users2, enable: true, cron: */5 * * * * ?, request: [url: https://reqres.in/api/users/2, method: GET, headers: None, body: None]]
5f00ef4403324c2692caddb51315f6ee 2023-05-27 12:01:20.003 Http request success, job name: users2
5f00ef4403324c2692caddb51315f6ee 2023-05-27 12:01:20.003 Http response: {"data":{"id":2,"email":"janet.weaver@reqres.in","first_name":"Janet","last_name":"Weaver","avatar":"https://reqres.in/img/faces/2-image.jpg"},"support":{"url":"https://reqres.in/#support-heading","text":"To keep ReqRes free, contributions towards server costs are appreciated!"}}
5f00ef4403324c2692caddb51315f6ee 2023-05-27 12:01:20.003 Http job end, job name: users2
4cd4a467890646c9ac96cc15d3ad3ab9 2023-05-27 12:01:20.003 Http request success, job name: login
4cd4a467890646c9ac96cc15d3ad3ab9 2023-05-27 12:01:20.003 Http response: {"token":"QpwL5tke4Pnpja7X4"}
4cd4a467890646c9ac96cc15d3ad3ab9 2023-05-27 12:01:20.003 Http job end, job name: login
```

## 编译

### 编译x86_64 Linux可执行程序。

1. 在项目中添加`.cargo`目录，添加`config.toml`文件，内容如下：
    ```toml
    [target.x86_64-unknown-linux-musl]
    linker = "x86_64-linux-musl-gcc"
    ```

2. 添加编译目标`x86_64-unknown-linux-musl`。
    ```bash
    rustup target add x86_64-unknown-linux-musl
    ```

3. 查看编译目标列表
    ```bash
    rustup target list
    ```

4. 编译
    ```bash
    cargo build --release --target=x86_64-unknown-linux-musl
    ```

### 编译ARM Linux可执行程序

1. 在项目中添加`.cargo`目录，添加`config.toml`文件，内容如下：
    ```toml
    [target.aarch64-unknown-linux-musl]
    linker = "aarch64-linux-musl-ld"
    ```

2. 添加编译目标`aarch64-unknown-linux-musl`。
    ```bash
    rustup target add aarch64-unknown-linux-musl
    ```

3. 查看编译目标列表
    ```bash
    rustup target list
    ```
4. 编译
    ```bash
    cargo build --release --target=aarch64-unknown-linux-musl
    ```

### 遇到的问题
1. MacOS环境交叉编译，需要安装 `musl-cross`。
   ```bash
   brew install filosottile/musl-cross/musl-cross
   ```
2. 在Debian/Ubuntu环境需要安装`build-essential`。
   ```bash
   sudo apt install build-essential
   ```
3. 在Debian/Ubuntu环境编译时报`error: failed to run custom build command for 'ring v0.16.20'`错误时，安装 `musl-tools`。
   ```bash
    sudo apt install musl-tools
   ```