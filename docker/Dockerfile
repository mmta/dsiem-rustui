FROM defenxor/dsiem:latest

# remove files but keep directories (e.g. assets)
RUN find ./dsiem/web/dist -maxdepth 1 -type f -print0 | xargs -0 rm

# add dsiem rust-ui files
COPY ./dist/* /dsiem/web/dist/

# keep everything else the same as before