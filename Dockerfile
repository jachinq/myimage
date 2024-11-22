
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM alpine:latest
#RUN addgroup -g 1000 myapp
#RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp
WORKDIR /home/myapp/
RUN mkdir data
COPY ./app .
#RUN chown myapp:myapp myimage
#USER myapp
CMD ["./myimage"]