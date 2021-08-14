FROM centos:7

RUN yum -y update &&\
    yum -y install curl cmake &&\
    yum -y group install "Development Tools"

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source $HOME/.cargo/env
