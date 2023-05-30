FROM amazonlinux:latest

WORKDIR /home

COPY target/x86_64-unknown-linux-musl/release/rjob /home/rjob
COPY jobs.json /home/jobs.json

RUN chmod +x /home/rjob

CMD ["/home/rjob"]