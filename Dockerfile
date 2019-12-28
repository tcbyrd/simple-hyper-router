FROM rustlang/rust:nightly as builder
WORKDIR /data/simple-hyper-router
COPY . .
# Install the linux-musl target
RUN ["rustup", "target", "add", "x86_64-unknown-linux-musl"]
RUN ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl"]
RUN ["cp", "./target/x86_64-unknown-linux-musl/release/simple-hyper-router", "/bin/"]
RUN ["chmod", "+x", "/bin/simple-hyper-router"]

FROM scratch
COPY --from=builder /bin/simple-hyper-router .
EXPOSE 3000
CMD ["/simple-hyper-router"]