FROM ubuntu:18.04
RUN mkdir /code
#RUN sed -i 's/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list
RUN sed -i 's/archive.ubuntu.com/mirrors.ustc.edu.cn/g' /etc/apt/sources.list
RUN apt-get clean && \
    apt-get update && \
    apt-get install -y --no-install-recommends g++ gcc openjdk-11-jdk python python3 clang rustc &&\
    rm -rf /var/lib/apt/lists/*

VOLUME ["/data"]
