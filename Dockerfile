FROM rustlang/rust:nightly AS build
COPY . ./
RUN cargo build --release

FROM scratch
COPY --from=build ./target/release/simple-hyper-router /
ENV PORT 3000
EXPOSE ${PORT}
CMD ["/simple-hyper-router"]