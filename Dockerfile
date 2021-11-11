FROM amazonlinux:2

RUN mkdir -p /build/src && \
    yum update -y && \
    yum install -y awscli gcc openssl-devel tree zip && \
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

WORKDIR /build
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin

CMD \
  cargo build --release --target-dir target_lambda && \
  size target_lambda/release/bootstrap && \
  ldd  target_lambda/release/bootstrap && \
  zip -9 -j target_lambda/deploy.zip target_lambda/release/bootstrap