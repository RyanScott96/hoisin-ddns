FROM alpine:latest AS build
WORKDIR /src/hoisin/
COPY src/ /src/hoisin/src/
COPY Cargo.toml .
COPY Cargo.lock .
COPY alpine/cargo-build.sh .
RUN chmod +x /src/hoisin/cargo-build.sh
RUN /src/hoisin/cargo-build.sh

FROM alpine:latest
COPY --from=build /src/hoisin/target/release/hoisin-ddns /usr/local/bin/hoisin-ddns
RUN mkdir -p /etc/hoisin/
RUN mkdir -p /var/log/hoisin/
COPY domains.toml /etc/hoisin/domains.toml
COPY alpine/hoisin-update-ddns.sh /etc/periodic/15min/hoisin-update-ddns.sh
RUN chmod +x /etc/periodic/15min/hoisin-update-ddns.sh
COPY alpine/run.sh /usr/local/bin/run.sh
RUN chmod +x /usr/local/bin/run.sh
ENTRYPOINT [ "run.sh" ]
CMD ["hoisin-ddns", "-c", "/etc/hoisin/domains.toml", "ping"]