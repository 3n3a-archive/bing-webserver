FROM scratch
COPY bing-webserver /
EXPOSE 7878
CMD ["/bing-webserver"]
