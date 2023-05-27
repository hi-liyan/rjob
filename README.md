# rjob - A Simple Job Scheduler By Rust

一个简单的定时任务调度程序，支持HTTP请求任务。

## 快速开始

1. 添加**任务定义**文件

    在可执行程序所在目录中添加任务定义文件，文件支持`json`/`yaml`两种类型。任务定义示例如下：
    
    **jobs.json**:
    ```json
    {
      "http_jobs": [
        {
          "enable": true,
          "name": "users2",
          "cron": "*/5 * * * * ?",
          "request": {
            "url": "https://reqres.in/api/users/2",
            "method": "GET"
          }
        },
        {
          "enable": true,
          "name": "login",
          "cron": "*/10 * * * * ?",
          "request": {
            "url": "https://reqres.in/api/login",
            "method": "POST",
            "headers": {},
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
    
    注意：
    1. 目录中只能有一个任务定义文件。
    2. `request`中的`headers`和`body`是可选参数。  

2. Linux 环境执行程序
    ```bash
    ./rjob
    ```
3. Linux nohup 命令执行程序
    ```bash
    # 在后台运行程序，程序日志会被输出到nohup.out文件中
    nohup ./rjob &
    
    # 查看进程
    ps -aux | grep rjob 
    
    # 结束进程
    kill -9  进程号PID
    ```
4. 任务执行时会打印日志，可以通过每条日志开头的UUID跟踪任务执行情况。
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