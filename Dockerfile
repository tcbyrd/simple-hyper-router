FROM rustlang/rust:nightly
WORKDIR /data/simple-hyper-router
COPY . .
RUN ["cargo", "build", "--release"]
RUN ["cp", "./target/release/simple-hyper-router", "/bin/"]
RUN ["chmod", "+x", "/bin/simple-hyper-router"]
EXPOSE 3000
CMD ["/bin/simple-hyper-router"]